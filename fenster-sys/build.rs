fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    cc::Build::new()
        .include("fenster")
        .file("src/wrapper.c")
        .compile("fenster");

    if target_os == "windows" {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
    } else if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=AppKit");
    } else {
        panic!("Unsupported OS: {target_os}");
    }

    println!("cargo:rerun-if-changed=src/wrapper.c");
}
