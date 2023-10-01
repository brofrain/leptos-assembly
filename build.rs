use std::process::Command;

fn main() {
    if cfg!(feature = "server") {
        return;
    }

    let profile =
        std::env::var("PROFILE").expect("Failed to access build profile");

    let vite_mode = match profile.as_str() {
        "debug" => "development",
        _ => "production",
    };

    assert!(
        Command::new("just")
            .arg("_prebuild")
            .arg(vite_mode)
            .status()
            .expect("Failed to prebuild")
            .success(),
        "Prebuild failed"
    );
}
