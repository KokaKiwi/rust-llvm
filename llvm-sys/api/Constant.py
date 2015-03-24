from bindgen.ast import *
from .ns import llvm
from .defs import *
from .ADT.APInt import APInt
from .ADT.ArrayRef import ArrayRef
from .ADT.StringRef import StringRef


@Constant.body
class Constant:

    def test_value():
        return Method(Bool, const=True)

    isNullValue = test_value()
    isAllOnesValue = test_value()
    isNegativeZeroValue = test_value()
    isZeroValue = test_value()
    isMinSignedValue = test_value()

    canTrap = test_value()
    isThreadDependent = test_value()
    isDLLImportDependent = test_value()
    isConstantUsed = test_value()

    getAggregateElement = Method(
        ptr(Constant), (UnsignedInt, 'Elt'), const=True)
    getAggregateElementConstant = Method(ptr(Constant), (ptr(
        Constant), 'Elt'), const=True).with_real_name('getAggregateElement')
    getSplatValue = Method(ptr(Constant), const=True)

    destroyConstant = Method()
    replaceUsesOfWithOnConstant = Method(
        Void, ptr(Value), ptr(Value), ptr(Use))
    removeDeadConstantUsers = Method(const=True)

    stripPointerCasts = Method(ptr(Constant, const=True), const=True)
    stripPointerCastsMut = Method(
        ptr(Constant)).with_real_name('stripPointerCasts')

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'V'))

    getNullValue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getAllOnesValue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getIntegerValue = StaticMethod(
        ptr(Constant), (ptr(Type), 'Ty'), (APInt, 'Value'))


@BlockAddress.body
class BlockAddress:
    destroyConstant = Method()

    getBasicBlock = Method(ptr(BasicBlock), const=True)
    getFunction = Method(ptr(Function), const=True)


@ConstantArray.body
class ConstantArray:
    getType = Method(ptr(Type), const=True)

    get = StaticMethod(
        ptr(Constant), (ptr(ArrayType), 'Ty'), (ArrayRef(ptr(Constant)), 'Values'))

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'V'))


@ConstantFP.body
class ConstantFP:
    isZero = Method(Bool, const=True)
    isNegative = Method(Bool, const=True)
    isNaN = Method(Bool, const=True)

    isExactlyValueFloat = Method(
        Bool, (Double, 'Val'), const=True).with_real_name('isExactlyValue')

    getZeroValueForNegation = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))

    get = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'), (Double, 'Val'))
    fromStr = StaticMethod(
        ptr(Constant), (ptr(Type), 'Ty'), (StringRef, 'Val')).with_real_name('get')

    getNegativeZero = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getInfinity = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'V'))


@ConstantInt.body
class ConstantInt:
    getTrue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getFalse = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))

    getTrueWithContext = StaticMethod(
        ptr(ConstantInt), (ref(LLVMContext), 'Context')).with_real_name('getTrue')
    getFalseWithContext = StaticMethod(
        ptr(ConstantInt), (ref(LLVMContext), 'Context')).with_real_name('getFalse')

    get = StaticMethod(
        ptr(ConstantInt), (ptr(IntegerType), 'Ty'), (UnsignedInt64, 'Value'))
    getSigned = StaticMethod(ptr(ConstantInt), (ptr(
        IntegerType), 'Ty'), (UnsignedInt64, 'Value'), (Bool, 'isSigned')).with_real_name('get')

    fromAPInt = StaticMethod(ptr(ConstantInt), (ref(
        LLVMContext), 'Context'), (APInt, 'Val')).with_real_name('get')
    fromStr = StaticMethod(ptr(ConstantInt), (ptr(
        IntegerType), 'Ty'), (StringRef, 'Str'), (UnsignedInt8, 'radix')).with_real_name('get')

    isValueValidForType = StaticMethod(
        Bool, (ptr(Type), 'Ty'), (UnsignedInt64, 'Val'))
    isSignedValueValidForType = StaticMethod(
        Bool, (ptr(Type), 'Ty'), (Int64, 'Val')).with_real_name('isValueValidForType')

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'Val'))

    # Unstable
    # getValue = Method(APInt, const=True)

    getBitWidth = Method(UnsignedInt, const=True)

    getZExtValue = Method(UnsignedInt64, const=True)
    getSExtValue = Method(Int64, const=True)

    equalsInt = Method(Bool, (UnsignedInt64, 'Val'), const=True)

    getType = Method(ptr(IntegerType), const=True)

    isNegative = Method(Bool, const=True)
    isZero = Method(Bool, const=True)
    isOne = Method(Bool, const=True)
    isMinusOne = Method(Bool, const=True)

    isMaxValue = Method(Bool, (Bool, 'isSigned'), const=True)
    isMinValue = Method(Bool, (Bool, 'isSigned'), const=True)

    uge = Method(Bool, (UnsignedInt64, 'Num'), const=True)


@ConstantPointerNull.body
class ConstantPointerNull:
    destroyConstant = Method()

    getType = Method(ptr(PointerType), const=True)

    get = StaticMethod(ptr(ConstantPointerNull), (ptr(PointerType), 'Ty'))

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'Val'))


@GlobalValue.body
class GlobalValue:
    delete = Destructor()

    getAlignment = Method(UnsignedInt, const=True)

    hasUnnamedAddr = Method(Bool, const=True)
    setUnnamedAddr = Method(Void, (Bool, 'Val'))

    hasDefaultVisibility = Method(Bool, const=True)
    hasHiddenVisibility = Method(Bool, const=True)
    hasProtectedVisibility = Method(Bool, const=True)

    isThreadLocal = Method(Bool, const=True)
    setThreadLocal = Method(Void, (Bool, 'Val'))

    hasDLLImportStorageClass = Method(Bool, const=True)
    hasDLLExportStorageClass = Method(Bool, const=True)

    hasSection = Method(Bool, const=True)

    getType = Method(ptr(PointerType), const=True)

    hasExternalLinkage = Method(Bool, const=True)
    hasAvailableExternallyLinkage = Method(Bool, const=True)
    hasLinkOnceLinkage = Method(Bool, const=True)
    hasWeakLinkage = Method(Bool, const=True)
    hasWeakAnyLinkage = Method(Bool, const=True)
    hasWeakODRLinkage = Method(Bool, const=True)
    hasAppendingLinkage = Method(Bool, const=True)
    hasInternalLinkage = Method(Bool, const=True)
    hasPrivateLinkage = Method(Bool, const=True)
    hasLocalLinkage = Method(Bool, const=True)
    hasExternalWeakLinkage = Method(Bool, const=True)
    hasCommonLinkage = Method(Bool, const=True)

    isDiscardableIfUnused = Method(Bool, const=True)
    mayBeOverridden = Method(Bool, const=True)
    isWeakForLinker = Method(Bool, const=True)

    copyAttributesFrom = Method(Void, (ptr(GlobalValue), 'Src'))
    destroyConstant = Method()

    isDeclaration = Method(Bool, const=True)

    removeFromParent = Method()
    eraseFromParent = Method()

    getParent = Method(ptr(Module, const=True), const=True)
    getParentMut = Method(ptr(Module)).with_real_name('getParent')

    getDataLayout = Method(ptr(DataLayout, const=True), const=True)


@GlobalObject.body
class GlobalObject:
    setSection = Method(Void, (StringRef, 'S'))


@GlobalVariable.body
class GlobalVariable:
    delete = Destructor()

    new = Constructor((ptr(Type), 'Ty'), (Bool, 'isConstant'),
                      (GlobalValue['LinkageTypes'], 'Linkage'), null=None)
    newWithModule = Constructor((ref(Module), 'Module'), (ptr(Type), 'Ty'), (Bool, 'isConstant'), (GlobalValue[
                                'LinkageTypes'], 'Linkage'), (ptr(Constant), 'Initializer'), null=None)

    hasInitializer = Method(Bool, const=True)
    hasDefinitiveInitializer = Method(Bool, const=True)
    hasUniqueInitializer = Method(Bool, const=True)

    getInitializer = Method(ptr(Constant, const=True), const=True)
    getInitializerMut = Method(ptr(Constant)).with_real_name('getInitializer')
    setInitializer = Method(Void, (ptr(Constant), 'InitVal'))

    isConstant = Method(Bool, const=True)
    setConstant = Method(Void, (Bool, 'Val'))

    isExternallyInitialized = Method(Bool, const=True)
    setExternallyInitialized = Method(Void, (Bool, 'Val'))

    copyAttributesFrom = Method(Void, (ptr(GlobalValue), 'Src'))

    removeFromParent = Method()
    eraseFromParent = Method()
