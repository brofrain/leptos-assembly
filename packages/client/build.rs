#![feature(let_chains)]

use std::{
    fs::File,
    io::{BufWriter, Write},
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

fn build_test_selectors() {
    let crate_name = std::env!("CARGO_CRATE_NAME");
    let selectors = generate_test_selectors!();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let file =
        File::create(format!("{out_dir}/{crate_name}_test_selectors.json"))
            .expect("JSON file should be created");

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

    if cfg!(feature = "vite-prebuild") {
        prebuild_vite_assets();
    }

    if let Ok(arch) = std::env::var("CARGO_CFG_TARGET_ARCH")
        && arch == "wasm32"
    {
        build_test_selectors();
    }
}
