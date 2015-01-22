from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath, copymodpath
from .ns import llvm

Value = llvm.Class('Value')
Value.modpath = submodpath(['value'])

BasicBlock = llvm.Class('BasicBlock', Value)

User = llvm.Class('User', Value)
User.modpath = submodpath(['user'])

Use = llvm.Class('Use')
Use.modpath = copymodpath(User)

Operator = llvm.Class('Operator', User)

Constant = llvm.Class('Constant', User)
Constant.modpath = submodpath(['constant'])

BlockAddress = llvm.Class('BlockAddress', Constant)
ConstantAggregateZero = llvm.Class('ConstantAggregateZero', Constant)
ConstantArray = llvm.Class('ConstantArray', Constant)
ConstantDataSequential = llvm.Class('ConstantDataSequential', Constant)
ConstantExpr = llvm.Class('ConstantExpr', Constant)
ConstantFP = llvm.Class('ConstantFP', Constant)
ConstantInt = llvm.Class('ConstantInt', Constant)
ConstantPointerNull = llvm.Class('ConstantPointerNull', Constant)
ConstantStruct = llvm.Class('ConstantStruct', Constant)
ConstantVector = llvm.Class('ConstantVector', Constant)
GlobalValue = llvm.Class('GlobalValue', Constant)
UndefValue = llvm.Class('UndefValue', Constant)

ConstantDataArray = llvm.Class('ConstantDataArray', ConstantDataSequential)
ConstantDataVector = llvm.Class('ConstantDataVector', ConstantDataSequential)

GlobalAlias = llvm.Class('GlobalAlias', GlobalValue)
GlobalObject = llvm.Class('GlobalObject', GlobalValue)

GlobalVariable = llvm.Class('GlobalVariable', GlobalObject)

DataLayout = llvm.Class('DataLayout')

DebugLoc = llvm.Class('DebugLoc')

CallingConv = llvm.Namespace('CallingConv')
CallingConv.modpath = submodpath(['calling_conv'])

Function = llvm.Class('Function', GlobalObject)

IRBuilderBase = llvm.Class('IRBuilderBase')
IRBuilder = llvm.Class('IRBuilder', IRBuilderBase)

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

LLVMContext = llvm.Class('LLVMContext')

Module = llvm.Class('Module')

Type = llvm.Class('Type')
Type.modpath = submodpath(['ty'])

CompositeType = llvm.Class('CompositeType', Type)
FunctionType = llvm.Class('FunctionType', Type)
IntegerType = llvm.Class('IntegerType', Type)

SequentialType = llvm.Class('SequentialType', CompositeType)
SequentialType.modpath = submodpath(['seq'])
StructType = llvm.Class('StructType', CompositeType)

ArrayType = llvm.Class('ArrayType', SequentialType)
PointerType = llvm.Class('PointerType', SequentialType)
VectorType = llvm.Class('VectorType', SequentialType)

Argument = llvm.Class('Argument', Value)
InlineAsm = llvm.Class('InlineAsm', Value)
MDNode = llvm.Class('MDNode', Value)
MDString = llvm.Class('MDString', Value)

ValueSymbolTable = llvm.Class('ValueSymbolTable')

Pass = llvm.Class('Pass')
Pass.modpath = submodpath(['pass'])

BasicBlockPass = llvm.Class('BasicBlockPass', Pass)
CallGraphSCCPass = llvm.Class('CallGraphSCCPass', Pass)
FunctionPass = llvm.Class('FunctionPass', Pass)
LoopPass = llvm.Class('LoopPass', Pass)
ModulePass = llvm.Class('ModulePass', Pass)
RegionPass = llvm.Class('RegionPass', Pass)

FunctionPassManager = llvm.Class('FunctionPassManager')
FunctionPassManager.modpath = submodpath(['pass'])
PassManager = llvm.Class('PassManager')
PassManager.modpath = submodpath(['pass'])
# PassManagerBuilder = llvm.Class('PassManagerBuilder')

from . import enums
