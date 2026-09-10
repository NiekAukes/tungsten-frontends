use std::process::Command;

use crate::paths::HarnessConfig;

/// Phase 1 (Rust side): compiles `rust_wrapper` (which `include!`s the
/// generated `density.rs`) into a cdylib via `cargo build`.
///
/// We shell out to `cargo` rather than raw `rustc` because the wrapper needs
/// the same external dependency (`md5`, used by xoroshiro.rs) that
/// tungsten-mc itself depends on; cargo resolves and builds that for us
/// instead of us having to hand-locate rlibs.
pub fn build_rust_cdylib(config: &HarnessConfig) {
    let density_rs = config.density_rs();
    assert!(
        density_rs.exists(),
        "generated density.rs not found at {}",
        density_rs.display()
    );

    let target_dir = config.build_dir.join("rust");

    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&config.rust_wrapper_manifest)
        .arg("--target-dir")
        .arg(&target_dir)
        .env(
            "DENSITY_RS_PATH",
            density_rs
                .canonicalize()
                .expect("failed to canonicalize density.rs path"),
        )
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .status()
        .expect("failed to spawn `cargo build` for the Rust wrapper");

    assert!(status.success(), "cargo build of rust_wrapper failed");

    let so_path = config.rust_cdylib_path();
    assert!(
        so_path.exists(),
        "expected Rust cdylib at {}, but it was not produced",
        so_path.display()
    );
}
