from bindgen.ast.objects import *
from .ns import llvm
from .defs import *

@DebugLoc.body
class DebugLoc:
    new = Constructor()

    isUnknown = Method(Bool, const=True)

    getLine = Method(UnsignedInt, const=True)
    getCol = Method(UnsignedInt, const=True)

    getScope = Method(ptr(MDNode), (ref(LLVMContext, const=True), 'Ctx'), const=True)
    getInlinedAt = Method(ptr(MDNode), (ref(LLVMContext, const=True), 'Ctx'), const=True)
    getScopeNode = Method(ptr(MDNode), (ref(LLVMContext, const=True), 'Ctx'), const=True)
    getAsMDNode = Method(ptr(MDNode), (ref(LLVMContext, const=True), 'Ctx'), const=True)

    dump = Method(Void, (ref(LLVMContext, const=True), 'Ctx'), const=True)
