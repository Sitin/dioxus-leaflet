use std::env::var;
use std::fs::copy;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Do not run in release mod
    if var("PROFILE").unwrap_or_default() != "debug" {
        return;
    }

    // Do not run when not in development repository
    if !base_path.join("..").join(".DIOXUS-LEAFLET").exists() {
        return;
    }

    // Force rerun if `./bindings/` has changed
    println!("cargo:rerun-if-changed=bindings");

    // Install NPM packages if required
    if !base_path.join("..").join("node_modules").exists() && !Command::new("npm")
        .args(["install"])
        .status()
        .expect("Unable to run 'npm' command").success() {
        panic!("Install of NPM packages");
    }

    // Copy Leaflet assets
    copy(
        base_path
            .join("..")
            .join("node_modules")
            .join("leaflet")
            .join("dist")
            .join("leaflet.css"),
        base_path
            .join("assets")
            .join("vendors")
            .join("leaflet.css"),
    ).expect("Failed to copy leaflet CSS files");

    // Generate TypeScript bindings for Rust types from `dioxus-leaflet-core`
    let dist = base_path
        .join("bindings")
        .join("src")
        .join("core.gen.ts");
    dioxus_leaflet_core::generate(dist)
        .expect("Core types from 'core.gen.ts' generation failed");

    // Bundle bindings using NPM
    if !Command::new("npm")
        .args(["run", "build"])
        .status()
        .expect("Unable to run 'npm' command").success() {
        panic!("Build via NPM failed!");
    }

    // Copy bindings into `assets` folder
    copy(
        base_path
            .join("bindings")
            .join("dist")
            .join("bindings.js"),
        base_path
            .join("assets")
            .join("bindings.js"),
    ).expect("Failed to copy generated JS bindings to 'assets' folder");
}
