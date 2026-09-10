use bumpalo::Bump;
use crate::parse::model::{Density, DensitySource, DensityType, SplineType, SplinePoint, SplineValue};

/// Utility module for rebuilding and compacting ASTs.

// ============================================================================
// 1. PATH REBUILDER (For Shrinking)
// ============================================================================

/// Traverses the AST and replaces the node at `target_strike` using the `modifier` function.
/// Unmodified branches return their original pointers to avoid memory bloat.
pub fn replace_nth_node<'m>(
    arena: &'m Bump,
    density: Density<'m>,
    target_strike: u32,
    current_strike: &mut u32,
    modifier: &impl Fn(&'m Bump, Density<'m>) -> Density<'m>,
    intern: &impl Fn(&'m Bump, DensityType<'m>) -> Density<'m>,
) -> Density<'m> {
    // 1. Is this the target node?
    if *current_strike == target_strike {
        *current_strike += 1;
        return modifier(arena, density);
    }
    *current_strike += 1;

    // 2. Not the target, traverse children
    match &*density {
        DensityType::Add { left, right } => {
            let new_left = replace_nth_node(arena, *left, target_strike, current_strike, modifier, intern);
            let new_right = replace_nth_node(arena, *right, target_strike, current_strike, modifier, intern);
            
            if std::ptr::eq(&*new_left, &**left) && std::ptr::eq(&*new_right, &**right) {
                density
            } else {
                intern(arena, DensityType::Add { left: new_left, right: new_right })
            }
        }
        DensityType::Multiply { left, right } => {
            let new_left = replace_nth_node(arena, *left, target_strike, current_strike, modifier, intern);
            let new_right = replace_nth_node(arena, *right, target_strike, current_strike, modifier, intern);
            
            if std::ptr::eq(&*new_left, &**left) && std::ptr::eq(&*new_right, &**right) {
                density
            } else {
                intern(arena, DensityType::Multiply { left: new_left, right: new_right })
            }
        }
        DensityType::Cache2d { argument } => {
            let new_arg = replace_nth_node(arena, *argument, target_strike, current_strike, modifier, intern);
            if std::ptr::eq(&*new_arg, &**argument) {
                density
            } else {
                intern(arena, DensityType::Cache2d { argument: new_arg })
            }
        }
        // TODO: Expand this pattern for all wrapper/binary nodes in DensityType...
        
        _ => density, // Leaf nodes return themselves
    }
}

// ============================================================================
// 2. ARENA COMPACTION (For Garbage Collection)
// ============================================================================

/// Deep clones a DensitySource into a completely new memory arena.
pub fn compact_source_to_new_arena<'old, 'new>(
    new_arena: &'new Bump,
    source: DensitySource<'old>,
    // intern: &impl Fn(&'new Bump, DensityType<'new>) -> Density<'new>,
    // intern_spline: &impl Fn(&'new Bump, SplineType<'new>) -> crate::parse::model::Spline<'new>,
    // intern_noise: &impl Fn(&'new Bump, crate::parse::model::NormalNoiseType) -> crate::parse::model::NormalNoise<'new>,
) -> DensitySource<'new> {
    fn intern<'new>(arena: &'new Bump, density_type: DensityType<'new>) -> Density<'new> {
        arena.alloc(density_type)
    }
    fn intern_spline<'new>(arena: &'new Bump, spline_type: SplineType<'new>) -> crate::parse::model::Spline<'new> {
        arena.alloc(spline_type)
    }
    fn intern_noise<'new>(arena: &'new Bump, noise_type: crate::parse::model::NormalNoiseType) -> crate::parse::model::NormalNoise<'new> {
        arena.alloc(noise_type)
    }
    match source {
        DensitySource::SingleSamplingDensity { density } => {
            DensitySource::SingleSamplingDensity {
                density: deep_clone_density(new_arena, density, &intern, &intern_spline, &intern_noise)
            }
        }
        DensitySource::MultiSamplingDensity { density, dimensions } => {
            DensitySource::MultiSamplingDensity {
                density: deep_clone_density(new_arena, density, &intern, &intern_spline, &intern_noise),
                dimensions,
            }
        }
    }
}

/// Recursively allocates every node of the AST into the new arena, discarding the old lifetime.
fn deep_clone_density<'old, 'new>(
    new_arena: &'new Bump,
    density: Density<'old>,
    intern: &impl Fn(&'new Bump, DensityType<'new>) -> Density<'new>,
    intern_spline: &impl Fn(&'new Bump, SplineType<'new>) -> crate::parse::model::Spline<'new>,
    intern_noise: &impl Fn(&'new Bump, crate::parse::model::NormalNoiseType) -> crate::parse::model::NormalNoise<'new>,
) -> Density<'new> {
    let new_type = match &*density {
        DensityType::Const(c) => DensityType::Const(*c),
        
        DensityType::Add { left, right } => DensityType::Add {
            left: deep_clone_density(new_arena, *left, intern, intern_spline, intern_noise),
            right: deep_clone_density(new_arena, *right, intern, intern_spline, intern_noise),
        },
        
        DensityType::Noise { name, noise, xz_scale, y_scale } => DensityType::Noise {
            name: name.clone(),
            noise: intern_noise(new_arena, (**noise).clone()),
            xz_scale: *xz_scale,
            y_scale: *y_scale,
        },
        
        DensityType::Cache2d { argument } => DensityType::Cache2d {
            argument: deep_clone_density(new_arena, *argument, intern, intern_spline, intern_noise),
        },
        
        DensityType::Spline { spline } => DensityType::Spline {
            spline: deep_clone_spline(new_arena, *spline, intern, intern_spline, intern_noise)
        },
        
        // TODO: Map the rest of your DensityType variants here...
        
        _ => unimplemented!("Implement deep clone mapping for remaining variants"),
    };

    intern(new_arena, new_type)
}

fn deep_clone_spline<'old, 'new>(
    new_arena: &'new Bump,
    spline: crate::parse::model::Spline<'old>,
    intern: &impl Fn(&'new Bump, DensityType<'new>) -> Density<'new>,
    intern_spline: &impl Fn(&'new Bump, SplineType<'new>) -> crate::parse::model::Spline<'new>,
    intern_noise: &impl Fn(&'new Bump, crate::parse::model::NormalNoiseType) -> crate::parse::model::NormalNoise<'new>,
) -> crate::parse::model::Spline<'new> {
    
    // 1. Deep clone the inner coordinate density
    let new_coord = deep_clone_density(new_arena, spline.coordinate, intern, intern_spline, intern_noise);
    
    // 2. Deep clone all the points
    let mut new_points = Vec::with_capacity(spline.spline_points.len());
    for point in spline.spline_points {
        let new_val = match &point.value {
            SplineValue::Const(c) => SplineValue::Const(*c),
            SplineValue::Spline(inner_s) => SplineValue::Spline(
                deep_clone_spline(new_arena, *inner_s, intern, intern_spline, intern_noise)
            ),
        };
        
        new_points.push(SplinePoint {
            derivative: point.derivative,
            location: point.location,
            value: new_val,
        });
    }

    // 3. Allocate the slice into the new Bump arena
    let allocated_points = new_arena.alloc_slice_clone(&new_points);

    intern_spline(new_arena, SplineType {
        coordinate: new_coord,
        spline_points: allocated_points,
    })
}