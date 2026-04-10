fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "linux" {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=GL");
    }
}
