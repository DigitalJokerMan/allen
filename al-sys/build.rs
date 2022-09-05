use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

const OPENAL_REPO: &str = "https://github.com/kcat/openal-soft.git";
const OPENAL_TAG: &str = "1.22.2";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // clone openal-soft (if needed)

    let oal_soft_dir = &out_dir.join("openal-soft");

    let clone_required = if oal_soft_dir.exists() {
        let describe_output = Command::new("git")
            .args(&["describe", "--tags"])
            .current_dir(&oal_soft_dir)
            .stdout(Stdio::piped())
            .spawn()
            .expect("git might not be installed")
            .wait_with_output()
            .expect("failed to describe status of repository");

        !describe_output.status.success()
            || !std::str::from_utf8(&describe_output.stdout)
                .unwrap()
                .starts_with(OPENAL_TAG)
    } else {
        true
    };

    if clone_required {
        let status = Command::new("git")
            .arg("clone")
            .args(&["--branch", OPENAL_TAG])
            .args(&["--depth", "1"])
            .arg(OPENAL_REPO)
            .arg(&oal_soft_dir)
            .status()
            .expect("unable to clone openal-soft");

        assert!(status.success(), "failed to clone openal-soft");
    }

    // build & link openal-soft
    /*let build_dir = cmake::Config::new(oal_soft_dir)
        .define("ALSOFT_UTILS", "OFF")
        .define("ALSOFT_EXAMPLES", "OFF")
        .define("ALSOFT_TESTS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        build_dir.display()
    );

    // for windows (is there a better way to do this?)
    println!(
        "cargo:rustc-link-search=native={}/build/Debug",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/Release",
        build_dir.display()
    );*/

    // Temporary
    println!("cargo:rustc-link-search=native=C:\\Program Files (x86)\\OpenAL 1.1 SDK\\libs\\Win64");

    //println!("cargo:rustc-link-lib=dylib=common");
    println!("cargo:rustc-link-lib=dylib=OpenAL32");

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
