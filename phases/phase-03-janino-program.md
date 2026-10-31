# Phase 3: Janino, Program and ProgramInstance

Status: not started
SPEC: §7.6, §7.6.1, §7.7, §8.5, §8.6, §15

## Goal

Generated Java source becomes a running, seed-bound program, with no Minecraft involved: Janino compiles it, `Program` binds the entry points, `ProgramInstance` holds `AutoConsts`, the orchestration pool and the ESH cache. Its output matches the reference implementation bitwise.

## Scope

1. **`tungsten.program.JaninoCompiler`**: `compile(List<Unit>) → ClassLoader` using a single multi-unit cook, parent = `Tungsten.class.getClassLoader()` (in tests: the test class loader), debug info off. On `CompileException` throw `ProgramCompileException` with Janino's message, location, and the offending source line. *(Proposed in SPEC: write the full source to `<gameDir>/.tungsten/failed/<timestamp>.java`; do it here.)* Log compile time.
2. **`tungsten.program.Program`**: binds `MethodHandle`s from the phase 1 manifest (`AutoConsts.create(long)`, `Orchestration(AutoConsts)`, the route methods, `HostConsts` constructor and fields). A missing member → `ProgramBindException` naming it.
3. **`tungsten.program.ProgramInstance`**: `Program` + seed.
   - Shared immutable `AutoConsts`.
   - `Pool` of `Slot {orchestration, hostConsts, Jigsaw[], Piece[]}` on a `ConcurrentLinkedQueue`, created on demand, peak size tracked.
   - `try (Slot s = instance.borrow()) { ... }` API so returning is automatic.
   - Methods: `finalDensityWithBeardify(slot, originX, originY, originZ)`, `veins(slot, ...)`, each returning the slot-owned buffer.
   - `EshCache`: `ConcurrentHashMap<Long, CompletableFuture<double[]>>` with `computeIfAbsent` (SPEC §8.6); `surfaceRegion(regionX, regionZ)`.
4. **`tungsten.generation.HostConstsPacker`** (copied from the old repo's `HostConsts` in phase 0): drop the old byte-buffer `pack()`; fill the generated `HostConsts` object through `Program`'s handles. Resolve the two open questions:
   - **Caps**: read the generated `HostConsts` array sizes from the manifest; delete the hardcoded `MAX_PIECES`/`MAX_JIGSAWS` disagreement. Exceeding the cap throws (SPEC §9).
   - **Unused-slot filler**: confirm in the generated beardifier code whether it iterates all slots or a count. Use the filler the harness uses, and write down the decision in the class Javadoc.
   The packer takes plain values (box, list of pieces, list of junctions) so it's unit-testable; the vanilla `StructureWeightSampler` adapter is a thin method on top.
5. **Method-size warning** (SPEC §15 proposal): after cooking, scan the generated class bytes for methods over 8000 bytes of bytecode and log a WARN listing them.

## Out of scope

Anything that touches a running world. `RewrittenAquiferSampler`.

## Acceptance

- [ ] **Reference test**: for vanilla worldgen, seed `0` and seed `123456789`, chunk origins `(0,0)`, `(-16, 32)`, `(1600, -4800)`: Janino-compiled `finalDensityWithBeardify` with an empty HostConsts equals the reference bitwise (NaNs equal).
  - Reference = `tungsten-diff-harness` Rust output for the same seed/origin. Add a small dump option to the harness if needed, and store fixtures under `src/test/resources/fixtures/` with a script `scripts/regen-fixtures.sh` that rebuilds them.
  - Same for the three vein routes and one `preliminary_surface_level` region.
- [ ] Reference test with one non-empty HostConsts (one piece, one junction near the origin): matches the harness.
- [ ] Janino compiles JJThunder output; log compile time and peak heap use (`MemoryMXBean` before/after) in the test output. Record the numbers in "Notes".
- [ ] Concurrency test: 16 threads × 50 calls on one `ProgramInstance` give the same buffers as single-threaded; pool peak ≤ 16.
- [ ] ESH test: 32 threads asking for the same region trigger exactly one computation.
- [ ] Bind test: a manifest naming a missing method gives `ProgramBindException` with the name.
- [ ] Unloading test: after dropping the instance and `System.gc()`, a `WeakReference` to the program class loader clears (best effort; mark `@Tag("slow")`).

## Review checklist

- `ProgramInstance` has no static state; nothing static references `Program` or its class loader.
- Buffers returned from the slot are documented as "valid until the slot is closed".
- `HostConstsPacker` Javadoc states caps, filler convention, and where they come from.
- Fixtures are small and the regen script is checked in.

## Notes for later phases

_(fill in: Janino timings, filler decision, cap value, buffer layouts and origin conventions)_
