use std::path::PathBuf;
use std::ffi::OsStr;
use std::env;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Build the WASM with Cargo
    let cargo_args: Vec<&OsStr> = vec![
        "build".as_ref(),
        "--release".as_ref(),
        "--package".as_ref(),
        "chippy".as_ref(),
        "--target".as_ref(),
        "wasm32-unknown-unknown".as_ref()
    ];
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(&cargo)
        .current_dir(&manifest_dir)
        .args(&cargo_args)
        .status()
        .unwrap();
    if !status.success() {
        println!("Failed due to cargo error");
        return;
    }

    // Generate the JS binds using wasm-bindgen
    let wasm_source = manifest_dir.clone()
        .join("..")
        .join("..")
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("chippy.wasm");
    let bindgen_dest = manifest_dir.clone()
        .join("..")
        .join("..")
        .join("web")
        .join("view")
        .join("wasm");
    let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
    bindgen
        .typescript(true)
        .web(true)
        .unwrap()
        .omit_default_module_path(false)
        .input_path(&wasm_source)
        .generate(&bindgen_dest)
        .unwrap();
}
