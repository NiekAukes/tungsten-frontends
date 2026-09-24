//! Individual shrink methods. Each method lives in its own file so new
//! strategies can be added without touching the harness itself.

mod replace_with_constant;
mod remove_named_references;
mod remove_operand;
mod remove_wrappers;
mod simplify_noise;
mod simplify_noise_params;
mod take_subtree;
mod simplify_spline;

pub use replace_with_constant::ReplaceWithConstant;
pub use remove_named_references::RemoveNamedReferences;
pub use remove_operand::RemoveOperand;
pub use remove_wrappers::RemoveWrappers;
pub use simplify_noise::SimplifyNoise;
pub use simplify_noise_params::SimplifyNoiseParams;
pub use take_subtree::TakeSubtree;
pub use simplify_spline::SimplifySpline;
