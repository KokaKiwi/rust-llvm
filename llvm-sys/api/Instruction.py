from rust_bindgen import ast
from rust_bindgen.ast import *
from .ns import llvm, CallingConv
from .defs import *
from .ADT.ArrayRef import ArrayRef
from .ADT.StringRef import StringRef


@Instruction.body
class Instruction:
    delete = Destructor()

    user_back = Method(ptr(Instruction, const=True), const=True)
    user_back_mut = Method(ptr(Instruction)).with_real_name('user_back')

    getDataLayout = Method(ptr(DataLayout, const=True), const=True)

    removeFromParent = Method()
    eraseFromParent = Method()

    insertBefore = Method(Void, (ptr(Instruction), 'InsertPos'))
    insertAfter = Method(Void, (ptr(Instruction), 'InsertPos'))
    moveBefore = Method(Void, (ptr(Instruction), 'MovePos'))

    getOpcode = Method(UnsignedInt, const=True)
    # getOpcodeName = Method(CString(), const=True)

    isTerminator = Method(Bool, const=True)
    isBinaryOp = Method(Bool, const=True)
    isShift = Method(Bool)
    isCast = Method(Bool, const=True)
    isLogicalShift = Method(Bool, const=True)
    isArithmeticShift = Method(Bool, const=True)

    hasMetadata = Method(Bool, const=True)
    hasMetadataOtherThanDebugLoc = Method(Bool, const=True)

    dropUnknownMetadata = Method(Void)
    dropUnknownMetadataFromIDS = Method(
        Void, (ArrayRef(UnsignedInt), 'KnownIDs')).with_real_name('dropUnknownMetadata')

    setDebugLoc = Method(Void, (ref(DebugLoc, const=True), 'Loc'))
    getDebugLoc = Method(ref(DebugLoc, const=True), const=True)

    setHasUnsafeAlgebra = Method(Void, (Bool, 'Val'))
    setHasNoNaNs = Method(Void, (Bool, 'Val'))
    setHasNoInfs = Method(Void, (Bool, 'Val'))
    setHasNoSignedZeros = Method(Void, (Bool, 'Val'))
    setHasAllowReciprocal = Method(Void, (Bool, 'Val'))

    hasUnsafeAlgebra = Method(Bool, const=True)
    hasNoNaNs = Method(Bool, const=True)
    hasNoInfs = Method(Bool, const=True)
    hasNoSignedZeros = Method(Bool, const=True)
    hasAllowReciprocal = Method(Bool, const=True)

    copyFastMathFlags = Method(Void, (ptr(Instruction, const=True), 'Inst'))

    isAssociative = Method(Bool, const=True)
    isCommutative = Method(Bool, const=True)
    isIdempotent = Method(Bool, const=True)
    isNilpotent = Method(Bool, const=True)
    mayWriteToMemory = Method(Bool, const=True)
    mayReadFromMemory = Method(Bool, const=True)
    mayReadOrWriteMemory = Method(Bool, const=True)
    mayThrow = Method(Bool, const=True)
    mayReturn = Method(Bool, const=True)
    mayHaveSideEffects = Method(Bool, const=True)

    clone = Method(ptr(Instruction), const=True)

    isIdenticalTo = Method(
        Bool, (ptr(Instruction, const=True), 'Inst'), const=True)
    isIdenticalToWhenDefined = Method(
        Bool, (ptr(Instruction, const=True), 'Inst'), const=True)

    isSameOperationAs = Method(
        Bool, (ptr(Instruction, const=True), 'Inst'), (UnsignedInt, 'flags'), const=True)

    getParent = Method(ptr(BasicBlock, const=True), const=True)
    getParentMut = Method(ptr(BasicBlock)).with_real_name('getParent')

    getMetadata = Method(ptr(MDNode), (UnsignedInt, 'KindID'), const=True)
    getMetadataStr = Method(
        ptr(MDNode), (StringRef, 'Kind'), const=True).with_real_name('getMetadata')

    setMetadata = Method(Void, (UnsignedInt, 'KindID'), (ptr(MDNode), 'Node'))
    setMetadataStr = Method(
        Void, (StringRef, 'Kind'), (ptr(MDNode), 'Node')).with_real_name('setMetadata')

    isUsedOutsideOfBlock = Method(
        Bool, (ptr(BasicBlock, const=True), 'BB'), const=True)


@SwitchInst.body
class SwitchInst:
    delete = Destructor()

    getCondition = Method(ptr(Value), const=True)
    setCondition = Method(Void, (ptr(Value), 'V'))

    getDefaultDest = Method(ptr(BasicBlock), const=True)
    setDefaultDest = Method(Void, (ptr(BasicBlock), 'DefaultCase'))

    getNumCases = Method(UnsignedInt, const=True)

    addCase = Method(
        Void, (ptr(ConstantInt), 'OnVal'), (ptr(BasicBlock), 'Dest'))


@PHINode.body
class PHINode:
    delete = Destructor()

    getNumIncomingValues = Method(UnsignedInt, const=True)

    getIncomingValue = Method(ptr(Value), (UnsignedInt, 'i'), const=True)
    setIncomingValue = Method(Void, (UnsignedInt, 'i'), (ptr(Value), 'V'))

    getIncomingBlock = Method(ptr(BasicBlock), (UnsignedInt, 'i'), const=True)
    setIncomingBlock = Method(
        Void, (UnsignedInt, 'i'), (ptr(BasicBlock), 'BB'))

    addIncoming = Method(Void, (ptr(Value), 'V'), (ptr(BasicBlock), 'BB'))
