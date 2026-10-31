use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use crate::error::ResourceKind;

/// Raw worldgen JSON by registry id (`minecraft:overworld/continents`), one map per kind.
#[derive(Clone)]
pub struct MinecraftDataRaw {
    pub(crate) noise_settings: HashMap<String, String>,
    pub(crate) density_functions: HashMap<String, String>,
    pub(crate) normal_noises: HashMap<String, String>,
    /// Fed files that weren't valid UTF-8; any of them fails the compile.
    pub(crate) invalid_utf8: BTreeSet<(ResourceKind, String)>,
}

impl MinecraftDataRaw {
    pub fn new() -> Self {
        MinecraftDataRaw {
            noise_settings: HashMap::new(),
            density_functions: HashMap::new(),
            normal_noises: HashMap::new(),
            invalid_utf8: BTreeSet::new(),
        }
    }

    fn map_mut(&mut self, kind: ResourceKind) -> &mut HashMap<String, String> {
        match kind {
            ResourceKind::NoiseSettings => &mut self.noise_settings,
            ResourceKind::DensityFunction => &mut self.density_functions,
            ResourceKind::Noise => &mut self.normal_noises,
        }
    }

    /// Stores the resource at `id`, a resource path as Minecraft's resource manager sees it, e.g.
    /// `minecraft:worldgen/density_function/overworld/continents.json`. Overwrites an earlier
    /// resource with the same id. Returns false (and stores nothing) for anything that isn't a noise
    /// settings, density function or noise file.
    pub fn insert_resource(&mut self, id: &str, bytes: &[u8]) -> bool {
        let Some((kind, key)) = classify(id) else {
            return false;
        };
        match std::str::from_utf8(bytes) {
            Ok(text) => {
                self.invalid_utf8.remove(&(kind, key.clone()));
                self.map_mut(kind).insert(key, text.to_string());
            }
            Err(_) => {
                self.map_mut(kind).remove(&key);
                self.invalid_utf8.insert((kind, key));
            }
        }
        true
    }
}

/// `ns:worldgen/<kind>/<path>.json` -> `(kind, ns:<path>)`.
fn classify(id: &str) -> Option<(ResourceKind, String)> {
    let (namespace, path) = id.split_once(':')?;
    let (folder, rest) = path.strip_prefix("worldgen/")?.split_once('/')?;
    let kind = ResourceKind::from_folder(folder)?;
    let name = rest.strip_suffix(".json")?;
    Some((kind, format!("{}:{}", namespace, name)))
}

impl Debug for MinecraftDataRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinecraftData")
            .field("noise_settings", &self.noise_settings.keys())
            .field("density_functions", &self.density_functions.keys())
            .field("normal_noises", &self.normal_noises.keys())
            .field("invalid_utf8", &self.invalid_utf8)
            .finish()
    }
}

/// Calls `feed(id, bytes)` for every file under `<root>/data/<namespace>/worldgen/`, with ids in
/// the form [`MinecraftDataRaw::insert_resource`] takes, in sorted order.
pub fn walk_datapack_folder(root: &Path, feed: &mut dyn FnMut(&str, &[u8])) -> std::io::Result<()> {
    let mut namespaces = sorted_entries(&root.join("data"))?;
    namespaces.retain(|p| p.is_dir());
    for namespace_dir in namespaces {
        let namespace = namespace_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let worldgen = namespace_dir.join("worldgen");
        if worldgen.is_dir() {
            walk_files(&worldgen, &format!("{}:worldgen", namespace), feed)?;
        }
    }
    Ok(())
}

/// Feeds every file under `dir` as `<prefix>/<relative path>`.
fn walk_files(dir: &Path, prefix: &str, feed: &mut dyn FnMut(&str, &[u8])) -> std::io::Result<()> {
    for path in sorted_entries(dir)? {
        let name = format!("{}/{}", prefix, path.file_name().unwrap().to_string_lossy());
        if path.is_dir() {
            walk_files(&path, &name, feed)?;
        } else {
            feed(&name, &fs::read(&path)?);
        }
    }
    Ok(())
}

fn sorted_entries(dir: &Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut entries = fs::read_dir(dir)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort();
    Ok(entries)
}

/// Loads a datapack folder (one containing `data/`) into `data`, overwriting by id.
pub fn load_datapack_folder(data: &mut MinecraftDataRaw, root: &Path) -> std::io::Result<()> {
    walk_datapack_folder(root, &mut |id, bytes| {
        data.insert_resource(id, bytes);
    })
}

pub fn reexport(data: &MinecraftDataRaw, out_dir: &str) {
    fs::create_dir_all(out_dir).unwrap();

    // helper macro to reduce repetition
    macro_rules! export_category {
        ($map:expr, $folder:expr, $ext:expr) => {{
            let category_dir = Path::new(out_dir).join($folder);
            fs::create_dir_all(&category_dir).unwrap();
            for (name, config) in $map {
                // turn "minecraft:overworld" → "minecraft_overworld.json"
                let safe_name = name.replace(':', "_").replace('/', "___");
                let filename = format!("{}.{}", safe_name, $ext);
                let filepath = Path::new(&category_dir).join(filename);
                fs::File::create(&filepath).unwrap();
                let content = format!("\"{}\"\n{}", name, config);
                fs::write(filepath, content).unwrap();
            }
        }};
    }
    export_category!(&data.noise_settings, "noise_settings", "w");
    export_category!(&data.density_functions, "density_functions", "density");
    export_category!(&data.normal_noises, "normal_noises", "noise");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_by_path() {
        assert_eq!(
            classify("minecraft:worldgen/density_function/overworld/continents.json"),
            Some((
                ResourceKind::DensityFunction,
                "minecraft:overworld/continents".into()
            ))
        );
        assert_eq!(
            classify("jj:worldgen/noise/ridge.json"),
            Some((ResourceKind::Noise, "jj:ridge".into()))
        );
        assert_eq!(
            classify("minecraft:worldgen/noise_settings/amplified.json"),
            Some((ResourceKind::NoiseSettings, "minecraft:amplified".into()))
        );
        assert_eq!(classify("minecraft:worldgen/biome/plains.json"), None);
        assert_eq!(classify("minecraft:tags/worldgen/noise/x.json"), None);
        assert_eq!(classify("minecraft:worldgen/noise/x.txt"), None);
    }

    #[test]
    fn last_resource_wins() {
        let mut data = MinecraftDataRaw::new();
        let id = "minecraft:worldgen/noise/a.json";
        data.insert_resource(id, b"\xff");
        assert!(
            data.invalid_utf8
                .contains(&(ResourceKind::Noise, "minecraft:a".into()))
        );
        data.insert_resource(id, b"{}");
        assert!(data.invalid_utf8.is_empty());
        assert_eq!(data.normal_noises["minecraft:a"], "{}");
    }
}
