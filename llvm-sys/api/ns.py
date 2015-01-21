from bindgen.ast.objects import *

root = Namespace()
llvm = root.Namespace('llvm')

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/Verifier.h']
