#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate fs_extra;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use fs_extra::dir::{get_dir_content2, CopyOptions, DirOptions};

fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}

fn cef_dir() -> PathBuf {
    PathBuf::from(env_var("CEF_DIR"))
}

fn cef_binary_dir_name() -> String {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => String::from("Release"),
        _ => String::from("Debug"),
    }
}

fn cef_resources_dir_name() -> String {
    String::from("Resources")
}

fn copy_file_to_target<P: AsRef<Path>>(file_name: String, src: P, target_dir: P) {
    copy_file_if_not_there(
        src.as_ref().join(&file_name),
        target_dir.as_ref().join(&file_name),
    )
}

fn copy_file_if_not_there<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dest: P2) {
    if dest.as_ref().is_file() {
        println!("Not copying {:?} because it already exists", dest.as_ref());
    } else {
        fs::copy(src.as_ref(), dest.as_ref()).expect(
            format!(
                "Failed to copy from {:?} to {:?}",
                src.as_ref(),
                dest.as_ref()
            )
            .as_ref(),
        );
    }
}

// CEF is bundled as a dynamic library with numerous other files that need to be present
// alongside the final executable.
pub fn copy_cef_binaries_to_target() {
    let cef_binary_dir_name = cef_binary_dir_name();
    let cef_binary_dir = cef_dir().join(cef_binary_dir_name);
    let cef_resources_dir_name = cef_resources_dir_name();
    let cef_resources_dir = cef_dir().join(&cef_resources_dir_name);
    // let target_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap());

    let options = CopyOptions::new();

    println!("Copying CEF binaries to target dir");
    copy_file_to_target(String::from("libcef.dll"), &cef_binary_dir, &target_dir);
    copy_file_to_target(String::from("chrome_elf.dll"), &cef_binary_dir, &target_dir);
    copy_file_to_target(
        String::from("d3dcompiler_47.dll"),
        &cef_binary_dir,
        &target_dir,
    );
    copy_file_to_target(String::from("libEGL.dll"), &cef_binary_dir, &target_dir);
    copy_file_to_target(String::from("libGLESv2.dll"), &cef_binary_dir, &target_dir);
    copy_file_to_target(
        String::from("natives_blob.bin"),
        &cef_binary_dir,
        &target_dir,
    );
    copy_file_to_target(
        String::from("snapshot_blob.bin"),
        &cef_binary_dir,
        &target_dir,
    );
    copy_file_to_target(
        String::from("v8_context_snapshot.bin"),
        &cef_binary_dir,
        &target_dir,
    );

    println!("Copying CEF resources to target dir");
    let dir_options = DirOptions {
        depth: 1, // List only first level contents
    };
    let cef_resources_dir_items = get_dir_content2(cef_resources_dir, &dir_options)
        .expect("Could not get cef Resources dir items");
    match fs_extra::copy_items(&cef_resources_dir_items.files, &target_dir, &options) {
        Ok(_) => println!("Succeeded"),
        Err(e) => println!("Not copying: {}", e),
    }

    let cef_resources_locales_dir = cef_dir().join(cef_resources_dir_name).join("locales");
    match fs_extra::dir::copy(&cef_resources_locales_dir, &target_dir, &options) {
        Ok(_) => println!("Succeeded"),
        Err(e) => println!("Not copying: {}", e),
    }
}
