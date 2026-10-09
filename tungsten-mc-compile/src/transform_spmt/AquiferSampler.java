package tungsten.generation;

import java.util.Arrays;
import net.minecraft.SharedConstants;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.ChunkPos;
import net.minecraft.util.math.ChunkSectionPos;
import net.minecraft.util.math.MathHelper;
import net.minecraft.util.math.random.Random;
import net.minecraft.util.math.random.RandomSplitter;
import net.minecraft.world.biome.source.util.VanillaBiomeParameters;
import net.minecraft.world.dimension.DimensionType;
import net.minecraft.world.gen.chunk.AquiferSampler;
import net.minecraft.world.gen.chunk.ChunkNoiseSampler;
import net.minecraft.world.gen.densityfunction.DensityFunction;
import net.minecraft.world.gen.noise.NoiseRouter;
import org.apache.commons.lang3.mutable.MutableDouble;
import org.jspecify.annotations.Nullable;

/**
 * Decides, for every block that terrain noise says is NOT solid, what fills it:
 * air, water, lava, or "solid" (null, meaning "leave it as stone").
 *
 * Overview of the "aquifer" idea:
 *  - Space is divided into a grid of cells (16 x 12 x 16 blocks). Every cell gets ONE random
 *    "site" point, deterministically derived from the world seed.
 *  - Every site has its own fluid level (e.g. "water up to Y=40" or "lava up to Y=-30" or "nothing").
 *  - A block takes the fluid of the NEAREST site, like a 3D Voronoi diagram.
 *  - Where two neighbouring sites have different fluid levels, a thin noisy wall of solid
 *    blocks is generated along their shared border so the fluids don't just spill into each other.
 *    That is what produces the water/lava pockets and the stone "walls" in caves.
 */
public interface RewrittenAquiferSampler {
    /** Supplies the default (global) fluid level at a position, e.g. sea level water or the lava lake level. */
    public static class FluidLevelSampler implements AquiferSampler.FluidLevelSampler {

        @Override
        public AquiferSampler.FluidLevel getFluidLevel(int x, int y, int z) {
            return null; // TODO
        }
    }

    /** Creates the full noise-based aquifer sampler (used where caves can hold groundwater/lava). */
    static RewrittenAquiferSampler aquifer(
            ChunkNoiseSampler chunkNoiseSampler,
            ChunkPos chunkPos,
            NoiseRouter noiseRouter,
            RandomSplitter randomSplitter,
            int minimumY,
            int height,
            AquiferSampler.FluidLevelSampler fluidLevelSampler
    ) {
        return new RewrittenAquiferSampler.Impl(chunkNoiseSampler, chunkPos, noiseRouter, randomSplitter, minimumY, height, fluidLevelSampler);
    }

    /**
     * Creates a trivial sampler with no aquifers at all: any non-solid block just gets
     * whatever the global fluid level (e.g. sea level) says.
     */
    static RewrittenAquiferSampler seaLevel(AquiferSampler.FluidLevelSampler fluidLevelSampler) {
        return new RewrittenAquiferSampler() {
            @Override
            public @Nullable BlockState apply(DensityFunction.NoisePos pos, double density) {
                // Positive density = solid terrain (null = keep terrain). Otherwise fill with the default fluid or air.
                return density > 0.0 ? null : fluidLevelSampler.getFluidLevel(pos.blockX(), pos.blockY(), pos.blockZ()).getBlockState(pos.blockY());
            }

            @Override
            public boolean needsFluidTick() {
                return false;
            }
        };
    }

    /**
     * @param density terrain density at this position (> 0 means solid)
     * @return the block to place, or null if the position should stay solid (regular terrain block)
     */
    @Nullable BlockState apply(DensityFunction.NoisePos pos, double density);

    /**
     * @return true if the block returned by the last call to {@link #apply} sits on a fluid boundary
     *         and should be scheduled for a fluid update so the fluid can settle/flow
     */
    boolean needsFluidTick();

    /** A fluid surface: everything BELOW height {@code y} is filled with {@code state}; everything at/above it is air. */
    record FluidLevel(int y, BlockState state) {

        public BlockState getBlockState(int y) {
            return y < this.y ? this.state : Blocks.AIR.getDefaultState();
        }
    }



    class Impl implements RewrittenAquiferSampler {
        // Each aquifer "cell" is 16 x 12 x 16 blocks; one random site point is placed inside each cell.
        // The site's offset within its cell is random in [0, JITTER_RANGE) on each axis.
        // (Note: the code below uses these values as literals: nextInt(10), nextInt(9), nextInt(10).)
        private static final int SITE_JITTER_RANGE_X = 10;
        private static final int SITE_JITTER_RANGE_Y = 9;
        private static final int SITE_JITTER_RANGE_Z = 10;
        // Cell dimensions in blocks (the code uses ">> 4" for 16 and "/ 12" or "* 12" for 12).
        private static final int CELL_SIZE_X = 16;
        private static final int CELL_SIZE_Y = 12;
        private static final int CELL_SIZE_Z = 16;
        private static final int LAST_BLOCK_OFFSET_IN_CELL_Y = 11;
        /**
         * If the "closeness factor" of two sites (see {@link #maxDistance}) is at least this value,
         * the two sites are considered close enough that a fluid boundary between them needs a fluid tick.
         * Evaluates to maxDistance(10^2, 12^2), i.e. sites whose distances differ by less than ~44 (squared).
         */
        private static final double NEEDS_FLUID_TICK_DISTANCE_THRESHOLD = maxDistance(MathHelper.square(10), MathHelper.square(12));
        // The remaining constants are unused in the decompiled source (the same values appear as literals in the code).
        private static final int UNUSED_BLOCK_SHIFT_X = -5;
        private static final int UNUSED_GRID_PADDING_X = 1;
        private static final int UNUSED_BLOCK_SHIFT_Z = -5;
        private static final int UNUSED_GRID_START_PADDING_Z = 0;
        private static final int UNUSED_GRID_START_PADDING_Y = -1;
        private static final int UNUSED_GRID_START_PADDING_X = 0;
        private static final int UNUSED_GRID_END_PADDING_X = 1;
        private static final int UNUSED_GRID_END_PADDING_Y = 1;
        private static final int UNUSED_GRID_END_PADDING_Z = 1;

        private final ChunkNoiseSampler chunkNoiseSampler;
        /** Noise used to make the solid wall between two different fluid regions irregular and cave-like. */
        private final DensityFunction barrierNoise;
        /** Noise deciding whether an aquifer site is flooded at all (high = flooded). */
        private final DensityFunction fluidLevelFloodednessNoise;
        /** Noise that randomizes the fluid height of non-default (cave) aquifers. */
        private final DensityFunction fluidLevelSpreadNoise;
        /** Noise that decides whether deep aquifers are lava instead of water. */
        private final DensityFunction fluidTypeNoise;
        private final RandomSplitter randomDeriver;
        /** Lazily computed fluid level for each cell's site point. */
        private final AquiferSampler.@Nullable FluidLevel[] cellFluidLevels;
        /** Lazily computed packed site position (BlockPos long) for each cell; Long.MAX_VALUE means "not yet computed". */
        private final long[] cellSitePositions;
        private final AquiferSampler.FluidLevelSampler fluidLevelSampler;
        private final DensityFunction erosionDensityFunction;
        private final DensityFunction depthDensityFunction;
        /** Output flag read through {@link #needsFluidTick()} after each {@link #apply}. */
        private boolean needsFluidTick;
        /** Above this Y, no aquifer logic is performed and the default fluid level is used directly. */
        private final int aquiferMaxY;
        // The cell-grid region covered by this chunk (cache arrays are indexed relative to these).
        private final int gridStartCellX;
        private final int gridStartCellY;
        private final int gridStartCellZ;
        private final int gridSizeX;
        private final int gridSizeZ;
        /**
         * Chunk offsets (in chunks) around a position that are sampled when estimating the local surface height.
         * The pattern is deliberately lopsided: it reaches 3 chunks toward -X but only 1 toward +X.
         */
        private static final int[][] SURFACE_SAMPLE_CHUNK_OFFSETS = new int[][]{
                {0, 0}, {-2, -1}, {-1, -1}, {0, -1}, {1, -1}, {-3, 0}, {-2, 0}, {-1, 0}, {1, 0}, {-2, 1}, {-1, 1}, {0, 1}, {1, 1}
        };

        Impl(
                ChunkNoiseSampler chunkNoiseSampler,
                ChunkPos chunkPos,
                NoiseRouter noiseRouter,
                RandomSplitter randomSplitter,
                int minimumY,
                int height,
                AquiferSampler.FluidLevelSampler fluidLevelSampler
        ) {
            this.chunkNoiseSampler = chunkNoiseSampler;
            this.barrierNoise = noiseRouter.barrierNoise();
            this.fluidLevelFloodednessNoise = noiseRouter.fluidLevelFloodednessNoise();
            this.fluidLevelSpreadNoise = noiseRouter.fluidLevelSpreadNoise();
            this.fluidTypeNoise = noiseRouter.lavaNoise();
            this.erosionDensityFunction = noiseRouter.erosion();
            this.depthDensityFunction = noiseRouter.depth();
            this.randomDeriver = randomSplitter;

            // Work out which cells of the grid this chunk can possibly touch, so the per-cell caches can be
            // plain arrays instead of hash maps. The grid is shifted by 5 blocks in X/Z and by 1 in Y
            // (see the same shifts in apply()), with 1 extra cell of padding where neighbours get read.
            this.gridStartCellX = blockToCellX(chunkPos.getStartX() + -5) + 0;
            this.fluidLevelSampler = fluidLevelSampler;
            int gridEndCellX = blockToCellX(chunkPos.getEndX() + -5) + 1;
            this.gridSizeX = gridEndCellX - this.gridStartCellX + 1;
            this.gridStartCellY = blockToCellY(minimumY + 1) + -1;
            int gridEndCellY = blockToCellY(minimumY + height + 1) + 1;
            int gridSizeY = gridEndCellY - this.gridStartCellY + 1;
            this.gridStartCellZ = blockToCellZ(chunkPos.getStartZ() + -5) + 0;
            int gridEndCellZ = blockToCellZ(chunkPos.getEndZ() + -5) + 1;
            this.gridSizeZ = gridEndCellZ - this.gridStartCellZ + 1;

            int totalCellCount = this.gridSizeX * gridSizeY * this.gridSizeZ;
            this.cellFluidLevels = new AquiferSampler.FluidLevel[totalCellCount];
            this.cellSitePositions = new long[totalCellCount];
            Arrays.fill(this.cellSitePositions, Long.MAX_VALUE); // sentinel: site not generated yet

            // Find the highest Y at which aquifers could matter: the highest terrain surface in this chunk
            // (plus margin), rounded up to a whole cell. Above that, apply() skips all the expensive work.
            int highestAquiferY = chunkNoiseSampler.estimateHighestSurfaceLevel(
                            cellToBlockX(this.gridStartCellX, 0),
                            cellToBlockZ(this.gridStartCellZ, 0),
                            cellToBlockX(gridEndCellX, 9),
                            cellToBlockZ(gridEndCellZ, 9)
                    ) + 8
            );
            int highestAquiferCellY = blockToCellY(highestAquiferY + 12) - -1;
            this.aquiferMaxY = cellToBlockY(highestAquiferCellY, 11) - 1;
        }

        /** Converts 3D cell coordinates into an index into the per-cell cache arrays. */
        private int index(int cellX, int cellY, int cellZ) {
            int localX = cellX - this.gridStartCellX;
            int localY = cellY - this.gridStartCellY;
            int localZ = cellZ - this.gridStartCellZ;
            return (localY * this.gridSizeZ + localZ) * this.gridSizeX + localX;
        }

        @Override
        public @Nullable BlockState apply(DensityFunction.NoisePos pos, double density) {
            // Step 0: solid terrain stays solid. Nothing to do.
            if (density > 0.0) {
                this.needsFluidTick = false;
                return null;
            }

            int blockX = pos.blockX();
            int blockY = pos.blockY();
            int blockZ = pos.blockZ();
            AquiferSampler.FluidLevel defaultFluidLevel = this.fluidLevelSampler.getFluidLevel(blockX, blockY, blockZ);

            // Step 1: way above any terrain, aquifers are irrelevant. Use the default fluid (sea level) or air.
            if (blockY > this.aquiferMaxY) {
                this.needsFluidTick = false;
                return defaultFluidLevel.getBlockState(blockY);
            }

            // Step 2: the global lava level (the lava lakes at the bottom of the world) overrides everything.
            if (defaultFluidLevel.getBlockState(blockY).isOf(Blocks.LAVA)) {
                this.needsFluidTick = false;
                return SharedConstants.DISABLE_FLUID_GENERATION ? Blocks.AIR.getDefaultState() : Blocks.LAVA.getDefaultState();
            }

            // Step 3: find the 4 closest aquifer sites to this block.
            // Convert the block position to a cell coordinate (with the grid's shifts). The 12 candidate
            // cells are the 2 (x) * 3 (y) * 2 (z) cells starting at this base cell.
            int baseCellX = blockToCellX(blockX + -5);
            int baseCellY = blockToCellY(blockY + 1);
            int baseCellZ = blockToCellZ(blockZ + -5);

            // Squared distances to the four closest sites (1 = closest) and the cell indices they belong to.
            int closestDistanceSq1 = Integer.MAX_VALUE;
            int closestDistanceSq2 = Integer.MAX_VALUE;
            int closestDistanceSq3 = Integer.MAX_VALUE;
            int closestDistanceSq4 = Integer.MAX_VALUE;
            int closestIndex1 = 0;
            int closestIndex2 = 0;
            int closestIndex3 = 0;
            int closestIndex4 = 0;

            for (int cellOffsetX = 0; cellOffsetX <= 1; cellOffsetX++) {
                for (int cellOffsetY = -1; cellOffsetY <= 1; cellOffsetY++) {
                    for (int cellOffsetZ = 0; cellOffsetZ <= 1; cellOffsetZ++) {
                        int neighborCellX = baseCellX + cellOffsetX;
                        int neighborCellY = baseCellY + cellOffsetY;
                        int neighborCellZ = baseCellZ + cellOffsetZ;
                        int neighborIndex = this.index(neighborCellX, neighborCellY, neighborCellZ);

                        // Get (or lazily generate and cache) this cell's random site position.
                        long cachedSitePosition = this.cellSitePositions[neighborIndex];
                        long sitePosition;
                        if (cachedSitePosition != Long.MAX_VALUE) {
                            sitePosition = cachedSitePosition;
                        } else {
                            // Seeded by the cell coordinates, so it is identical no matter which chunk asks first.
                            Random random = this.randomDeriver.split(neighborCellX, neighborCellY, neighborCellZ);
                            sitePosition = BlockPos.asLong(
                                    cellToBlockX(neighborCellX, random.nextInt(10)),
                                    cellToBlockY(neighborCellY, random.nextInt(9)),
                                    cellToBlockZ(neighborCellZ, random.nextInt(10))
                            );
                            this.cellSitePositions[neighborIndex] = sitePosition;
                        }

                        int deltaX = BlockPos.unpackLongX(sitePosition) - blockX;
                        int deltaY = BlockPos.unpackLongY(sitePosition) - blockY;
                        int deltaZ = BlockPos.unpackLongZ(sitePosition) - blockZ;
                        int distanceSq = deltaX * deltaX + deltaY * deltaY + deltaZ * deltaZ;

                        // Insert into the sorted "top 4 closest" list, shifting the farther entries down.
                        if (closestDistanceSq1 >= distanceSq) {
                            closestIndex4 = closestIndex3;
                            closestIndex3 = closestIndex2;
                            closestIndex2 = closestIndex1;
                            closestIndex1 = neighborIndex;
                            closestDistanceSq4 = closestDistanceSq3;
                            closestDistanceSq3 = closestDistanceSq2;
                            closestDistanceSq2 = closestDistanceSq1;
                            closestDistanceSq1 = distanceSq;
                        } else if (closestDistanceSq2 >= distanceSq) {
                            closestIndex4 = closestIndex3;
                            closestIndex3 = closestIndex2;
                            closestIndex2 = neighborIndex;
                            closestDistanceSq4 = closestDistanceSq3;
                            closestDistanceSq3 = closestDistanceSq2;
                            closestDistanceSq2 = distanceSq;
                        } else if (closestDistanceSq3 >= distanceSq) {
                            closestIndex4 = closestIndex3;
                            closestIndex3 = neighborIndex;
                            closestDistanceSq4 = closestDistanceSq3;
                            closestDistanceSq3 = distanceSq;
                        } else if (closestDistanceSq4 >= distanceSq) {
                            closestIndex4 = neighborIndex;
                            closestDistanceSq4 = distanceSq;
                        }
                    }
                }
            }

            // Step 4: the block "belongs" to the nearest site, so it uses that site's fluid.
            AquiferSampler.FluidLevel closestFluidLevel = this.getCellFluidLevel(closestIndex1);
            // How close the 1st and 2nd nearest sites are to each other (as seen from this block):
            // 1.0 = exactly on the border between them, <= 0 = comfortably inside the nearest site's region.
            double closestPairFactor = maxDistance(closestDistanceSq1, closestDistanceSq2);
            BlockState closestFluidState = closestFluidLevel.getBlockState(blockY);
            BlockState resultState = SharedConstants.DISABLE_FLUID_GENERATION ? Blocks.AIR.getDefaultState() : closestFluidState;

            if (closestPairFactor <= 0.0) {
                // Well inside one site's region: no wall needed, just return its fluid.
                // Only flag a fluid tick if we're still reasonably near a border with a different fluid level.
                if (closestPairFactor >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD) {
                    AquiferSampler.FluidLevel secondClosestFluidLevelForTick = this.getCellFluidLevel(closestIndex2);
                    this.needsFluidTick = !closestFluidLevel.equals(secondClosestFluidLevelForTick);
                } else {
                    this.needsFluidTick = false;
                }

                return resultState;
            } else {
                // Special case: water sitting directly on top of lava must always tick (so they interact).
                if (closestFluidState.isOf(Blocks.WATER)
                        && this.fluidLevelSampler.getFluidLevel(blockX, blockY - 1, blockZ).getBlockState(blockY - 1).isOf(Blocks.LAVA)) {
                    this.needsFluidTick = true;
                    return resultState;
                }

                // Step 5: we're near a border between sites. Between each pair of nearby sites with different
                // fluid levels, compute a "barrier" value; if terrain density + barrier > 0, the block becomes solid.
                // The cache avoids sampling the (expensive) barrier noise more than once for this block.
                MutableDouble barrierNoiseCache = new MutableDouble(Double.NaN);

                // Barrier between sites 1 and 2.
                AquiferSampler.FluidLevel secondClosestFluidLevel = this.getCellFluidLevel(closestIndex2);
                double densityAdjustment12 = closestPairFactor * this.calculateDensity(pos, barrierNoiseCache, closestFluidLevel, secondClosestFluidLevel);
                if (density + densityAdjustment12 > 0.0) {
                    this.needsFluidTick = false;
                    return null;
                }

                // Barrier between sites 1 and 3 (only if the 3rd site is also reasonably close).
                AquiferSampler.FluidLevel thirdClosestFluidLevel = this.getCellFluidLevel(closestIndex3);
                double pairFactor13 = maxDistance(closestDistanceSq1, closestDistanceSq3);
                if (pairFactor13 > 0.0) {
                    double densityAdjustment13 = closestPairFactor * pairFactor13
                            * this.calculateDensity(pos, barrierNoiseCache, closestFluidLevel, thirdClosestFluidLevel);
                    if (density + densityAdjustment13 > 0.0) {
                        this.needsFluidTick = false;
                        return null;
                    }
                }

                // Barrier between sites 2 and 3.
                double pairFactor23 = maxDistance(closestDistanceSq2, closestDistanceSq3);
                if (pairFactor23 > 0.0) {
                    double densityAdjustment23 = closestPairFactor * pairFactor23
                            * this.calculateDensity(pos, barrierNoiseCache, secondClosestFluidLevel, thirdClosestFluidLevel);
                    if (density + densityAdjustment23 > 0.0) {
                        this.needsFluidTick = false;
                        return null;
                    }
                }

                // Step 6: no barrier was placed, so this block is fluid. Decide whether it needs a fluid tick:
                // yes if it's next to a differing fluid level (sites 1/2, 2/3 or 1/3 differ and are close)...
                boolean fluidLevels12Differ = !closestFluidLevel.equals(secondClosestFluidLevel);
                boolean fluidLevels23DifferAndClose = pairFactor23 >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD
                        && !secondClosestFluidLevel.equals(thirdClosestFluidLevel);
                boolean fluidLevels13DifferAndClose = pairFactor13 >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD
                        && !closestFluidLevel.equals(thirdClosestFluidLevel);
                if (!fluidLevels12Differ && !fluidLevels23DifferAndClose && !fluidLevels13DifferAndClose) {
                    // ...or, as a last check, if the 4th-closest site is close and has a different fluid level.
                    this.needsFluidTick = pairFactor13 >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD
                            && maxDistance(closestDistanceSq1, closestDistanceSq4) >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD
                            && !closestFluidLevel.equals(this.getCellFluidLevel(closestIndex4));
                } else {
                    this.needsFluidTick = true;
                }

                return resultState;
            }
        }

        @Override
        public boolean needsFluidTick() {
            return this.needsFluidTick;
        }

        /**
         * Returns a "closeness" factor between two squared distances (distanceSqA is expected to be the smaller one):
         * 1.0 when they are equal (the point is equally far from both sites, i.e. right on their border),
         * decreasing linearly to 0.0 when the squared distances differ by 25, and negative beyond that.
         */
        private static double maxDistance(int distanceSqA, int distanceSqB) {
            double falloffRange = 25.0;
            return 1.0 - (distanceSqB - distanceSqA) / 25.0;
        }

        /**
         * Computes the "barrier" density between two neighbouring aquifer fluid levels at a position.
         * A positive result pushes the block toward being solid (a wall); a negative result lets fluid through.
         *
         * @param barrierNoiseCache holds the barrier noise value for this position once sampled (NaN = not sampled yet)
         */
        private double calculateDensity(
                DensityFunction.NoisePos pos,
                MutableDouble barrierNoiseCache,
                AquiferSampler.FluidLevel firstFluidLevel,
                AquiferSampler.FluidLevel secondFluidLevel
        ) {
            int blockY = pos.blockY();
            BlockState firstFluidState = firstFluidLevel.getBlockState(blockY);
            BlockState secondFluidState = secondFluidLevel.getBlockState(blockY);

            // Lava touching water: always a strong solid wall so they never mix directly.
            if ((!firstFluidState.isOf(Blocks.LAVA) || !secondFluidState.isOf(Blocks.WATER))
                    && (!firstFluidState.isOf(Blocks.WATER) || !secondFluidState.isOf(Blocks.LAVA))) {
                int fluidLevelDifference = Math.abs(firstFluidLevel.y() - secondFluidLevel.y());
                if (fluidLevelDifference == 0) {
                    // Same fluid height on both sides: no wall needed.
                    return 0.0;
                }

                // The wall is centered on the midpoint between the two fluid surfaces.
                double midpointY = 0.5 * (firstFluidLevel.y() + secondFluidLevel.y());
                double relativeY = blockY + 0.5 - midpointY; // > 0 above the midpoint, < 0 below it
                double halfDifference = fluidLevelDifference / 2.0;
                // Tuning constants for the vertical profile of the wall (same values appear as literals below).
                double upperOffset = 0.0;
                double upperOutsideDivisor = 2.5;
                double upperInsideDivisor = 1.5;
                double lowerOffset = 3.0;
                double lowerOutsideDivisor = 10.0;
                double lowerInsideDivisor = 3.0;
                // How far inside the band [midpoint - halfDifference, midpoint + halfDifference] this block is
                // (positive inside the band between the two fluid surfaces, negative outside it).
                double distanceFromBoundary = halfDifference - Math.abs(relativeY);

                // "pressure": a vertical profile that is strongest between the fluid surfaces and fades out
                // outside them. Above the midpoint it falls off faster than below it, so walls stay thicker
                // toward the bottom (the lower side gets a +3 offset and a gentler falloff).
                double pressure;
                if (relativeY > 0.0) {
                    double shiftedDistance = 0.0 + distanceFromBoundary;
                    if (shiftedDistance > 0.0) {
                        pressure = shiftedDistance / 1.5;
                    } else {
                        pressure = shiftedDistance / 2.5;
                    }
                } else {
                    double shiftedDistance = 3.0 + distanceFromBoundary;
                    if (shiftedDistance > 0.0) {
                        pressure = shiftedDistance / 3.0;
                    } else {
                        pressure = shiftedDistance / 10.0;
                    }
                }

                // Add noise so the wall is irregular. If |pressure| > 2 the noise could not change the outcome
                // anyway, so skip sampling it (an optimization), and sample it at most once per block via the cache.
                double barrierThreshold = 2.0;
                double barrierValue;
                if (!(pressure < -2.0) && !(pressure > 2.0)) {
                    double cachedBarrier = barrierNoiseCache.doubleValue();
                    if (Double.isNaN(cachedBarrier)) {
                        double sampledBarrier = this.barrierNoise.sample(pos);
                        barrierNoiseCache.setValue(sampledBarrier);
                        barrierValue = sampledBarrier;
                    } else {
                        barrierValue = cachedBarrier;
                    }
                } else {
                    barrierValue = 0.0;
                }

                return 2.0 * (barrierValue + pressure);
            } else {
                return 2.0;
            }
        }

        // --- Conversions between block coordinates and cell coordinates. ---
        // X/Z cells are 16 blocks wide; Y cells are 12 blocks tall.
        // "offsetInCell" is where inside the cell the result lands (used to place the random site point).

        private static int blockToCellX(int blockX) {
            return blockX >> 4;
        }

        private static int cellToBlockX(int cellX, int offsetInCell) {
            return (cellX << 4) + offsetInCell;
        }

        private static int blockToCellY(int blockY) {
            return Math.floorDiv(blockY, 12);
        }

        private static int cellToBlockY(int cellY, int offsetInCell) {
            return cellY * 12 + offsetInCell;
        }

        private static int blockToCellZ(int blockZ) {
            return blockZ >> 4;
        }

        private static int cellToBlockZ(int cellZ, int offsetInCell) {
            return (cellZ << 4) + offsetInCell;
        }

        /** Returns the cached fluid level for the cell at the given index, computing it from the cell's site position if needed. */
        private AquiferSampler.FluidLevel getCellFluidLevel(int cellIndex) {
            AquiferSampler.FluidLevel cachedFluidLevel = this.cellFluidLevels[cellIndex];
            if (cachedFluidLevel != null) {
                return cachedFluidLevel;
            }

            long sitePosition = this.cellSitePositions[cellIndex];
            AquiferSampler.FluidLevel computedFluidLevel = this.getFluidLevel(
                    BlockPos.unpackLongX(sitePosition), BlockPos.unpackLongY(sitePosition), BlockPos.unpackLongZ(sitePosition)
            );
            this.cellFluidLevels[cellIndex] = computedFluidLevel;
            return computedFluidLevel;
        }

        /**
         * Determines the fluid level for an aquifer site at the given position.
         * In order of priority:
         *  1. If the site is clearly above the ground, use the default fluid level (open air / sea level).
         *  2. If the site is near enough to the surface and there's fluid at the surface there, copy that fluid level
         *     (so lakes and oceans continue underground near their edges).
         *  3. Otherwise decide using noise: default level, a noise-randomized cave level, or "no fluid".
         */
        private AquiferSampler.FluidLevel getFluidLevel(int blockX, int blockY, int blockZ) {
            AquiferSampler.FluidLevel defaultFluidLevel = this.fluidLevelSampler.getFluidLevel(blockX, blockY, blockZ);
            int lowestSurfaceY = Integer.MAX_VALUE;
            int upperY = blockY + 12;
            int lowerY = blockY - 12;
            boolean originColumnHasSurfaceFluid = false;

            // Look at the terrain surface at several nearby columns (offsets measured in chunks).
            for (int[] chunkOffset : SURFACE_SAMPLE_CHUNK_OFFSETS) {
                int sampleX = blockX + ChunkSectionPos.getBlockCoord(chunkOffset[0]);
                int sampleZ = blockZ + ChunkSectionPos.getBlockCoord(chunkOffset[1]);
                int surfaceY = this.chunkNoiseSampler.estimateSurfaceHeight(sampleX, sampleZ);
                int aquiferSurfaceY = this.addSurfaceMargin(surfaceY);
                boolean isOriginColumn = chunkOffset[0] == 0 && chunkOffset[1] == 0;

                // The site is entirely above the ground at its own column: it's open air, use the default level.
                if (isOriginColumn && lowerY > aquiferSurfaceY) {
                    return defaultFluidLevel;
                }

                // Is the site within reach of this column's surface?
                boolean isNearSurface = upperY > aquiferSurfaceY;
                if (isNearSurface || isOriginColumn) {
                    AquiferSampler.FluidLevel surfaceFluidLevel = this.fluidLevelSampler.getFluidLevel(sampleX, aquiferSurfaceY, sampleZ);
                    if (!surfaceFluidLevel.getBlockState(aquiferSurfaceY).isAir()) {
                        // There is fluid (e.g. ocean water) at the surface of this column.
                        if (isOriginColumn) {
                            originColumnHasSurfaceFluid = true;
                        }

                        if (isNearSurface) {
                            // Near a flooded surface: copy that surface's fluid level.
                            return surfaceFluidLevel;
                        }
                    }
                }

                lowestSurfaceY = Math.min(lowestSurfaceY, surfaceY);
            }

            // Deep enough underground: decide the level (and fluid type) using noise.
            int fluidY = this.getFluidBlockY(blockX, blockY, blockZ, defaultFluidLevel, lowestSurfaceY, originColumnHasSurfaceFluid);
            return new AquiferSampler.FluidLevel(fluidY, this.getFluidBlockState(blockX, blockY, blockZ, defaultFluidLevel, fluidY));
        }

        /** Adds the aquifer margin (8 blocks) above an estimated surface height. */
        private int addSurfaceMargin(int surfaceY) {
            return surfaceY + 8;
        }

        /**
         * Chooses the fluid surface height for a deep aquifer site, using the "floodedness" noise:
         *  - very flooded: use the default fluid level (e.g. sea level, so caves under oceans fill up),
         *  - moderately flooded: use a random cave-specific level from {@link #getNoiseBasedFluidLevel},
         *  - not flooded: use a level far below the world, i.e. the aquifer is empty.
         * Being close to a flooded surface lowers the flooding thresholds, so flooding is more likely nearby.
         * The Deep Dark biome is never flooded.
         */
        private int getFluidBlockY(
                int blockX,
                int blockY,
                int blockZ,
                AquiferSampler.FluidLevel defaultFluidLevel,
                int surfaceHeightEstimate,
                boolean originColumnHasSurfaceFluid
        ) {
            DensityFunction.UnblendedNoisePos noisePos = new DensityFunction.UnblendedNoisePos(blockX, blockY, blockZ);
            double noiseBasedFluidScore; // > 0 -> use a noise-based cave fluid level
            double defaultFluidScore; // > 0 -> use the default fluid level
            if (VanillaBiomeParameters.inDeepDarkParameters(this.erosionDensityFunction, this.depthDensityFunction, noisePos)) {
                // Deep Dark: both scores are negative, so aquifers here are always empty.
                noiseBasedFluidScore = -1.0;
                defaultFluidScore = -1.0;
            } else {
                int depthBelowSurface = surfaceHeightEstimate + 8 - blockY;
                int maxDepth = 64;
                // 1.0 right at the surface, fading to 0.0 at 64 blocks deep (and 0 if the column has no surface fluid).
                double surfaceProximity = originColumnHasSurfaceFluid ? MathHelper.clampedMap(depthBelowSurface, 0.0, 64.0, 1.0, 0.0) : 0.0;
                double floodednessSample = MathHelper.clamp(this.fluidLevelFloodednessNoise.sample(noisePos), -1.0, 1.0);
                // Thresholds the noise must beat; they are easier to beat (lower) the closer we are to a flooded surface.
                double defaultFluidThreshold = MathHelper.map(surfaceProximity, 1.0, 0.0, -0.3, 0.8);
                double noiseFluidThreshold = MathHelper.map(surfaceProximity, 1.0, 0.0, -0.8, 0.4);
                noiseBasedFluidScore = floodednessSample - noiseFluidThreshold;
                defaultFluidScore = floodednessSample - defaultFluidThreshold;
            }

            int fluidY;
            if (defaultFluidScore > 0.0) {
                fluidY = defaultFluidLevel.y();
            } else if (noiseBasedFluidScore > 0.0) {
                fluidY = this.getNoiseBasedFluidLevel(blockX, blockY, blockZ, surfaceHeightEstimate);
            } else {
                // Sentinel meaning "far below the world": nothing is below this level, so no fluid at all.
                fluidY = DimensionType.field_35479;
            }

            return fluidY;
        }

        /**
         * Picks a random fluid height for a cave aquifer. Space is split into 16 x 40 x 16 blocks; each
         * region has a base level at the middle of its vertical range (y = band * 40 + 20), shifted up or
         * down by up to ~10 blocks using noise (snapped to multiples of 3). The result never exceeds the surface.
         */
        private int getNoiseBasedFluidLevel(int blockX, int blockY, int blockZ, int surfaceHeightEstimate) {
            int cellSizeXZ = 16;
            int cellSizeY = 40;
            int cellX = Math.floorDiv(blockX, 16);
            int cellY = Math.floorDiv(blockY, 40);
            int cellZ = Math.floorDiv(blockZ, 16);
            int cellCenterY = cellY * 40 + 20;
            int spreadAmplitude = 10;
            double spreadOffset = this.fluidLevelSpreadNoise.sample(new DensityFunction.UnblendedNoisePos(cellX, cellY, cellZ)) * 10.0;
            int roundedSpreadOffset = MathHelper.roundDownToMultiple(spreadOffset, 3);
            int noiseFluidY = cellCenterY + roundedSpreadOffset;
            return Math.min(surfaceHeightEstimate, noiseFluidY);
        }

        /**
         * Chooses the fluid type for an aquifer: normally the default fluid (water), but deep aquifers
         * (level at or below y = -10) become lava where the lava noise (sampled per 64 x 40 x 64 region) is strong.
         */
        private BlockState getFluidBlockState(int blockX, int blockY, int blockZ, AquiferSampler.FluidLevel defaultFluidLevel, int fluidY) {
            BlockState fluidState = defaultFluidLevel.state();
            if (fluidY <= -10 && fluidY != DimensionType.field_35479 && defaultFluidLevel.state() != Blocks.LAVA.getDefaultState()) {
                int cellSizeXZ = 64;
                int cellSizeY = 40;
                int cellX = Math.floorDiv(blockX, 64);
                int cellY = Math.floorDiv(blockY, 40);
                int cellZ = Math.floorDiv(blockZ, 64);
                double lavaNoiseValue = this.fluidTypeNoise.sample(new DensityFunction.UnblendedNoisePos(cellX, cellY, cellZ));
                if (Math.abs(lavaNoiseValue) > 0.3) {
                    fluidState = Blocks.LAVA.getDefaultState();
                }
            }

            return fluidState;
        }
    }
}