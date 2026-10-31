//! In-memory compilation: feed worldgen files by id, pick a noise-settings root, get Java source.
//!
//! This is the path the mod uses (through `tungsten-mc-compile-jni`). Unlike [`crate::run_generation`]
//! it bundles nothing, reads and writes no files, and runs only the Java backend.

use bumpalo::Bump;
use serde::Serialize;

use crate::compile::compile_java;
use crate::config_load::MinecraftDataRaw;
use crate::contract;
use crate::error::{normalize_id, CompileError, ResourceKind};
use crate::parse::MinecraftData;
use crate::transform_spmt::Transformer;

/// The transforms recurse deeply; this is the stack they get regardless of the caller's.
const COMPILE_STACK_SIZE: usize = 16 * 1024 * 1024;

/// Minecraft's chunk width, the horizontal size of the generated routes.
const CHUNK_SIZE: usize = 16;

/// The crate version, reported to the mod alongside its output.
pub fn compiler_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Generated Java and what the mod needs to load and check it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JavaOutput {
    /// Compilation units, to be compiled together.
    pub units: Vec<JavaUnit>,
    /// Density functions and noises reachable from the root, sorted by kind and id.
    pub reachable: Vec<ReachableId>,
    pub layout: Layout,
    /// Non-fatal backend diagnostics, e.g. methods close to the JVM's size limits.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JavaUnit {
    /// Fully qualified name of the unit's top-level class.
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ReachableId {
    pub kind: ResourceKind,
    /// Registry id, e.g. `minecraft:overworld/continents`.
    pub id: String,
}

/// The vertical extent of the compiled noise settings, which sets the y size of every route except
/// `preliminary_surface_level`. Everything else about the generated code is fixed, see
/// [`crate::contract`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Layout {
    pub min_y: i32,
    pub height: i32,
}

/// Runs `f` on a thread with [`COMPILE_STACK_SIZE`] of stack and resumes its panic, if any, here.
fn with_compile_stack<R: Send>(f: impl FnOnce() -> R + Send) -> Result<R, CompileError> {
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("tungsten-compile".into())
            .stack_size(COMPILE_STACK_SIZE)
            .spawn_scoped(scope, f)
            .map_err(|e| CompileError::Internal(format!("could not start compile thread: {}", e)))?;
        Ok(handle.join().unwrap_or_else(|panic| std::panic::resume_unwind(panic)))
    })
}

/// One compilation: feed resources, set the root, compile.
#[derive(Debug)]
pub struct CompilerSession {
    data: MinecraftDataRaw,
    root: Option<String>,
}

impl Default for CompilerSession {
    fn default() -> Self {
        Self::new()
    }
}

impl CompilerSession {
    pub fn new() -> Self {
        CompilerSession {
            data: MinecraftDataRaw::new(),
            root: None,
        }
    }

    /// Feeds one file. `id` is its resource path, e.g.
    /// `minecraft:worldgen/density_function/overworld/continents.json`; a later file with the same
    /// id replaces an earlier one. Returns false for files the compiler doesn't use (anything but
    /// `worldgen/noise_settings/`, `worldgen/density_function/` and `worldgen/noise/`).
    pub fn load_resource(&mut self, id: &str, bytes: &[u8]) -> bool {
        self.data.insert_resource(id, bytes)
    }

    /// The noise settings to compile, e.g. `minecraft:overworld` or `minecraft:amplified`.
    pub fn set_root(&mut self, id: &str) {
        self.root = Some(normalize_id(id));
    }

    /// Compiles the root on a thread with a large stack. A panic in the compiler is resumed on the
    /// calling thread.
    pub fn compile(&self) -> Result<JavaOutput, CompileError> {
        let Some(root) = &self.root else {
            return Err(CompileError::Internal("compile called before set_root".into()));
        };
        with_compile_stack(|| self.compile_root(root))?
    }

    fn compile_root(&self, root: &str) -> Result<JavaOutput, CompileError> {
        let arena = Bump::with_capacity(1024 * 1024);
        let mut mcdata = MinecraftData::new(&arena, self.data.clone(), CHUNK_SIZE);
        mcdata.parse_root(root)?;
        let reachable = mcdata
            .reachable()
            .into_iter()
            .map(|(kind, id)| ReachableId { kind, id })
            .collect();
        let noise_generator = mcdata.root().expect("root was just parsed");
        let layout = Layout { min_y: noise_generator.noise.min_y, height: noise_generator.noise.height };

        let transform_arena = Bump::with_capacity(1024 * 1024);
        let program = Transformer::new(&transform_arena).transform(noise_generator);
        let java = compile_java(&program)?;

        contract::check(&java.source).map_err(CompileError::Internal)?;

        Ok(JavaOutput {
            units: vec![JavaUnit { name: contract::qualified_class_name(), source: java.source }],
            reachable,
            layout,
            warnings: java.warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_thread_panic_reaches_the_caller() {
        let caught = std::panic::catch_unwind(|| with_compile_stack(|| -> () { panic!("boom") }));
        let payload = caught.expect_err("the panic must propagate");
        assert_eq!(payload.downcast_ref::<&str>(), Some(&"boom"));
    }

    #[test]
    fn compile_thread_returns_its_value() {
        assert_eq!(with_compile_stack(|| 7), Ok(7));
    }
}
