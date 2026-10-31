# Phase 0: New repository and build baseline

Status: not started
SPEC: §12 (distribution and build), §1.2

## Goal

A new, clean repository for the v1 mod, with the build, dependencies, test setup and CI in place, and only the pieces of the old test mod that v1 reuses copied over. After this phase the mod loads and does **nothing**: every world generates as vanilla.

The old repo (`tungsten`, the test mod) stays as it is and is the reference for ported code. Nothing in the new repo depends on it.

## Scope

1. **Scaffold** from the current Fabric example-mod template for 1.21.11 (Loom, `splitEnvironmentSourceSets`, Yarn `1.21.11+build.6`, loader 0.19.5, Fabric API 0.141.6+1.21.11 — same versions as the old repo's `gradle.properties`). Mod id `tungsten`, package root `tungsten`.
2. **Gradle**
   - `:runtime` subproject (`java-library`, release 21, no Minecraft deps), `implementation` + `include` in the root project.
   - Janino + `commons-compiler` (pinned version) as `implementation` + `include`.
   - YACL (`yet_another_config_lib_v3` for 1.21.11) and Mod Menu (`modCompileOnly` + `modLocalRuntime`).
   - Check whether Fabric Loader 0.19.5 bundles MixinExtras (it should; `@WrapMethod` needs ≥ 0.5). If not, add it.
   - Sodium, Lithium, Distant Horizons as dev-only runtime mods (`modLocalRuntime`), not dependencies.
   - Tests: JUnit 5 for `:runtime` and the main project. `net.fabricmc:fabric-loader-junit` for tests that touch Minecraft classes.
   - **No native build tasks.** (Phase 2 adds a copy task for a prebuilt compiler native.)
3. **Copy from the old repo** (and only these; each copied file keeps its content apart from package fixes, so the review is "same as old repo"):
   - `runtime/src/main/java/tungsten/runtime/*` (all 12 files).
   - `HostConsts.java` → `src/main/java/tungsten/generation/HostConstsPacker.java` as a starting point (phase 3 reworks it).
   - `StructureWeightSamplerAccessor`, `ChunkNoiseSamplerInvoker` mixins (trim to the members the old `HostConsts` uses).
   - `RewrittenAquiferSampler` is **not** copied (v2 only; it lives in the old repo's history).
4. **`fabric.mod.json`**: real name/description/author, `minecraft: "1.21.11"`, depends on `fabricloader`, `fabric-api`, `yet_another_config_lib_v3`; suggests `modmenu`; `environment: "*"`. One mixin config with the two accessors; a client mixin config, empty for now.
5. **Entry points**: `Tungsten.onInitialize` logs the version; `TungstenClient` is empty.
6. **Docs**: copy `SPEC.md` and this `phases/` directory into the new repo (they are the plan from now on). A short `CLAUDE.md` (build/test commands, layout, "read `phases/README.md` first"); phase 9 extends it. `README.md` placeholder. `LICENSE`.
7. **CI**: `.github/workflows/build.yml` runs `./gradlew build test` on JDK 21.

## Out of scope

Any feature code. Changes to `:runtime` sources.

## Acceptance

- [ ] `./gradlew clean build test` passes with a minimal `PATH` (`env PATH=/usr/bin:/bin ./gradlew build test`): no cargo, nvcc or other native tools needed.
- [ ] One JUnit test runs in `:runtime` (e.g. a `Xoroshiro128PlusPlusRandom` known-value test against vanilla's `Xoroshiro128PlusPlusRandom` output for a fixed seed) and one `fabric-loader-junit` test runs in the main project (e.g. `HostConstsPacker.empty()` builds). Proves both test setups work.
- [ ] `unzip -l build/libs/tungsten-*.jar` shows `META-INF/jars/` with `runtime`, `janino`, `commons-compiler`, and no `natives/`.
- [ ] `./gradlew runClient`: create a new world, fly around, no errors mentioning Tungsten. `./gradlew runServer` starts and generates spawn.
- [ ] CI is green on the first push.

## Review checklist

- `diff -r old/runtime/src new/runtime/src` is empty.
- Copied mixins/classes differ from the old repo only in what the PR explains.
- `build.gradle`: no `Exec` tasks; every dependency version pinned (no `+`).
- `fabric.mod.json` lists only mixin configs and classes that exist.

## Notes for later phases

_(fill in: new repo location/name, Janino version, MixinExtras version)_
