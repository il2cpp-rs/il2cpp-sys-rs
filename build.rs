use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment variable not set");

    let unity_version = env::var("UNITY_VERSION")
        .expect("UNITY_VERSION environment variable must be set. Find versions at https://unity.com/releases/editor/archive");
    let unity_dll_name = env::var("UNITY_DLL_NAME").unwrap_or_else(|_| "GameAssembly".to_owned());

    println!("cargo:rerun-if-env-changed=UNITY_VERSION");
    println!("cargo:rerun-if-env-changed=UNITY_DLL_NAME");

    let header_path = Path::new("include/headers").join(format!("{unity_version}.h"));
    let api_path = Path::new("include/api").join(format!("{unity_version}.h"));

    for path in &[&header_path, &api_path] {
        if !fs::exists(path).unwrap() {
            panic!(
                "Unity version \"{unity_version}\" is not supported (missing file: {path:?}).\n\
                Try using the closest version possible or verify the file paths.\n\
                Versions list can be found at https://unity.com/releases/editor/archive",
            );
        }
    }

    let bindings = bindgen::Builder::default()
        .clang_args(["-include", "stdint.h"])
        .clang_args(["-include", "stdbool.h"])
        .clang_args(["-D", "DO_API(r, n, p) = r n p;"])
        .header(header_path.to_str().unwrap())
        .header(api_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    let output_path = out_dir.join("il2cpp_sys.rs");
    let mut output_content = Cursor::new(Vec::new());

    bindings
        .write(Box::new(&mut output_content))
        .expect("Failed to write bindings");

    let mut output_str = String::from_utf8(output_content.into_inner())
        .expect("Generated bindings contained invalid UTF-8");

    output_str = output_str.replace(
        r#"unsafe extern "C" {"#,
        &format!(
            r#"#[link(name = "{unity_dll_name}", kind = "raw-dylib", modifiers = "+verbatim")]
unsafe extern "C" {{"#
        ),
    );

    if let Err(err) = fs::write(&output_path, output_str) {
        panic!("Failed to write bindings to {output_path:?} {err:?}")
    }
}
