#![allow(unstable)]
extern crate llvm;

use llvm::api;
use llvm::api::{Module, ModuleExt};

fn main() {
    let ctx = api::get_global_context();
    let m = Module::new("toto", ctx);

    m.dump();
}
