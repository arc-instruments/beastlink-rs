use std::env;

#[cfg(any(target_os="linux"))]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    match env::var("LIBRARY_PATH") {
        Ok(path) => {
            let parts = path.split(":");
            for s in parts {
                println!("cargo:rustc-link-search={}", s);
            }
        },
        Err(_) => {}
    }

    println!("cargo:rustc-link-search={}/build", out_dir);

    println!("cargo:rustc-link-lib=dylib=beastlink-1.0");
}

