use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    /* === Building for embedded === */
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    /* === Linking to microbit libs === */
    let includes_dir = PathBuf::from("./includes");
    let prebuilt_dir = PathBuf::from("./prebuilts");

    if !includes_dir.exists() {
        create_dir_all(&includes_dir).unwrap();
    }
    let includes_dir = includes_dir.canonicalize().unwrap();
    if !prebuilt_dir.exists() {
        create_dir_all(&prebuilt_dir).unwrap();
    }
    let prebuilt_dir = prebuilt_dir.canonicalize().unwrap();

    #[cfg(not(feature = "prebuilt"))]
    {
        let microbit_path = PathBuf::from("./external/microbit").canonicalize().unwrap();
        let header = microbit_path.join("inc").join("MicroBit.h");
        let yotta_modules_path = microbit_path.join("yotta_modules");
        let microbit_dal_headers = yotta_modules_path.join("microbit-dal").join("inc");
        let static_lib = microbit_path
            .join("build")
            .join("bbc-microbit-classic-gcc")
            .join("source")
            .join("microbit.a");
        println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
        println!("cargo:rerun-if-changed={}", static_lib.to_str().unwrap());

        let headers = vec![
            header,
            microbit_dal_headers.join("types").join("ManagedString.h"),
        ];

        std::process::Command::new("yotta")
            .args(["build"])
            .current_dir(&microbit_path)
            .spawn()
            .unwrap();

        for header in headers.iter() {
            std::fs::copy(header, includes_dir.join(header.file_name().unwrap())).unwrap();
        }
        std::fs::copy(static_lib, prebuilt_dir.join("libmicrobit.a")).unwrap();
    }
    #[cfg(feature = "prebuilt")]
    {
        if !includes_dir.join("MicroBit.h").exists() || !prebuilt_dir.join("libmicrobit.a").exists()
        {
            panic!("Please build without the feature `prebuilt` to compile MicroBit from source (prebuilts not found)");
        }
        println!(
            "cargo:rerun-if-changed={}",
            includes_dir.join("MicroBit.h").to_str().unwrap()
        );
        println!(
            "cargo:rerun-if-changed={}",
            prebuilt_dir.join("libmicrobit.a").to_str().unwrap()
        );
    }

    println!(
        "cargo:rustc-link-search=native={}",
        prebuilt_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static:+whole-archive,-bundle=microbit");
}
