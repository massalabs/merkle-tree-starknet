use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build"])
        .status()
        .unwrap();

    if !status.success() {
        panic!("cargo build failed");
    }

    let bindings_src = project_root().join("rust_ffi").join("bindings.h");
    let bindings_dst = project_root().join("bindings.h");

    fs::copy(&bindings_src, bindings_dst).unwrap();

    // find rust_ffi produced artefacts and copy them to the project root
    // it depends on the os and the target architecture
    match env::consts::OS {
        "linux" => {
            fs::copy(
                project_root().join("target/debug/librust_ffi.so"),
                project_root().join("librust_ffi.so"),
            )
            .unwrap();
        }
        "macos" => {
            fs::copy(
                project_root().join("target/debug/librust_ffi.dylib"),
                project_root().join("librust_ffi.dylib"),
            )
            .unwrap();
        }
        "windows" => {
            fs::copy(
                project_root().join("target/debug/rust_ffi.dll"),
                project_root().join("rust_ffi.dll"),
            )
            .unwrap();
        }
        _ => panic!("unsupported OS"),
    }
    

}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
