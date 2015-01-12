#![allow(unstable)]
extern crate llvm;

use llvm::api;
use llvm::api::ty::{Type};
use llvm::api::ty::seq::{ArrayType};
use llvm::api::value::{ValueExt};
use llvm::api::value::user::constant::{Constant, ConstantArray};

fn main() {
    let ctx = api::get_global_context();

    let ty = Type::get_int16_ty(&ctx);
    let value = Constant::get_integer_value(&ty, (16, &[42]));

    let array_ty = ArrayType::get(&ty, 2);
    let array = ConstantArray::get(&array_ty, &[&value, &value]);
    array.dump();
}
