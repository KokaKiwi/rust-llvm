from bindgen.ast import *
from .ns import llvm

# Includes


@llvm.body
class llvm:
    _includes_ = {
        'llvm/Analysis/CallGraphSCCPass.h',
        'llvm/IR/Constants.h',
        'llvm/IR/DataLayout.h',
        'llvm/IR/DerivedTypes.h',
        'llvm/IR/Instruction.h',
        'llvm/IR/Instructions.h',
        'llvm/IR/IRBuilder.h',
        'llvm/IR/LLVMContext.h',
        'llvm/IR/Module.h',
        'llvm/IR/Operator.h',
        'llvm/IR/Type.h',
        'llvm/IR/Value.h',
        'llvm/IR/ValueSymbolTable.h',
        'llvm/IR/Verifier.h',
        'llvm/LinkAllPasses.h',
        'llvm/PassManager.h',
    }

# Values
Value = llvm.Class('Value')

BasicBlock = llvm.Class('BasicBlock', Value)

Argument = llvm.Class('Argument', Value)
InlineAsm = llvm.Class('InlineAsm', Value)
MDNode = llvm.Class('MDNode', Value)
MDString = llvm.Class('MDString', Value)

User = llvm.Class('User', Value)
Use = llvm.Class('Use')

Operator = llvm.Class('Operator', User)
Constant = llvm.Class('Constant', User)
Instruction = llvm.Class('Instruction', User)

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
Function = llvm.Class('Function', GlobalObject)

# Instructions
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

# Types
Type = llvm.Class('Type')

CompositeType = llvm.Class('CompositeType', Type)
FunctionType = llvm.Class('FunctionType', Type)
IntegerType = llvm.Class('IntegerType', Type)

SequentialType = llvm.Class('SequentialType', CompositeType)
StructType = llvm.Class('StructType', CompositeType)

ArrayType = llvm.Class('ArrayType', SequentialType)
PointerType = llvm.Class('PointerType', SequentialType)
VectorType = llvm.Class('VectorType', SequentialType)

# Passes
Pass = llvm.Class('Pass')

BasicBlockPass = llvm.Class('BasicBlockPass', Pass)
CallGraphSCCPass = llvm.Class('CallGraphSCCPass', Pass)
FunctionPass = llvm.Class('FunctionPass', Pass)
LoopPass = llvm.Class('LoopPass', Pass)
ModulePass = llvm.Class('ModulePass', Pass)
RegionPass = llvm.Class('RegionPass', Pass)

FunctionPassManager = llvm.Class('FunctionPassManager')
PassManager = llvm.Class('PassManager')
# PassManagerBuilder = llvm.Class('PassManagerBuilder')

# IR Builder
IRBuilderBase = llvm.Class('IRBuilderBase')
IRBuilder = llvm.Class('IRBuilder', IRBuilderBase)
IRBuilder.real_name = 'IRBuilder<>'

# Other classes
DataLayout = llvm.Class('DataLayout')
DebugLoc = llvm.Class('DebugLoc')
LLVMContext = llvm.Class('LLVMContext')
Module = llvm.Class('Module')
ValueSymbolTable = llvm.Class('ValueSymbolTable')
