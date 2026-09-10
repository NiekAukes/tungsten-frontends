use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType};
use crate::shrink::ShrinkMethod;

/// Takes an immediate subtree of the density function, effectively reducing its complexity, without adding any memory overhead.
pub struct TakeSubtree {
    /// Indicates whether this shrink method has been exhausted and can no longer shrink.
    /// Prevents further shrinking once this method has been exhausted.
    pub exhausted: bool,
}

impl TakeSubtree {
    /// Extracts only the immediate child densities from the root node.
    fn get_immediate_children<'m>(density: Density<'m>) -> Vec<Density<'m>> {
        match &*density {
            DensityType::Add { left, right }
            | DensityType::Multiply { left, right }
            | DensityType::Min { left, right }
            | DensityType::Max { left, right } => vec![*left, *right],

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
            | DensityType::WeirdScaledSampler { input: argument, .. } => vec![*argument],

            DensityType::RangeChoice {
                input,
                when_in_range,
                when_out_of_range,
                ..
            } => vec![*input, *when_in_range, *when_out_of_range],

            DensityType::ShiftedNoise {
                shift_x,
                shift_y,
                shift_z,
                ..
            } => vec![*shift_x, *shift_y, *shift_z],

            DensityType::Spline { spline } => vec![spline.coordinate],

            _ => vec![],
        }
    }
}

impl<'m> ShrinkMethod<'m> for TakeSubtree {
    fn name(&self) -> &str {
        "take_subtree"
    }

    fn can_shrink(&mut self, remaining_strikes: u32, source: DensitySource) -> (bool, u32) {
        if self.exhausted {
            return (false, 0);
        }
        
        let root_density = match source {
            DensitySource::MultiSamplingDensity { density, .. } => density,
            DensitySource::SingleSamplingDensity { density } => density,
        };

        let child_count = Self::get_immediate_children(root_density).len() as u32;

        if remaining_strikes < child_count {
            (true, remaining_strikes)
        } else {
            self.exhausted = true;
            (false, child_count)
        }
    }

    fn perform_shrink(
        &mut self,
        _arena: &'m Bump,
        remaining_strikes: u32,
        source: DensitySource<'m>,
    ) -> DensitySource<'m> {
        let (root_density, dimensions) = match source {
            DensitySource::MultiSamplingDensity { density, dimensions } => (density, Some(dimensions)),
            DensitySource::SingleSamplingDensity { density } => (density, None),
        };

        let mut children = Self::get_immediate_children(root_density);
        
        // Extract the exact child mapped to the current strike index
        let selected_child = children.remove(remaining_strikes as usize);

        match dimensions {
            Some(dim) => DensitySource::MultiSamplingDensity {
                density: selected_child,
                dimensions: dim,
            },
            None => DensitySource::SingleSamplingDensity {
                density: selected_child,
            },
        }
    }
}