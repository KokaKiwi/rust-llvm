#![unstable]
#![feature(io, path, env)]
use std::env;
use std::old_io::Command;
use std::old_io::process::ProcessExit;

pub fn llvm_config(args: &[&str]) -> String {
    let llvm_config = env::var("LLVM_CONFIG").unwrap_or("llvm-config".to_string());

    let mut cmd = Command::new(llvm_config);
    cmd.args(args);

    let (output, _, _) = run(&cmd);
    output
}

pub fn libs() -> Vec<String> {
    let flags_str = llvm_config(&["--libs", "--system-libs"]);
    let flags = split(&flags_str[]);

    let mut libs = Vec::new();
    for flag in flags.into_iter() {
        if !flag.starts_with("-l") {
            continue;
        }

        let lib = &flag[2..];
        libs.push(lib.to_string());
    }

    libs
}

pub fn link_dirs() -> Vec<Path> {
    let flags_str = llvm_config(&["--ldflags"]);
    let flags = split(&flags_str[]);

    let mut link_dirs = Vec::new();
    for flag in flags.into_iter() {
        if !flag.starts_with("-L") {
            continue;
        }

        let dirname = &flag[2..];
        link_dirs.push(Path::new(dirname));
    }

    link_dirs
}

pub fn cxxflags() -> Vec<String> {
    let flags_str = llvm_config(&["--cxxflags"]);
    let flags = split(&flags_str[]);

    flags.into_iter().map(|flag| flag.to_string()).collect()
}

fn split(s: &str) -> Vec<&str> {
    let is_sep = |&: c: char| [' ', '\n'].contains(&c);
    s.split(is_sep).collect()
}

fn run(cmd: &Command) -> (String, String, ProcessExit) {
    let mut process = cmd.spawn().unwrap();

    let status = process.wait().unwrap();
    let output = process.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.output).unwrap();
    let stderr = String::from_utf8(output.error).unwrap();

    if !status.success() {
        panic!("nonzero exit status: {}", status);
    }

    (stdout, stderr, status)
}
