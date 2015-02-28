extern crate "llvm-sys" as llvm_sys;

use llvm_sys::ffi::llvm;

macro_rules! cast(
    ($expr:expr) => ($expr as *mut _);
);

macro_rules! objects(
    ($($elem:expr),+) => ([$(cast!($elem)),+]);
    ($($elem:expr),+,) => (objects![$($elem),+]);
);

/*
def fib(n):
    if n == 0 or n == 1:
        return n

    return fib(n - 1) + fib(n - 2)
*/

fn main() {
    let ctx = llvm::getGlobalContext();
    let module = llvm::Module::new("fibonacci", ctx);

    // Make fib function type
    let fib_fn_ty = llvm::FunctionType::get(cast!(llvm::Type::getInt32Ty(ctx)), &objects![llvm::Type::getInt32Ty(ctx)], false);

    // Make function
    let fib_fn = llvm::Function::Create(fib_fn_ty, 0, Some("fib"), Some(module));

    // Build function
    let builder = llvm::IRBuilder::new(ctx);

    let set_insert_point = |block| {
        llvm::IRBuilderBase::SetInsertPoint(cast!(builder), block);
    };

    let entry = llvm::BasicBlock::Create(ctx, Some("entry"), Some(fib_fn), None);
    set_insert_point(entry);

    let zero = llvm::ConstantInt::get(llvm::Type::getInt32Ty(ctx), 0);
    let one = llvm::ConstantInt::get(llvm::Type::getInt32Ty(ctx), 1);
    let two = llvm::ConstantInt::get(llvm::Type::getInt32Ty(ctx), 2);

    let n = llvm::Function::getFirstArgMut(fib_fn);
    llvm::Value::setName(cast!(n), "n");

    let fib_block = llvm::BasicBlock::Create(ctx, Some("fib"), Some(fib_fn), None);
    let ret = llvm::BasicBlock::Create(ctx, Some("ret"), Some(fib_fn), None);

    let switch = llvm::IRBuilder::CreateSwitch(builder, cast!(n), fib_block, Some(2));
    llvm::SwitchInst::addCase(switch, zero, ret);
    llvm::SwitchInst::addCase(switch, one, ret);

    set_insert_point(fib_block);

    let n_minus_one = llvm::IRBuilder::CreateSub(builder, cast!(n), cast!(one), None);
    let n_minus_two = llvm::IRBuilder::CreateSub(builder, cast!(n), cast!(two), None);

    let fib_one = llvm::IRBuilder::CreateCall(builder, cast!(fib_fn), &objects![n_minus_one], None);
    let fib_two = llvm::IRBuilder::CreateCall(builder, cast!(fib_fn), &objects![n_minus_two], None);

    let result = llvm::IRBuilder::CreateAdd(builder, cast!(fib_one), cast!(fib_two), None);

    llvm::IRBuilder::CreateBr(builder, ret);

    set_insert_point(ret);
    let phi = llvm::IRBuilder::CreatePHI(builder, cast!(llvm::Type::getInt32Ty(ctx)), 2, None);
    llvm::PHINode::addIncoming(phi, cast!(n), entry);
    llvm::PHINode::addIncoming(phi, cast!(n), entry);
    llvm::PHINode::addIncoming(phi, cast!(result), fib_block);
    llvm::IRBuilder::CreateRet(builder, cast!(phi));

    llvm::IRBuilder::delete(builder);

    llvm::Module::dump(module);
    llvm::Module::delete(module);
}
