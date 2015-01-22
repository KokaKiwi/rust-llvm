from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath, copymodpath
from .ns import llvm
from .defs import *
from .ADT.StringRef import StringRef

@llvm.body
class llvm_body:
    _includes_ = ['llvm/PassManager.h']

@FunctionPassManager.body
class FunctionPassManager:
    new = Constructor((ptr(Module), 'Module'))

    add = Method(Void, (ptr(FunctionPass, owned=True), 'Pass'))
    run = Method(Void, (ref(Function), 'Function'))

    doInitialization = Method(Bool)
    doFinalization = Method(Bool)

@PassManager.body
class PassManager:
    new = Constructor()

    add = Method(Void, (ptr(Pass, owned=True), 'Pass'))
    run = Method(Void, (ref(Module), 'Module'))
