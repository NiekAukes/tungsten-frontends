use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType, SplinePoint, SplineType, SplineValue};
use crate::shrink::ShrinkMethod;

/// Replaces a single, specific subtree anywhere in the AST with `Const(0.0)`.
pub struct ReplaceWithConstant {
    pub exhausted: bool,
}

impl ReplaceWithConstant {
    /// Checks if a density is already a zero constant to prevent redundant shrinking.
    fn is_zero(density: Density<'_>) -> bool {
        matches!(&*density, DensityType::Const(c) if *c == 0.0)
    }

    /// Recursively counts all nodes in the tree that can be replaced by 0.0.
    fn count_candidates<'m>(density: Density<'m>) -> u32 {
        if Self::is_zero(density) {
            return 0; // Skip counting this node and its children (it has none).
        }

        let mut count = 1; // 1 for the current node itself

        match &*density {
            // Binary Nodes
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => {
                count += Self::count_candidates(*left);
                count += Self::count_candidates(*right);
            }
            
            // Unary Nodes & Wrappers
            DensityType::Cache2d { argument }
            | DensityType::Squeeze { argument }
            | DensityType::Interpolated { argument }
            | DensityType::FlatCache { argument }
            | DensityType::CacheOnce { argument }
            | DensityType::Abs { argument }
            | DensityType::Square { argument }
            | DensityType::Cube { argument }
            | DensityType::NamedDensityReference { argument, .. }
            | DensityType::XNegative { argument, .. }
            | DensityType::Clamp { input: argument, .. }
            | DensityType::WeirdScaledSampler { input: argument, .. } => {
                count += Self::count_candidates(*argument);
            }
            
            // Ternary Nodes
            DensityType::RangeChoice {
                input,
                when_in_range,
                when_out_of_range,
                ..
            } => {
                count += Self::count_candidates(*input);
                count += Self::count_candidates(*when_in_range);
                count += Self::count_candidates(*when_out_of_range);
            }
            DensityType::ShiftedNoise {
                shift_x,
                shift_y,
                shift_z,
                ..
            } => {
                count += Self::count_candidates(*shift_x);
                count += Self::count_candidates(*shift_y);
                count += Self::count_candidates(*shift_z);
            }
            
            // Deep Spline Traversal
            DensityType::Spline { spline } => {
                count += Self::count_spline_candidates(spline);
            }
            
            // Leaf Nodes (Noise, EndIslands, Const, ShiftA, ShiftB, YClampedGradient, OldBlendedNoise)
            _ => {}
        }

        count
    }

    /// Helper to count candidates hidden deeply inside spline point arrays.
    fn count_spline_candidates<'m>(spline: &SplineType<'m>) -> u32 {
        let mut count = Self::count_candidates(spline.coordinate);
        for point in spline.spline_points {
            if let SplineValue::Spline(inner_spline) = &point.value {
                count += Self::count_spline_candidates(inner_spline);
            }
        }
        count
    }

    /// Walks the AST and rebuilds it, replacing exactly the `target_strike` node with 0.0.
    fn replace_nth<'m>(
        arena: &'m Bump,
        density: Density<'m>,
        target_strike: u32,
        current_strike: &mut u32,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> Density<'m> {
        if Self::is_zero(density) {
            return density;
        }

        if *current_strike == target_strike {
            *current_strike += 1;
            return intern(arena, DensityType::Const(0.0));
        }

        *current_strike += 1;

        match &*density {
            // Grouped Binary Nodes
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => {
                let new_left = Self::replace_nth(arena, *left, target_strike, current_strike, intern);
                let new_right = Self::replace_nth(arena, *right, target_strike, current_strike, intern);
                
                if std::ptr::eq(&*new_left, &**left) && std::ptr::eq(&*new_right, &**right) {
                    density
                } else {
                    let new_dt = match &*density {
                        DensityType::Add { .. } => DensityType::Add { left: new_left, right: new_right },
                        DensityType::Multiply { .. } => DensityType::Multiply { left: new_left, right: new_right },
                        DensityType::Min { .. } => DensityType::Min { left: new_left, right: new_right },
                        DensityType::Max { .. } => DensityType::Max { left: new_left, right: new_right },
                        _ => unreachable!(),
                    };
                    intern(arena, new_dt)
                }
            }
            
            // Grouped Simple Unary Nodes
            DensityType::Cache2d { argument }
            | DensityType::Squeeze { argument }
            | DensityType::Interpolated { argument }
            | DensityType::FlatCache { argument }
            | DensityType::CacheOnce { argument }
            | DensityType::Abs { argument }
            | DensityType::Square { argument }
            | DensityType::Cube { argument } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) {
                    density
                } else {
                    let new_dt = match &*density {
                        DensityType::Cache2d { .. } => DensityType::Cache2d { argument: new_arg },
                        DensityType::Squeeze { .. } => DensityType::Squeeze { argument: new_arg },
                        DensityType::Interpolated { .. } => DensityType::Interpolated { argument: new_arg },
                        DensityType::FlatCache { .. } => DensityType::FlatCache { argument: new_arg },
                        DensityType::CacheOnce { .. } => DensityType::CacheOnce { argument: new_arg },
                        DensityType::Abs { .. } => DensityType::Abs { argument: new_arg },
                        DensityType::Square { .. } => DensityType::Square { argument: new_arg },
                        DensityType::Cube { .. } => DensityType::Cube { argument: new_arg },
                        _ => unreachable!(),
                    };
                    intern(arena, new_dt)
                }
            }

            // Complex Unary Wrappers (Cannot be easily grouped due to distinct fields)
            DensityType::Clamp { input, min, max } => {
                let new_input = Self::replace_nth(arena, *input, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_input, &**input) { density } else { intern(arena, DensityType::Clamp { input: new_input, min: *min, max: *max }) }
            }
            DensityType::XNegative { argument, neg_x_multiplier } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) { density } else { intern(arena, DensityType::XNegative { argument: new_arg, neg_x_multiplier: *neg_x_multiplier }) }
            }
            DensityType::NamedDensityReference { name, argument } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) { density } else { intern(arena, DensityType::NamedDensityReference { name: *name, argument: new_arg }) }
            }
            DensityType::WeirdScaledSampler { input, noise_name, noise_to_sample, rarity_value_mapper } => {
                let new_input = Self::replace_nth(arena, *input, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_input, &**input) { 
                    density 
                } else { 
                    intern(arena, DensityType::WeirdScaledSampler { 
                        input: new_input, noise_name: noise_name.clone(), noise_to_sample: *noise_to_sample, rarity_value_mapper: rarity_value_mapper.clone() 
                    }) 
                }
            }

            // Ternary Nodes
            DensityType::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
                let new_input = Self::replace_nth(arena, *input, target_strike, current_strike, intern);
                let new_in = Self::replace_nth(arena, *when_in_range, target_strike, current_strike, intern);
                let new_out = Self::replace_nth(arena, *when_out_of_range, target_strike, current_strike, intern);
                
                if std::ptr::eq(&*new_input, &**input) && std::ptr::eq(&*new_in, &**when_in_range) && std::ptr::eq(&*new_out, &**when_out_of_range) {
                    density
                } else {
                    intern(arena, DensityType::RangeChoice { input: new_input, min_inclusive: *min_inclusive, max_exclusive: *max_exclusive, when_in_range: new_in, when_out_of_range: new_out })
                }
            }
            DensityType::ShiftedNoise { name, noise, shift_x, shift_y, shift_z, xz_scale, y_scale } => {
                let new_x = Self::replace_nth(arena, *shift_x, target_strike, current_strike, intern);
                let new_y = Self::replace_nth(arena, *shift_y, target_strike, current_strike, intern);
                let new_z = Self::replace_nth(arena, *shift_z, target_strike, current_strike, intern);
                
                if std::ptr::eq(&*new_x, &**shift_x) && std::ptr::eq(&*new_y, &**shift_y) && std::ptr::eq(&*new_z, &**shift_z) {
                    density
                } else {
                    intern(arena, DensityType::ShiftedNoise { name: name.clone(), noise: *noise, shift_x: new_x, shift_y: new_y, shift_z: new_z, xz_scale: *xz_scale, y_scale: *y_scale })
                }
            }

            // Complex Spline Traversal
            DensityType::Spline { spline } => {
                let new_spline = Self::replace_nth_spline(arena, *spline, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_spline, &**spline) {
                    density
                } else {
                    intern(arena, DensityType::Spline { spline: new_spline })
                }
            }
            
            _ => density,
        }
    }

    /// Helper to rebuild inner splines and their arrays without leaking memory.
    fn replace_nth_spline<'m>(
        arena: &'m Bump,
        spline: crate::parse::model::Spline<'m>,
        target_strike: u32,
        current_strike: &mut u32,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> crate::parse::model::Spline<'m> {
        let new_coord = Self::replace_nth(arena, spline.coordinate, target_strike, current_strike, intern);
        
        let mut points_changed = false;
        let mut new_points = Vec::new();
        
        for (i, pt) in spline.spline_points.iter().enumerate() {
            if let SplineValue::Spline(inner_s) = &pt.value {
                let new_inner = Self::replace_nth_spline(arena, *inner_s, target_strike, current_strike, intern);
                
                if !std::ptr::eq(&*new_inner, &**inner_s) {
                    if !points_changed {
                        new_points.extend_from_slice(&spline.spline_points[..i]);
                        points_changed = true;
                    }
                    new_points.push(SplinePoint {
                        derivative: pt.derivative,
                        location: pt.location,
                        value: SplineValue::Spline(new_inner),
                    });
                    continue;
                }
            }
            if points_changed {
                new_points.push(pt.clone());
            }
        }
        
        if std::ptr::eq(&*new_coord, &*spline.coordinate) && !points_changed {
            return spline; // No changes, return the identical pointer
        }
        
        let final_points = if points_changed {
            arena.alloc_slice_clone(&new_points)
        } else {
            spline.spline_points
        };
        
        arena.alloc(SplineType {
            coordinate: new_coord,
            spline_points: final_points,
        })
    }
}

impl<'m> ShrinkMethod<'m> for ReplaceWithConstant {
    fn name(&self) -> &str {
        "replace_with_constant"
    }

    fn can_shrink(&mut self, remaining_strikes: u32, source: DensitySource) -> (bool, u32) {
        if self.exhausted {
            return (false, 0);
        }

        let root_density = match source {
            DensitySource::MultiSamplingDensity { density, .. } => density,
            DensitySource::SingleSamplingDensity { density } => density,
        };

        let total_candidates = Self::count_candidates(root_density);

        if remaining_strikes < total_candidates {
            (true, 0) // Fits seamlessly with the exhaustion loop in mod.rs[cite: 1]
        } else {
            self.exhausted = true;
            (false, total_candidates)
        }
    }

    fn perform_shrink(
        &mut self,
        arena: &'m Bump,
        remaining_strikes: u32,
        source: DensitySource<'m>,
    ) -> DensitySource<'m> {
        let (root_density, dimensions) = match source {
            DensitySource::MultiSamplingDensity { density, dimensions } => (density, Some(dimensions)),
            DensitySource::SingleSamplingDensity { density } => (density, None),
        };

        // Initialize the interner wrapping logic exactly once
        let interner = |arena: &'m Bump, dt: DensityType<'m>| -> Density<'m> {
            arena.alloc(dt)
        };

        let mut current_strike = 0;
        let new_density = Self::replace_nth(
            arena,
            root_density,
            remaining_strikes,
            &mut current_strike,
            &interner,
        );

        match dimensions {
            Some(dim) => DensitySource::MultiSamplingDensity {
                density: new_density,
                dimensions: dim,
            },
            None => DensitySource::SingleSamplingDensity {
                density: new_density,
            },
        }
    }
}