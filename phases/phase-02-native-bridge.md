# Phase 2: Native bridge and availability

Status: not started
SPEC: §4, §4.1, §9.1

## Goal

From Java, Tungsten can find, extract, and load the compiler for this platform, and drive it: feed files, set a root, compile, read the result. Availability is computed once and has a human-readable reason.

## Scope

1. **Native packaging (dev)**: a Gradle task that copies a prebuilt `libtungsten_mc_compile_jni.so` from `../tungsten-frontends/target/release/` (path overridable with a Gradle property) to `src/main/resources/natives/linux-x86_64/libtungsten_compile.so`. It doesn't build Rust; it fails with a clear message if the file is missing. *(Phase 9 replaces this with CI artifacts.)*
2. **`tungsten.compiler.Platform`**: maps `os.name`/`os.arch` to `linux-x86_64` / `windows-x86_64` / unsupported, and to the library file name.
3. **`tungsten.compiler.NativeExtractor`** (SPEC §4.1): reads `natives/<platform>/<lib>` from the class loader, hashes it (SHA-256), extracts to `<gameDir>/.tungsten/natives/<sha>/<lib>` if not already there (write to a temp file in the same directory, then atomic move), `System.load`s it. Idempotent within a JVM.
4. **`tungsten.compiler.NativeCompiler`**: the `native` declarations from phase 1, plus a Java wrapper:
   - `CompilerSession implements AutoCloseable` (`loadResource`, `setRoot`, `compile() → CompileResult`, `close()` disposes).
   - `CompileResult` = `Success(units, reachable, manifest)` | `Unsupported(kind, id, message)` | `Internal(message)`, parsed from the JSON with Gson (bundled with Minecraft).
   - `TungstenCompilerException` for panics.
5. **`tungsten.compiler.Availability`**: `{available, reasonText}`. Checks 1–2 (config enabled, platform) at startup; native load happens lazily on first `CompilerSession` (SPEC §4 proposal). Until phase 4 exists, "config enabled" is a constant `true` with a `TODO(phase 4)`.

## Out of scope

Janino, Minecraft resource walking (phase 5), toast/Mod Menu text (phase 8).

## Acceptance

- [ ] JUnit `NativeCompilerTest` (plain JUnit, no Minecraft): extracts into a temp game dir, feeds every file under `../tungsten-frontends/tungsten-mc-compile/vanilla_worldgen/` with ids built from their paths, root `minecraft:overworld`, gets `Success` with ≥ 1 unit and a non-empty manifest.
- [ ] Same test with a corrupted density function gets `Unsupported` naming the id.
- [ ] Extraction test: running twice reuses the same file (same mtime); a different library content gives a different directory.
- [ ] `Platform` unit test covers Linux, Windows, macOS (unsupported), `aarch64` (unsupported).
- [ ] Tests skip (JUnit `assumeTrue`) with a clear message when the native isn't present, so CI without the native still passes until phase 9.

## Review checklist

- No `ThreadLocal`, no static mutable state except the "loaded" flag in `NativeExtractor`.
- `CompilerSession.close()` is always called (try-with-resources in tests).
- `native` method signatures match phase 1's "Notes for later phases" exactly.
- JSON parsing fails loudly on an unknown schema version.

## Notes for later phases

_(fill in)_
