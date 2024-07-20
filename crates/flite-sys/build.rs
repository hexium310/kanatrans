use std::{env, path::PathBuf};

use bindgen::Builder;

#[cfg(not(feature = "vendored"))]
const INCLUDES: &str = "
#include <flite/flite.h>
";

#[cfg(feature = "vendored")]
const INCLUDES: &str = "
#include <flite/flite.h>
#include <cmu_grapheme_lang/cmu_grapheme_lang.h>
#include <cmu_grapheme_lex/cmu_grapheme_lex.h>
#include <cmu_indic_lang/cmu_indic_lang.h>
#include <cmu_indic_lex/cmu_indic_lex.h>
#include <cmulex/cmu_lex.h>
#include <usenglish/usenglish.h>
";

#[cfg(not(feature = "vendored"))]
const LIBS: &[&str] = &[
    "flite",
    "flite_cmu_grapheme_lang",
    "flite_cmu_grapheme_lex",
    "flite_cmu_indic_lang",
    "flite_cmu_indic_lex",
    "flite_cmu_time_awb",
    "flite_cmu_us_awb",
    "flite_cmu_us_kal16",
    "flite_cmu_us_kal",
    "flite_cmu_us_rms",
    "flite_cmu_us_slt",
    "flite_cmulex",
    "flite_usenglish",
];

#[cfg(feature = "vendored")]
fn builder() -> Builder {
    let mut build = flite_src::Build::new();
    let artifacts = build.build();
    artifacts.print_cargo_metadata();

    let include_dir = artifacts.include_dir();
    let builder = bindgen::builder().header_contents("includes.h", INCLUDES).clang_args([
        "-I",
        &include_dir.display().to_string(),
        "-I",
        &include_dir.join("flite").display().to_string(),
    ]);

    match env::var_os("CARGO_CFG_TARGET_OS") {
        Some(os) if os == "wasi" => {
            let wasi_sdk_path = env::var("WASI_SDK_PATH").unwrap();
            let wasi_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
            builder.clang_args([
                "-I",
                &format!("{wasi_sdk_path}/share/wasi-sysroot/include/wasm32-wasi{wasi_env}"),
                "-D",
                "CST_NO_SOCKETS",
                "-D",
                "DIE_ON_ERROR",
                "-D",
                "WASM32_WASI",
                "-fvisibility=default",
            ])
        },
        _ => builder,
    }
}

#[cfg(not(feature = "vendored"))]
fn builder() -> Builder {
    for lib in LIBS {
        println!("cargo::rustc-link-lib=dylib={}", lib);
    }

    bindgen::builder().header_contents("includes.h", INCLUDES)
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    builder()
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
}
