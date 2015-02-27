extern crate llvm;

use llvm::ffi;

macro_rules! cast(
    ($expr:expr) => ($expr as *mut _)
);

macro_rules! objects(
    ($($elem:expr),+) => ([$(cast!($elem)),+])
);

fn main() {
    let ctx = ffi::llvm_getGlobalContext();
    let module = ffi::llvm_Module_new("simple", ctx);

    // Make main function type
    let ret_ty = ffi::llvm_Type_getInt32Ty(ctx);

    let argc_ty = ffi::llvm_Type_getInt32Ty(ctx);
    let argv_ty = ffi::llvm_Type_getInt8PtrTy(ctx);

    let main_fn_args = objects![argc_ty, argv_ty];
    let main_fn_ty = ffi::llvm_FunctionType_get(ret_ty as *mut _, &main_fn_args, false);

    let main_fn = ffi::llvm_Module_getOrInsertFunction(module, "main", main_fn_ty);

    let entry_block = ffi::llvm_BasicBlock_Create(ctx, Some("entry"), Some(main_fn as *mut _), None);

    let builder = ffi::llvm_IRBuilder_new_in_block(entry_block);

    let ret = ffi::llvm_ConstantInt_get(ret_ty, 0);
    ffi::llvm_IRBuilder_CreateRet(builder, ret as *mut _);

    ffi::llvm_IRBuilder_delete(builder);

    ffi::llvm_Module_dump(module);
    ffi::llvm_Module_delete(module);
}
