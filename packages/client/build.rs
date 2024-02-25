use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command,
};

use client_macros::generate_test_selectors;
use common::vendor::serde_json;

fn prebuild_vite_assets() {
    assert!(
        Command::new("npx")
            .arg("vite")
            .arg("build")
            .status()
            .expect("Failed to prebuild")
            .success(),
        "Prebuild failed"
    );
}

fn get_target_dir_path() -> PathBuf {
    let skip_triple_dir =
        std::env::var("TARGET").unwrap() == std::env::var("HOST").unwrap();
    let num_dirs_to_skip = if skip_triple_dir { 4 } else { 5 };

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut current = out_dir.as_path();
    for _ in 0..num_dirs_to_skip {
        current = current.parent().unwrap();
    }

    PathBuf::from(current)
}

fn build_test_selectors() {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let selectors = generate_test_selectors!();

    let mut out = get_target_dir_path();
    out.push(format!("{crate_name}_test_selectors.json"));

    let file = File::create(out).expect("JSON file should be created");

    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &selectors).unwrap();
    writer.flush().unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=app");
    println!("cargo:rerun-if-changed=components");
    println!("cargo:rerun-if-changed=hooks");
    println!("cargo:rerun-if-changed=layouts");
    println!("cargo:rerun-if-changed=pages");
    println!("cargo:rerun-if-changed=styles");
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=uno.config.ts");
    println!("cargo:rerun-if-changed=vite.config.ts");

    // Force rebuilding after each app build
    println!("cargo:rerun-if-changed=../../target/client");

    if cfg!(feature = "vite-prebuild") {
        prebuild_vite_assets();
    }

    if cfg!(feature = "e2e-selectors") {
        build_test_selectors();
    }
}
