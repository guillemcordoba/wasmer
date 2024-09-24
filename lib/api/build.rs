const WAMR_ZIP: &str = "https://github.com/bytecodealliance/wasm-micro-runtime/archive/refs/tags/WAMR-2.1.0.zip";
const WAMR_DIR: &str = "wasm-micro-runtime-WAMR-2.1.0";

fn main() {
    #[cfg(feature = "wamr")]
    {
        use cmake::Config;
        use std::{env, path::PathBuf};

        let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wamr_dir = PathBuf::from(&crate_root).join("third_party").join("wamr");

        let zip = ureq::get(WAMR_ZIP).call().expect("failed to download wamr");

        let mut zip_data = Vec::new();
        zip.into_reader().read_to_end(&mut zip_data).expect("failed to download wamr");

        zip::read::ZipArchive::new(std::io::Cursor::new(zip_data))
            .expect("failed to open wamr zip file")
            .extract(&crate_root)
            .expect("failed to extract wamr zip file");

        let _ = std::fs::remove_dir_all(&wamr_dir);

        let zip_dir = PathBuf::from(&crate_root).join(WAMR_DIR);

        std::fs::rename(zip_dir, &wamr_dir).expect("failed to rename wamr dir");

        /*
        let mut fetch_submodules = std::process::Command::new("git");
        fetch_submodules
            .current_dir(crate_root)
            .arg("submodule")
            .arg("update")
            .arg("--init");

        let res = fetch_submodules.output();

        if let Err(e) = res {
            panic!("fetching submodules failed: {e}");
        }
        */

        /* Taken from https://github.com/bytecodealliance/wasm-micro-runtime/tree/21330990a8f5963dd09d81e491ca4a34f7196ab1/product-mini#android */

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let dst= if target_os.as_str() == "android" {
            let target = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            let wamr_build_target = match target.as_str() {
                "aarch64" => "AARCH64",
                "armv7" => "ARMV7A",
                "i686" => "X86",
                "x86_64" => "X86_64",
                _ => ""
            };
            let wamr_android_abi = match target.as_str() {
                "aarch64" => "arm64-v8a",
                "armv7" => "armeabi-v7a",
                "i686" => "x86",
                "x86_64" => "x86_64",
                _ => ""
            };

            std::fs::create_dir(wamr_dir.clone().join("product-mini/platforms/android/build")).expect("Could not create build directory");
            Config::new(wamr_dir.clone().join("product-mini/platforms/android"))
            .out_dir(wamr_dir.clone().join("product-mini/platforms/android"))
            .build_target("all")
            .always_configure(true)
            .generator("Unix Makefiles")
            .define(
                "CMAKE_BUILD_TYPE",
                if cfg!(debug_assertions) {
                    "RelWithDebInfo"
                } else {
                    "Release"
                },
            )
            .define("WAMR_BUILD_AOT", "0")
            .define("WAMR_BUILD_TARGET", wamr_build_target) 
            .define("ANDROID_ABI", wamr_android_abi)
            //.define("WAMR_BUILD_TAIL_CALL", "1")
            //.define("WAMR_BUILD_DUMP_CALL_STACK", "1")
            // .define("WAMR_BUILD_CUSTOM_NAME_SECTION", "1")
            // .define("WAMR_BUILD_LOAD_CUSTOM_SECTION", "1")
            .define("WAMR_BUILD_BULK_MEMORY", "1")
            .define("WAMR_BUILD_REF_TYPES", "1")
            .define("WAMR_BUILD_SIMD", "1")
            .define("WAMR_ENABLE_FAST_INTERP", "1")
            .define("WAMR_BUILD_LIB_PTHREAD", "1")
            .define("WAMR_BUILD_LIB_WASI_THREADS", "0")
            .define("WAMR_BUILD_LIBC_WASI", "0")
            .define("WAMR_BUILD_LIBC_BUILTIN", "0")
            .define("WAMR_BUILD_SHARED_MEMORY", "1")
            .define("WAMR_BUILD_MULTI_MODULE", "0")
            .define("WAMR_DISABLE_HW_BOUND_CHECK", "1")
            .build()
        } else {
            Config::new(wamr_dir.clone())
            .always_configure(true)
            .generator("Unix Makefiles")
            .define(
                "CMAKE_BUILD_TYPE",
                if cfg!(debug_assertions) {
                    "RelWithDebInfo"
                } else {
                    "Release"
                },
            )
            .define("WAMR_BUILD_AOT", "0")
            //.define("WAMR_BUILD_TAIL_CALL", "1")
            //.define("WAMR_BUILD_DUMP_CALL_STACK", "1")
            // .define("WAMR_BUILD_CUSTOM_NAME_SECTION", "1")
            // .define("WAMR_BUILD_LOAD_CUSTOM_SECTION", "1")
            .define("WAMR_BUILD_BULK_MEMORY", "1")
            .define("WAMR_BUILD_REF_TYPES", "1")
            .define("WAMR_BUILD_SIMD", "1")
            .define("WAMR_ENABLE_FAST_INTERP", "1")
            .define("WAMR_BUILD_LIB_PTHREAD", "1")
            .define("WAMR_BUILD_LIB_WASI_THREADS", "0")
            .define("WAMR_BUILD_LIBC_WASI", "0")
            .define("WAMR_BUILD_LIBC_BUILTIN", "0")
            .define("WAMR_BUILD_SHARED_MEMORY", "1")
            .define("WAMR_BUILD_MULTI_MODULE", "0")
            .define("WAMR_DISABLE_HW_BOUND_CHECK", "1")
            .build()
        };


        // Check output of `cargo build --verbose`, should see something like:
        // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
        // That contains output from cmake
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build").display()
        );
        println!("cargo:rustc-link-lib=vmlib");


        let mut bindings = bindgen::Builder::default()
            .header(
                wamr_dir
                    .join("core/iwasm/include/wasm_c_api.h")
                    .to_str()
                    .unwrap(),
            )
            .header(
                wamr_dir
                    .join("core/iwasm/include/wasm_c_api.h")
                    .to_str()
                    .unwrap(),
            )
            .header(
                wamr_dir
                    .join("core/iwasm/include/wasm_export.h")
                    .to_str()
                    .unwrap(),
            )
            .derive_default(true)
            .derive_debug(true);
        
        if target_os.as_str() == "android" {
            let target = std::env::var("TARGET").expect("Can't find the TARGET triple variable");
            let sysroot = std::env::var("BINDGEN_EXTRA_CLANG_ARGS").expect("Need to set the BINDGEN_EXTRA_CLANG_ARGS environment variable to the sysroot directory ");
            bindings = bindings             
                .clang_args([format!("-I{sysroot}/usr/include"),format!("-I{sysroot}/usr/include/{target}")]);
        }
                

        let bindings = bindings
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings");
    }
}
