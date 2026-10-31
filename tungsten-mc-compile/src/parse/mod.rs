use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    fmt::Debug,
};

use bumpalo::Bump;
use serde_json::Value;

use crate::{
    config_load::MinecraftDataRaw,
    error::{normalize_id, CompileError, ResourceKind},
    parse::{
        density::DensityParseFunctions,
        model::{
            CoordinateType, DensitySource, NoiseGeneratorSettings, NoiseRouter,
            NoiseSettings, NormalNoise, NormalNoiseType,
        },
    },
};

type JsonObject = serde_json::Map<String, Value>;

pub mod density;
pub mod dot;
pub mod model;
pub mod pretty;
pub mod spline;
/*
📂 Folder → Registry mapping

worldgen/noise_settings → NoiseGeneratorSettings registry

worldgen/density_function → DensityFunction registry

worldgen/noise → NormalNoise.NoiseParameters registry

worldgen/configured_carver → ConfiguredCarver registry

worldgen/configured_feature → ConfiguredFeature registry

worldgen/placed_feature → PlacedFeature registry

worldgen/biome → Biome registry

worldgen/structure → StructureType (or in newer versions, configured structures live under configured_structure_feature)

worldgen/structure_set → StructureSet registry

worldgen/template_pool → StructureTemplatePool (for jigsaw structures like villages)

worldgen/processor_list → StructureProcessorList (for structure block modifications)
*/

/*
{
  "aquifers_enabled": true,
  "default_block": {
    "Name": "minecraft:stone"
  },
  "default_fluid": {
    "Name": "minecraft:water",
    "Properties": {
      "level": "0"
    }
  },
  "disable_mob_generation": false,
  "legacy_random_source": false,
  "noise": {
    "height": 384,
    "min_y": -64,
    "size_horizontal": 1,
    "size_vertical": 2
  },
  "noise_router": {
    "barrier": {
      "type": "minecraft:noise",
      "noise": "minecraft:aquifer_barrier",
      "xz_scale": 1.0,
      "y_scale": 0.5
    },
    "continents": "minecraft:overworld/continents",
    "depth": "minecraft:overworld/depth",
    "erosion": "minecraft:overworld/erosion",
    "final_density": { ... },
    "fluid_level_floodedness": {
      "type": "minecraft:noise",
      "noise": "minecraft:aquifer_fluid_level_floodedness",
      "xz_scale": 1.0,
      "y_scale": 0.67
    },
    "fluid_level_spread": {
      "type": "minecraft:noise",
      "noise": "minecraft:aquifer_fluid_level_spread",
      "xz_scale": 1.0,
      "y_scale": 0.7142857142857143
    },
    "initial_density_without_jaggedness": { ... },
    "lava": {
      "type": "minecraft:noise",
      "noise": "minecraft:aquifer_lava",
      "xz_scale": 1.0,
      "y_scale": 1.0
    },
    "ridges": "minecraft:overworld/ridges",
    "temperature": {
      "type": "minecraft:shifted_noise",
      "noise": "minecraft:temperature",
      "shift_x": "minecraft:shift_x",
      "shift_y": 0.0,
      "shift_z": "minecraft:shift_z",
      "xz_scale": 0.25,
      "y_scale": 0.0
    },
    "vegetation": {
      "type": "minecraft:shifted_noise",
      "noise": "minecraft:vegetation",
      "shift_x": "minecraft:shift_x",
      "shift_y": 0.0,
      "shift_z": "minecraft:shift_z",
      "xz_scale": 0.25,
      "y_scale": 0.0
    },
    "vein_gap": {
      "type": "minecraft:noise",
      "noise": "minecraft:ore_gap",
      "xz_scale": 1.0,
      "y_scale": 1.0
    },
    "vein_ridged": { ... },
    "vein_toggle": {
      "type": "minecraft:interpolated",
      "argument": {
        "type": "minecraft:range_choice",
        "input": "minecraft:y",
        "max_exclusive": 51.0,
        "min_inclusive": -60.0,
        "when_in_range": {
          "type": "minecraft:noise",
          "noise": "minecraft:ore_veininess",
          "xz_scale": 1.5,
          "y_scale": 1.5
        },
        "when_out_of_range": 0.0
      }
    }
  },
*/

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct SlideSettings {
//     pub target: i32,
//     pub size: i32,
//     pub offset: i32,
// }

pub type Interned<'m, T> = &'m T; // TODO intern later

pub type ParseResult<T> = Result<T, CompileError>;

pub struct MinecraftData<'m> {
    arena: &'m Bump,
    raw_data: MinecraftDataRaw,
    /// Parsed noise settings, filled by [`MinecraftData::parse_root`].
    pub noise_settings: HashMap<String, NoiseGeneratorSettings<'m>>,
    /// The id of the last root parsed.
    root: Option<String>,
    /// Parsed on first reference.
    normal_noises: RefCell<HashMap<String, NormalNoise<'m>>>,
    chunk_size: usize,
    /// The resources being parsed, innermost last; errors name the innermost one.
    context: RefCell<Vec<(ResourceKind, String)>>,
    /// Every density function and noise resolved by reference so far.
    reachable: RefCell<BTreeSet<(ResourceKind, String)>>,
}

impl<'m> Debug for MinecraftData<'m> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinecraftData")
            .field("noise_settings", &self.noise_settings)
            .field("normal_noises", &self.normal_noises)
            .finish()
    }
}

impl<'m> MinecraftData<'m> {
    pub fn new(arena: &'m Bump, raw: MinecraftDataRaw, chunk_size: usize) -> MinecraftData<'m> {
        MinecraftData {
            arena,
            raw_data: raw,
            noise_settings: HashMap::new(),
            root: None,
            normal_noises: RefCell::new(HashMap::new()),
            chunk_size,
            context: RefCell::new(Vec::new()),
            reachable: RefCell::new(BTreeSet::new()),
        }
    }

    /// Parses the noise settings `id` (e.g. `minecraft:overworld`) and everything it references.
    /// Nothing else is parsed, so broken unrelated files don't matter.
    pub fn parse_root(&mut self, id: &str) -> ParseResult<&NoiseGeneratorSettings<'m>> {
        if let Some(bad) = self.raw_data.invalid_utf8.iter().next() {
            let (kind, bad_id) = bad.clone();
            return Err(CompileError::Unsupported { kind, id: bad_id, message: "file is not valid UTF-8".into() });
        }
        let id = normalize_id(id);
        let Some(json_str) = self.raw_data.noise_settings.get(&id) else {
            return Err(CompileError::Unsupported {
                kind: ResourceKind::NoiseSettings,
                id,
                message: "noise settings were not provided".into(),
            });
        };
        let parsed = self.with_context(ResourceKind::NoiseSettings, &id, || self.parse_noise_settings(json_str))?;
        self.noise_settings.insert(id.clone(), parsed);
        self.root = Some(id.clone());
        Ok(&self.noise_settings[&id])
    }

    /// The noise settings of the last [`MinecraftData::parse_root`], as currently stored in
    /// `noise_settings` (callers may have replaced parts of it).
    pub fn root(&self) -> Option<&NoiseGeneratorSettings<'m>> {
        self.noise_settings.get(self.root.as_ref()?)
    }

    /// The density functions and noises reached from the parsed roots, sorted by kind and id.
    pub fn reachable(&self) -> Vec<(ResourceKind, String)> {
        self.reachable.borrow().iter().cloned().collect()
    }

    /// Runs `f` with `(kind, id)` as the resource that errors are attributed to.
    pub(crate) fn with_context<T>(&self, kind: ResourceKind, id: &str, f: impl FnOnce() -> ParseResult<T>) -> ParseResult<T> {
        self.context.borrow_mut().push((kind, id.to_string()));
        let result = f();
        self.context.borrow_mut().pop();
        result
    }

    /// An `Unsupported` error about the resource being parsed.
    pub(crate) fn unsupported(&self, message: impl Into<String>) -> CompileError {
        let (kind, id) = self
            .context
            .borrow()
            .last()
            .cloned()
            .unwrap_or((ResourceKind::NoiseSettings, String::new()));
        CompileError::Unsupported { kind, id, message: message.into() }
    }

    /// An `Unsupported` error naming the missing resource `id` and who referenced it.
    pub(crate) fn missing_reference(&self, kind: ResourceKind, id: &str) -> CompileError {
        let from = self.context.borrow().last().map(|(_, from)| from.clone()).unwrap_or_default();
        CompileError::Unsupported {
            kind,
            id: id.to_string(),
            message: format!("{} `{}` is referenced from `{}` but was not provided", kind, id, from),
        }
    }

    pub(crate) fn field<'v>(&self, obj: &'v JsonObject, ty: &str, key: &str) -> ParseResult<&'v Value> {
        obj.get(key)
            .ok_or_else(|| self.unsupported(format!("missing field `{}` in `{}`", key, ty)))
    }

    pub(crate) fn f64_field(&self, obj: &JsonObject, ty: &str, key: &str) -> ParseResult<f64> {
        self.field(obj, ty, key)?
            .as_f64()
            .ok_or_else(|| self.unsupported(format!("field `{}` in `{}` must be a number", key, ty)))
    }

    pub(crate) fn i32_field(&self, obj: &JsonObject, ty: &str, key: &str) -> ParseResult<i32> {
        self.field(obj, ty, key)?
            .as_i64()
            .map(|v| v as i32)
            .ok_or_else(|| self.unsupported(format!("field `{}` in `{}` must be an integer", key, ty)))
    }

    pub(crate) fn str_field<'v>(&self, obj: &'v JsonObject, ty: &str, key: &str) -> ParseResult<&'v str> {
        self.field(obj, ty, key)?
            .as_str()
            .ok_or_else(|| self.unsupported(format!("field `{}` in `{}` must be a string", key, ty)))
    }

    /// The noise `name`, parsed on first use.
    pub(crate) fn noise(&self, name: &str) -> ParseResult<NormalNoise<'m>> {
        let id = normalize_id(name);
        if let Some(noise) = self.normal_noises.borrow().get(&id) {
            return Ok(*noise);
        }
        let Some(json_str) = self.raw_data.normal_noises.get(&id) else {
            return Err(self.missing_reference(ResourceKind::Noise, &id));
        };
        let parsed: NormalNoiseType = self.with_context(ResourceKind::Noise, &id, || {
            serde_json::from_str(json_str).map_err(|e| self.unsupported(format!("invalid noise: {}", e)))
        })?;
        let noise: NormalNoise<'m> = self.arena.alloc(parsed);
        self.normal_noises.borrow_mut().insert(id.clone(), noise);
        self.reachable.borrow_mut().insert((ResourceKind::Noise, id));
        Ok(noise)
    }

    /// Records a density function reference as reachable.
    pub(crate) fn reached_density_function(&self, id: &str) {
        self.reachable.borrow_mut().insert((ResourceKind::DensityFunction, id.to_string()));
    }

    fn parse_noise_settings(&self, json_str: &str) -> ParseResult<NoiseGeneratorSettings<'m>> {
        let value: Value = serde_json::from_str(json_str)
            .map_err(|e| self.unsupported(format!("invalid JSON: {}", e)))?;
        let Some(obj) = value.as_object() else {
            return Err(self.unsupported("noise settings must be an object"));
        };
        let aquifers_enabled = obj
            .get("aquifers_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let default_block = self.field(obj, "noise settings", "default_block")?;
        let default_block = default_block
            .as_object()
            .ok_or_else(|| self.unsupported("`default_block` must be an object"))
            .and_then(|b| self.str_field(b, "default_block", "Name"))?;
        let fluid = self.field(obj, "noise settings", "default_fluid")?;
        let fluid = fluid
            .as_object()
            .ok_or_else(|| self.unsupported("`default_fluid` must be an object"))?;
        let default_fluid = self.str_field(fluid, "default_fluid", "Name")?.to_string();
        let default_fluid_level = fluid
            .get("Properties")
            .and_then(|v| v.get("level"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        let disable_mob_generation = obj
            .get("disable_mob_generation")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let sea_level = obj
            .get("sea_level")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(63);

        // legacy_random_source is ignored

        let noise_value = self.field(obj, "noise settings", "noise")?;
        let noise: NoiseSettings = serde_json::from_value(noise_value.clone())
            .map_err(|e| self.unsupported(format!("invalid `noise`: {}", e)))?;
        let noise_router_value = self.field(obj, "noise settings", "noise_router")?;
        let noise_router = self.parse_noise_router(noise_router_value, &noise)?;

        Ok(NoiseGeneratorSettings {
            aquifers_enabled,
            default_block: default_block.to_string(),
            default_fluid,
            default_fluid_level,
            disable_mob_generation,
            noise,
            noise_router,
            sea_level,
        })
    }

    fn parse_noise_router(&self, value: &Value, settings: &NoiseSettings) -> ParseResult<NoiseRouter<'m>> {
        let Some(router) = value.as_object() else {
            return Err(self.unsupported("`noise_router` must be an object"));
        };
        let route = |key: &str| {
            let value = self.field(router, "noise_router", key)?;
            self.parse_density_function_from_value_and_name(value, key)
        };
        let barrier = route("barrier")?;
        let continents = route("continents")?;
        let depth = route("depth")?;
        let erosion = route("erosion")?;
        let final_density = route("final_density")?;
        let final_density_with_beard = self.arena.alloc(model::DensityType::NamedDensityReference {
            name: self.arena.alloc("final_density_with_beardify".to_string()),
            argument: self.arena.alloc(model::DensityType::Add {
                left: route("final_density")?,
                right: self.arena.alloc(model::DensityType::Beardify),
            }),
        });
        let fluid_level_floodedness = route("fluid_level_floodedness")?;
        let fluid_level_spread = route("fluid_level_spread")?;
        let preliminary_surface_level = route("preliminary_surface_level")?;
        let lava = route("lava")?;
        let ridges = route("ridges")?;
        let temperature = route("temperature")?;
        let vegetation = route("vegetation")?;
        let vein_gap = route("vein_gap")?;
        let vein_ridged = route("vein_ridged")?;
        let vein_toggle = route("vein_toggle")?;

        let aquifer_sampler = self.arena.alloc(model::DensityType::NamedDensityReference {
            name: self.arena.alloc("aquifer".into()),
            argument: self.arena.alloc(model::DensityType::Aquifer {
                final_density: final_density,
                depth,
                erosion,
                floodedness: fluid_level_floodedness,
                spread: fluid_level_spread,
                fluid_type: lava,
                barrier,
            }),
        });

        Ok(NoiseRouter {
            barrier: DensitySource::SingleSamplingDensity { density: barrier },
            // continents: DensitySource::SingleSamplingDensity {
            //     density: continents,
            // },
            continents: DensitySource::MultiSamplingDensity {
                density: continents,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },
            depth: DensitySource::MultiSamplingDensity {
                density: depth,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },
            erosion: DensitySource::MultiSamplingDensity {
                density: erosion,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },
            final_density: DensitySource::MultiSamplingDensity {
                density: final_density,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
            final_density_with_beard: DensitySource::MultiSamplingDensity {
                density: final_density_with_beard,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
            fluid_level_floodedness: DensitySource::SingleSamplingDensity {
                density: fluid_level_floodedness,
            },
            fluid_level_spread: DensitySource::SingleSamplingDensity {
                density: fluid_level_spread,
            },
            preliminary_surface_level: DensitySource::MultiSamplingDensity {
                density: preliminary_surface_level,
                dimensions: (32 * 4, 1, 32 * 4),
                coordinate_type: CoordinateType::PreliminarySurface,
            },
            // initial_density_without_jaggedness: DensitySource::SingleSamplingDensity { density: initial_density_without_jaggedness },
            lava: DensitySource::SingleSamplingDensity { density: lava },
            // aka weirdness
            ridges: DensitySource::MultiSamplingDensity {
                density: ridges,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },
            temperature: DensitySource::MultiSamplingDensity {
                density: temperature,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },

            vegetation: DensitySource::MultiSamplingDensity {
                density: vegetation,
                dimensions: (
                    self.chunk_size as i32 >> 2,
                    settings.height >> 2,
                    self.chunk_size as i32 >> 2,
                ),
                coordinate_type: CoordinateType::Biome,
            },
            vein_gap: DensitySource::MultiSamplingDensity {
                density: vein_gap,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
            vein_ridged: DensitySource::MultiSamplingDensity {
                density: vein_ridged,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
            vein_toggle: DensitySource::MultiSamplingDensity {
                density: vein_toggle,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
            // vein_ridged: DensitySource::SingleSamplingDensity { density: vein_ridged },
            // vein_toggle: DensitySource::SingleSamplingDensity { density: vein_toggle },
            aquifer_sampler: DensitySource::MultiSamplingDensity {
                density: aquifer_sampler,
                dimensions: (
                    self.chunk_size as i32,
                    settings.height,
                    self.chunk_size as i32,
                ),
                coordinate_type: CoordinateType::Terrain,
            },
        })
    }
}

impl<'m> NoiseRouter<'m> {
    pub fn all_densities(&self) -> Vec<DensitySource<'m>> {
        // vec![
        //     self.barrier,
        //     self.continents,
        //     self.depth,
        //     self.erosion,
        //     self.final_density,
        //     self.fluid_level_floodedness,
        //     self.fluid_level_spread,
        //     self.preliminary_surface_level,
        //     self.lava,
        //     self.ridges,
        //     self.temperature,
        //     self.vegetation,
        //     self.vein_gap,
        //     self.vein_ridged,
        //     self.vein_toggle,
        // ]
        vec![
            self.final_density_with_beard,
            self.final_density,
            self.vein_gap,
            self.vein_ridged,
            self.vein_toggle,
            self.preliminary_surface_level,
            self.aquifer_sampler,
        ]
    }
}
