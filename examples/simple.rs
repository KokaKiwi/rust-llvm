extern crate "llvm-sys" as llvm_sys;

use llvm_sys::ffi::llvm;

macro_rules! cast(
    ($expr:expr) => ($expr as *mut _)
);

macro_rules! objects(
    ($($elem:expr),+) => ([$(cast!($elem)),+])
);

fn main() {
    let ctx = llvm::getGlobalContext();
    let module = llvm::Module::new("simple", ctx);

    // Make main function type
    let ret_ty = llvm::Type::getInt32Ty(ctx);

    let argc_ty = llvm::Type::getInt32Ty(ctx);
    let argv_ty = llvm::Type::getInt8PtrTy(ctx);

    let main_fn_args = objects![argc_ty, argv_ty];
    let main_fn_ty = llvm::FunctionType::get(ret_ty as *mut _, &main_fn_args, false);

    let main_fn = llvm::Module::getOrInsertFunction(module, "main", main_fn_ty);

    let entry_block = llvm::BasicBlock::Create(ctx, Some("entry"), Some(main_fn as *mut _), None);

    let builder = llvm::IRBuilder::new_in_block(entry_block);

    let ret = llvm::ConstantInt::get(ret_ty, 0);
    llvm::IRBuilder::CreateRet(builder, ret as *mut _);

    llvm::IRBuilder::delete(builder);

    llvm::Module::dump(module);
    llvm::Module::delete(module);
}
