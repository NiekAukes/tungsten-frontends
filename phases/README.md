# Tungsten v1: implementation phases

This splits [`SPEC.md`](../SPEC.md) (the v1 user-facing mod) into phases. The mod is built in a **new, clean repository** (phase 0 creates it, and copies `SPEC.md` and this directory into it). The old test mod repo (`tungsten`) is only a reference for ported code. Each phase is sized for **one session** and ends with:

- **a testable result**: automated tests, or a short scripted manual check, with a clear pass/fail;
- **a reviewable diff**: one branch/PR per phase, limited to the files the phase names, with a review checklist for the human reviewer.

## Phase list

| # | Phase | Repo | Depends on | Result you can test |
|---|---|---|---|---|
| 0 | [New repository and build baseline](phase-00-new-repo.md) | new mod repo | – | `./gradlew build test` with no native tools; mod loads and worlds generate as vanilla; test setup runs; CI green |
| 1 | [Compiler as a JNI library](phase-01-compiler-jni.md) | tungsten-frontends | – | `cargo test`: in-memory API gives the same Java as the CLI; panics become errors |
| 2 | [Native bridge and availability](phase-02-native-bridge.md) | mod | 0, 1 | JUnit: extract + load native, compile vanilla worldgen to Java source from Java |
| 3 | [Janino, Program and ProgramInstance](phase-03-janino-program.md) | mod | 2 | JUnit: Janino-compiled program matches harness reference bitwise; pool and ESH cache tests |
| 4 | [Config and world record](phase-04-config-and-record.md) | mod | 0 | JUnit: §6.3 truth table, record round-trip; YACL screen opens |
| 5 | [Preparation on the dedicated server](phase-05-preparation-server.md) | mod | 3, 4 | `runServer` scenarios: vanilla/amplified/flat/datapack/inline settings give the expected log or refusal |
| 6 | [Validation mode](phase-06-validation-mode.md) | mod | 5 | Server with `validation.enabled`: zero mismatches on vanilla, dumps on injected error |
| 7 | [Noise filler (generation)](phase-07-noise-filler.md) | mod | 6 | World-diff tool: Tungsten world == vanilla world, block for block |
| 8 | [Single-player flow and screens](phase-08-singleplayer-flow.md) | mod | 5, 7 | Manual checklist: create/load/cancel/unsupported/off paths; world lock released |
| 9 | [Distribution: natives, jar, CI](phase-09-distribution.md) | mod + tungsten-frontends | 8 | CI builds one jar with Linux+Windows natives; jar runs on a launcher JRE |
| 10 | [Release gate](phase-10-release-gate.md) | mod | 9 | §14 criteria measured and written up in `RELEASE-v1.md` |

```
0 ──┬──────────────► 4 ──┐
    │                    ▼
1 ──┴► 2 ──► 3 ─────────► 5 ──► 6 ──► 7 ──► 8 ──► 9 ──► 10
```

Phases 1 and 4 can run in parallel with their neighbours. Everything else is sequential.

## Conventions for every phase

- **Start of session**: read this file, the phase file, and the SPEC sections it cites. Check the "Status" line of earlier phases.
- **Branch**: `phase-NN-<slug>`. One PR per phase. Don't mix in work from another phase. If you find something that belongs elsewhere, add it to that phase file under "Carried over".
- **End of session**:
  1. All tests named in "Acceptance" pass. Paste the command and its output summary in the PR.
  2. Set the phase file's `Status:` line to `done (<date>, <commit>)` and fill in "Notes for later phases" with anything the next phase must know (decisions, surprises, deviations from SPEC).
  3. If the SPEC turned out wrong, edit SPEC.md in the same PR and say so in the PR description.
- **Reviewability**:
  - Prefer small classes with one job, named as in SPEC (`TungstenPreparation`, `ProgramInstance`, `HostConstsPacker`, …).
  - Every ported piece of vanilla logic cites the vanilla class and method it follows (Yarn name) in a comment, so a reviewer can put them side by side.
  - Mixins stay thin: they call into a plain class that holds the logic, so the logic can be unit-tested.
  - No dead code, no commented-out experiments. Git history keeps the old code.
- **Ported code**: when a class is ported from the old test-mod repo, the PR says which file it came from, so the reviewer can diff against it.
- **Package layout** (proposed, adjust in phase 0 if needed):
  - `tungsten.compiler`: native compiler bridge, extraction, availability (phase 2)
  - `tungsten.program`: Janino, `Program`, `ProgramInstance`, pool, ESH cache (phase 3)
  - `tungsten.config`, `tungsten.record`: config and world record (phase 4)
  - `tungsten.prepare`: preparation pipeline and `TungstenRuntime` (phase 5)
  - `tungsten.generation`: `HostConstsPacker`, `TungstenNoiseFiller`, ore vein chooser (phases 6–7)
  - `tungsten.validation`: validator and dumps (phase 6)
  - `tungsten.client` (client source set): screens, toast, Mod Menu (phases 4, 8)

## Open questions assigned to phases

| Question (SPEC §15 and inline) | Resolved in |
|---|---|
| Compiler output stays in the Janino dialect | 1 (check), 3 (Janino test) |
| Entry-point manifest vs naming conventions | 1 |
| Unused `Jigsaw` slot filler (`FAR_AWAY` vs zero) and HostConsts caps (50 vs 500) | 3 |
| Janino time and memory on JJThunder | 3 (measure), 10 (report) |
| `ChunkNoiseSampler` local capture (`ordinal`) | 6 |
| `ChainedBlockSource` order in 1.21.11 | 7 |
| Session/SaveLoader release on cancel | 8 |
| Distant Horizons shape guard | 10 |
