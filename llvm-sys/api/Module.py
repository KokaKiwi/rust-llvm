from bindgen.ast.objects import *
from .ns import llvm
from .ADT.StringRef import StringRef
from .Constant import Constant, GlobalValue, GlobalVariable
from .DataLayout import DataLayout
from .LLVMContext import LLVMContext
from .Type import Type, FunctionType, StructType

Module = llvm.Class('Module')

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

    getOrInsertFunction = Method(ptr(Constant), (StringRef, 'Name'), (ptr(FunctionType), 'ty'))

@GlobalValue.body
class GlobalValue:
    getParent = Method(ptr(Module, const=True), const=True)
    getParentMut = Method(ptr(Module)).with_call_name('getParent')

    getDataLayout = Method(ptr(DataLayout, const=True), const=True)

@GlobalVariable.body
class GlobalVariable:
    newWithModule = Constructor((ref(Module), 'Module'), (ptr(Type), 'Ty'), (Bool, 'isConstant'), (GlobalValue.LinkageTypes, 'Linkage'), (ptr(Constant), 'Initializer'))
