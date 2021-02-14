#[cfg(target_os="linux")]
use std::env;

#[cfg(target_os="linux")]
fn main() {

    // Also look in $LIBRARY_PATH
    match env::var("LIBRARY_PATH") {
        Ok(path) => {
            let parts = path.split(":");
            for s in parts {
                println!("cargo:rustc-link-search={}", s);
            }
        },
        Err(_) => {}
    }

    println!("cargo:rustc-link-lib=dylib=beastlink-1.0");
}

#[cfg(target_os="windows")]
fn main() {
    #[cfg(target_arch="x86_64")]
    println!("cargo:rustc-link-lib=dylib=beastlink-1.0-x86_64");

    #[cfg(target_arch="x86")]
    println!("cargo:rustc-link-lib=dylib=beastlink-1.0-x86");
}
