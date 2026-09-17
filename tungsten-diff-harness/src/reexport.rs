use std::fs;
use std::path::Path;

use serde_json::{json, Value};
use tungsten_mc_compile::parse::model::{Density, DensitySource, DensityType, Spline, SplineValue};

/// Serializes a density function tree back into the Minecraft datapack JSON
/// format understood by the parser (`parse_density_function_from_value`).
///
/// A `NamedDensityReference` is exported as a plain name string, matching how
/// real datapacks reference other density functions (e.g. spline
/// coordinates referencing `"minecraft:overworld/continents"`).
///
/// Noise references are exported by name only (as the format expects), so a
/// `NormalNoise` whose amplitudes were changed by a shrink method (e.g.
/// `SimplifyNoiseParams`) will re-parse using the *original* noise under
/// that name rather than the modified one.
pub fn density_to_json(density: Density) -> Value {
    match &*density {
        DensityType::Const(c) => json!(c),

        DensityType::Noise { name, xz_scale, y_scale, .. } => json!({
            "type": "minecraft:noise",
            "noise": name,
            "xz_scale": xz_scale,
            "y_scale": y_scale,
        }),

        DensityType::Add { left, right } => json!({
            "type": "minecraft:add",
            "argument1": density_to_json(*left),
            "argument2": density_to_json(*right),
        }),
        DensityType::Multiply { left, right } => json!({
            "type": "minecraft:mul",
            "argument1": density_to_json(*left),
            "argument2": density_to_json(*right),
        }),
        DensityType::Min { left, right } => json!({
            "type": "minecraft:min",
            "argument1": density_to_json(*left),
            "argument2": density_to_json(*right),
        }),
        DensityType::Max { left, right } => json!({
            "type": "minecraft:max",
            "argument1": density_to_json(*left),
            "argument2": density_to_json(*right),
        }),

        DensityType::Cache2d { argument } => json!({
            "type": "minecraft:cache_2d",
            "argument": density_to_json(*argument),
        }),
        DensityType::Squeeze { argument } => json!({
            "type": "minecraft:squeeze",
            "argument": density_to_json(*argument),
        }),
        DensityType::Interpolated { argument } => json!({
            "type": "minecraft:interpolated",
            "argument": density_to_json(*argument),
        }),
        DensityType::FlatCache { argument } => json!({
            "type": "minecraft:flat_cache",
            "argument": density_to_json(*argument),
        }),
        DensityType::CacheOnce { argument } => json!({
            "type": "minecraft:cache_once",
            "argument": density_to_json(*argument),
        }),
        DensityType::Abs { argument } => json!({
            "type": "minecraft:abs",
            "argument": density_to_json(*argument),
        }),
        DensityType::Square { argument } => json!({
            "type": "minecraft:square",
            "argument": density_to_json(*argument),
        }),
        DensityType::Cube { argument } => json!({
            "type": "minecraft:cube",
            "argument": density_to_json(*argument),
        }),

        DensityType::EndIslands => json!({ "type": "minecraft:end_islands" }),

        DensityType::YClampedGradient { from_y, to_y, from_value, to_value } => json!({
            "type": "minecraft:y_clamped_gradient",
            "from_y": from_y,
            "to_y": to_y,
            "from_value": from_value,
            "to_value": to_value,
        }),

        DensityType::OldBlendedNoise { smear_scale_multiplier, xz_factor, xz_scale, y_factor, y_scale } => json!({
            "type": "minecraft:old_blended_noise",
            "smear_scale_multiplier": smear_scale_multiplier,
            "xz_factor": xz_factor,
            "xz_scale": xz_scale,
            "y_factor": y_factor,
            "y_scale": y_scale,
        }),

        DensityType::ShiftedNoise { name, shift_x, shift_y, shift_z, xz_scale, y_scale, .. } => json!({
            "type": "minecraft:shifted_noise",
            "noise": name,
            "shift_x": density_to_json(*shift_x),
            "shift_y": density_to_json(*shift_y),
            "shift_z": density_to_json(*shift_z),
            "xz_scale": xz_scale,
            "y_scale": y_scale,
        }),
        DensityType::ShiftA { name, .. } => json!({
            "type": "minecraft:shift_a",
            "argument": name,
        }),
        DensityType::ShiftB { name, .. } => json!({
            "type": "minecraft:shift_b",
            "argument": name,
        }),

        DensityType::Spline { spline } => json!({
            "type": "minecraft:spline",
            "spline": spline_to_json(*spline),
        }),

        DensityType::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => json!({
            "type": "minecraft:range_choice",
            "input": density_to_json(*input),
            "min_inclusive": min_inclusive,
            "max_exclusive": max_exclusive,
            "when_in_range": density_to_json(*when_in_range),
            "when_out_of_range": density_to_json(*when_out_of_range),
        }),

        // quarter_negative / half_negative both parse into XNegative; tell them
        // apart by the multiplier the parser assigns to each.
        DensityType::XNegative { argument, neg_x_multiplier } => json!({
            "type": if *neg_x_multiplier == 0.25 { "minecraft:quarter_negative" } else { "minecraft:half_negative" },
            "argument": density_to_json(*argument),
        }),

        DensityType::Clamp { input, min, max } => json!({
            "type": "minecraft:clamp",
            "input": density_to_json(*input),
            "min": min,
            "max": max,
        }),

        DensityType::WeirdScaledSampler { input, noise_name, rarity_value_mapper, .. } => json!({
            "type": "minecraft:weird_scaled_sampler",
            "input": density_to_json(*input),
            "noise": noise_name,
            "rarity_value_mapper": rarity_value_mapper,
        }),

        // Exported as a plain name reference, matching the real format.
        DensityType::NamedDensityReference { name, .. } => json!(name),
    }
}

fn spline_to_json(spline: Spline) -> Value {
    let points: Vec<Value> = spline
        .spline_points
        .iter()
        .map(|point| {
            let value = match &point.value {
                SplineValue::Const(c) => json!(c),
                SplineValue::Spline(inner) => spline_to_json(*inner),
            };
            json!({
                "location": point.location,
                "value": value,
                "derivative": point.derivative,
            })
        })
        .collect();

    json!({
        "coordinate": density_to_json(spline.coordinate),
        "points": points,
    })
}

/// Serializes a whole `final_density`-style density source back into JSON,
/// unwrapping the outermost debug label(s) the `Shrinker` adds so the
/// resulting tree is exported in full rather than as a bare name reference.
pub fn density_source_to_json(source: &DensitySource) -> Value {
    let mut density = *source.get_density();
    while let DensityType::NamedDensityReference { argument, .. } = &*density {
        density = *argument;
    }
    density_to_json(density)
}

/// Writes a shrink candidate's density function to `dir`, tagging the
/// filename with whether the mismatch still reproduced with it.
pub fn save_shrink_iteration(dir: &Path, iteration: u32, source: &DensitySource, reproduced: bool) {
    fs::create_dir_all(dir).expect("failed to create shrink log directory");

    let status = if reproduced { "kept" } else { "reverted" };
    let path = dir.join(format!("iteration_{iteration:04}_{status}.json"));

    let json = density_source_to_json(source);
    let pretty = serde_json::to_string_pretty(&json).expect("failed to serialize density function");
    fs::write(&path, pretty).expect("failed to write shrink iteration snapshot");
}
