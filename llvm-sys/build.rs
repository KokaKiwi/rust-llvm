#![allow(unstable)]
extern crate "llvm-config" as llvm_config;
extern crate gxx;

use std::default::Default;
use std::os;

fn main() {
    let static_link: bool = os::getenv("STATIC_LINK").unwrap_or("false".to_string()).parse().unwrap();

    // Build library
    let mut config: gxx::Config = Default::default();
    for flag in llvm_config::cxxflags().into_iter() {
        config.flags.push(flag);
    }

    gxx::compile_library("llvm_ffi", &config, &["src/ffi.cpp"]);

    // Link flags
    let mut link_flags = Vec::new();

    for link_dir in llvm_config::link_dirs().into_iter() {
        link_flags.push(format!("-L native={}", link_dir.display()));
    }

    for lib in llvm_config::libs().into_iter() {
        let kind = if lib.contains("LLVM") && static_link {
            ":static"
        } else {
            ""
        };

        link_flags.push(format!("-l {}{}", lib, kind));
    }

    let flags_str = llvm_config::llvm_config(&["--cxxflags"]);
    if static_link {
        assert!(!flags_str.contains("stdlib=libc++"));
        link_flags.push("-l stdc++:static".to_string());
    } else {
        if flags_str.contains("stdlib=libc++") {
            link_flags.push("-l c++".to_string());
        } else {
            link_flags.push("-l stdc++".to_string());
        }
    }

    let link_flags = link_flags.connect(" ");

    println!("cargo:rustc-flags={}", link_flags);
}
