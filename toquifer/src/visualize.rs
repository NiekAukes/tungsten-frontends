use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::collections::HashMap;
use std::io::{self, Write};

const CHUNK_SIZE: i32 = 16;
const WORLD_HEIGHT: i32 = 384;
const VIEW_X: i32 = 16;
const VIEW_Z: i32 = 16;
const VIEW_Y: i32 = 32;

/// Lazily loads and caches chunks (laid out as `(z * 384 + y) * 16 + x`).
struct World<F> {
    load: F,
    chunks: HashMap<(i32, i32), Vec<i32>>,
}

impl<F: FnMut(i32, i32) -> Vec<i32>> World<F> {
    fn block(&mut self, x: i32, y: i32, z: i32) -> i32 {
        let key = (x.div_euclid(CHUNK_SIZE), z.div_euclid(CHUNK_SIZE));
        let load = &mut self.load;
        let chunk = self.chunks.entry(key).or_insert_with(|| load(key.0, key.1));
        let (lx, lz) = (x.rem_euclid(CHUNK_SIZE), z.rem_euclid(CHUNK_SIZE));
        chunk[((lz * WORLD_HEIGHT + y) * CHUNK_SIZE + lx) as usize]
    }
}

/// Interactive viewer over an unbounded world; `load_chunk(chunk_x, chunk_z)` is called on demand.
pub fn interactive_viewer(load_chunk: impl FnMut(i32, i32) -> Vec<i32>) -> io::Result<()> {
    let mut world = World {
        load: load_chunk,
        chunks: HashMap::new(),
    };
    let mut stdout = io::stdout();
    let (mut x, mut slice_z, mut top_z, mut y) = (0, 0, 0, 0);
    let mut top_down = false;

    terminal::enable_raw_mode()?;

    let result = (|| -> io::Result<()> {
        loop {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;

            print_x_header(x);
            if top_down {
                print_top_down(&mut world, x, top_z);
                print!(
                    "\r\nTop-down | X: {}..{} Z: {}..{} | chunk ({}, {})\r\n",
                    x,
                    x + VIEW_X - 1,
                    top_z,
                    top_z + VIEW_Z - 1,
                    x.div_euclid(CHUNK_SIZE),
                    top_z.div_euclid(CHUNK_SIZE)
                );
                print!("[Q/E] X-/X+  [←→/A/D] X-/X+  [↑↓/W/S] Z+/Z-  [Shift] x16  [T] Slice view  [Esc] Quit\r\n");
            } else {
                print_z_slice(&mut world, x, slice_z, y);
                print!(
                    "\r\nZ slice: {} | X: {}..{} Y: {}..{} | chunk ({}, {})\r\n",
                    slice_z,
                    x,
                    x + VIEW_X - 1,
                    y,
                    y + VIEW_Y - 1,
                    x.div_euclid(CHUNK_SIZE),
                    slice_z.div_euclid(CHUNK_SIZE)
                );
                print!("[Q/E] X-/X+  [←→/A/D] Z-/Z+  [↑↓/W/S] Y+/Y-  [Shift] x16  [T] Top-down  [Esc] Quit\r\n");
            }
            stdout.flush()?;

            let Event::Key(key) = event::read()? else {
                continue;
            };
            if key.kind != KeyEventKind::Press {
                continue;
            }

            let (code, shift) = match key.code {
                KeyCode::Char(c) => (
                    KeyCode::Char(c.to_ascii_lowercase()),
                    key.modifiers.contains(KeyModifiers::SHIFT) || c.is_ascii_uppercase(),
                ),
                other => (other, key.modifiers.contains(KeyModifiers::SHIFT)),
            };
            // Raw mode swallows SIGINT, so handle Ctrl-C explicitly.
            if key.modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                break;
            }
            let step = if shift { CHUNK_SIZE } else { 1 };

            match code {
                KeyCode::Char('q') => x -= step,
                KeyCode::Char('e') => x += step,
                KeyCode::Char('t') => top_down = !top_down,
                KeyCode::Esc => break,
                KeyCode::Right | KeyCode::Char('d') if top_down => x += step,
                KeyCode::Left | KeyCode::Char('a') if top_down => x -= step,
                KeyCode::Up | KeyCode::Char('w') if top_down => top_z += step,
                KeyCode::Down | KeyCode::Char('s') if top_down => top_z -= step,
                KeyCode::Right | KeyCode::Char('d') => slice_z += step,
                KeyCode::Left | KeyCode::Char('a') => slice_z -= step,
                KeyCode::Up | KeyCode::Char('w') => y = (y + step).min(WORLD_HEIGHT - VIEW_Y),
                KeyCode::Down | KeyCode::Char('s') => y = (y - step).max(0),
                _ => {}
            }
        }
        Ok(())
    })();

    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    result
}

fn print_x_header(x_offset: i32) {
    print!("\r\nX →\r\n     ");
    for x in (x_offset..x_offset + VIEW_X).step_by(4) {
        print!("{:<8}", x);
    }
    print!("\r\n");
}

fn print_z_slice<F: FnMut(i32, i32) -> Vec<i32>>(
    world: &mut World<F>,
    x_offset: i32,
    z: i32,
    y_offset: i32,
) {
    for y in (y_offset..y_offset + VIEW_Y).rev() {
        if y % 8 == 0 {
            print!("{:>4} ", y - 64);
        } else {
            print!("     ");
        }
        for x in x_offset..x_offset + VIEW_X {
            print_block(Some(world.block(x, y, z)));
        }
        print!("\r\n");
    }
}

fn print_top_down<F: FnMut(i32, i32) -> Vec<i32>>(
    world: &mut World<F>,
    x_offset: i32,
    z_offset: i32,
) {
    for z in (z_offset..z_offset + VIEW_Z).rev() {
        print!("{:>4} ", z);
        for x in x_offset..x_offset + VIEW_X {
            let top = (0..WORLD_HEIGHT)
                .rev()
                .map(|y| world.block(x, y, z))
                .find(|block| block >> 1 != 2);
            print_block(top);
        }
        print!("\r\n");
    }
}

fn print_block(block: Option<i32>) {
    const RESET: &str = "\x1b[0m";
    const SOLID: &str = "\x1b[38;5;245m";
    const SPECIAL: &str = "\x1b[38;5;214m";
    const AIR: &str = "\x1b[38;5;255m";
    const WATER: &str = "\x1b[38;5;33m";
    const LAVA: &str = "\x1b[38;5;196m";

    let (color, symbol) = match block.map(|b| b >> 1) {
        Some(0) => (SOLID, "██"),
        Some(1) => (SPECIAL, "▓▓"),
        Some(2) | None => (AIR, "  "),
        Some(3) => (WATER, "≈≈"),
        Some(4) => (LAVA, "~~"),
        Some(_) => (RESET, "??"),
    };
    print!("{color}{symbol}{RESET}");
}
