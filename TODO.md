# Features to implement for The Minecraft Frontend

## Upgrade to Minecraft 26.3

### Issue: Upgrade world generation support to Minecraft 26.3

**Problem**

Minecraft 26.3 changes the world-generation data model and density-function
behavior. The frontend needs to be updated to understand the new formats and
evaluate world generation consistently with this version.

**Changes to account for**

- In 26.2 snapshots, `clamp` began accepting a density-function ID, and
  `interval_select` was added to replace `weird_scale_sampler`.
- Minecraft 26.3 added the `ceil`, `div`, `floor`, `negate`, `lerp`, `round`,
  `sub`, `truncate`, `distance_to_point`, `log`, `pow`, `sign`, `slice`,
  `sqrt`, and `ore_vein` density functions.
- `invert` was renamed to `reciprocal`, `end_islands` to `end_outer_islands`,
  and `cache_once` to `cache`. `y_clamped_gradient` was replaced by
  `gradient`.
- Several density-function argument fields changed names. The `noise`
  function gained `shift_x`, `shift_y`, and `shift_z`; the interpolated
  function gained `cell_size_xz` and `cell_size_y`.
- `cache_2d`, `cache_all_in_cell`, `flat_cache`, and `shifted_noise` were
  removed.
- Density-function calculations now use single-precision rather than
  double-precision floating-point values.
- Other changes: Check against decompiled sources.

**Desired outcome**

Update the frontend's world-generation parsing and evaluation for these
changes, and verify that supported 26.3 world-generation data loads and
produces results consistent with Minecraft 26.3.

## Explore value caching strategies to improve GPU performance in world generation

### Issue: Investigate value caching for GPU world generation

**Problem**

Some compute units (CUs) are too small to use the GPU efficiently. Recomputing
values that are reusable across a larger grid can therefore add overhead and
limit world-generation performance. The current approach does not define how
to identify, store, or reuse these values; introducing caching may require a
significant change to the evaluation architecture.

**Context**

`cache2d` and `flatcache` are examples of
evaluations that might benefit from being evaluated on a larger grid and reused.
The suitable cache scope, lifetime, and integration with GPU execution still
need to be determined.

**Desired outcome**

Investigate viable caching strategies and their performance trade-offs, then
define an approach that avoids redundant evaluation while preserving correct
world-generation results. The design should clarify which values can be
cached, at what grid scope, and how cache storage and reuse fit into GPU
execution before implementation begins.

## Develop proper Mod for Minecraft 1.21.11

### Issue: Develop the Tungsten mod for Minecraft 1.21.11

Minecraft 1.21.11 compatibility has been verified, but there is not yet a
mod that integrates the frontend with the game. There is a testing mod available, but it is not suitable for end users.

**Requirements**

- Replace the overworld's default world-generation logic with the frontend's
  implementation.
- Integrate with the Java Tungsten backend, which is still in development.
  The Java code must be injected into the Minecraft runtime so it can take
  over the relevant world-generation behavior.
- Support the data packs supplied by Minecraft.

**Desired outcome**

Provide a working mod for Minecraft 1.21.11 that connects the game's
overworld generation to this frontend through the Java Tungsten backend.
Verify that it loads the supplied Minecraft data packs and generates the
overworld using the frontend implementation.
