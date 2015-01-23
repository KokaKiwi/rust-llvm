#![allow(unstable)]
extern crate llvm;

use llvm::api;
use llvm::api::{Module, IRBuilder};
use llvm::api::ty::{Type, FunctionType};
use llvm::api::value::{BasicBlock};
use llvm::traits::*;

fn create_main_fntype<Ctx: LLVMContextObj>(ctx: &mut Ctx) -> FunctionType {
    let mut ret_ty = Type::get_int32_ty(ctx).unwrap();

    let arg_tys = [
        &Type::get_int32_ty(ctx).unwrap() as &TypeObj,
        &Type::get_int8_ptr_ty(ctx).unwrap() as &TypeObj,
    ];

    FunctionType::get(&mut ret_ty, &arg_tys, false).unwrap()
}

fn main() {
    let mut ctx = api::get_global_context();
    let mut m = Module::new("simple", &mut ctx);

    let mut main_fn_ty = create_main_fntype(&mut ctx);
    m.get_or_insert_function("main", &mut main_fn_ty);
    let mut function = m.get_function("main").unwrap();

    let mut entry = BasicBlock::create(&mut ctx, Some("entry"), Some(&mut function), None::<&mut BasicBlock>);

    let mut builder = IRBuilder::new(&mut ctx);
    builder.set_insert_point(&mut entry);

    builder.create_ret_void();

    function.dump();
}
