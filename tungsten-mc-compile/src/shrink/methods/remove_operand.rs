use bumpalo::Bump;

use crate::parse::model::DensitySource;
use crate::shrink::ShrinkMethod;

/// Replaces a binary operation (e.g. `Add`, `Multiply`) with one of its operands.
pub struct RemoveOperand;

impl<'m> ShrinkMethod<'m> for RemoveOperand {
    fn name(&self) -> &str {
        "remove_operand"
    }

    fn can_shrink(&mut self, remaining_strikes: u32, _source: DensitySource) -> (bool, u32) {
        (false, 0)
    }

    fn perform_shrink(&mut self, arena: &'m Bump, remaining_strikes: u32, _source: DensitySource<'m>) -> DensitySource<'m> {
        todo!()
    }
}
