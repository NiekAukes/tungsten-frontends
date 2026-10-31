use crate::error::{ResourceKind, normalize_id};
use crate::parse::{
    MinecraftData, ParseResult, model::Density, model::DensityType, spline::SplineParseFunctions,
};

pub trait DensityParseFunctions<'m> {
    fn parse_density_function(&self, json_str: &str) -> ParseResult<Density<'m>>;
    fn parse_density_function_from_value(
        &self,
        value: &serde_json::Value,
    ) -> ParseResult<Density<'m>>;
    fn parse_density_function_from_value_and_name(
        &self,
        value: &serde_json::Value,
        canonical_name: &str,
    ) -> ParseResult<Density<'m>>;
}

impl<'m> DensityParseFunctions<'m> for MinecraftData<'m> {
    fn parse_density_function(&self, json_str: &str) -> ParseResult<Density<'m>> {
        // Implement parsing logic for DensityFunction
        // convert the JSON string to Values, and parse recursively
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| self.unsupported(format!("invalid JSON: {}", e)))?;
        self.parse_density_function_from_value(&value)
    }

    fn parse_density_function_from_value(
        &self,
        value: &serde_json::Value,
    ) -> ParseResult<Density<'m>> {
        // Implement recursive parsing logic for DensityFunction from serde_json::Value
        // the value may be an object with a "type" field indicating the variant
        // or a primitive value for Const
        if let Some(obj) = value.as_object() {
            let type_str = self.str_field(obj, "density function", "type")?;
            // a density function argument of this type
            let arg =
                |key: &str| self.parse_density_function_from_value(self.field(obj, type_str, key)?);
            let number = |key: &str| self.f64_field(obj, type_str, key);
            // optional numbers default to 1.0, as in vanilla's codecs
            let scale = |key: &str| obj.get(key).and_then(|v| v.as_f64()).unwrap_or(1.0);

            let density = match type_str {
                "minecraft:noise" => {
                    let noise_name = self.str_field(obj, type_str, "noise")?;
                    // noise is already defined, so just reference it
                    let noise = self.noise(noise_name)?;
                    self.arena.alloc(DensityType::Noise {
                        name: noise_name.to_string(),
                        noise,
                        xz_scale: scale("xz_scale"),
                        y_scale: scale("y_scale"),
                    })
                }
                "minecraft:cache_2d" => self.arena.alloc(DensityType::Cache2d {
                    argument: arg("argument")?,
                }),
                "minecraft:squeeze" => self.arena.alloc(DensityType::Squeeze {
                    argument: arg("argument")?,
                }),
                "minecraft:add" => {
                    let left = arg("argument1")?;
                    let right = arg("argument2")?;
                    self.arena.alloc(DensityType::Add { left, right })
                }
                "minecraft:mul" => {
                    let left = arg("argument1")?;
                    let right = arg("argument2")?;
                    self.arena.alloc(DensityType::Multiply { left, right })
                }
                "minecraft:interpolated" => self.arena.alloc(DensityType::Interpolated {
                    argument: arg("argument")?,
                }),
                // no blend_density is used, because it is for compatibility with older worlds
                "minecraft:blend_density" => arg("argument")?,
                "minecraft:end_islands" => self.arena.alloc(DensityType::EndIslands),
                "minecraft:y_clamped_gradient" => self.arena.alloc(DensityType::YClampedGradient {
                    from_y: number("from_y")?,
                    to_y: number("to_y")?,
                    from_value: number("from_value")?,
                    to_value: number("to_value")?,
                }),
                "minecraft:flat_cache" => self.arena.alloc(DensityType::FlatCache {
                    argument: arg("argument")?,
                }),
                "minecraft:old_blended_noise" => self.arena.alloc(DensityType::OldBlendedNoise {
                    smear_scale_multiplier: scale("smear_scale_multiplier"),
                    xz_factor: scale("xz_factor"),
                    xz_scale: scale("xz_scale"),
                    y_factor: scale("y_factor"),
                    y_scale: scale("y_scale"),
                }),
                "minecraft:shifted_noise" => {
                    let noise_name = self.str_field(obj, type_str, "noise")?;
                    // noise is already defined, so just reference it
                    let noise = self.noise(noise_name)?;
                    let shift_x = arg("shift_x")?;
                    let shift_y = arg("shift_y")?;
                    let shift_z = arg("shift_z")?;
                    self.arena.alloc(DensityType::ShiftedNoise {
                        name: noise_name.to_string(),
                        noise,
                        shift_x,
                        shift_y,
                        shift_z,
                        xz_scale: scale("xz_scale"),
                        y_scale: scale("y_scale"),
                    })
                }
                "minecraft:shift_a" => {
                    let noise_name = self.str_field(obj, type_str, "argument")?;
                    let argument = self.noise(noise_name)?;
                    self.arena.alloc(DensityType::ShiftA {
                        argument,
                        name: noise_name.to_string(),
                    })
                }
                "minecraft:shift_b" => {
                    let noise_name = self.str_field(obj, type_str, "argument")?;
                    let argument = self.noise(noise_name)?;
                    self.arena.alloc(DensityType::ShiftB {
                        argument,
                        name: noise_name.to_string(),
                    })
                }
                // blend_alpha and blend_offset are used for compatibility with older worlds, they
                // blend terrain heights near world borders; for new terrain they are constants
                "minecraft:blend_alpha" => self.arena.alloc(DensityType::Const(1.0)),
                "minecraft:blend_offset" => self.arena.alloc(DensityType::Const(0.0)),
                "minecraft:cache_once" => self.arena.alloc(DensityType::CacheOnce {
                    argument: arg("argument")?,
                }),
                "minecraft:spline" => {
                    let spline = self.parse_spline(self.field(obj, type_str, "spline")?)?;
                    self.arena.alloc(DensityType::Spline { spline })
                }
                "minecraft:abs" => self.arena.alloc(DensityType::Abs {
                    argument: arg("argument")?,
                }),
                "minecraft:min" => {
                    let left = arg("argument1")?;
                    let right = arg("argument2")?;
                    self.arena.alloc(DensityType::Min { left, right })
                }
                "minecraft:max" => {
                    let left = arg("argument1")?;
                    let right = arg("argument2")?;
                    self.arena.alloc(DensityType::Max { left, right })
                }
                "minecraft:range_choice" => {
                    let input = arg("input")?;
                    let min_inclusive = number("min_inclusive")?;
                    let max_exclusive = number("max_exclusive")?;
                    let when_in_range = arg("when_in_range")?;
                    let when_out_of_range = arg("when_out_of_range")?;
                    self.arena.alloc(DensityType::RangeChoice {
                        input,
                        min_inclusive,
                        max_exclusive,
                        when_in_range,
                        when_out_of_range,
                    })
                }
                // f(x) = min(x, 0) / 4, represented as a multiplier on the negative half
                "minecraft:quarter_negative" => self.arena.alloc(DensityType::XNegative {
                    argument: arg("argument")?,
                    neg_x_multiplier: 0.25,
                }),
                // f(x) = x if x >= 0, x / 2 if x < 0
                "minecraft:half_negative" => self.arena.alloc(DensityType::XNegative {
                    argument: arg("argument")?,
                    neg_x_multiplier: 0.5,
                }),
                "minecraft:clamp" => {
                    let input = arg("input")?;
                    let min = number("min")?;
                    let max = number("max")?;
                    self.arena.alloc(DensityType::Clamp { input, min, max })
                }
                "minecraft:weird_scaled_sampler" => {
                    let input = arg("input")?;
                    let noise_name = self.str_field(obj, type_str, "noise")?;
                    let noise_to_sample = self.noise(noise_name)?;
                    let rarity_value_mapper = self
                        .str_field(obj, type_str, "rarity_value_mapper")?
                        .to_string();
                    if !matches!(rarity_value_mapper.as_str(), "type_1" | "type_2") {
                        return Err(self.unsupported(format!(
                            "unknown rarity_value_mapper `{}` (expected `type_1` or `type_2`)",
                            rarity_value_mapper
                        )));
                    }
                    self.arena.alloc(DensityType::WeirdScaledSampler {
                        input,
                        noise_name: noise_name.to_string(),
                        noise_to_sample,
                        rarity_value_mapper,
                    })
                }
                "minecraft:square" => self.arena.alloc(DensityType::Square {
                    argument: arg("argument")?,
                }),
                "minecraft:cube" => self.arena.alloc(DensityType::Cube {
                    argument: arg("argument")?,
                }),
                "minecraft:invert" => self.arena.alloc(DensityType::Invert {
                    argument: arg("argument")?,
                }),
                "minecraft:find_top_surface" => {
                    let density = arg("density")?;
                    let cell_height = self.i32_field(obj, type_str, "cell_height")?;
                    if cell_height <= 0 {
                        return Err(self.unsupported(format!(
                            "`cell_height` must be positive, found {}",
                            cell_height
                        )));
                    }
                    let lower_bound = self.i32_field(obj, type_str, "lower_bound")?;
                    // upper bound is a density function
                    let upper_bound = arg("upper_bound")?;
                    self.arena.alloc(DensityType::FindTopSurface {
                        density,
                        cell_height,
                        lower_bound,
                        upper_bound,
                    })
                }
                _ => {
                    return Err(
                        self.unsupported(format!("unknown density function type `{}`", type_str))
                    );
                }
            };
            Ok(density)
        } else if let Some(num) = value.as_f64() {
            Ok(self.arena.alloc(DensityType::Const(num)))
        } else if let Some(s) = value.as_str() {
            // a reference to another density function
            let id = normalize_id(s);
            let Some(referenced) = self.raw_data.density_functions.get(&id) else {
                return Err(self.missing_reference(ResourceKind::DensityFunction, &id));
            };
            let argument = self.with_context(ResourceKind::DensityFunction, &id, || {
                self.parse_density_function(referenced)
            })?;
            self.reached_density_function(&id);
            Ok(self.arena.alloc(DensityType::NamedDensityReference {
                name: self.arena.alloc(id),
                argument,
            }))
        } else {
            Err(self.unsupported(format!(
                "a density function must be an object, number or id, found `{}`",
                value
            )))
        }
    }

    fn parse_density_function_from_value_and_name(
        &self,
        value: &serde_json::Value,
        canonical_name: &str,
    ) -> ParseResult<Density<'m>> {
        let density = self.parse_density_function_from_value(value)?;
        // create a new named reference with the given canonical name
        let name = self.arena.alloc(canonical_name.to_string());
        Ok(self.arena.alloc(DensityType::NamedDensityReference {
            name,
            argument: density,
        }))
    }
}
