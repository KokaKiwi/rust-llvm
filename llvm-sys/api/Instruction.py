from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath
from .ns import llvm
from .ADT.ArrayRef import ArrayRef
from .ADT.StringRef import StringRef
from .DataLayout import DataLayout
from .DebugLoc import DebugLoc
from .User import User
from .Value import BasicBlock, MDNode

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/Instruction.h']

Instruction = llvm.Class('Instruction', User)
Instruction.modpath = submodpath(['inst'])

AtomicCmpXchgInst = llvm.Class('AtomicCmpXchgInst', Instruction)
AtomicRMWInst = llvm.Class('AtomicRMWInst', Instruction)
BinaryOperator = llvm.Class('BinaryOperator', Instruction)
CallInst = llvm.Class('CallInst', Instruction)
CmpInst = llvm.Class('CmpInst', Instruction)
ExtractElementInst = llvm.Class('ExtractElementInst', Instruction)
FenceInst = llvm.Class('FenceInst', Instruction)
GetElementPtrInst = llvm.Class('GetElementPtrInst', Instruction)
InsertElementInst = llvm.Class('InsertElementInst', Instruction)
InsertValueInst = llvm.Class('InsertValueInst', Instruction)
LandingPadInst = llvm.Class('LandingPadInst', Instruction)
PHINode = llvm.Class('PHINode', Instruction)
SelectInst = llvm.Class('SelectInst', Instruction)
ShuffleVectorInst = llvm.Class('ShuffleVectorInst', Instruction)
StoreInst = llvm.Class('StoreInst', Instruction)
TerminatorInst = llvm.Class('TerminatorInst', Instruction)
UnaryInstruction = llvm.Class('UnaryInstruction', Instruction)

BranchInst = llvm.Class('BranchInst', TerminatorInst)
IndirectBrInst = llvm.Class('IndirectBrInst', TerminatorInst)
InvokeInst = llvm.Class('InvokeInst', TerminatorInst)
ResumeInst = llvm.Class('ResumeInst', TerminatorInst)
ReturnInst = llvm.Class('ReturnInst', TerminatorInst)
SwitchInst = llvm.Class('SwitchInst', TerminatorInst)
UnreachableInst = llvm.Class('UnreachableInst', TerminatorInst)

AllocaInst = llvm.Class('AllocaInst', UnaryInstruction)
CastInst = llvm.Class('CastInst', UnaryInstruction)
ExtractValueInst = llvm.Class('ExtractValueInst', UnaryInstruction)
LoadInst = llvm.Class('LoadInst', UnaryInstruction)
VAArgInst = llvm.Class('VAArgInst', UnaryInstruction)

AddrSpaceCastInst = llvm.Class('AddrSpaceCastInst', CastInst)
BitCastInst = llvm.Class('BitCastInst', CastInst)
FPExtInst = llvm.Class('FPExtInst', CastInst)
FPToSIInst = llvm.Class('FPToSIInst', CastInst)

@Instruction.body
class Instruction:
    delete = Destructor()

    user_back = Method(ptr(Instruction, const=True), const=True)
    user_back_mut = Method(ptr(Instruction)).with_call_name('user_back')

    getParent = Method(ptr(BasicBlock, const=True), const=True)
    getParentMut = Method(ptr(BasicBlock)).with_call_name('getParent')

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

    getMetadata = Method(ptr(MDNode), (UnsignedInt, 'KindID'), const=True)
    getMetadataStr = Method(ptr(MDNode), (StringRef, 'Kind'), const=True).with_call_name('getMetadata')

    setMetadata = Method(Void, (UnsignedInt, 'KindID'), (ptr(MDNode), 'Node'))
    setMetadataStr = Method(Void, (StringRef, 'Kind'), (ptr(MDNode), 'Node')).with_call_name('setMetadata')

    dropUnknownMetadata = Method(Void)
    dropUnknownMetadataFromIDS = Method(Void, (ArrayRef(UnsignedInt), 'KnownIDs')).with_call_name('dropUnknownMetadata')

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

    isIdenticalTo = Method(Bool, (ptr(Instruction, const=True), 'Inst'), const=True)
    isIdenticalToWhenDefined = Method(Bool, (ptr(Instruction, const=True), 'Inst'), const=True)

    isSameOperationAs = Method(Bool, (ptr(Instruction, const=True), 'Inst'), (UnsignedInt, 'flags'), const=True)

    isUsedOutsideOfBlock = Method(Bool, (ptr(BasicBlock, const=True), 'BB'), const=True)
