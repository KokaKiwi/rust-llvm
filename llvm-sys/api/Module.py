from bindgen.ast.objects import *
from .ns import llvm
from .defs import *
from .ADT.StringRef import StringRef

@llvm.body
class llvm_body:
    verifyModule = fn(Bool, (ref(Module, const=True), 'Module'))

@Module.body
class Module:
    _includes_ = ['llvm/IR/Module.h']

    new = Constructor((StringRef, 'ModuleID'), (ref(LLVMContext), 'Context'))
    delete = Destructor()

    dump = Method(const=True)

    getModuleIdentifier = Method(String(const=True), const=True)
    setModuleIdentifier = Method(Void, (StringRef, 'ID'))

    getDataLayout = Method(ptr(DataLayout, const=True), const=True)
    setDataLayout = Method(Void, (ptr(DataLayout, const=True), 'Other'))
    getDataLayoutStr = Method(String(const=True), const=True)
    setDataLayoutStr = Method(Void, (StringRef, 'Desc'))
    setDataLayoutStr.call_name = 'setDataLayout'

    getTargetTriple = Method(String(const=True), const=True)
    setTargetTriple = Method(Void, (StringRef, 'Triple'))

    getModuleInlineAsm = Method(String(const=True), const=True)
    setModuleInlineAsm = Method(Void, (StringRef, 'Asm'))
    appendModuleInlineAsm = Method(Void, (StringRef, 'Asm'))

    getContext = Method(ref(LLVMContext), const=True)

    getNamedValue = Method(ptr(GlobalValue), (StringRef, 'Name'), const=True)
    getMDKindID = Method(UnsignedInt, (StringRef, 'Name'), const=True)

    getTypeByName = Method(ptr(StructType), (StringRef, 'Name'), const=True)

    getOrInsertFunction = Method(ptr(Constant, null=None), (StringRef, 'Name'), (ptr(FunctionType), 'ty'))

    getFunction = Method(ptr(Function), (StringRef, 'Name'), const=True)
