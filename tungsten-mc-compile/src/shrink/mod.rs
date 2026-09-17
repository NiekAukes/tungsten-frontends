use bumpalo::Bump;

use crate::parse::model::{Density, DensitySource, DensityType};

/// Module for shrinking the search space in the Minecraft world generation harness.
/// Very useful for automatic debugging of world generation issues.

pub mod methods;
pub mod rebuild;

pub struct Shrinker<'m> {
    arena: &'m bumpalo::Bump,
    initial: DensitySource<'m>,
    strikes: u32,
    last_shrink: Option<DensitySource<'m>>,
    shrink_methods: Vec<Box<dyn ShrinkMethod<'m>>>,
    density_function_name: Option<&'m String>,
}

pub trait ShrinkMethod<'m> {
    fn name(&self) -> &str;
    /// Returns a tuple where the first element indicates if the source can be shrunk,
    /// and the second element represents the strikes count used.
    fn can_shrink(&mut self, remaining_strikes: u32, source: DensitySource) -> (bool, u32);
    fn perform_shrink(&mut self, arena: &'m Bump, remaining_strikes: u32, source: DensitySource<'m>) -> DensitySource<'m>;
    /// Re-enables the shrink method after it has been struck out.
    fn reenable(&mut self);
}


impl<'m> Shrinker<'m> {



    pub fn new(arena: &'m bumpalo::Bump, initial: DensitySource<'m>) -> Self {
        // let shrink_methods: &[&dyn ShrinkMethod<'m>] = &[
        //     &methods::TakeSubtree {
        //         exhausted: false,
        //     },
        //     &methods::ReplaceWithConstant,
        //     &methods::RemoveOperand,
        //     &methods::SimplifyNoise,
        // ];
        let shrink_methods: Vec<Box<dyn ShrinkMethod<'m>>> = vec![
            Box::new(methods::RemoveNamedReferences {
                exhausted: false,
            }),
            Box::new(methods::TakeSubtree {
                exhausted: false,
            }),
            Box::new(methods::ReplaceWithConstant {
                exhausted: false,
            }),
            Box::new(methods::RemoveOperand),
            Box::new(methods::RemoveWrappers {
                exhausted: false,
            }),
            Box::new(methods::SimplifyNoise {
                exhausted: false,
            }),
            Box::new(methods::SimplifyNoiseParams {
                exhausted: false,
            }),
        ];

        Self {
            arena,
            initial,
            strikes: 0,
            last_shrink: None,
            shrink_methods,
            density_function_name: None,
        }
    }
    pub fn set_density_function_name(&mut self, name: String) {
        self.density_function_name = Some(self.arena.alloc(name));
    }

    /// Tries the next applicable shrink method, starting after the ones already
    /// struck out. Returns `None` once every method has been exhausted.
    pub fn shrink(&mut self) -> Option<DensitySource<'m>> {
        
        assert!(self.last_shrink.is_none(), "Shrink already performed. Did you forget to mark the last shrink?");
        let mut strikes = self.strikes;
        for _ in 0..2 {
            for method in self.shrink_methods.iter_mut(){
                let (can_shrink, strikes_used) = method.can_shrink(strikes, self.initial);
                if can_shrink {
                    let shrunk: DensitySource<'m> = method.perform_shrink(self.arena,strikes, self.initial);
                    println!("Performed shrink with method: {:?} using {} strikes", method.name(), self.strikes);
                    self.last_shrink = Some(shrunk);
                    return Some(shrunk);
                } else {
                    strikes -= strikes_used;
                }
            }
            // Re-enable all shrink methods for the next round.
            for method in self.shrink_methods.iter_mut() {
                method.reenable();
            }
        }

        None
    }
    pub fn mark_last_shrink_successful(&mut self) {
        // the previous shrink was successful, so revert to the initial state.
        // additionally, add a strike to skip the shrinking step next time.
        self.strikes += 1;
        assert!(self.last_shrink.is_some(), "No shrink available. Did you forget to call shrink()?");
        self.last_shrink = None;
    }
    pub fn mark_last_shrink_buggy(&mut self) {
        // the previous shrink still had a bug, so keep the last shrink as the current state.
        // additionally, reset the strikes to allow shrinking again.
        self.strikes = 0;
        self.initial = self.last_shrink.take().expect("No shrink available. Did you forget to call shrink()?");
    }

    pub fn finish(self) -> DensitySource<'m> {
        self.initial
    }
}