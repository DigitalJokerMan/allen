use std::{
    env,
    path::PathBuf,
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // clone openal-soft (if needed)
    let oal_soft_dir = PathBuf::from("openal-soft");

    let profile = if let Ok(opt_level) = env::var("OPT_LEVEL") {
        if opt_level == "z" || opt_level == "s" {
            "MinSizeRel"
        } else {
            "Release"
        }
    } else {
        "Release"
    };

    let build_dir = cmake::Config::new(&oal_soft_dir)
        .profile(profile)
        .define("ALSOFT_UTILS", "OFF")
        .define("ALSOFT_EXAMPLES", "OFF")
        .define("ALSOFT_TESTS", "OFF")
        .define("ALSOFT_INSTALL", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search={}",
        build_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search={}",
        build_dir.join("build").join("Release").display()
    );

    match &target_os[..] {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=OpenAL32");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=openal");
        }
    }

    // generate bindings
    bindgen::Builder::default()
        .header_contents(
            "main.h",
            &format!(
                "#include \"{}\"\n#include \"{}\"\n",
                &oal_soft_dir.join("include/AL/al.h").display(),
                &oal_soft_dir.join("include/AL/alc.h").display()
            ),
        )
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(&out_dir.join("bindings.rs"))
        .expect("failed to write bindings");
}
