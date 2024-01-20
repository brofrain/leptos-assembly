use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=app");
    println!("cargo:rerun-if-changed=components");
    println!("cargo:rerun-if-changed=composables");
    println!("cargo:rerun-if-changed=layouts");
    println!("cargo:rerun-if-changed=pages");
    println!("cargo:rerun-if-changed=styles");
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=uno.config.ts");
    println!("cargo:rerun-if-changed=vite.config.ts");

    if !cfg!(feature = "vite-prebuild") {
        return;
    }

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
