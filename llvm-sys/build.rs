extern crate gcc;
extern crate llvm_config;

use std::env;

fn main() {
    let static_link: bool = env::var("LLVM_STATIC_LINK").ok()
                            .and_then(|value| value.parse().ok())
                            .unwrap_or(false);

    // Build library
    let mut config = gcc::Config::new();
    config
        .cpp(true)
        .file("src/ffi.cpp");

    for flag in llvm_config::cxxflags().into_iter() {
        config.flag(&flag);
    }

    config
        .compile("libllvm_ffi.a");

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
