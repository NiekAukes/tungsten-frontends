use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType};
use crate::shrink::ShrinkMethod;

/// Takes an immediate subtree of the density function, effectively reducing its complexity, without adding any memory overhead.
pub struct TakeSubtree {
    /// Indicates whether this shrink method has been exhausted and can no longer shrink.
    /// Prevents further shrinking once this method has been exhausted.
    pub exhausted: bool,
}

enum VitalPreservation {
    Cache2d,
    FlatCache,
    NamedDensityReference(String),
}

impl TakeSubtree {
    fn unwrap_vital<'m>(density: Density<'m>) -> Density<'m> {
        Self::preserve_vital(density).1
    }

    fn preserve_vital<'m>(density: Density<'m>) -> (Vec<VitalPreservation>, Density<'m>) {
        match &*density {
            DensityType::Cache2d { argument } => {
                let (mut vitals, base) = Self::preserve_vital(*argument);
                vitals.insert(0, VitalPreservation::Cache2d);
                (vitals, base)
            },

            DensityType::FlatCache { argument } => {
                let (mut vitals, base) = Self::preserve_vital(*argument);
                vitals.insert(0, VitalPreservation::FlatCache);
                (vitals, base)
            },
            DensityType::NamedDensityReference { name, argument } => {
                let (mut vitals, base) = Self::preserve_vital(*argument);
                vitals.insert(0, VitalPreservation::NamedDensityReference((*name).clone()));
                (vitals, base)
            },
            density => (vec![], density),
        }
    }

    fn reconstruct_vital<'m>(arena: &'m Bump, vitals: Vec<VitalPreservation>, base: Density<'m>) -> Density<'m> {
        let mut density = base;
        for vital in vitals.into_iter().rev() {
            density = match vital {
                VitalPreservation::Cache2d => arena.alloc(DensityType::Cache2d { argument: density }),
                VitalPreservation::FlatCache => arena.alloc(DensityType::FlatCache { argument: density }),
                VitalPreservation::NamedDensityReference(name) => arena.alloc(DensityType::NamedDensityReference { name: arena.alloc(name), argument: density }),
            };
        }
        density
    }

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


        let density = Self::unwrap_vital(root_density);

        let child_count = Self::get_immediate_children(density).len() as u32;

        if remaining_strikes < child_count {
            (true, remaining_strikes)
        } else {
            self.exhausted = true;
            (false, child_count)
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

        let (vital,density) = Self::preserve_vital(root_density);

        let mut children = Self::get_immediate_children(density);
        
        // Extract the exact child mapped to the current strike index
        let selected_child = children.remove(remaining_strikes as usize);
        let selected_child = Self::reconstruct_vital(arena, vital, selected_child);

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

    fn reenable(&mut self) {
        self.exhausted = false;
    }
}