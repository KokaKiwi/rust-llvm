from bindgen.ast.objects import *
from .ns import llvm
from .defs import *

@DataLayout.body
class DataLayout:
    _includes_ = ['llvm/IR/DataLayout.h']
