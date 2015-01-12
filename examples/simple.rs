#![allow(unstable)]
extern crate llvm;

use llvm::api;
use llvm::api::ty::{Type};
use llvm::api::value::{ValueExt};
use llvm::api::value::user::constant::{Constant};

fn main() {
    let ctx = api::get_global_context();

    let ty = Type::get_int16_ty(ctx);
    let value = Constant::get_integer_value(ty, (2048, &[0, 0, 0, 0, 1]));
    value.dump();
}
