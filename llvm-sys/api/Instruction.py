from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath
from .ns import llvm
from .defs import *
from .ADT.ArrayRef import ArrayRef
from .ADT.StringRef import StringRef

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/Instruction.h', 'llvm/IR/Instructions.h']

@Instruction.body
class Instruction:
    delete = Destructor()

    user_back = Method(ptr(Instruction, const=True), const=True)
    user_back_mut = Method(ptr(Instruction)).with_call_name('user_back')

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

    getParent = Method(ptr(BasicBlock, const=True), const=True)
    getParentMut = Method(ptr(BasicBlock)).with_call_name('getParent')

    getMetadata = Method(ptr(MDNode), (UnsignedInt, 'KindID'), const=True)
    getMetadataStr = Method(ptr(MDNode), (StringRef, 'Kind'), const=True).with_call_name('getMetadata')

    setMetadata = Method(Void, (UnsignedInt, 'KindID'), (ptr(MDNode), 'Node'))
    setMetadataStr = Method(Void, (StringRef, 'Kind'), (ptr(MDNode), 'Node')).with_call_name('setMetadata')

    isUsedOutsideOfBlock = Method(Bool, (ptr(BasicBlock, const=True), 'BB'), const=True)

# Enums
@Instruction.body
class Instruction:
    TermOps = Enum(values=[
        ('Ret', 1), ('Br', 2), ('Switch', 3),
        ('IndirectBr', 4), ('Invoke', 5), ('Resume', 6),
        ('Unreachable', 7),
    ])

    BinaryOps = Enum(values=[
        ('Add', 8 ), ('FAdd', 9 ), ('Sub', 10 ),
        ('FSub', 11 ), ('Mul', 12 ), ('FMul', 13 ),
        ('UDiv', 14 ), ('SDiv', 15 ), ('FDiv', 16 ),
        ('URem', 17 ), ('SRem', 18 ), ('FRem', 19 ),
        ('Shl', 20 ), ('LShr', 21 ), ('AShr', 22 ),
        ('And', 23 ), ('Or', 24 ), ('Xor', 25 ),
    ])

    MemoryOps = Enum(values=[
        ('Alloca', 26), ('Load', 27),
        ('Store', 28), ('GetElementPtr', 29),
        ('Fence', 30), ('AtomicCmpXchg', 31),
        ('AtomicRMW', 32),
    ])

    CastOps = Enum(values=[
        ('Trunc', 33), ('ZExt', 34), ('SExt', 35),
        ('FPToUI', 36), ('FPToSI', 37), ('UIToFP', 38),
        ('SIToFP', 39), ('FPTrunc', 40), ('FPExt', 41),
        ('PtrToInt', 42), ('IntToPtr', 43), ('BitCast', 44),
        ('AddrSpaceCast', 45),
    ])

    OtherOps = Enum(values=[
        ('ICmp', 46), ('FCmp', 47), ('PHI', 48),
        ('Call', 49), ('Select', 50), ('UserOp1', 51),
        ('UserOp2', 52), ('VAArg', 53), ('ExtractElement', 54),
        ('InsertElement', 55), ('ShuffleVector', 56), ('ExtractValue', 57),
        ('InsertValue', 58), ('LandingPad', 59),
    ])

@CmpInst.body
class CmpInst:
    Predicate = Enum(values=[
        ('FCMP_FALSE', 0),
        ('FCMP_OEQ', 1),
        ('FCMP_OGT', 2),
        ('FCMP_OGE', 3),
        ('FCMP_OLT', 4),
        ('FCMP_OLE', 5),
        ('FCMP_ONE', 6),
        ('FCMP_ORD', 7),
        ('FCMP_UNO', 8),
        ('FCMP_UEQ', 9),
        ('FCMP_UGT', 10),
        ('FCMP_UGE', 11),
        ('FCMP_ULT', 12),
        ('FCMP_ULE', 13),
        ('FCMP_UNE', 14),
        ('FCMP_TRUE', 15),
        ('FIRST_FCMP_PREDICATE', 'FCMP_FALSE'),
        ('LAST_FCMP_PREDICATE', 'FCMP_TRUE'),
        ('BAD_FCMP_PREDICATE', 16),
        ('ICMP_EQ', 32),
        ('ICMP_NE', 33),
        ('ICMP_UGT', 34),
        ('ICMP_UGE', 35),
        ('ICMP_ULT', 36),
        ('ICMP_ULE', 37),
        ('ICMP_SGT', 38),
        ('ICMP_SGE', 39),
        ('ICMP_SLT', 40),
        ('ICMP_SLE', 41),
        ('FIRST_ICMP_PREDICATE', 'ICMP_EQ'),
        ('LAST_ICMP_PREDICATE', 'ICMP_SLE'),
        ('BAD_ICMP_PREDICATE', 42),
    ])

@llvm.body
class llvm_body:
    AtomicOrdering = Enum(values=[
        ('NotAtomic', 0), ('Unordered', 1), ('Monotonic', 2),
        ('Acquire', 4), ('Release', 5), ('AcquireRelease', 6),
        ('SequentiallyConsistent', 7),
    ])

    SynchronizationScope = Enum(values=[
        ('SingleThread', 0), ('CrossThread', 1),
    ])
