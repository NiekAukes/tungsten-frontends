# Phase 9: Distribution: natives, jar, CI

Status: not started
Repos: tungsten, tungsten-frontends
SPEC: §4.1, §12, §14 item 4

## Goal

CI produces one release jar containing the Linux and Windows compiler natives, the runtime, Janino and the mod, and that jar works outside the dev environment on a full JDK and on a launcher-bundled JRE.

## Scope

1. **tungsten-frontends CI**: build `tungsten-mc-compile-jni` for `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc` (Windows runner) or `-gnu` (cross-compile; check `catch_unwind` works with the chosen target's panic strategy). Publish as workflow artifacts (or a release), with the compiler version in the name.
   - Also: the Janino dialect CI test from SPEC §7.6.1 — compile vanilla, JJThunder and Terralith output with Janino in CI (a tiny Java test harness in that repo).
2. **tungsten CI**: download the native artifacts for a pinned compiler version, place them at `natives/linux-x86_64/libtungsten_compile.so` and `natives/windows-x86_64/tungsten_compile.dll`, run `./gradlew build test`, upload the jar. Phase 2's local-copy task stays for dev.
3. **Windows**: `Platform` and `NativeExtractor` work on Windows paths; extraction directory survives a second launch while the DLL from the first launch is still loaded (different JVM — fine; same JVM — reuses).
4. **Production class loading**: confirm Janino resolves `tungsten.runtime.*` from jar-in-jar (Knot class loader).
5. **Docs**: extend `CLAUDE.md` (from phase 0) to cover the full v1 layout; `README.md` user-facing (what it does, the Exact/blending note from §8.4, unsupported cases, config keys). Mod description in `fabric.mod.json`.
6. **Licensing check** for the vanilla ports (SPEC §12): write the finding in "Notes"; it's a decision for the maintainer, not code.

## Out of scope

macOS, Linux arm64.

## Acceptance

- [ ] CI green on both repos; jar artifact contains both natives (`unzip -l`).
- [ ] **Linux, launcher JRE**: install the CI jar + Fabric API + YACL in a real launcher instance (e.g. Prism) using its bundled Java 21 runtime (`java --list-modules | grep jdk.compiler` shows nothing, or confirm it's a JRE). Create a world: prepare screen, world opens, record written.
- [ ] **Windows, launcher JRE**: same.
- [ ] **Dedicated server from the jar** (Linux): vanilla scenario from phase 5 passes.
- [ ] Second launch reuses `.tungsten/natives/<sha>/` (no new directory).

## Review checklist

- Native versions are pinned in one place.
- No natives committed to git.
- README states the blending exception and the unsupported cases plainly.

## Notes for later phases

_(fill in: licensing finding, JRE details)_
