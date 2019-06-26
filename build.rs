extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::Write;

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
        let relative_name = entry.path().strip_prefix(&cef_dir).
          unwrap().to_str().unwrap().replace("\\", "/");

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
      .clang_arg("--include-directory").clang_arg(cef_dir().to_str().unwrap())
      // Some of the c api includes seem to pull in C++!
      .clang_arg("-x").clang_arg("c++")
      .layout_tests(false)
      .derive_default(true)
      // TODO: waiting for fix of https://github.com/servo/rust-bindgen/issues/648
      .opaque_type("tagMONITORINFOEXA")
      .rustfmt_bindings(true)
      // Cef is huge! Pull in only the things we need or else the generated bindings is very large.
      .whitelist_function("cef_execute_process")
      .whitelist_function("cef_initialize")
      .whitelist_function("cef_string_utf16_set")
      .whitelist_function("cef_browser_host_create_browser")
      // Finish the builder and generate the bindings.
      .generate()
      // Unwrap the Result and panic on failure.
      .expect("Unable to generate bindings");

    bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
  // }
}

fn config_linker() {
  // Tell the linker the lib name and the path
  println!("cargo:rustc-link-lib=libcef");
  println!(
    "cargo:rustc-link-search={}",
    cef_dir().join("Release").to_str().unwrap()
  );
  // cef_dir.join(if env::var("PROFILE") == "release" { "Release" } else { "Debug" }).to_str().unwrap());
}

fn main() {
  create_wrapper_file();
  generate_bindings();
  config_linker();
}
// extern crate bindgen;

// use std::env;
// use std::fs;
// use std::path::PathBuf;
// use std::io::Write;

// fn main() {
//     builder().build();
// }

// struct Builder {
//     cef_dir: PathBuf
// }

// fn builder() -> Builder {
//     Builder { cef_dir: PathBuf::from(env_var("CEF_DIR")) }
// }

// impl Builder {
//     fn build(&self) {
//         self.write_wrapper_file();
//         self.generate_bindings();
//         self.cargo_config();
//     }

//     fn write_wrapper_file(&self) {
//         // Let's create a wrapper.h file if it's not there
//         let wrapper_file = PathBuf::from(env_var("CARGO_MANIFEST_DIR")).join("wrapper.h");
//         if !wrapper_file.is_file() {
//             // We want to include all capi headers
//             let include_files = fs::read_dir(self.cef_dir.join("include").join("capi")).unwrap();
//             let mut file = fs::File::create(wrapper_file).unwrap();
//             for entry_res in include_files {
//                 let entry = entry_res.unwrap();
//                 // If it's a header, include it in the file as a string relative to cef_dir
//                 if entry.file_name().to_str().unwrap().ends_with(".h") {
//                     let relative_name = entry.path().strip_prefix(&self.cef_dir).
//                         unwrap().to_str().unwrap().replace("\\", "/");
//                     writeln!(file, "#include \"{}\"", relative_name).unwrap();
//                 }
//             }
//         } else {
//             println!("Not writing wrapper.h because it already exists");
//         }
//     }

//     fn generate_bindings(&self) {
//         let out_file = PathBuf::from(env_var("OUT_DIR")).join("bindings.rs");
//         if !out_file.is_file() {
//             let bindings = bindgen::builder()
//                 .header("wrapper.h")
//                 .clang_arg("--include-directory")
//                 .clang_arg(self.cef_dir.to_str().unwrap())
//                 .layout_tests(false)
//                 .derive_default(true)
//                 // TODO: waiting for fix of https://github.com/servo/rust-bindgen/issues/648
//                 .opaque_type("tagMONITORINFOEXA")
//                 .generate()
//                 .expect("Unable to generate bindings");
//             bindings.write_to_file(out_file).map_err(|e| format!("Unable to write bindings: {}", e)).unwrap();
//         } else {
//             println!("Not generating bindings.rs because it already exists");
//         }
//     }

//     fn cargo_config(&self) {
//         // Tell the linker the lib name and the path
//         // TODO: make this just "cef" on non-win
//         println!("cargo:rustc-link-lib=libcef");
//         println!("cargo:rustc-link-search={}", self.cef_dir.
//             join(if env_var("PROFILE") == "release" { "Release" } else { "Debug" }).to_str().unwrap());
//     }
// }

fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}
