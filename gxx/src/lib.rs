#![allow(unstable)]
use std::default::Default;
use std::process::Command;
use std::process::ExitStatus;
use std::os;
// XXX_ How to set permissions the portable way
use std::os::unix::prelude::PermissionsExt;

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

    let mut objects = Vec::new();
    for source in sources.iter() {
        let mut cmd = Command::new(&gxx);
        cmd.arg(&format!("-O{}", opt_level));
        cmd.args(&["-c", "-ffunction-sections", "-fdata-sections"]);

        for include_dir in config.include_dirs.iter() {
            cmd.arg("-I").arg(include_dir);
        }

        for &(ref key, ref value) in config.definitions.iter() {
            if let &Some(ref value) = value {
                cmd.arg(&format!("-D{}={}", key, value));
            } else {
                cmd.arg(&format!("-D{}", key));
            }
        }

        cmd.args(&config.flags[]);

        let object = out_dir.join(*source).with_extension("o");
        ::std::fs::create_dir_all(&object.dir_path());
        //let mut perms = ::std::fs::metadata(&object.dir_path()).unwrap().permissions();
        //perms.set_mode(0x777);
        //::std::fs::set_permissions(&object.dir_path(), perms).unwrap();
        
        run(cmd.arg(&root_dir.join(*source)).arg("-o").arg(&object));

        objects.push(object);
    }

    run(Command::new(&ar)
        .arg("crus")
        .arg(&out_dir.join(output))
        .args(&objects[]));

    println!("cargo:rustc-flags=-L native={} -l {}:static", out_dir.display(), name);
}

fn run(cmd: &mut Command) -> (String, String, ExitStatus) {
    println!("running: {:?}", cmd);

    let mut process = cmd.spawn().unwrap();

    let status = process.wait().unwrap();
    let output = process.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !status.success() {
        panic!("nonzero exit status: {}\n{}", status, stderr);
    }

    println!("result:\n{}", stdout);

    (stdout, stderr, status)
}
