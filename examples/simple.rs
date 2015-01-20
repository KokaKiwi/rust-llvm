#![allow(unstable)]
extern crate llvm;

use llvm::api;
use llvm::api::{Module};
use llvm::api::ty::{Type, FunctionType};
use llvm::traits::*;

fn create_main_fntype(ctx: &LLVMContextExt) -> FunctionType {
    let ret_ty = Type::get_int32_ty(ctx).unwrap();

    let arg_tys = [
        &Type::get_int32_ty(ctx).unwrap() as &TypeExt,
        &Type::get_int8_ptr_ty(ctx).unwrap() as &TypeExt,
    ];

    FunctionType::get(&ret_ty, &arg_tys, false).unwrap()
}

fn main() {
    let ctx = api::get_global_context();
    let mut m = Module::new("simple", &ctx);

    let main_fn_ty = create_main_fntype(&ctx);
    let function = m.get_or_insert_function("main", &main_fn_ty);

    function.dump();
}
