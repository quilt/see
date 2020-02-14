use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let status = Command::new("cargo")
        .args(&[
            "build",
            "--target=wasm32-unknown-unknown",
            "--manifest-path=../ee/Cargo.toml",
        ])
        .arg(&format!("--target-dir={}", out_dir))
        .status()
        .unwrap();

    if status.success() {
        println!("ewasm build successful");
        println!("out_dir: {}", out_dir);
    } else {
        println!("ewasm build failed...\n{}", status);
    }
}
