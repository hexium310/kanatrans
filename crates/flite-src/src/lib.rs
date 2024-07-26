use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use copy_dir::copy_dir;

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("flite")
}

pub struct Build {
    out_dir: Option<PathBuf>,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs: &'static [&'static str],
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

impl Build {
    pub fn new() -> Self {
        Self {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("flite-build")),
        }
    }

    pub fn build(&mut self) -> Artifacts {
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let build_dir = out_dir.join("build");
        let install_dir = out_dir.join("install");

        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir).unwrap();
        }

        fs::create_dir_all(&build_dir).unwrap();

        let inner_dir = build_dir.join("src");
        copy_dir(source_dir(), &inner_dir).unwrap();

        let mut configure = Command::new("./configure");
        configure
            .arg(format!("--prefix={}", &install_dir.to_str().unwrap()))
            .arg("--with-audio=none")
            .current_dir(&inner_dir);

        if env::var_os("CARGO_CFG_TARGET_OS").is_some_and(|os| os == "wasi") {
            let wasi_sdk_path = PathBuf::from(env::var_os("WASI_SDK_PATH").unwrap());
            configure
                .arg("--host=wasm32-wasi")
                .arg(format!("CC={}", wasi_sdk_path.join("bin/clang").to_str().unwrap()))
                .arg(format!("AR={}", wasi_sdk_path.join("bin/llvm-ar").to_str().unwrap()))
                .arg(format!(
                    "RANLIB={}",
                    wasi_sdk_path.join("bin/llvm-ranlib").to_str().unwrap()
                ));
        }

        self.run_command(configure, "configuring flite");

        let mut install = Command::new("make");
        install.current_dir(&inner_dir);
        self.run_command(install, "building flite");

        let mut install = Command::new("make");
        install.arg("install").current_dir(&inner_dir);
        self.run_command(install, "installing flite");

        for entry in fs::read_dir(inner_dir.join("lang")).unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            if !file_type.is_dir() {
                continue;
            }

            let dir = install_dir.join("include").join(entry.file_name());
            fs::create_dir(&dir).unwrap();

            for entry in fs::read_dir(entry.path()).unwrap() {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();
                if !file_type.is_file() {
                    continue;
                }

                let path = entry.path();
                if !path.extension().is_some_and(|s| s == "h") {
                    continue;
                }

                fs::copy(path, dir.join(entry.file_name())).unwrap();
            }
        }

        fs::remove_dir_all(&inner_dir).unwrap();

        Artifacts {
            include_dir: install_dir.join("include"),
            lib_dir: install_dir.join("lib"),
            libs: &[
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
            ],
        }
    }

    fn run_command(&self, mut command: Command, desc: &str) {
        match command.status() {
            Ok(status) if status.success() => (),
            Ok(status) => panic!("Error {desc}: exited with {status}"),
            Err(e) => panic!("Error {desc}: {e}"),
        }
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn libs(&self) -> &[&str] {
        self.libs
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo::rustc-link-search=native={}", self.lib_dir.display());

        for lib in self.libs {
            println!("cargo::rustc-link-lib=static={}", lib);
        }
    }
}
