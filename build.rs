#[cfg(not(windows))]
fn print_platform_settings() {}

#[cfg(windows)]
fn print_platform_settings() {
    println!("cargo:rustc-link-lib=dylib=secur32");
    println!("cargo:rustc-link-lib=dylib=netapi32");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    print_platform_settings();
}
