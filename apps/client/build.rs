use std::process::Command;

fn main() {
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
