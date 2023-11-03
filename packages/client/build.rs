use std::process::Command;

fn main() {
    if cfg!(feature = "ssr") {
        return;
    }

    // @kw remove all just --unstable flags and move the prebuild process here
    assert!(
        Command::new("just")
            .arg("--unstable")
            .arg("_prebuild")
            .status()
            .expect("Failed to prebuild")
            .success(),
        "Prebuild failed"
    );
}
