from bindgen.ast.objects import *
from .ns import llvm
from .defs import *

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/LLVMContext.h']

    getGlobalContext = fn(ref(LLVMContext))

@LLVMContext.body
class LLVMContext:
    new = Constructor()

    delete = Constructor()
