from .prelude import *
from .ADT.StringRef import StringRef
from .iplist import iplist

@llvm.body
class llvm_body:
    verifyFunction = ast.Function(Bool, (ref(Function, const=True), 'Function'))

@Function.body
class Function:
    delete = Destructor()

    # TODO: We should panic if this method return null, as it's like a constructor.
    Create = StaticMethod(ptr(Function), (ptr(FunctionType), 'Ty'), (GlobalValue['LinkageTypes'], 'Linkage'), (OptionString(), 'Name'), (OptionPointer(Module), 'Module'))

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'Val'))

    getReturnType = Method(ptr(Type), const=True)
    getFunctionType = Method(ptr(FunctionType), const=True)
    getContext = Method(ref(LLVMContext), const=True)
    isVarArg = Method(Bool, const=True)

    getIntrinsicID = Method(UnsignedInt, const=True)
    isIntrinsic = Method(Bool, const=True)

    getCallingConv = Method(CallingConv['ID'], const=True)
    setCallingConv = Method(Void, (CallingConv['ID'], 'CC'))

    addFnAttr = Method(Void, (StringRef, 'Kind'), (Option(StringRef, '""'), 'Val'))
    hasFnAttr = Method(Bool, (StringRef, 'Kind'), const=True).with_real_name('hasFnAttribute')

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

    doesNotAccessMemoryParam = Method(Bool, (UnsignedInt, 'n'), const=True).with_real_name('doesNotAccessMemory')
    setDoesNotAccessMemoryParam = Method(Void, (UnsignedInt, 'n')).with_real_name('setDoesNotAccessMemory')

    onlyReadsMemoryParam = Method(Bool, (UnsignedInt, 'n'), const=True).with_real_name('onlyReadsMemory')
    setOnlyReadsMemoryParam = Method(Void, (UnsignedInt, 'n')).with_real_name('setOnlyReadsMemory')

    copyAttributesFrom = Method(Void, (ptr(GlobalValue), 'Src'))

    deleteBody = Method()
    removeFromParent = Method()
    eraseFromParent = Method()

    getArgumentList = Method(ref(iplist(Argument), const=True), const=True)
    getArgumentListMut = Method(ref(iplist(Argument))).with_real_name('getArgumentList')

    getFirstArg = Method(ptr(Argument, const=True), const=True).with_real_name('arg_begin')
    getFirstArgMut = Method(ptr(Argument)).with_real_name('arg_begin')

    getEntryBlock = Method(ref(BasicBlock, const=True), const=True)
    getEntryBlockMut = Method(ref(BasicBlock)).with_real_name('getEntryBlock')

    getValueSymbolTable = Method(ref(ValueSymbolTable, const=True), const=True)
    getValueSymbolTableMut = Method(ref(ValueSymbolTable)).with_real_name('getValueSymbolTable')
