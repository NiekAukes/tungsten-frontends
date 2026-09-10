//! Individual shrink methods. Each method lives in its own file so new
//! strategies can be added without touching the harness itself.

mod replace_with_constant;
mod remove_operand;
mod simplify_noise;
mod take_subtree;

pub use replace_with_constant::ReplaceWithConstant;
pub use remove_operand::RemoveOperand;
pub use simplify_noise::SimplifyNoise;
pub use take_subtree::TakeSubtree;
