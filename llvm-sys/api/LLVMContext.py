from .prelude import *


@llvm.body
class llvm_body:
    getGlobalContext = ast.Function(ref(LLVMContext))


@LLVMContext.body
class LLVMContext:
    new = Constructor()

    delete = Constructor()
