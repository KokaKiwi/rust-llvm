from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath, copymodpath
from .ns import llvm
from .defs import *

GlobalValue['LinkageTypes'] = Enum(values=[
    ('ExternalLinkage', 0), 'AvailableExternallyLinkage', 'LinkOnceAnyLinkage',
    'LinkOnceODRLinkage', 'WeakAnyLinkage', 'WeakODRLinkage',
    'AppendingLinkage', 'InternalLinkage', 'PrivateLinkage',
    'ExternalWeakLinkage', 'CommonLinkage',
])

CallingConv['ID'] = Enum(values=[
    ('C', 0), ('Fast', 8), ('Cold', 9), ('GHC', 10),
    ('HiPE', 11), ('WebKit_JS', 12), ('AnyReg', 13), ('PreserveMost', 14),
    ('PreserveAll', 15), ('FirstTargetCC', 'X86_StdCall'), ('X86_StdCall', 64), ('X86_FastCall', 65),
    ('ARM_APCS', 66), ('ARM_AAPCS', 67), ('ARM_AAPCS_VFP', 68), ('MSP430_INTR', 69),
    ('X86_ThisCall', 70), ('PTX_Kernel', 71), ('PTX_Device', 72), ('SPIR_FUNC', 75),
    ('SPIR_KERNEL', 76), ('Intel_OCL_BI', 77), ('X86_64_SysV', 78), ('X86_64_Win64', 79),
])

Instruction['TermOps'] = Enum(values=[
    ('Ret', 1), ('Br', 2), ('Switch', 3),
    ('IndirectBr', 4), ('Invoke', 5), ('Resume', 6),
    ('Unreachable', 7),
])

Instruction['BinaryOps'] = Enum(values=[
    ('Add', 8 ), ('FAdd', 9 ), ('Sub', 10 ),
    ('FSub', 11 ), ('Mul', 12 ), ('FMul', 13 ),
    ('UDiv', 14 ), ('SDiv', 15 ), ('FDiv', 16 ),
    ('URem', 17 ), ('SRem', 18 ), ('FRem', 19 ),
    ('Shl', 20 ), ('LShr', 21 ), ('AShr', 22 ),
    ('And', 23 ), ('Or', 24 ), ('Xor', 25 ),
])

Instruction['MemoryOps'] = Enum(values=[
    ('Alloca', 26), ('Load', 27),
    ('Store', 28), ('GetElementPtr', 29),
    ('Fence', 30), ('AtomicCmpXchg', 31),
    ('AtomicRMW', 32),
])

Instruction['CastOps'] = Enum(values=[
    ('Trunc', 33), ('ZExt', 34), ('SExt', 35),
    ('FPToUI', 36), ('FPToSI', 37), ('UIToFP', 38),
    ('SIToFP', 39), ('FPTrunc', 40), ('FPExt', 41),
    ('PtrToInt', 42), ('IntToPtr', 43), ('BitCast', 44),
    ('AddrSpaceCast', 45),
])

Instruction['OtherOps'] = Enum(values=[
    ('ICmp', 46), ('FCmp', 47), ('PHI', 48),
    ('Call', 49), ('Select', 50), ('UserOp1', 51),
    ('UserOp2', 52), ('VAArg', 53), ('ExtractElement', 54),
    ('InsertElement', 55), ('ShuffleVector', 56), ('ExtractValue', 57),
    ('InsertValue', 58), ('LandingPad', 59),
])

CmpInst['Predicate'] = Enum(values=[
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

llvm['AtomicOrdering'] = Enum(values=[
    ('NotAtomic', 0), ('Unordered', 1), ('Monotonic', 2),
    ('Acquire', 4), ('Release', 5), ('AcquireRelease', 6),
    ('SequentiallyConsistent', 7),
])

llvm['SynchronizationScope'] = Enum(values=[
    ('SingleThread', 0), ('CrossThread', 1),
])

llvm['PassManagerType'] = Enum(values=[
    ('PMT_Unknown', 0),
    ('PMT_ModulePassManager', 1),
    ('PMT_CallGraphPassManager', 2),
    ('PMT_FunctionPassManager', 3),
    ('PMT_LoopPassManager', 4),
    ('PMT_RegionPassManager', 5),
    ('PMT_BasicBlockPassManager', 6),
    ('PMT_Last', 7),
])
llvm['PassManagerType'].modpath = submodpath(['pass'])

llvm['PassKind'] = Enum(values=[
    ('PT_BasicBlock', 0),
    ('PT_Region', 1),
    ('PT_Loop', 2),
    ('PT_Function', 3),
    ('PT_CallGraphSCC', 4),
    ('PT_Module', 5),
    ('PT_PassManager', 6),
])
llvm['PassKind'].modpath = submodpath(['pass'])
