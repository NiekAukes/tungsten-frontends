# Phase 1: Compiler as a JNI library

Status: done (2026-10-29, not yet committed; branch `phase-01-compiler-jni` in tungsten-frontends and tungsten-wg)
Repo: `../tungsten-frontends` (and `../tungsten-wg` if the Java backend needs changes)
SPEC: §7.3, §7.4, §7.5, §7.6.1, §7.7, §9.1

## Goal

`tungsten-mc-compile` can be driven entirely in memory through a C-ABI/JNI library: you feed it worldgen files and a root id, and it returns Java source plus metadata. It never panics across the boundary.

## Scope

1. **In-memory input** (`tungsten-mc-compile`)
   - Public API: `CompilerSession::new()`, `load_resource(id: &str, bytes: &[u8])`, `set_root(id: &str)`, `compile() -> Result<JavaOutput, CompileError>`.
   - `id` is the resource path as the mod sees it, e.g. `minecraft:worldgen/density_function/overworld/continents.json`. The session classifies it by path (`worldgen/noise_settings/`, `worldgen/density_function/`, `worldgen/noise/`, ignore the rest) and overwrites by id (last one wins).
   - **No bundled vanilla.** The session uses only what was fed. The CLI keeps its current behaviour by feeding `vanilla_worldgen/` plus `--mod-folder` through the same API.
   - The root is any noise-settings id (`minecraft:overworld`, `minecraft:amplified`, `minecraft:large_biomes`), not hardcoded.
   - Only the Java backend runs in this path (no Rust/CUDA/Inspect output, no files written to CWD).
2. **Output** (`JavaOutput`)
   - `units: Vec<(name, source)>` (one or more compilation units).
   - `reachable: Vec<(kind, id)>` for `density_function` and `noise` ids reachable from the root (SPEC §7.5).
   - `manifest`: entry-point names (class names, `AutoConsts.create`, `Orchestration` ctor, the orchestrate methods for `final_density_with_beardify`, `vein_toggle`/`vein_ridged`/`vein_gap`, `preliminary_surface_level`, `HostConsts` fields), plus the output dims of each route. This resolves SPEC §7.7's proposed manifest so the mod doesn't depend on naming conventions.
   - `compiler_version()` = crate version.
3. **Errors**: `CompileError::Unsupported { kind, id, message }` (user problem: unknown density function type, missing reference…) vs `CompileError::Internal(message)` (bug). Replace `panic!`/`unwrap` on user-input paths with `Unsupported` where practical. Everything else stays a panic and is caught at the boundary.
4. **JNI crate** `tungsten-mc-compile-jni` (cdylib, `jni` crate), exporting `Java_tungsten_compiler_NativeCompiler_*` (for the class `tungsten.compiler.NativeCompiler` in the new mod repo):
   - `create() -> long`, `loadResource(long, String, byte[])`, `setRoot(long, String)`, `compile(long) -> int`, then getters for units, reachable ids and manifest (or one JSON string, see below), `version() -> String`, `dispose(long)`.
   - *Proposed:* `compile` returns the result as **one JSON string** (units, reachable, manifest, or error). One crossing, easy to inspect, easy to version.
   - Every export wrapped in `catch_unwind`. A panic throws `tungsten.compiler.TungstenCompilerException` (internal) and returns a neutral value.
   - The compile runs on a thread with a 16 MB stack inside the library (the transforms recurse deeply; don't rely on the caller's stack).
5. **Dialect check**: a test that greps the generated Java for forbidden constructs (`->`, `::`, `var `, `record `, `<` used for generics, `switch` expressions, text blocks). The real Janino test is in phase 3.

## Out of scope

The Java side (phase 2). Windows build (phase 9). Vein-route correctness (phase 6 validates it).

## Acceptance

- [x] `cargo test -p tungsten-mc-compile`: new test feeds `vanilla_worldgen/` through `CompilerSession` and gets **byte-identical** Java to the CLI path.
- [x] Test: feeding JJThunder on top of vanilla compiles; `reachable` contains JJThunder ids.
- [x] Test: a density function with an unknown `type` gives `Unsupported` naming the type and id, not a panic.
- [x] Test: root `minecraft:amplified` compiles and its source differs from `minecraft:overworld`.
- [x] Test (in the JNI crate, via a forced-panic debug hook or a test-only export): a panic is caught.
- [x] Dialect grep test passes for vanilla and JJThunder.
- [x] `cargo build --release -p tungsten-mc-compile-jni` produces `libtungsten_mc_compile_jni.so`, and `nm -D` shows only the expected `Java_tungsten_compiler_NativeCompiler_*` symbols.

## Review checklist

- The CLI and the JNI path share one code path for loading (no duplicated classification logic).
- No `std::env::current_dir`/`CARGO_MANIFEST_DIR` use on the library path.
- Every `extern "system"` fn has `catch_unwind`; none calls `.unwrap()` on a JNI call outside it.
- The JSON output schema is written down (doc comment or `docs/jni-api.md`) with one example.

## Notes for later phases

- **Native signatures** (all `static`, class `tungsten.compiler.NativeCompiler`):
  `long create()`, `void loadResource(long, String, byte[])`, `void setRoot(long, String)`, `String compile(long)`, `String version()`, `void dispose(long)`. `compile` returns one JSON document, **schema 1**, described with examples in `docs/jni-api.md` (tungsten-frontends). `tungsten.compiler.TungstenCompilerException` needs a public `(String)` constructor (thrown with `ThrowNew`). It is thrown for panics, failed JNI calls and handle 0. `unsupported`/`internal` results are returned in the JSON, not thrown.
- **Manifest** (resolves SPEC §7.7): `package`, `class_name`, `host_consts_class`, `auto_consts_class`, `auto_consts_factory` (`create(long seed)`), `orchestration_class` (ctor takes AutoConsts), `entries[] {route, method, element_type, dimensions, returns_array}`, `host_consts[] {name, field, java_type}`. Routes: `final_density_with_beardify`, `final_density`, `vein_gap`, `vein_ridged`, `vein_toggle` (16×height×16 double), `preliminary_surface_level` (128×1×128 double), `aquifer` (16×height×16 byte). Bind by `route`, never by method name.
- **`vein_gap` is now a route.** It used to be a `SingleSamplingDensity` that the transform skipped, so no Java was generated for it. It is now a per-block terrain route like `vein_ridged`. Phase 6 validates it.
- **Ids**: `loadResource` takes resource paths (`ns:worldgen/<registry>/<path>.json`). `reachable` and error ids are registry ids (`ns:<path>`). `reachable` lists ids reached *by reference*. Inline definitions have no id. Only the root noise settings is parsed, so unrelated broken files don't fail a compile. Exception: any fed file of the three kinds that isn't UTF-8 fails it.
- **Output naming**: every compile produces `tungsten.generated.GeneratedWorldgen` (one unit). Two programs in one JVM need separate class loaders, which Janino gives anyway.
- **Workspace `panic = "unwind"`** in `[profile.release]`. With the old `abort`, `catch_unwind` couldn't catch anything. Phase 9's CI must not override it.
- **tungsten-wg change required**: `JavaOutput.manifest` (`JavaManifest`) in `crates/tungsten-java`, branch `phase-01-compiler-jni` of `../tungsten-wg`. It must be merged together with this phase.
- The release `.so` is ~16 MB because `tungsten-mc-compile` links every backend (Rust, CUDA, Inspect). Making those optional features is a possible phase-9 size win.
- Transform-layer problems are still panics and reach Java as `TungstenCompilerException`. While testing, three inputs that panicked are now `Unsupported` in the parser: a bad `rarity_value_mapper`, `cell_height <= 0`, and a spline with no points. More may exist.
- **Pre-existing, not fixed here** (relevant to phase 3):
  - The diff harness doesn't build its Rust wrapper: `rust_wrapper/src/lib.rs` doesn't set the newer HostConsts fields `aquifer_max_y`, `estimate_surface_height`, `splitter`. The same failure occurs without this phase's changes.
  - The harness Java runtime (`tungsten-diff-harness/java_wrapper/runtime`) lacks what the aquifer/ESH routes call: classes `EstimatedSurfaceHeight`, `XoroshiroRandomSplitter`, and `Helpers.abs_i32`, `floor_div`, `min_i32`, `max_distance`, `block_state`, `biome_column_index`, `cell_to_block_*`, `unpack_long_*`, `get_block_state_from_packed`, `get_fluid_level_from_packed`, `random_next_int`. javac reports 38 "cannot find symbol" errors on the full program, identical before and after this phase. The mod's `tungsten.runtime` must provide these before the Janino test can compile the whole program.
- Not covered: Terralith (no copy in this repo). The dialect test runs on vanilla and JJThunder.
