use std::fmt;

use serde::Serialize;

/// The worldgen registries the compiler reads. Everything else under `worldgen/` is ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    NoiseSettings,
    DensityFunction,
    Noise,
}

impl ResourceKind {
    /// The registry folder under `worldgen/`, which is also the serialized name.
    pub fn folder(self) -> &'static str {
        match self {
            ResourceKind::NoiseSettings => "noise_settings",
            ResourceKind::DensityFunction => "density_function",
            ResourceKind::Noise => "noise",
        }
    }

    pub fn from_folder(folder: &str) -> Option<Self> {
        match folder {
            "noise_settings" => Some(ResourceKind::NoiseSettings),
            "density_function" => Some(ResourceKind::DensityFunction),
            "noise" => Some(ResourceKind::Noise),
            _ => None,
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.folder())
    }
}

/// Why a compile failed. `Unsupported` is a problem with the worldgen that was fed in (the user can
/// act on it); `Internal` is a compiler bug. Panics are neither: they propagate to the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    /// `id` is a registry id such as `minecraft:overworld/continents`.
    Unsupported {
        kind: ResourceKind,
        id: String,
        message: String,
    },
    Internal(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Unsupported { kind, id, message } => {
                write!(f, "unsupported {} `{}`: {}", kind, id, message)
            }
            CompileError::Internal(message) => write!(f, "internal compiler error: {}", message),
        }
    }
}

impl std::error::Error for CompileError {}

/// Adds the `minecraft:` namespace to an id that has none, as Minecraft does.
pub fn normalize_id(id: &str) -> String {
    if id.contains(':') {
        id.to_string()
    } else {
        format!("minecraft:{}", id)
    }
}
