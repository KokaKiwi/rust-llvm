#![allow(unstable)]
use std::default::Default;
use std::io::Command;
use std::io::process::ProcessExit;
use std::os;

pub struct Config {
    pub include_dirs: Vec<Path>,
    pub definitions: Vec<(String, Option<String>)>,
    pub flags: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            include_dirs: Vec::new(),
            definitions: Vec::new(),
            flags: Vec::new(),
        }
    }
}

pub fn compile_library(name: &str, config: &Config, sources: &[&str]) {
    let output = format!("lib{}.a", name);

    let opt_level = os::getenv("OPT_LEVEL").unwrap();
    let gxx = os::getenv("CXX").unwrap_or("g++".to_string());
    let ar = os::getenv("AR").unwrap_or("ar".to_string());
    let root_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut cmd = Command::new(gxx);
    cmd.arg(format!("-O{}", opt_level));
    cmd.args(&["-c", "-ffunction-sections", "-fdata-sections"]);

    for include_dir in config.include_dirs.iter() {
        cmd.arg("-I").arg(include_dir);
    }

    for &(ref key, ref value) in config.definitions.iter() {
        if let &Some(ref value) = value {
            cmd.arg(format!("-D{}={}", key, value));
        } else {
            cmd.arg(format!("-D{}", key));
        }
    }

    cmd.args(&config.flags[]);

    let mut objects = Vec::new();
    for source in sources.iter() {
        let object = out_dir.join(*source).with_extension("o");
        ::std::io::fs::mkdir_recursive(&object.dir_path(), ::std::io::USER_RWX).unwrap();
        run(cmd.clone().arg(root_dir.join(*source)).arg("-o").arg(&object));

        objects.push(object);
    }

    run(Command::new(ar)
        .arg("crus")
        .arg(out_dir.join(output))
        .args(&objects[]));

    println!("cargo:rustc-flags=-L native={} -l {}:static", out_dir.display(), name);
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
