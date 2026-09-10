use bumpalo::Bump;

use crate::parse::model::DensitySource;
use crate::shrink::ShrinkMethod;

/// Replaces a noise-based density function with a cheaper approximation.
pub struct SimplifyNoise;

impl<'m> ShrinkMethod<'m> for SimplifyNoise {
    fn name(&self) -> &str {
        "simplify_noise"
    }
    fn can_shrink(&mut self, remaining_strikes: u32, _source: DensitySource) -> (bool, u32) {
        (false, 0)
    }

    fn perform_shrink(&mut self, arena: &'m Bump, remaining_strikes: u32, _source: DensitySource<'m>) -> DensitySource<'m> {
        todo!()
    }
}
