use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

pub fn interactive_z_viewer(
    blocks: &[i32],
    width: usize,
    height: usize,
    depth: usize,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut z = 0;
    let mut y_offset = 0;

    // Define the maximum rows to display at once
    let viewport_height = 32.min(height);

    terminal::enable_raw_mode()?;

    let result = (|| -> io::Result<()> {
        loop {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;

            // Render the selected slice with the Y viewport
            print_z_slice(blocks, width, height, depth, z, y_offset, viewport_height);

            print!("\r\n");
            print!(
                "Z slice: {}/{} | Y range: {}..{}\r\n",
                z,
                depth - 1,
                y_offset,
                y_offset + viewport_height - 1
            );
            print!(
                "[← / A] Prev Z  [→ / D] Next Z  [↑ / W] Up Y  [↓ / S] Down Y  [Q / Esc] Quit\r\n"
            );

            stdout.flush()?;

            // Read a single key press.
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                let max_y_offset = height.saturating_sub(viewport_height);

                match key.code {
                    KeyCode::Right | KeyCode::Char('d') => {
                        z = (z + 1).min(depth - 1);
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        z = z.saturating_sub(1);
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        y_offset = (y_offset + 1).min(max_y_offset);
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        y_offset = y_offset.saturating_sub(1);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
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

fn print_z_slice(
    blocks: &[i32],
    width: usize,
    height: usize,
    depth: usize,
    slice_z: usize,
    y_offset: usize,
    viewport_height: usize,
) {
    assert_eq!(blocks.len(), width * height * depth);
    assert!(slice_z < depth);

    // ANSI colors
    const RESET: &str = "\x1b[0m";
    const SOLID: &str = "\x1b[38;5;245m";
    const SPECIAL: &str = "\x1b[38;5;214m";
    const AIR: &str = "\x1b[38;5;255m";
    const WATER: &str = "\x1b[38;5;33m";
    const LAVA: &str = "\x1b[38;5;196m";

    print!("\r\nZ SLICE: z = {slice_z}\r\n");
    print!("X →\r\n");
    print!(
        "    {}\r\n",
        (0..width)
            .step_by(4)
            .map(|x| format!("{:>4}", x))
            .collect::<String>()
    );

    // Render only the rows within the Y viewport (from top to bottom)
    for y in (y_offset..y_offset + viewport_height).rev() {
        if y % 8 == 0 {
            print!("{:>4} ", (y as i32) - 64);
        } else {
            print!("     ");
        }

        for x in 0..width {
            let index = (slice_z * height + y) * width + x;
            let block = blocks[index];
            let block_type = block >> 1;

            let (color, symbol) = match block_type {
                0 => (SOLID, "██"),
                1 => (SPECIAL, "▓▓"),
                2 => (AIR, "  "),
                3 => (WATER, "≈≈"),
                4 => (LAVA, "~~"),
                n => {
                    println!("Unknown block type: {}", n);
                    (RESET, "??")
                }
            };

            print!("{color}{symbol}{RESET}");
        }

        print!("\r\n");
    }
}
