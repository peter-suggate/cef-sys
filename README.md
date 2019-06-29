# cef-sys
Low-level Rust bindings for CEF.

This library was created because as of mid-2019 all existing CEF rust wrappers were at least 4 years old and severely out of date.

Contact the repo owner if you'd like to contribute to keeping this up to date!

Currently supports Windows only.

## Installation

### Get CEF
CEF is required which contains the required header files and binaries for all CEF applications.

Download latest version for your platform form:
http://opensource.spotify.com/cefbuilds/index.html

### LLVM
LLVM's Clang is required to generate rust bindings to CEF.

You can either:
```bash
choco install llvm
```

Or download latest version for your platform from:
http://releases.llvm.org/download.html

## Usage

### Copying CEF binaries to target dir
Applications that make use of CEF need to load associated dynamic libraries and resources at runtime. `cef-sys` exposes a convenenience function for this reason. Example:

In your package's Cargo.toml, add:
```rust
[build-dependencies]
cef-sys = { /* */ }
```

In your package's build.rs, add:
```rust
extern crate cef_sys;

fn main() {
  // ... other build stuff.

  cef_sys::copy_cef_binaries_to_target();
}
```
