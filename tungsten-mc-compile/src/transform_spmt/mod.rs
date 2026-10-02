use crate::{
    parse::model::{
        CoordinateType, Density, DensitySource, DensityType, NoiseGeneratorSettings, NoiseSettings,
    },
    transform_spmt::density::{DensityBuilder, DensityKey, NormalNoiseKey},
};

use tungsten_wg::{
    orchestrate::Scale,
    spmt::builder::SPMTBuilder,
    spmt::model::{
        DensityFunctionRef, DensityInput, MainDensityFunction, Name, Var, Variable, VariableType,
        SPMT,
    },
};

pub mod beardify;
pub mod density;
pub mod find_top_surface;
pub mod noise;
pub mod spline;

pub fn newvar<'m>(arena: &'m bumpalo::Bump, name: &str, t: VariableType) -> Var<'m> {
    Var::new(arena.alloc(Variable {
        name: Name::Named(name.into()),
        t,
    }))
}

pub fn prefixvar<'m>(arena: &'m bumpalo::Bump, prefix: &str, t: VariableType) -> Var<'m> {
    Var::new(arena.alloc(Variable {
        name: Name::Prefixed(prefix.into()),
        t,
    }))
}

pub fn anonvar<'m>(arena: &'m bumpalo::Bump, t: VariableType) -> Var<'m> {
    Var::new(arena.alloc(Variable {
        name: Name::Anonymous,
        t,
    }))
}

type DensityFunctionCache<'a, 'm> =
    std::collections::HashMap<DensityKey<'a>, DensityFunctionRef<'m>>;
type NoiseCache<'a, 'm> = std::collections::HashMap<NormalNoiseKey<'a>, DensityFunctionRef<'m>>;

#[derive(Debug, Clone, Copy)]
pub struct ScalingSet {
    pub dimensions: (i32, i32, i32),
    pub scaled_position: (f64, f64, f64),
    pub scaled_origin: (f64, f64, f64),
}

impl Default for ScalingSet {
    fn default() -> Self {
        Self {
            dimensions: (1, 1, 1),
            scaled_position: (1.0, 1.0, 1.0),
            scaled_origin: (1.0, 1.0, 1.0),
        }
    }
}

impl ScalingSet {
    pub fn interpolation(self, add: bool) -> Self {
        if !add {
            return self;
        }
        ScalingSet {
            dimensions: (
                self.dimensions.0 + 1,
                self.dimensions.1,
                self.dimensions.2 + 1,
            ),
            scaled_position: self.scaled_position,
            scaled_origin: self.scaled_origin,
        }
    }
}

pub struct BuilderState<'a, 'm> {
    density_function_cache: DensityFunctionCache<'a, 'm>,
    pub noise_cache: NoiseCache<'a, 'm>,

    // working_dimensions: (i32, i32, i32),
    // original_dimensions: (i32, i32, i32),
    // working_scaled_position: (f64, f64, f64),
    // original_scaled_position: (f64, f64, f64),
    // working_scaled_origin: (f64, f64, f64),
    // original_scaled_origin: (f64, f64, f64),

    // represents the current scaling environment for density calculations.
    working_set: ScalingSet,
    // represents the original scaling environment at the source
    sampling_set: ScalingSet,
    // represents the scaling environment for flatcache calculations.
    flatcache_set: ScalingSet,

    add_interpolation_dimension: bool,

    known_y_sample_point: Option<i32>, // for cache2d, the y value at which the density is sampled. Flatcache can set this to 0.

    pub noise_settings: NoiseSettings,
    density_counter: usize,
}

impl<'a, 'm> BuilderState<'a, 'm> {
    pub fn get_cached_density(&self, density: &Density<'a>) -> Option<DensityFunctionRef<'m>> {
        let key = DensityKey {
            density: *density,
            dimensions: self.working_set.dimensions,
            scaled_position: Scale::default(),
            scaled_origin: Scale::default(),
        };
        self.density_function_cache.get(&key).cloned()
    }

    pub fn insert_density_cache(&mut self, density: Density<'a>, func: DensityFunctionRef<'m>) {
        let key = DensityKey {
            density,
            dimensions: self.working_set.dimensions,
            scaled_position: Scale::default(),
            scaled_origin: Scale::default(),
        };
        if self.density_function_cache.contains_key(&key) {
            // panic!("Density function already exists in cache: {:?}", func.canonical_name);
        }
        self.density_function_cache.insert(key, func);
    }
}

pub struct Transformer<'a, 'm> {
    pub final_model: SPMT<'m>,
    pub arena: &'m bumpalo::Bump,
    pub builder_state: Option<BuilderState<'a, 'm>>,
}

impl<'a, 'm> Transformer<'a, 'm> {
    pub fn new(arena: &'m bumpalo::Bump) -> Self {
        Self {
            final_model: SPMT {
                density_functions: Vec::new(),
                functions: Vec::new(),
                main_density_functions: Vec::new(),
            },
            arena,
            // density_function_cache: Option::Some(std::collections::HashMap::new()),
            // noise_cache: Option::Some(std::collections::HashMap::new()),
            builder_state: Option::Some(BuilderState {
                density_function_cache: std::collections::HashMap::new(),
                noise_cache: std::collections::HashMap::new(),
                // working_dimensions: (0, 0, 0),
                // original_dimensions: (0, 0, 0),
                // working_scaled_position: (1.0, 1.0, 1.0),
                // original_scaled_position: (1.0, 1.0, 1.0),
                // working_scaled_origin: (1.0, 1.0, 1.0),
                // original_scaled_origin: (1.0, 1.0, 1.0),
                working_set: ScalingSet::default(),
                flatcache_set: ScalingSet::default(),
                sampling_set: ScalingSet::default(),
                add_interpolation_dimension: false,
                known_y_sample_point: None,
                noise_settings: NoiseSettings {
                    min_y: 0,
                    height: 256,
                    size_horizontal: 4,
                    size_vertical: 4,
                },
                density_counter: 0,
            }),
        }
    }

    pub fn transform(mut self, noise_generator: &'a NoiseGeneratorSettings<'a>) -> SPMT<'m> {
        // For each density function in the Minecraft data, lower it and add it to the final model
        for density in noise_generator.noise_router.all_densities() {
            // set the working dimensions and scaled origin in the builder state based on the density source type

            match density {
                DensitySource::SingleSamplingDensity { density: _ } => {
                    {
                        let bs = self.builder_state.as_mut().unwrap();
                        // bs.working_dimensions = (1, 1, 1);
                        // bs.original_dimensions = (1, 1, 1);
                        // bs.working_scaled_origin = (1.0, 1.0, 1.0);
                        // bs.original_scaled_origin = (1.0, 1.0, 1.0);
                        // bs.working_scaled_position = (1.0, 1.0, 1.0);
                        // bs.original_scaled_position = (1.0, 1.0, 1.0);
                        bs.working_set = ScalingSet::default();
                        bs.flatcache_set = ScalingSet::default();
                        bs.sampling_set = ScalingSet::default();
                        bs.noise_settings = noise_generator.noise.clone();
                    }

                    // let density_function = self.lower_density_function(density);

                    // self.final_model
                    //     .main_density_functions
                    //     .push((density_function, (1, 1, 1)));
                    // self.final_model.density_functions.push(density_function);
                }
                DensitySource::MultiSamplingDensity {
                    density,
                    dimensions,
                    coordinate_type,
                } => {
                    let sampling_set = {
                        let bs = self.builder_state.as_mut().unwrap();
                        // bs.working_dimensions = dimensions;
                        // bs.original_dimensions = dimensions;
                        // bs.working_scaled_origin = (1.0, 1.0, 1.0);
                        // bs.original_scaled_origin = (1.0, 1.0, 1.0);
                        // bs.working_scaled_position = (1.0, 1.0, 1.0);
                        // bs.original_scaled_position = (1.0, 1.0, 1.0);

                        //
                        match coordinate_type {
                            CoordinateType::Biome => {
                                bs.working_set = ScalingSet {
                                    dimensions,
                                    scaled_origin: (1.0, 1.0, 1.0),
                                    scaled_position: (
                                        4.0,
                                        noise_generator.noise.size_vertical as f64 * 4.0,
                                        4.0,
                                    ),
                                };
                                bs.flatcache_set = ScalingSet {
                                    dimensions: (dimensions.0, 1, dimensions.2),
                                    scaled_origin: (1.0, 0.0, 1.0),
                                    scaled_position: (4.0, 0.0, 4.0),
                                };
                            }
                            CoordinateType::Terrain => {
                                bs.working_set = ScalingSet {
                                    dimensions,
                                    ..Default::default()
                                };
                                bs.flatcache_set = ScalingSet {
                                    dimensions: (dimensions.0 >> 2, 1, dimensions.2 >> 2),
                                    scaled_origin: (1.0, 0.0, 1.0),
                                    scaled_position: (4.0, 0.0, 4.0),
                                    ..Default::default()
                                };
                            }
                            CoordinateType::PreliminarySurface => {
                                assert!(dimensions.1 == 1);
                                bs.working_set = ScalingSet {
                                    dimensions,
                                    scaled_origin: (1.0, 1.0, 1.0),
                                    scaled_position: (
                                        4.0,
                                        noise_generator.noise.size_vertical as f64 * 4.0,
                                        4.0,
                                    ),
                                };
                                bs.flatcache_set = ScalingSet {
                                    dimensions: (dimensions.0, 1, dimensions.2),
                                    scaled_origin: (1.0, 0.0, 1.0),
                                    scaled_position: (4.0, 0.0, 4.0),
                                };
                                bs.known_y_sample_point = Some(0);
                            }
                        }
                        let sampling_set = bs.working_set.clone();
                        bs.sampling_set = bs.working_set.clone();
                        bs.noise_settings = noise_generator.noise.clone();
                        sampling_set
                    };

                    let density_function =
                        self.lower_density_function(density, &noise_generator.noise);

                    //self.final_model.density_functions.push(density_function);
                    self.final_model
                        .main_density_functions
                        .push(MainDensityFunction {
                            density_function,
                            dimensions,
                            scaled_origin: sampling_set.scaled_origin,
                            scaled_position: sampling_set.scaled_position,
                        });
                    self.final_model.density_functions.push(density_function);
                }
            }
        }

        let BuilderState {
            density_function_cache,
            noise_cache,
            ..
        } = self.builder_state.take().unwrap();

        for density_function in density_function_cache.values() {
            // check if the density function is already in the main density functions, and if not, add it to the final model
            if !self
                .final_model
                .main_density_functions
                .iter()
                .any(|mdf| mdf.density_function.canonical_name == density_function.canonical_name)
            {
                self.final_model.density_functions.push(*density_function);
            }
        }
        for noise in noise_cache.values() {
            self.final_model.density_functions.push(*noise);
        }

        self.final_model
    }

    pub fn lower_density_function(
        &mut self,
        density: Density<'a>,
        noise_settings: &'a NoiseSettings,
    ) -> DensityFunctionRef<'m> {
        let bs = self.builder_state.take().unwrap();
        if let Some(cached) = bs.get_cached_density(&density) {
            let ret = cached.clone();
            self.builder_state = Some(bs);
            return ret;
        }

        let mut name = None;
        let mut inner_density = density;
        if let DensityType::NamedDensityReference {
            name: dname,
            argument,
        } = *density
        {
            inner_density = argument;
            name = Some(dname.clone())
        }

        // create a density function builder
        let mut builder = DensityBuilder::new_named(self.arena, bs, noise_settings, name);
        // lower the density into the builder
        let r = builder.lower_density(inner_density);
        // build the density function
        let (density_function, helpers, mut bs) = builder.finish(r);
        self.final_model.functions.extend(helpers);

        let density_function: DensityFunctionRef<'m> =
            DensityFunctionRef::new(self.arena.alloc(density_function));
        let _a = density_function.canonical_name.as_ref().unwrap();

        // explicitly leave the NamedDensityReference as the outer density
        // such that there are no cache conflicts between the NamedDensityReference and its inner density function.
        bs.insert_density_cache(density, density_function);
        self.builder_state = Some(bs);
        density_function
    }
}

impl<'a, 'm> BuilderState<'a, 'm> {
    pub fn use_density_counter(&mut self) -> usize {
        let current = self.density_counter;
        self.density_counter += 1;
        current
    }
}
