use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(rand_nightly_simd)");

    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));
    let Ok(output) = Command::new(rustc).arg("--version").output() else {
        return;
    };
    let Ok(version) = String::from_utf8(output.stdout) else {
        return;
    };

    if version.contains("nightly") || version.contains("dev") {
        println!("cargo:rustc-cfg=rand_nightly_simd");
    }
}
