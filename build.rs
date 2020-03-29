extern crate bindgen;

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn cef_dir() -> PathBuf {
    PathBuf::from(env_var("CEF_DIR"))
}

// The wrapper file is the entry point to the native code we are creating bindings for.
fn create_wrapper_file() {
    let wrapper_file = PathBuf::from(env_var("CARGO_MANIFEST_DIR")).join("wrapper.h");

    let cef_dir = cef_dir();

    if !wrapper_file.is_file() {
        let file = fs::File::create(wrapper_file).expect("Could not create wrapper.h file");
        let mut file_writer = std::io::LineWriter::new(file);

        // We want to include all capi headers
        let include_files = fs::read_dir(cef_dir.join("include").join("capi")).unwrap();

        for entry_res in include_files {
            let entry = entry_res.unwrap();
            // If it's a header, include it in the file as a string relative to cef_dir
            if entry.file_name().to_str().unwrap().ends_with(".h") {
                let relative_name = entry
                    .path()
                    .strip_prefix(&cef_dir)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("\\", "/");

                writeln!(&mut file_writer, "#include \"{}\"", relative_name)
                    .expect("Could not write #include to wrapper.h");
            }
        }
    }
}

fn generate_bindings() {
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env_var("OUT_DIR"));

    // if !out_path.is_file() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("--include-directory")
        .clang_arg(cef_dir().to_str().unwrap())
        // Some of the c api includes seem to pull in C++!
        .clang_arg("-x")
        .clang_arg("c++")
        .layout_tests(false)
        .derive_default(true)
        // TODO: waiting for fix of https://github.com/servo/rust-bindgen/issues/648
        .opaque_type("tagMONITORINFOEXA")
        .rustfmt_bindings(true)
        // Cef is huge! Pull in only the things we need or else the generated bindings is very large.
        .whitelist_function("cef_execute_process")
        .whitelist_function("cef_initialize")
        .whitelist_function("cef_do_message_loop_work")
        .whitelist_function("cef_browser_host_create_browser")
        .whitelist_function("cef_browser_host_create_browser_sync")
        .whitelist_function("cef_process_message_create")
        .whitelist_function("cef_string_utf8_to_utf16")
        .whitelist_function("cef_string_utf16_to_utf8")
        .whitelist_function("cef_v8value_create_undefined")
        .whitelist_function("cef_v8value_create_null")
        .whitelist_function("cef_v8value_create_bool")
        .whitelist_function("cef_v8value_create_int")
        .whitelist_function("cef_v8value_create_uint")
        .whitelist_function("cef_v8value_create_double")
        .whitelist_function("cef_v8value_create_date")
        .whitelist_function("cef_v8value_create_string")
        .whitelist_function("cef_v8value_create_object")
        .whitelist_function("cef_v8value_create_array")
        .whitelist_function("cef_v8value_create_array_buffer")
        .whitelist_function("cef_v8value_create_function")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // }
}

enum Platform {
    Windows,
    Mac,
    Linux,
}

fn get_platform() -> Platform {
    match env::var("TARGET").unwrap().split('-').nth(2).unwrap() {
        "win32" | "windows" => Platform::Windows,
        "darwin" => Platform::Mac,
        "linux" => Platform::Linux,
        other => panic!("Sorry, platform \"{}\" is not supported by CEF.", other),
    }
}

fn get_build_type() -> String {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => String::from("Release"),
        _ => String::from("Debug"),
    }
}

fn config_linker() {
    let lib_name = match get_platform() {
        Platform::Mac => return, // CEF_PATH is not necessarily needed for Mac
        Platform::Windows => "libcef",
        Platform::Linux => "cef",
    };

    // Tell the linker the lib name and the path
    println!("cargo:rustc-link-lib={}", lib_name);
    println!(
        "cargo:rustc-link-search={}",
        cef_dir().join(get_build_type()).to_str().unwrap()
    );
}

fn main() {
    create_wrapper_file();
    generate_bindings();
    config_linker();
}

fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}
