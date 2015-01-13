from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath
from .ns import llvm
from .ADT.StringRef import StringRef
from .Constant import BlockAddress, GlobalObject, GlobalValue
from .LLVMContext import LLVMContext
from .Type import Type, FunctionType
from .Value import Value

CallingConv = llvm.Namespace('CallingConv')
CallingConv.modpath = submodpath(['calling_conv'])

Function = llvm.Class('Function', GlobalObject)

@CallingConv.body
class CallingConv:
    ID = Enum(values=[
        ('C', 0), ('Fast', 8), ('Cold', 9), ('GHC', 10),
        ('HiPE', 11), ('WebKit_JS', 12), ('AnyReg', 13), ('PreserveMost', 14),
        ('PreserveAll', 15), ('FirstTargetCC', 'X86_StdCall'), ('X86_StdCall', 64), ('X86_FastCall', 65),
        ('ARM_APCS', 66), ('ARM_AAPCS', 67), ('ARM_AAPCS_VFP', 68), ('MSP430_INTR', 69),
        ('X86_ThisCall', 70), ('PTX_Kernel', 71), ('PTX_Device', 72), ('SPIR_FUNC', 75),
        ('SPIR_KERNEL', 76), ('Intel_OCL_BI', 77), ('X86_64_SysV', 78), ('X86_64_Win64', 79),
    ])

@Function.body
class Function:
    delete = Destructor()

    getReturnType = Method(ptr(Type), const=True)
    getFunctionType = Method(ptr(FunctionType), const=True)
    getContext = Method(ref(LLVMContext), const=True)
    isVarArg = Method(Bool, const=True)

    getIntrinsicID = Method(UnsignedInt, const=True)
    isIntrinsic = Method(Bool, const=True)

    getCallingConv = Method(CallingConv.ID, const=True)
    setCallingConv = Method(Void, (CallingConv.ID, 'CC'))

    addFnAttr = Method(Void, (StringRef, 'Kind'))
    addFnAttrWithValue = Method(Void, (StringRef, 'Kind'), (StringRef, 'Val')).with_call_name('addFnAttr')
    hasFnAttr = Method(Bool, (StringRef, 'Kind'), const=True).with_call_name('hasFnAttribute')

    hasGC = Method(Bool, const=True)
    # getGC = Method(ptr(Char, const=True), const=True)
    # setGC = Method(Void, (ptr(Char, const=True), 'Str'))
    clearGC = Method()

    getParamAlignment = Method(UnsignedInt, (UnsignedInt, 'idx'), const=True)
    getDereferenceableBytes = Method(UnsignedInt64, (UnsignedInt, 'idx'), const=True)

    doesNotAccessMemory = Method(Bool, const=True)
    setDoesNotAccessMemory = Method()

    onlyReadsMemory = Method(Bool, const=True)
    setOnlyReadsMemory = Method()

    doesNotReturn = Method(Bool, const=True)
    setDoesNotReturn = Method()

    doesNotThrow = Method(Bool, const=True)
    setDoesNotThrow = Method()

    cannotDuplicate = Method(Bool, const=True)
    setCannotDuplicate = Method()

    hasUWTable = Method(Bool, const=True)
    setHasUWTable = Method()
    needsUnwindTableEntry = Method(Bool, const=True)

    hasStructRetAttr = Method(Bool, const=True)

    doesNotAlias = Method(Bool, (UnsignedInt, 'n'), const=True)
    setDoesNotAlias = Method(Void, (UnsignedInt, 'n'))

    doesNotCapture = Method(Bool, (UnsignedInt, 'n'), const=True)
    setDoesNotCapture = Method(Void, (UnsignedInt, 'n'))

    doesNotAccessMemoryParam = Method(Bool, (UnsignedInt, 'n'), const=True).with_call_name('doesNotAccessMemory')
    setDoesNotAccessMemoryParam = Method(Void, (UnsignedInt, 'n')).with_call_name('setDoesNotAccessMemory')

    onlyReadsMemoryParam = Method(Bool, (UnsignedInt, 'n'), const=True).with_call_name('onlyReadsMemory')
    setOnlyReadsMemoryParam = Method(Void, (UnsignedInt, 'n')).with_call_name('setOnlyReadsMemory')

    copyAttributesFrom = Method(Void, (ptr(GlobalValue), 'Src'))

    deleteBody = Method()
    removeFromParent = Method()
    eraseFromParent = Method()

    Create = StaticMethod(ptr(Function), (ptr(FunctionType), 'Ty'), (GlobalValue.LinkageTypes, 'Linkage'))
    CreateWithName = StaticMethod(ptr(Function), (ptr(FunctionType), 'Ty'), (GlobalValue.LinkageTypes, 'Linkage'), (StringRef, 'Name')).with_call_name('Create')

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'Val'))

@BlockAddress.body
class BlockAddress:
    getFunction = Method(ptr(Function), const=True)
