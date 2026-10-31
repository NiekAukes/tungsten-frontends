# Phase 10: Release gate

Status: not started
SPEC: §11, §14, §15

## Goal

Measure every v1 release criterion (SPEC §14) and write the results in `RELEASE-v1.md`. Fix small issues found along the way; anything bigger becomes a new phase file (`phase-10a-…`) rather than growing this session.

This phase is mostly running things. Prepare scripts so the runs are repeatable and could be done again on a later compiler version.

## Scope

1. **Scripts** (`scripts/release/`): set up a dedicated server with a given worldgen config (vanilla normal/amplified/large biomes, JJThunder, Terralith), seed and mod set; run a Chunky pregeneration with fixed radius and thread count; collect logs.
2. **Validation sweep** (§14.1): every config × ≥ 2 seeds × ≥ 10,000 chunks, `validation.enabled=true`, including structure-heavy areas. Table of chunks validated and mismatches per route.
3. **Benchmark** (§14.2): `populateNoise` wall time per chunk (debug counter from phase 7) and end-to-end pregen time, vanilla vs Tungsten, same machine, same settings. Report the speedup and the machine.
4. **Compat matrix** (§11, §14.3): Sodium, Lithium, Chunky, Distant Horizons on client and dedicated server.
   - **DH shape guard**: log the arguments DH passes to `populateNoise` in each DH generation mode; confirm `chunks_only` uses Tungsten and other modes fall through without errors. Adjust `PopulateDispatch` if the guard is wrong, and update SPEC §8.1.
   - C2ME: *(SPEC proposal)* detect at startup and log a warning.
5. **Resource numbers** (§15): Janino compile time and heap for JJThunder; ESH cache size and pool peak after a 20k-block-radius pregeneration (or the largest you can run).
6. Update SPEC §15 with what was learned.

## Acceptance

- [ ] `RELEASE-v1.md` exists with one section per §14 criterion, each marked **pass**/**fail** with the evidence (tables, numbers, log excerpts).
- [ ] All four criteria pass, or each failure has a linked follow-up phase file.

## Review checklist

- Numbers come from scripts in the repo, with the exact command line recorded.
- Vanilla and Tungsten benchmark runs differ only in `enabled`.

## Notes

_(fill in)_
