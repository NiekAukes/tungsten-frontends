use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType, Spline, SplineValue};
use crate::shrink::ShrinkMethod;

/// Unwraps every `NamedDensityReference` debug label in the tree at once,
/// except the outermost one at the root (the current label added by the
/// `Shrinker`), which must never be removed.
///
/// Stripping all of them in a single shrink means this method only ever
/// needs to run once, instead of once per leftover label.
///
/// Run first so later shrink methods don't waste strikes navigating around
/// leftover debug labels from previous shrink iterations.
pub struct RemoveNamedReferences {
    pub exhausted: bool,
}

impl RemoveNamedReferences {
    /// Whether any non-root `NamedDensityReference` exists in the tree.
    fn has_any(density: Density, is_root: bool) -> bool {
        if !is_root && matches!(&*density, DensityType::NamedDensityReference { .. }) {
            return true;
        }

        match &*density {
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => {
                Self::has_any(*left, false) || Self::has_any(*right, false)
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
            | DensityType::WeirdScaledSampler { input: argument, .. } => Self::has_any(*argument, false),

            DensityType::RangeChoice {
                input,
                when_in_range,
                when_out_of_range,
                ..
            } => {
                Self::has_any(*input, false)
                    || Self::has_any(*when_in_range, false)
                    || Self::has_any(*when_out_of_range, false)
            }
            DensityType::ShiftedNoise {
                shift_x,
                shift_y,
                shift_z,
                ..
            } => Self::has_any(*shift_x, false) || Self::has_any(*shift_y, false) || Self::has_any(*shift_z, false),

            DensityType::Spline { spline } => Self::has_any_in_spline(spline),

            // Leaf nodes (Noise, EndIslands, Const, ShiftA, ShiftB, YClampedGradient, OldBlendedNoise)
            _ => false,
        }
    }

    /// Helper to look for candidates hidden deeply inside spline point arrays.
    fn has_any_in_spline<'m>(spline: Spline<'m>) -> bool {
        if Self::has_any(spline.coordinate, false) {
            return true;
        }
        spline.spline_points.iter().any(|point| match &point.value {
            SplineValue::Spline(inner_spline) => Self::has_any_in_spline(inner_spline),
            SplineValue::Const(_) => false,
        })
    }

    /// Walks the AST and rebuilds it, stripping every non-root
    /// `NamedDensityReference` node in a single pass.
    fn strip_all<'m>(
        arena: &'m Bump,
        density: Density<'m>,
        is_root: bool,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> Density<'m> {
        if !is_root {
            if let DensityType::NamedDensityReference { argument, .. } = &*density {
                return Self::strip_all(arena, *argument, false, intern);
            }
        }

        match &*density {
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => {
                let new_left = Self::strip_all(arena, *left, false, intern);
                let new_right = Self::strip_all(arena, *right, false, intern);

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

            DensityType::NamedDensityReference { name, argument } => {
                let new_arg = Self::strip_all(arena, *argument, false, intern);
                if std::ptr::eq(&*new_arg, &**argument) {
                    density
                } else {
                    intern(arena, DensityType::NamedDensityReference { name: *name, argument: new_arg })
                }
            }

            DensityType::Cache2d { argument }
            | DensityType::FlatCache { argument }
            | DensityType::Squeeze { argument }
            | DensityType::Interpolated { argument }
            | DensityType::CacheOnce { argument }
            | DensityType::Abs { argument }
            | DensityType::Square { argument }
            | DensityType::Cube { argument } => {
                let new_arg = Self::strip_all(arena, *argument, false, intern);
                if std::ptr::eq(&*new_arg, &**argument) {
                    density
                } else {
                    let new_dt = match &*density {
                        DensityType::Cache2d { .. } => DensityType::Cache2d { argument: new_arg },
                        DensityType::FlatCache { .. } => DensityType::FlatCache { argument: new_arg },
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
                let new_input = Self::strip_all(arena, *input, false, intern);
                if std::ptr::eq(&*new_input, &**input) { density } else { intern(arena, DensityType::Clamp { input: new_input, min: *min, max: *max }) }
            }
            DensityType::XNegative { argument, neg_x_multiplier } => {
                let new_arg = Self::strip_all(arena, *argument, false, intern);
                if std::ptr::eq(&*new_arg, &**argument) { density } else { intern(arena, DensityType::XNegative { argument: new_arg, neg_x_multiplier: *neg_x_multiplier }) }
            }
            DensityType::WeirdScaledSampler { input, noise_name, noise_to_sample, rarity_value_mapper } => {
                let new_input = Self::strip_all(arena, *input, false, intern);
                if std::ptr::eq(&*new_input, &**input) {
                    density
                } else {
                    intern(arena, DensityType::WeirdScaledSampler {
                        input: new_input, noise_name: noise_name.clone(), noise_to_sample: *noise_to_sample, rarity_value_mapper: rarity_value_mapper.clone()
                    })
                }
            }

            DensityType::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
                let new_input = Self::strip_all(arena, *input, false, intern);
                let new_in = Self::strip_all(arena, *when_in_range, false, intern);
                let new_out = Self::strip_all(arena, *when_out_of_range, false, intern);

                if std::ptr::eq(&*new_input, &**input) && std::ptr::eq(&*new_in, &**when_in_range) && std::ptr::eq(&*new_out, &**when_out_of_range) {
                    density
                } else {
                    intern(arena, DensityType::RangeChoice { input: new_input, min_inclusive: *min_inclusive, max_exclusive: *max_exclusive, when_in_range: new_in, when_out_of_range: new_out })
                }
            }
            DensityType::ShiftedNoise { name, noise, shift_x, shift_y, shift_z, xz_scale, y_scale } => {
                let new_x = Self::strip_all(arena, *shift_x, false, intern);
                let new_y = Self::strip_all(arena, *shift_y, false, intern);
                let new_z = Self::strip_all(arena, *shift_z, false, intern);

                if std::ptr::eq(&*new_x, &**shift_x) && std::ptr::eq(&*new_y, &**shift_y) && std::ptr::eq(&*new_z, &**shift_z) {
                    density
                } else {
                    intern(arena, DensityType::ShiftedNoise { name: name.clone(), noise: *noise, shift_x: new_x, shift_y: new_y, shift_z: new_z, xz_scale: *xz_scale, y_scale: *y_scale })
                }
            }

            DensityType::Spline { spline } => {
                let new_spline = Self::strip_all_spline(arena, *spline, intern);
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
    fn strip_all_spline<'m>(
        arena: &'m Bump,
        spline: Spline<'m>,
        intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
    ) -> Spline<'m> {
        let new_coord = Self::strip_all(arena, spline.coordinate, false, intern);

        let mut points_changed = false;
        let mut new_points = Vec::new();

        for (i, pt) in spline.spline_points.iter().enumerate() {
            if let SplineValue::Spline(inner_s) = &pt.value {
                let new_inner = Self::strip_all_spline(arena, *inner_s, intern);

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

impl<'m> ShrinkMethod<'m> for RemoveNamedReferences {
    fn name(&self) -> &str {
        "remove_named_references"
    }

    fn can_shrink(&mut self, remaining_strikes: u32, source: DensitySource) -> (bool, u32) {
        if self.exhausted {
            return (false, 0);
        }

        // There's only ever one possible action: strip every non-root
        // reference at once.
        let root_density = *source.get_density();
        let candidate_count = if Self::has_any(root_density, true) { 1 } else { 0 };

        if remaining_strikes < candidate_count {
            (true, remaining_strikes)
        } else {
            self.exhausted = true;
            (false, candidate_count)
        }
    }

    fn perform_shrink(&mut self, arena: &'m Bump, _remaining_strikes: u32, source: DensitySource<'m>) -> DensitySource<'m> {
        let intern = |arena: &'m Bump, density_type: DensityType<'m>| -> Density<'m> { arena.alloc(density_type) };

        match source {
            DensitySource::MultiSamplingDensity { density, dimensions } => {
                let new_density = Self::strip_all(arena, density, true, &intern);
                DensitySource::MultiSamplingDensity { density: new_density, dimensions }
            }
            DensitySource::SingleSamplingDensity { density } => {
                let new_density = Self::strip_all(arena, density, true, &intern);
                DensitySource::SingleSamplingDensity { density: new_density }
            }
        }
    }

    fn reenable(&mut self) {
        self.exhausted = false;
    }
}
