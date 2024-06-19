use std::{env, path::PathBuf};

const INCLUDES: &str = "
#include <flite.h>
#include <cmu_grapheme_lang/cmu_grapheme_lang.h>
#include <cmu_grapheme_lex/cmu_grapheme_lex.h>
#include <cmu_indic_lang/cmu_indic_lang.h>
#include <cmu_indic_lex/cmu_indic_lex.h>
#include <cmulex/cmu_lex.h>
#include <usenglish/usenglish.h>
";

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut build = flite_src::Build::new();
    let artifacts = build.build();
    let include_dir = artifacts.include_dir();

    bindgen::builder().header_contents("includes.h", INCLUDES).clang_args([
        "-I",
        &include_dir.display().to_string(),
        "-I",
        &include_dir.join("flite").display().to_string(),
    ])
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    artifacts.print_cargo_metadata();
}
