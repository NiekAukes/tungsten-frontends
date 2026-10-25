use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ffi::DIMS;
use crate::paths::HarnessConfig;

/// Recursively collects every `*.java` file under `dir`, sorted so the
/// `javac` command line is stable.
fn collect_java_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read Java source dir {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("failed to read directory entry").path();
        if path.is_dir() {
            collect_java_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "java") {
            out.push(path);
        }
    }
    out.sort();
}

/// Java side of phase 1: compiles the runtime, the driver and the generated
/// `GeneratedWorldgen.java` into a fresh classes directory with `javac`.
///
/// The directory is wiped first so a class left over from an earlier shrink
/// iteration can't survive.
pub fn build_java(config: &HarnessConfig) {
    let generated = config.java_source();
    assert!(
        generated.exists(),
        "generated GeneratedWorldgen.java not found at {}",
        generated.display()
    );
    assert!(
        config.java_runtime_dir.is_dir(),
        "Java runtime dir {} does not exist",
        config.java_runtime_dir.display()
    );

    let mut sources = Vec::new();
    collect_java_files(&config.java_runtime_dir, &mut sources);
    sources.push(config.java_driver_source.clone());
    sources.push(generated);

    let classes_dir = config.java_classes_dir();
    if classes_dir.exists() {
        fs::remove_dir_all(&classes_dir).expect("failed to clear Java classes dir");
    }
    fs::create_dir_all(&classes_dir).expect("failed to create Java classes dir");

    let output = Command::new("javac")
        .args(["--release", "21", "-d"])
        .arg(&classes_dir)
        .args(&sources)
        .output()
        .expect("failed to spawn `javac`; is a JDK 21 installed and on PATH?");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{stderr}");
        let hint = if stderr.contains("code too large") {
            "\nhint: a generated method exceeds the 64 KB bytecode limit; the Java backend's \
             size warnings (printed by tungsten-mc-compile) should name it"
        } else {
            ""
        };
        panic!("javac failed{hint}");
    }
}

/// Java side of phase 2 + 3: runs the driver in a JVM and reads back the
/// `DIMS` little-endian doubles it wrote.
///
/// A subprocess instead of an embedded JVM: the harness already shells out
/// to `cargo` and `nvcc`, and embedding would need `libjvm` on the link path.
pub fn run_java_pipeline(config: &HarnessConfig) -> Box<[f64]> {
    let output_path = config.java_output_path();
    if output_path.exists() {
        fs::remove_file(&output_path).expect("failed to remove stale Java output");
    }

    let (ox, oy, oz) = config.origin;
    let output = Command::new("java")
        // The recursive generated code needs a bigger stack, like the Rust side.
        .arg("-Xss64m")
        .arg("-cp")
        .arg(config.java_classes_dir())
        .arg("tungsten.harness.HarnessMain")
        // `{:?}` always prints a parseable f64 and keeps -0.0.
        .args([config.seed.to_string(), format!("{ox:?}"), format!("{oy:?}"), format!("{oz:?}")])
        .arg(&output_path)
        .output()
        .expect("failed to spawn `java`; is a JDK 21 installed and on PATH?");

    if !output.status.success() {
        panic!(
            "Java pipeline failed ({}):\n{}{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    let bytes = fs::read(&output_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", output_path.display()));
    assert_eq!(
        bytes.len(),
        DIMS * 8,
        "Java output has {} bytes, expected {} ({DIMS} f64 samples)",
        bytes.len(),
        DIMS * 8
    );
    bytes
        .chunks_exact(8)
        .map(|c| f64::from_le_bytes(c.try_into().unwrap()))
        .collect()
}
