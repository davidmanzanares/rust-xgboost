use cmake::Config;

extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let xgb_root = Path::new(&out_dir).join("xgboost");

    // copy source code into OUT_DIR for compilation if it doesn't exist
    if !xgb_root.exists() {
        Command::new("cp")
            .args(&["-r", "xgboost", xgb_root.to_str().unwrap()])
            .status()
            .unwrap_or_else(|e| {
                panic!("Failed to copy ./xgboost to {}: {}", xgb_root.display(), e);
            });
    }

    // TODO: allow for dynamic/static linking
    // TODO: check whether rabit should be built/linked
    // if !xgb_root.join("lib").exists() {
    // TODO: better checks for build completion, currently xgboost's build script can run
    // `make clean_all` if openmp build fails
    // Command::new("cmake .")
    //     .current_dir(&xgb_root)
    //     .status()
    //     .expect("Failed to execute cmake");

    //     Command::new(xgb_root.join("make -j20"))
    //     .current_dir(&xgb_root)
    //     .status()
    //     .expect("Failed to execute make");
    let _xgboost = Config::new("xgboost").profile("Release").define("BUILD_STATIC_LIB", "ON").define("RABIT_MOCK", "ON").define("USE_OPENMP", "OFF").build();

    // }

    let mut xgb_root = xgb_root.canonicalize().unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", xgb_root.join("include").display()))
        // .clang_arg(format!("-I{}", xgb_root.join("rabit/include").display()))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");

    xgb_root.pop();
    println!(
        "cargo:rustc-link-search={}",
        xgb_root.join("lib").display()
    );
    // println!("cargo:rustc-link-search={}", xgb_root.join("rabit/lib").display());


    // check if built with multithreading support, otherwise link to dummy lib
    // if xgb_root.join("rabit/lib/librabit.a").exists() {
    //     println!("cargo:rustc-link-lib=static=rabit");
    //     println!("cargo:rustc-link-lib=dylib=gomp");
    // } else {
    //     println!("cargo:rustc-link-lib=static=rabit_empty");
    // }

    // link to appropriate C++ lib
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rustc-link-lib=static=dmlc");
    println!("cargo:rustc-link-lib=static=xgboost");

    println!(
        "cargo:rustc-link-search={}",
        xgb_root.join("dmlc-core").display()
    );
}
