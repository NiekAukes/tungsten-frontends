# Phase 7: Noise filler (generation)

Status: not started
SPEC: §8.2, §8.3 (Exact), §8.4, §8.5, §9 (runtime failure)

## Goal

With Tungsten active and validation off, `populateNoise` places blocks from Tungsten's buffers. The resulting world is block-for-block identical to vanilla (outside blending), and the NOISE stage is faster.

## Scope

1. **`tungsten.generation.TungstenNoiseFiller`** (SPEC §8.2 steps 1–6). Write it as a side-by-side port of vanilla `NoiseChunkGenerator.populateNoise` (1.21.11 Yarn); keep vanilla's structure and variable names where possible and cite the method in the class Javadoc:
   - get/create the vanilla `ChunkNoiseSampler`;
   - pack HostConsts, borrow slot, final density + vein buffers;
   - walk blocks in vanilla order (cell-major, top-down Y);
   - aquifer → ore vein chooser → default block, in the **`ChainedBlockSource` order of 1.21.11** (check the source and note the result);
   - section writes, `OCEAN_FLOOR_WG`/`WORLD_SURFACE_WG` heightmaps, fluid post-processing, `isOutsideGenerationArea` — same as vanilla.
2. **`tungsten.generation.OreVeinChooser`**: port of `OreVeinSampler`'s block-choice logic reading toggle/ridged/gap from the buffers and using the positional `RandomSplitter` from the `NoiseConfig`'s ore random deriver. Javadoc cites the vanilla method; licensing note per SPEC §12.
3. **Blending**: non-empty `Blender` is ignored (SPEC §8.4). One comment, no special code.
4. **Crashes**: exceptions propagate; wrap in a `CrashException` with a `CrashReport` section "Tungsten" naming the chunk position, profile and compiler version.
5. **Dispatch**: replace phase 6's `TODO(phase 7)` with the filler.
6. **World-diff tool** (test aid, `src/test/java/.../WorldDiff` or `scripts/`): compares two world folders' region files for the overworld, per chunk, per section: block state palettes + indices, heightmaps `OCEAN_FLOOR_WG`/`WORLD_SURFACE_WG` if still present. Output: number of chunks compared, number differing, first differences. Use vanilla `NbtIo`/`RegionFile` via `fabric-loader-junit`, or a standalone NBT reader.
   - To compare the NOISE stage directly, stop generation at NOISE if feasible (e.g. a dev-only flag that keeps chunks at `ChunkStatus.NOISE` via `/forceload` of proto chunks), otherwise compare full chunks: later stages are vanilla and deterministic given the same NOISE output, so a full-chunk match is also a pass.

## Out of scope

Tungsten ESH for the aquifer (v1.1). Performance tuning beyond obvious allocations.

## Acceptance

- [ ] **Equality**: same seed, same pregeneration area (≥ 32-chunk radius around spawn + a village + an ancient city + a trial chamber), once with `enabled=false` and once with Tungsten active. `WorldDiff` reports **0 differing chunks**. Repeat with seed 2 and amplified.
- [ ] **Validation still clean**: a run with `validation.enabled=true` on the same world produces zero mismatches (regression check for phase 6).
- [ ] **Crash path**: `-Dtungsten.dev.throwInFiller=true` produces a crash report with the "Tungsten" section and the chunk position; the world is not saved half-written (vanilla behaviour).
- [ ] **Rough speed**: log average `populateNoise` ms per chunk (debug counter, off by default) for vanilla vs Tungsten on the same area. Put the numbers in "Notes". Not the release gate — that's phase 10.

## Review checklist

- Put `TungstenNoiseFiller` next to vanilla's `populateNoise`: the loop structure, the heightmap and fluid-tick calls should line up one to one.
- `OreVeinChooser` next to `OreVeinSampler`: same constants, same order of random calls.
- The slot is closed before `populateNoise` returns, and buffers are not used after that.
- No per-block allocation.

## Notes for later phases

_(fill in: ChainedBlockSource order, speed numbers, any differences found and fixed)_
