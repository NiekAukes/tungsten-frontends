use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType, Spline, SplineValue};
use crate::shrink::ShrinkMethod;

/// Unwraps `Cache2d` and `FlatCache` nodes anywhere in the tree, replacing
/// them with their inner argument.
///
/// `Cache2d` may only ever occur directly under a `FlatCache`, so removing a
/// `FlatCache` whose argument is a `Cache2d` must strip both at once instead
/// of orphaning the `Cache2d`.
pub struct RemoveWrappers {
    pub exhausted: bool,
}

impl RemoveWrappers {
    fn is_candidate(density: &DensityType) -> bool {
        matches!(density, DensityType::Cache2d { .. } | DensityType::FlatCache { .. })
    }

    /// The density that should replace `density` if it's removed.
    fn removed<'m>(density: &DensityType<'m>) -> Density<'m> {
        match density {
            DensityType::Cache2d { argument } => *argument,
            DensityType::FlatCache { argument } => match &**argument {
                // Never orphan a Cache2d: strip it along with its FlatCache.
                DensityType::Cache2d { argument: inner } => *inner,
                _ => *argument,
            },
            _ => unreachable!("removed called on a non-wrapper density"),
        }
    }

    /// Recursively counts all removable wrapper nodes in the tree.
    fn count_candidates<'m>(density: Density<'m>) -> u32 {
        let mut count = if Self::is_candidate(&density) { 1 } else { 0 };

        match &*density {
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => {
                count += Self::count_candidates(*left);
                count += Self::count_candidates(*right);
            }

            DensityType::Cache2d { argument }
            | DensityType::FlatCache { argument }
            | DensityType::NamedDensityReference { argument, .. }
            | DensityType::Squeeze { argument }
            | DensityType::Interpolated { argument }
            | DensityType::CacheOnce { argument }
            | DensityType::Abs { argument }
            | DensityType::Square { argument }
            | DensityType::Cube { argument }
            | DensityType::XNegative { argument, .. }
            | DensityType::Clamp { input: argument, .. }
            | DensityType::WeirdScaledSampler { input: argument, .. } => {
                count += Self::count_candidates(*argument);
            }

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

            DensityType::Spline { spline } => {
                count += Self::count_spline_candidates(spline);
            }

            // Leaf nodes (Noise, EndIslands, Const, ShiftA, ShiftB, YClampedGradient, OldBlendedNoise)
            _ => {}
        }

        count
    }

    /// Helper to count candidates hidden deeply inside spline point arrays.
    fn count_spline_candidates<'m>(spline: Spline<'m>) -> u32 {
        let mut count = Self::count_candidates(spline.coordinate);
        for point in spline.spline_points {
            if let SplineValue::Spline(inner_spline) = &point.value {
                count += Self::count_spline_candidates(inner_spline);
            }
        }
        count
    }

    /// Walks the AST and rebuilds it, removing exactly the `target_strike`-th
    /// wrapper node.
    fn replace_nth<'m>(
        arena: &'m Bump,
        density: Density<'m>,
        target_strike: u32,
        current_strike: &mut u32,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> Density<'m> {
        if Self::is_candidate(&density) {
            if *current_strike == target_strike {
                *current_strike += 1;
                return Self::removed(&density);
            }
            *current_strike += 1;
        }

        match &*density {
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

            DensityType::Cache2d { argument } | DensityType::FlatCache { argument } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) {
                    density
                } else {
                    let new_dt = match &*density {
                        DensityType::Cache2d { .. } => DensityType::Cache2d { argument: new_arg },
                        DensityType::FlatCache { .. } => DensityType::FlatCache { argument: new_arg },
                        _ => unreachable!(),
                    };
                    intern(arena, new_dt)
                }
            }

            DensityType::Squeeze { argument }
            | DensityType::Interpolated { argument }
            | DensityType::CacheOnce { argument }
            | DensityType::Abs { argument }
            | DensityType::Square { argument }
            | DensityType::Cube { argument } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) {
                    density
                } else {
                    let new_dt = match &*density {
                        DensityType::Squeeze { .. } => DensityType::Squeeze { argument: new_arg },
                        DensityType::Interpolated { .. } => DensityType::Interpolated { argument: new_arg },
                        DensityType::CacheOnce { .. } => DensityType::CacheOnce { argument: new_arg },
                        DensityType::Abs { .. } => DensityType::Abs { argument: new_arg },
                        DensityType::Square { .. } => DensityType::Square { argument: new_arg },
                        DensityType::Cube { .. } => DensityType::Cube { argument: new_arg },
                        _ => unreachable!(),
                    };
                    intern(arena, new_dt)
                }
            }

            DensityType::Clamp { input, min, max } => {
                let new_input = Self::replace_nth(arena, *input, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_input, &**input) { density } else { intern(arena, DensityType::Clamp { input: new_input, min: *min, max: *max }) }
            }
            DensityType::XNegative { argument, neg_x_multiplier } => {
                let new_arg = Self::replace_nth(arena, *argument, target_strike, current_strike, intern);
                if std::ptr::eq(&*new_arg, &**argument) { density } else { intern(arena, DensityType::XNegative { argument: new_arg, neg_x_multiplier: *neg_x_multiplier }) }
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
        spline: Spline<'m>,
        target_strike: u32,
        current_strike: &mut u32,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> Spline<'m> {
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
                    let mut new_pt = pt.clone();
                    new_pt.value = SplineValue::Spline(new_inner);
                    new_points.push(new_pt);
                    continue;
                }
            }
            if points_changed {
                new_points.push(pt.clone());
            }
        }

        if !points_changed && std::ptr::eq(&*new_coord, &*spline.coordinate) {
            spline
        } else {
            let points = if points_changed { new_points } else { spline.spline_points.to_vec() };
            arena.alloc(crate::parse::model::SplineType {
                coordinate: new_coord,
                spline_points: arena.alloc_slice_clone(&points),
            })
        }
    }
}

impl<'m> ShrinkMethod<'m> for RemoveWrappers {
    fn name(&self) -> &str {
        "remove_wrappers"
    }

    fn can_shrink(&mut self, remaining_strikes: u32, source: DensitySource) -> (bool, u32) {
        if self.exhausted {
            return (false, 0);
        }

        let root_density = *source.get_density();
        let candidate_count = Self::count_candidates(root_density);

        if remaining_strikes < candidate_count {
            (true, remaining_strikes)
        } else {
            self.exhausted = true;
            (false, candidate_count)
        }
    }

    fn perform_shrink(&mut self, arena: &'m Bump, remaining_strikes: u32, source: DensitySource<'m>) -> DensitySource<'m> {
        let intern = |arena: &'m Bump, density_type: DensityType<'m>| -> Density<'m> { arena.alloc(density_type) };

        match source {
            DensitySource::MultiSamplingDensity { density, dimensions } => {
                let mut current_strike = 0;
                let new_density = Self::replace_nth(arena, density, remaining_strikes, &mut current_strike, &intern);
                DensitySource::MultiSamplingDensity { density: new_density, dimensions }
            }
            DensitySource::SingleSamplingDensity { density } => {
                let mut current_strike = 0;
                let new_density = Self::replace_nth(arena, density, remaining_strikes, &mut current_strike, &intern);
                DensitySource::SingleSamplingDensity { density: new_density }
            }
        }
    }

    fn reenable(&mut self) {
        self.exhausted = false;
    }
}
