from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath, copymodpath
from .ns import llvm
from .defs import *

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/PassManager.h']
