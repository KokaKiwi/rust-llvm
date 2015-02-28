#ifndef FFI_H_
#define FFI_H_

#ifdef __cplusplus
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Analysis/CallGraphSCCPass.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Instruction.h"
#include "llvm/IR/Instructions.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Operator.h"
#include "llvm/IR/Type.h"
#include "llvm/IR/Value.h"
#include "llvm/IR/ValueSymbolTable.h"
#include "llvm/IR/Verifier.h"
#include "llvm/LinkAllPasses.h"
#include "llvm/PassManager.h"
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::AddrSpaceCastInst llvm_AddrSpaceCastInst;
#else
typedef struct {} llvm_AddrSpaceCastInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::AllocaInst llvm_AllocaInst;
#else
typedef struct {} llvm_AllocaInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Argument llvm_Argument;
#else
typedef struct {} llvm_Argument;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ArrayType llvm_ArrayType;
#else
typedef struct {} llvm_ArrayType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::AtomicCmpXchgInst llvm_AtomicCmpXchgInst;
#else
typedef struct {} llvm_AtomicCmpXchgInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::AtomicRMWInst llvm_AtomicRMWInst;
#else
typedef struct {} llvm_AtomicRMWInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BasicBlock llvm_BasicBlock;
#else
typedef struct {} llvm_BasicBlock;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BasicBlockPass llvm_BasicBlockPass;
#else
typedef struct {} llvm_BasicBlockPass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BinaryOperator llvm_BinaryOperator;
#else
typedef struct {} llvm_BinaryOperator;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BitCastInst llvm_BitCastInst;
#else
typedef struct {} llvm_BitCastInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BlockAddress llvm_BlockAddress;
#else
typedef struct {} llvm_BlockAddress;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::BranchInst llvm_BranchInst;
#else
typedef struct {} llvm_BranchInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CallGraphSCCPass llvm_CallGraphSCCPass;
#else
typedef struct {} llvm_CallGraphSCCPass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CallInst llvm_CallInst;
#else
typedef struct {} llvm_CallInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CastInst llvm_CastInst;
#else
typedef struct {} llvm_CastInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CmpInst llvm_CmpInst;
#else
typedef struct {} llvm_CmpInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CompositeType llvm_CompositeType;
#else
typedef struct {} llvm_CompositeType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Constant llvm_Constant;
#else
typedef struct {} llvm_Constant;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantAggregateZero llvm_ConstantAggregateZero;
#else
typedef struct {} llvm_ConstantAggregateZero;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantArray llvm_ConstantArray;
#else
typedef struct {} llvm_ConstantArray;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantDataArray llvm_ConstantDataArray;
#else
typedef struct {} llvm_ConstantDataArray;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantDataSequential llvm_ConstantDataSequential;
#else
typedef struct {} llvm_ConstantDataSequential;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantDataVector llvm_ConstantDataVector;
#else
typedef struct {} llvm_ConstantDataVector;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantExpr llvm_ConstantExpr;
#else
typedef struct {} llvm_ConstantExpr;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantFP llvm_ConstantFP;
#else
typedef struct {} llvm_ConstantFP;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantInt llvm_ConstantInt;
#else
typedef struct {} llvm_ConstantInt;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantPointerNull llvm_ConstantPointerNull;
#else
typedef struct {} llvm_ConstantPointerNull;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantStruct llvm_ConstantStruct;
#else
typedef struct {} llvm_ConstantStruct;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ConstantVector llvm_ConstantVector;
#else
typedef struct {} llvm_ConstantVector;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::DataLayout llvm_DataLayout;
#else
typedef struct {} llvm_DataLayout;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::DebugLoc llvm_DebugLoc;
#else
typedef struct {} llvm_DebugLoc;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ExtractElementInst llvm_ExtractElementInst;
#else
typedef struct {} llvm_ExtractElementInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ExtractValueInst llvm_ExtractValueInst;
#else
typedef struct {} llvm_ExtractValueInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FPExtInst llvm_FPExtInst;
#else
typedef struct {} llvm_FPExtInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FPToSIInst llvm_FPToSIInst;
#else
typedef struct {} llvm_FPToSIInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FenceInst llvm_FenceInst;
#else
typedef struct {} llvm_FenceInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Function llvm_Function;
#else
typedef struct {} llvm_Function;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FunctionPass llvm_FunctionPass;
#else
typedef struct {} llvm_FunctionPass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FunctionPassManager llvm_FunctionPassManager;
#else
typedef struct {} llvm_FunctionPassManager;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::FunctionType llvm_FunctionType;
#else
typedef struct {} llvm_FunctionType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GetElementPtrInst llvm_GetElementPtrInst;
#else
typedef struct {} llvm_GetElementPtrInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GlobalAlias llvm_GlobalAlias;
#else
typedef struct {} llvm_GlobalAlias;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GlobalObject llvm_GlobalObject;
#else
typedef struct {} llvm_GlobalObject;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GlobalValue llvm_GlobalValue;
#else
typedef struct {} llvm_GlobalValue;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GlobalVariable llvm_GlobalVariable;
#else
typedef struct {} llvm_GlobalVariable;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::IRBuilder<> llvm_IRBuilder;
#else
typedef struct {} llvm_IRBuilder;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::IRBuilderBase llvm_IRBuilderBase;
#else
typedef struct {} llvm_IRBuilderBase;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::IndirectBrInst llvm_IndirectBrInst;
#else
typedef struct {} llvm_IndirectBrInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::InlineAsm llvm_InlineAsm;
#else
typedef struct {} llvm_InlineAsm;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::InsertElementInst llvm_InsertElementInst;
#else
typedef struct {} llvm_InsertElementInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::InsertValueInst llvm_InsertValueInst;
#else
typedef struct {} llvm_InsertValueInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction llvm_Instruction;
#else
typedef struct {} llvm_Instruction;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::IntegerType llvm_IntegerType;
#else
typedef struct {} llvm_IntegerType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::InvokeInst llvm_InvokeInst;
#else
typedef struct {} llvm_InvokeInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::LLVMContext llvm_LLVMContext;
#else
typedef struct {} llvm_LLVMContext;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::LandingPadInst llvm_LandingPadInst;
#else
typedef struct {} llvm_LandingPadInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::LoadInst llvm_LoadInst;
#else
typedef struct {} llvm_LoadInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::LoopPass llvm_LoopPass;
#else
typedef struct {} llvm_LoopPass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::MDNode llvm_MDNode;
#else
typedef struct {} llvm_MDNode;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::MDString llvm_MDString;
#else
typedef struct {} llvm_MDString;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Module llvm_Module;
#else
typedef struct {} llvm_Module;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ModulePass llvm_ModulePass;
#else
typedef struct {} llvm_ModulePass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Operator llvm_Operator;
#else
typedef struct {} llvm_Operator;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::PHINode llvm_PHINode;
#else
typedef struct {} llvm_PHINode;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Pass llvm_Pass;
#else
typedef struct {} llvm_Pass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::PassManager llvm_PassManager;
#else
typedef struct {} llvm_PassManager;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::PointerType llvm_PointerType;
#else
typedef struct {} llvm_PointerType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::RegionPass llvm_RegionPass;
#else
typedef struct {} llvm_RegionPass;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ResumeInst llvm_ResumeInst;
#else
typedef struct {} llvm_ResumeInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ReturnInst llvm_ReturnInst;
#else
typedef struct {} llvm_ReturnInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::SelectInst llvm_SelectInst;
#else
typedef struct {} llvm_SelectInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::SequentialType llvm_SequentialType;
#else
typedef struct {} llvm_SequentialType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ShuffleVectorInst llvm_ShuffleVectorInst;
#else
typedef struct {} llvm_ShuffleVectorInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::StoreInst llvm_StoreInst;
#else
typedef struct {} llvm_StoreInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::StructType llvm_StructType;
#else
typedef struct {} llvm_StructType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::SwitchInst llvm_SwitchInst;
#else
typedef struct {} llvm_SwitchInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::TerminatorInst llvm_TerminatorInst;
#else
typedef struct {} llvm_TerminatorInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Type llvm_Type;
#else
typedef struct {} llvm_Type;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::UnaryInstruction llvm_UnaryInstruction;
#else
typedef struct {} llvm_UnaryInstruction;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::UndefValue llvm_UndefValue;
#else
typedef struct {} llvm_UndefValue;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::UnreachableInst llvm_UnreachableInst;
#else
typedef struct {} llvm_UnreachableInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Use llvm_Use;
#else
typedef struct {} llvm_Use;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::User llvm_User;
#else
typedef struct {} llvm_User;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::VAArgInst llvm_VAArgInst;
#else
typedef struct {} llvm_VAArgInst;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Value llvm_Value;
#else
typedef struct {} llvm_Value;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::ValueSymbolTable llvm_ValueSymbolTable;
#else
typedef struct {} llvm_ValueSymbolTable;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::VectorType llvm_VectorType;
#else
typedef struct {} llvm_VectorType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::iplist<::llvm::Argument> llvm_iplist_llvm_Argument;
#else
typedef struct {} llvm_iplist_llvm_Argument;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::AtomicOrdering llvm_AtomicOrdering;
#else
typedef enum {
    AtomicOrdering_NotAtomic = 0,
    AtomicOrdering_Unordered = 1,
    AtomicOrdering_Monotonic = 2,
    AtomicOrdering_Acquire = 4,
    AtomicOrdering_Release = 5,
    AtomicOrdering_AcquireRelease = 6,
    AtomicOrdering_SequentiallyConsistent = 7,
} llvm_AtomicOrdering;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction::BinaryOps llvm_Instruction_BinaryOps;
#else
typedef enum {
    BinaryOps_Add = 8,
    BinaryOps_FAdd = 9,
    BinaryOps_Sub = 10,
    BinaryOps_FSub = 11,
    BinaryOps_Mul = 12,
    BinaryOps_FMul = 13,
    BinaryOps_UDiv = 14,
    BinaryOps_SDiv = 15,
    BinaryOps_FDiv = 16,
    BinaryOps_URem = 17,
    BinaryOps_SRem = 18,
    BinaryOps_FRem = 19,
    BinaryOps_Shl = 20,
    BinaryOps_LShr = 21,
    BinaryOps_AShr = 22,
    BinaryOps_And = 23,
    BinaryOps_Or = 24,
    BinaryOps_Xor = 25,
} llvm_Instruction_BinaryOps;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction::CastOps llvm_Instruction_CastOps;
#else
typedef enum {
    CastOps_Trunc = 33,
    CastOps_ZExt = 34,
    CastOps_SExt = 35,
    CastOps_FPToUI = 36,
    CastOps_FPToSI = 37,
    CastOps_UIToFP = 38,
    CastOps_SIToFP = 39,
    CastOps_FPTrunc = 40,
    CastOps_FPExt = 41,
    CastOps_PtrToInt = 42,
    CastOps_IntToPtr = 43,
    CastOps_BitCast = 44,
    CastOps_AddrSpaceCast = 45,
} llvm_Instruction_CastOps;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CallingConv::ID llvm_CallingConv_ID;
#else
typedef enum {
    ID_C = 0,
    ID_Fast = 8,
    ID_Cold = 9,
    ID_GHC = 10,
    ID_HiPE = 11,
    ID_WebKit_JS = 12,
    ID_AnyReg = 13,
    ID_PreserveMost = 14,
    ID_PreserveAll = 15,
    ID_FirstTargetCC = ID_X86_StdCall,
    ID_X86_StdCall = 64,
    ID_X86_FastCall = 65,
    ID_ARM_APCS = 66,
    ID_ARM_AAPCS = 67,
    ID_ARM_AAPCS_VFP = 68,
    ID_MSP430_INTR = 69,
    ID_X86_ThisCall = 70,
    ID_PTX_Kernel = 71,
    ID_PTX_Device = 72,
    ID_SPIR_FUNC = 75,
    ID_SPIR_KERNEL = 76,
    ID_Intel_OCL_BI = 77,
    ID_X86_64_SysV = 78,
    ID_X86_64_Win64 = 79,
} llvm_CallingConv_ID;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::GlobalValue::LinkageTypes llvm_GlobalValue_LinkageTypes;
#else
typedef enum {
    LinkageTypes_ExternalLinkage = 0,
    LinkageTypes_LinkOnceAnyLinkage = 1,
    LinkageTypes_LinkOnceODRLinkage = 2,
    LinkageTypes_WeakAnyLinkage = 3,
    LinkageTypes_WeakODRLinkage = 4,
    LinkageTypes_AppendingLinkage = 5,
    LinkageTypes_InternalLinkage = 6,
    LinkageTypes_PrivateLinkage = 7,
    LinkageTypes_ExternalWeakLinkage = 8,
    LinkageTypes_CommonLinkage = 9,
} llvm_GlobalValue_LinkageTypes;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction::MemoryOps llvm_Instruction_MemoryOps;
#else
typedef enum {
    MemoryOps_Alloca = 26,
    MemoryOps_Load = 27,
    MemoryOps_Store = 28,
    MemoryOps_GetElementPtr = 29,
    MemoryOps_Fence = 30,
    MemoryOps_AtomicCmpXchg = 31,
    MemoryOps_AtomicRMW = 32,
} llvm_Instruction_MemoryOps;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction::OtherOps llvm_Instruction_OtherOps;
#else
typedef enum {
    OtherOps_ICmp = 46,
    OtherOps_FCmp = 47,
    OtherOps_PHI = 48,
    OtherOps_Call = 49,
    OtherOps_Select = 50,
    OtherOps_UserOp1 = 51,
    OtherOps_UserOp2 = 52,
    OtherOps_VAArg = 53,
    OtherOps_ExtractElement = 54,
    OtherOps_InsertElement = 55,
    OtherOps_ShuffleVector = 56,
    OtherOps_ExtractValue = 57,
    OtherOps_InsertValue = 58,
    OtherOps_LandingPad = 59,
} llvm_Instruction_OtherOps;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::PassKind llvm_PassKind;
#else
typedef enum {
    PassKind_PT_BasicBlock = 0,
    PassKind_PT_Region = 1,
    PassKind_PT_Loop = 2,
    PassKind_PT_Function = 3,
    PassKind_PT_CallGraphSCC = 4,
    PassKind_PT_Module = 5,
    PassKind_PT_PassManager = 6,
} llvm_PassKind;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::PassManagerType llvm_PassManagerType;
#else
typedef enum {
    PassManagerType_PMT_Unknown = 0,
    PassManagerType_PMT_ModulePassManager = 1,
    PassManagerType_PMT_CallGraphPassManager = 2,
    PassManagerType_PMT_FunctionPassManager = 3,
    PassManagerType_PMT_LoopPassManager = 4,
    PassManagerType_PMT_RegionPassManager = 5,
    PassManagerType_PMT_BasicBlockPassManager = 6,
    PassManagerType_PMT_Last = 7,
} llvm_PassManagerType;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::CmpInst::Predicate llvm_CmpInst_Predicate;
#else
typedef enum {
    Predicate_FCMP_FALSE = 0,
    Predicate_FCMP_OEQ = 1,
    Predicate_FCMP_OGT = 2,
    Predicate_FCMP_OGE = 3,
    Predicate_FCMP_OLT = 4,
    Predicate_FCMP_OLE = 5,
    Predicate_FCMP_ONE = 6,
    Predicate_FCMP_ORD = 7,
    Predicate_FCMP_UNO = 8,
    Predicate_FCMP_UEQ = 9,
    Predicate_FCMP_UGT = 10,
    Predicate_FCMP_UGE = 11,
    Predicate_FCMP_ULT = 12,
    Predicate_FCMP_ULE = 13,
    Predicate_FCMP_UNE = 14,
    Predicate_FCMP_TRUE = 15,
    Predicate_FIRST_FCMP_PREDICATE = Predicate_FCMP_FALSE,
    Predicate_LAST_FCMP_PREDICATE = Predicate_FCMP_TRUE,
    Predicate_BAD_FCMP_PREDICATE = 16,
    Predicate_ICMP_EQ = 32,
    Predicate_ICMP_NE = 33,
    Predicate_ICMP_UGT = 34,
    Predicate_ICMP_UGE = 35,
    Predicate_ICMP_ULT = 36,
    Predicate_ICMP_ULE = 37,
    Predicate_ICMP_SGT = 38,
    Predicate_ICMP_SGE = 39,
    Predicate_ICMP_SLT = 40,
    Predicate_ICMP_SLE = 41,
    Predicate_FIRST_ICMP_PREDICATE = Predicate_ICMP_EQ,
    Predicate_LAST_ICMP_PREDICATE = Predicate_ICMP_SLE,
    Predicate_BAD_ICMP_PREDICATE = 42,
} llvm_CmpInst_Predicate;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::SynchronizationScope llvm_SynchronizationScope;
#else
typedef enum {
    SynchronizationScope_SingleThread = 0,
} llvm_SynchronizationScope;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Instruction::TermOps llvm_Instruction_TermOps;
#else
typedef enum {
    TermOps_Ret = 1,
    TermOps_Br = 2,
    TermOps_Switch = 3,
    TermOps_IndirectBr = 4,
    TermOps_Invoke = 5,
    TermOps_Resume = 6,
    TermOps_Unreachable = 7,
} llvm_Instruction_TermOps;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Type::TypeID llvm_Type_TypeID;
#else
typedef enum {
    TypeID_VoidTyID = 0,
    TypeID_FloatTyID = 1,
    TypeID_DoubleTyID = 2,
    TypeID_X86_FP80TyID = 3,
    TypeID_FP128TyID = 4,
    TypeID_PPC_FP128TyID = 5,
    TypeID_LabelTyID = 6,
    TypeID_MetadataTyID = 7,
    TypeID_X86_MMXTyID = 8,
    TypeID_IntegerTyID = 9,
    TypeID_FunctionTyID = 10,
    TypeID_StructTyID = 11,
    TypeID_ArrayTyID = 12,
    TypeID_PointerTyID = 13,
    TypeID_VectorTyID = 14,
} llvm_Type_TypeID;
#endif /* __cplusplus */

#ifdef __cplusplus
typedef ::llvm::Value::ValueTy llvm_Value_ValueTy;
#else
typedef enum {
    ValueTy_ArgumentVal = 0,
    ValueTy_BasicBlockVal = 1,
    ValueTy_FunctionVal = 2,
    ValueTy_GlobalAliasVal = 3,
    ValueTy_GlobalVariableVal = 4,
    ValueTy_UndefValueVal = 5,
    ValueTy_BlockAddressVal = 6,
    ValueTy_ConstantExprVal = 7,
    ValueTy_ConstantAggregateZeroVal = 8,
    ValueTy_ConstantDataArrayVal = 9,
    ValueTy_ConstantDataVectorVal = 10,
    ValueTy_ConstantIntVal = 11,
    ValueTy_ConstantFPVal = 12,
    ValueTy_ConstantArrayVal = 13,
    ValueTy_ConstantStructVal = 14,
    ValueTy_ConstantVectorVal = 15,
    ValueTy_ConstantPointerNullVal = 16,
    ValueTy_MetadataAsValueVal = 17,
    ValueTy_InlineAsmVal = 18,
    ValueTy_InstructionVal = 19,
    ValueTy_ConstantFirstVal = ValueTy_FunctionVal,
    ValueTy_ConstantLastVal = ValueTy_ConstantPointerNullVal,
} llvm_Value_ValueTy;
#endif /* __cplusplus */

typedef struct {
    llvm_Constant* const* data;
    size_t size;
} llvm_ArrayRef_ptr_llvm_Constant;

typedef struct {
    llvm_Type* const* data;
    size_t size;
} llvm_ArrayRef_ptr_llvm_Type;

typedef struct {
    llvm_Value* const* data;
    size_t size;
} llvm_ArrayRef_ptr_llvm_Value;

typedef struct {
    uint64_t const* data;
    size_t size;
} llvm_ArrayRef_uint64;

typedef struct {
    unsigned int const* data;
    size_t size;
} llvm_ArrayRef_uint;

typedef struct {
    unsigned int numbits;
    llvm_ArrayRef_uint64 data;
} llvm_APInt;

typedef struct {
    char const* data;
    size_t length;
} llvm_StringRef;

typedef struct {
    char* data;
    size_t length;
} std_string;

typedef struct {
    char const* data;
    size_t length;
} std_string_const;

extern "C"
llvm_FunctionPass* llvm_createAddDiscriminatorsPass();

extern "C"
llvm_FunctionPass* llvm_createAddressSanitizerFunctionPass();

extern "C"
llvm_ModulePass* llvm_createAddressSanitizerModulePass();

extern "C"
llvm_FunctionPass* llvm_createAggressiveDCEPass();

extern "C"
llvm_Pass* llvm_createAlwaysInlinerPass(int* InsertLifetime);

extern "C"
llvm_Pass* llvm_createArgumentPromotionPass(unsigned int* maxElements);

extern "C"
llvm_ModulePass* llvm_createBarrierNoopPass();

extern "C"
llvm_ModulePass* llvm_createBlockExtractorPass();

extern "C"
llvm_FunctionPass* llvm_createBoundsCheckingPass();

extern "C"
llvm_FunctionPass* llvm_createBreakCriticalEdgesPass();

extern "C"
llvm_FunctionPass* llvm_createCFGSimplificationPass();

extern "C"
llvm_FunctionPass* llvm_createConstantHoistingPass();

extern "C"
llvm_ModulePass* llvm_createConstantMergePass();

extern "C"
llvm_FunctionPass* llvm_createConstantPropagationPass();

extern "C"
llvm_Pass* llvm_createCorrelatedValuePropagationPass();

extern "C"
llvm_ModulePass* llvm_createDataFlowSanitizerPass();

extern "C"
llvm_ModulePass* llvm_createDeadArgEliminationPass();

extern "C"
llvm_ModulePass* llvm_createDeadArgHackingPass();

extern "C"
llvm_FunctionPass* llvm_createDeadCodeEliminationPass();

extern "C"
llvm_Pass* llvm_createDeadInstEliminationPass();

extern "C"
llvm_FunctionPass* llvm_createDeadStoreEliminationPass();

extern "C"
llvm_ModulePass* llvm_createDebugIRPass();

extern "C"
llvm_FunctionPass* llvm_createDemoteRegisterToMemoryPass();

extern "C"
llvm_FunctionPass* llvm_createEarlyCSEPass();

extern "C"
llvm_FunctionPass* llvm_createFlattenCFGPass();

extern "C"
llvm_Pass* llvm_createFunctionAttrsPass();

extern "C"
llvm_Pass* llvm_createFunctionInliningPass();

extern "C"
llvm_Pass* llvm_createFunctionInliningPassWithOptLevel(unsigned int OptLevel, unsigned int SizeOptLevel);

extern "C"
llvm_ModulePass* llvm_createGCOVProfilerPass();

extern "C"
llvm_FunctionPass* llvm_createGVNPass(int* NoLoads);

extern "C"
llvm_ModulePass* llvm_createGlobalDCEPass();

extern "C"
llvm_Pass* llvm_createGlobalMergePass();

extern "C"
llvm_ModulePass* llvm_createGlobalOptimizerPass();

extern "C"
llvm_ModulePass* llvm_createIPConstantPropagationPass();

extern "C"
llvm_ModulePass* llvm_createIPSCCPPass();

extern "C"
llvm_Pass* llvm_createIndVarSimplifyPass();

extern "C"
llvm_FunctionPass* llvm_createInstructionCombiningPass();

extern "C"
llvm_FunctionPass* llvm_createInstructionNamerPass();

extern "C"
llvm_FunctionPass* llvm_createInstructionSimplifierPass();

extern "C"
llvm_ModulePass* llvm_createInternalizePass();

extern "C"
llvm_FunctionPass* llvm_createJumpThreadingPass();

extern "C"
llvm_Pass* llvm_createLCSSAPass();

extern "C"
llvm_Pass* llvm_createLICMPass();

extern "C"
llvm_BasicBlockPass* llvm_createLoadCombinePass();

extern "C"
llvm_Pass* llvm_createLoopDeletionPass();

extern "C"
llvm_Pass* llvm_createLoopExtractorPass();

extern "C"
llvm_Pass* llvm_createLoopIdiomPass();

extern "C"
llvm_Pass* llvm_createLoopInstSimplifyPass();

extern "C"
llvm_Pass* llvm_createLoopRerollPass();

extern "C"
llvm_Pass* llvm_createLoopRotatePass(int* MaxHeaderSize);

extern "C"
llvm_Pass* llvm_createLoopSimplifyPass();

extern "C"
llvm_Pass* llvm_createLoopStrengthReducePass();

extern "C"
llvm_Pass* llvm_createLoopUnrollPass();

extern "C"
llvm_Pass* llvm_createLoopUnswitchPass(int* OptimizeForSize);

extern "C"
llvm_Pass* llvm_createLowerAtomicPass();

extern "C"
llvm_FunctionPass* llvm_createLowerExpectIntrinsicPass();

extern "C"
llvm_FunctionPass* llvm_createLowerInvokePass();

extern "C"
llvm_FunctionPass* llvm_createLowerSwitchPass();

extern "C"
llvm_FunctionPass* llvm_createMemCpyOptPass();

extern "C"
llvm_FunctionPass* llvm_createMemorySanitizerPass(int* TrackOrigins);

extern "C"
llvm_ModulePass* llvm_createMergeFunctionsPass();

extern "C"
llvm_FunctionPass* llvm_createMergedLoadStoreMotionPass();

extern "C"
llvm_ModulePass* llvm_createMetaRenamerPass();

extern "C"
llvm_Pass* llvm_createObjCARCAPElimPass();

extern "C"
llvm_Pass* llvm_createObjCARCContractPass();

extern "C"
llvm_Pass* llvm_createObjCARCExpandPass();

extern "C"
llvm_Pass* llvm_createObjCARCOptPass();

extern "C"
llvm_ModulePass* llvm_createPartialInliningPass();

extern "C"
llvm_FunctionPass* llvm_createPartiallyInlineLibCallsPass();

extern "C"
llvm_FunctionPass* llvm_createPromoteMemoryToRegisterPass();

extern "C"
llvm_Pass* llvm_createPruneEHPass();

extern "C"
llvm_FunctionPass* llvm_createReassociatePass();

extern "C"
llvm_FunctionPass* llvm_createSCCPPass();

extern "C"
llvm_FunctionPass* llvm_createSROAPass(int* RequiresDomTree);

extern "C"
llvm_FunctionPass* llvm_createSampleProfileLoaderPass();

extern "C"
llvm_FunctionPass* llvm_createScalarReplAggregatesPass();

extern "C"
llvm_FunctionPass* llvm_createScalarizerPass();

extern "C"
llvm_FunctionPass* llvm_createSeparateConstOffsetFromGEPPass();

extern "C"
llvm_Pass* llvm_createSimpleLoopUnrollPass();

extern "C"
llvm_Pass* llvm_createSingleLoopExtractorPass();

extern "C"
llvm_FunctionPass* llvm_createSinkingPass();

extern "C"
llvm_ModulePass* llvm_createStripDeadDebugInfoPass();

extern "C"
llvm_ModulePass* llvm_createStripDeadPrototypesPass();

extern "C"
llvm_ModulePass* llvm_createStripDebugDeclarePass();

extern "C"
llvm_ModulePass* llvm_createStripNonDebugSymbolsPass();

extern "C"
llvm_ModulePass* llvm_createStripSymbolsPass(int* OnlyDebugInfo);

extern "C"
llvm_Pass* llvm_createStructurizeCFGPass();

extern "C"
llvm_FunctionPass* llvm_createTailCallEliminationPass();

extern "C"
llvm_FunctionPass* llvm_createThreadSanitizerPass();

extern "C"
llvm_LLVMContext* llvm_getGlobalContext();

extern "C"
int llvm_verifyFunction(llvm_Function const* Function);

extern "C"
int llvm_verifyModule(llvm_Module const* Module);

extern "C"
unsigned int llvm_Argument_getArgNo(llvm_Argument const* self);

extern "C"
llvm_Function const* llvm_Argument_getParent(llvm_Argument const* self);

extern "C"
llvm_Function* llvm_Argument_getParentMut(llvm_Argument* self);

extern "C"
llvm_Argument* llvm_Argument_new(llvm_Type* Ty, std_string_const* Name, llvm_Function** F);

extern "C"
llvm_Argument const* llvm_Argument_next(llvm_Argument const* self);

extern "C"
llvm_Argument* llvm_Argument_nextMut(llvm_Argument* self);

extern "C"
llvm_Argument const* llvm_Argument_prev(llvm_Argument const* self);

extern "C"
llvm_Argument* llvm_Argument_prevMut(llvm_Argument* self);

extern "C"
int llvm_ArrayType_classof(llvm_Type const* ty);

extern "C"
llvm_ArrayType* llvm_ArrayType_get(llvm_Type* ElementType, uint64_t NumElements);

extern "C"
uint64_t llvm_ArrayType_getNumElements(llvm_ArrayType const* self);

extern "C"
int llvm_ArrayType_isValidElementType(llvm_Type* ty);

extern "C"
llvm_BasicBlock* llvm_BasicBlock_Create(llvm_LLVMContext* Context, std_string* Name, llvm_Function** Parent, llvm_BasicBlock** InsertBefore);

extern "C"
int llvm_BasicBlock_classof(llvm_Value const* Val);

extern "C"
void llvm_BasicBlock_delete(llvm_BasicBlock* self);

extern "C"
void llvm_BasicBlock_dropAllReferences(llvm_BasicBlock* self);

extern "C"
void llvm_BasicBlock_eraseFromParent(llvm_BasicBlock* self);

extern "C"
llvm_DataLayout const* llvm_BasicBlock_getDataLayout(llvm_BasicBlock const* self);

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHI(llvm_BasicBlock const* self);

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIMut(llvm_BasicBlock* self);

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbg(llvm_BasicBlock const* self);

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgMut(llvm_BasicBlock* self);

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(llvm_BasicBlock const* self);

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(llvm_BasicBlock* self);

extern "C"
llvm_LandingPadInst const* llvm_BasicBlock_getLandingPadInst(llvm_BasicBlock const* self);

extern "C"
llvm_LandingPadInst* llvm_BasicBlock_getLandingPadInstMut(llvm_BasicBlock* self);

extern "C"
llvm_Function const* llvm_BasicBlock_getParent(llvm_BasicBlock const* self);

extern "C"
llvm_Function* llvm_BasicBlock_getParentMut(llvm_BasicBlock* self);

extern "C"
llvm_BasicBlock const* llvm_BasicBlock_getSinglePredecessor(llvm_BasicBlock const* self);

extern "C"
llvm_BasicBlock* llvm_BasicBlock_getSinglePredecessorMut(llvm_BasicBlock* self);

extern "C"
llvm_TerminatorInst const* llvm_BasicBlock_getTerminator(llvm_BasicBlock const* self);

extern "C"
llvm_TerminatorInst* llvm_BasicBlock_getTerminatorMut(llvm_BasicBlock* self);

extern "C"
llvm_BasicBlock const* llvm_BasicBlock_getUniquePredecessor(llvm_BasicBlock const* self);

extern "C"
llvm_BasicBlock* llvm_BasicBlock_getUniquePredecessorMut(llvm_BasicBlock* self);

extern "C"
llvm_ValueSymbolTable* llvm_BasicBlock_getValueSymbolTable(llvm_BasicBlock* self);

extern "C"
int llvm_BasicBlock_hasAddressTaken(llvm_BasicBlock const* self);

extern "C"
int llvm_BasicBlock_isLandingPad(llvm_BasicBlock const* self);

extern "C"
void llvm_BasicBlock_moveAfter(llvm_BasicBlock* self, llvm_BasicBlock* MovePos);

extern "C"
void llvm_BasicBlock_moveBefore(llvm_BasicBlock* self, llvm_BasicBlock* MovePos);

extern "C"
void llvm_BasicBlock_removeFromParent(llvm_BasicBlock* self);

extern "C"
void llvm_BasicBlock_removePredecessor(llvm_BasicBlock* self, llvm_BasicBlock* Pred, int* DontDeleteUselessPHIs);

extern "C"
void llvm_BasicBlock_replaceSuccessorsPhiUsesWith(llvm_BasicBlock* self, llvm_BasicBlock* New);

extern "C"
void llvm_BlockAddress_destroyConstant(llvm_BlockAddress* self);

extern "C"
llvm_BasicBlock* llvm_BlockAddress_getBasicBlock(llvm_BlockAddress const* self);

extern "C"
llvm_Function* llvm_BlockAddress_getFunction(llvm_BlockAddress const* self);

extern "C"
int llvm_CompositeType_classof(llvm_Type const* ty);

extern "C"
llvm_Type* llvm_CompositeType_getTypeAtIndex(llvm_CompositeType* self, unsigned int idx);

extern "C"
int llvm_CompositeType_indexValid(llvm_CompositeType const* self, unsigned int idx);

extern "C"
int llvm_ConstantArray_classof(llvm_Value const* V);

extern "C"
llvm_Constant* llvm_ConstantArray_get(llvm_ArrayType* Ty, llvm_ArrayRef_ptr_llvm_Constant Values);

extern "C"
llvm_Type* llvm_ConstantArray_getType(llvm_ConstantArray const* self);

extern "C"
int llvm_ConstantFP_classof(llvm_Value const* V);

extern "C"
llvm_Constant* llvm_ConstantFP_fromStr(llvm_Type* Ty, llvm_StringRef Val);

extern "C"
llvm_Constant* llvm_ConstantFP_get(llvm_Type* Ty, double Val);

extern "C"
llvm_Constant* llvm_ConstantFP_getInfinity(llvm_Type* Ty);

extern "C"
llvm_Constant* llvm_ConstantFP_getNegativeZero(llvm_Type* Ty);

extern "C"
llvm_Constant* llvm_ConstantFP_getZeroValueForNegation(llvm_Type* Ty);

extern "C"
int llvm_ConstantFP_isExactlyValueFloat(llvm_ConstantFP const* self, double Val);

extern "C"
int llvm_ConstantFP_isNaN(llvm_ConstantFP const* self);

extern "C"
int llvm_ConstantFP_isNegative(llvm_ConstantFP const* self);

extern "C"
int llvm_ConstantFP_isZero(llvm_ConstantFP const* self);

extern "C"
int llvm_ConstantInt_classof(llvm_Value const* Val);

extern "C"
int llvm_ConstantInt_equalsInt(llvm_ConstantInt const* self, uint64_t Val);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_fromAPInt(llvm_LLVMContext* Context, llvm_APInt Val);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_fromStr(llvm_IntegerType* Ty, llvm_StringRef Str, uint8_t radix);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_get(llvm_IntegerType* Ty, uint64_t Value);

extern "C"
unsigned int llvm_ConstantInt_getBitWidth(llvm_ConstantInt const* self);

extern "C"
llvm_Constant* llvm_ConstantInt_getFalse(llvm_Type* Ty);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getFalseWithContext(llvm_LLVMContext* Context);

extern "C"
int64_t llvm_ConstantInt_getSExtValue(llvm_ConstantInt const* self);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getSigned(llvm_IntegerType* Ty, uint64_t Value, int isSigned);

extern "C"
llvm_Constant* llvm_ConstantInt_getTrue(llvm_Type* Ty);

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getTrueWithContext(llvm_LLVMContext* Context);

extern "C"
llvm_IntegerType* llvm_ConstantInt_getType(llvm_ConstantInt const* self);

extern "C"
uint64_t llvm_ConstantInt_getZExtValue(llvm_ConstantInt const* self);

extern "C"
int llvm_ConstantInt_isMaxValue(llvm_ConstantInt const* self, int isSigned);

extern "C"
int llvm_ConstantInt_isMinValue(llvm_ConstantInt const* self, int isSigned);

extern "C"
int llvm_ConstantInt_isMinusOne(llvm_ConstantInt const* self);

extern "C"
int llvm_ConstantInt_isNegative(llvm_ConstantInt const* self);

extern "C"
int llvm_ConstantInt_isOne(llvm_ConstantInt const* self);

extern "C"
int llvm_ConstantInt_isSignedValueValidForType(llvm_Type* Ty, int64_t Val);

extern "C"
int llvm_ConstantInt_isValueValidForType(llvm_Type* Ty, uint64_t Val);

extern "C"
int llvm_ConstantInt_isZero(llvm_ConstantInt const* self);

extern "C"
int llvm_ConstantInt_uge(llvm_ConstantInt const* self, uint64_t Num);

extern "C"
int llvm_ConstantPointerNull_classof(llvm_Value const* Val);

extern "C"
void llvm_ConstantPointerNull_destroyConstant(llvm_ConstantPointerNull* self);

extern "C"
llvm_ConstantPointerNull* llvm_ConstantPointerNull_get(llvm_PointerType* Ty);

extern "C"
llvm_PointerType* llvm_ConstantPointerNull_getType(llvm_ConstantPointerNull const* self);

extern "C"
int llvm_Constant_canTrap(llvm_Constant const* self);

extern "C"
int llvm_Constant_classof(llvm_Value const* V);

extern "C"
void llvm_Constant_destroyConstant(llvm_Constant* self);

extern "C"
llvm_Constant* llvm_Constant_getAggregateElement(llvm_Constant const* self, unsigned int Elt);

extern "C"
llvm_Constant* llvm_Constant_getAggregateElementConstant(llvm_Constant const* self, llvm_Constant* Elt);

extern "C"
llvm_Constant* llvm_Constant_getAllOnesValue(llvm_Type* Ty);

extern "C"
llvm_Constant* llvm_Constant_getIntegerValue(llvm_Type* Ty, llvm_APInt Value);

extern "C"
llvm_Constant* llvm_Constant_getNullValue(llvm_Type* Ty);

extern "C"
llvm_Constant* llvm_Constant_getSplatValue(llvm_Constant const* self);

extern "C"
int llvm_Constant_isAllOnesValue(llvm_Constant const* self);

extern "C"
int llvm_Constant_isConstantUsed(llvm_Constant const* self);

extern "C"
int llvm_Constant_isDLLImportDependent(llvm_Constant const* self);

extern "C"
int llvm_Constant_isMinSignedValue(llvm_Constant const* self);

extern "C"
int llvm_Constant_isNegativeZeroValue(llvm_Constant const* self);

extern "C"
int llvm_Constant_isNullValue(llvm_Constant const* self);

extern "C"
int llvm_Constant_isThreadDependent(llvm_Constant const* self);

extern "C"
int llvm_Constant_isZeroValue(llvm_Constant const* self);

extern "C"
void llvm_Constant_removeDeadConstantUsers(llvm_Constant const* self);

extern "C"
void llvm_Constant_replaceUsesOfWithOnConstant(llvm_Constant* self, llvm_Value* arg_1, llvm_Value* arg_2, llvm_Use* arg_3);

extern "C"
llvm_Constant const* llvm_Constant_stripPointerCasts(llvm_Constant const* self);

extern "C"
llvm_Constant* llvm_Constant_stripPointerCastsMut(llvm_Constant* self);

extern "C"
void llvm_FunctionPassManager_add(llvm_FunctionPassManager* self, llvm_FunctionPass* Pass);

extern "C"
int llvm_FunctionPassManager_doFinalization(llvm_FunctionPassManager* self);

extern "C"
int llvm_FunctionPassManager_doInitialization(llvm_FunctionPassManager* self);

extern "C"
llvm_FunctionPassManager* llvm_FunctionPassManager_new(llvm_Module* Module);

extern "C"
void llvm_FunctionPassManager_run(llvm_FunctionPassManager* self, llvm_Function* Function);

extern "C"
int llvm_FunctionType_classof(llvm_Type const* ty);

extern "C"
llvm_FunctionType* llvm_FunctionType_get(llvm_Type* Result, llvm_ArrayRef_ptr_llvm_Type Params, int isVarArg);

extern "C"
unsigned int llvm_FunctionType_getNumParams(llvm_FunctionType const* self);

extern "C"
llvm_Type* llvm_FunctionType_getParamType(llvm_FunctionType const* self, unsigned int idx);

extern "C"
llvm_Type* llvm_FunctionType_getReturnType(llvm_FunctionType const* self);

extern "C"
int llvm_FunctionType_isValidArgumentType(llvm_Type* ty);

extern "C"
int llvm_FunctionType_isValidReturnType(llvm_Type* ty);

extern "C"
int llvm_FunctionType_isVarArg(llvm_FunctionType const* self);

extern "C"
llvm_Function* llvm_Function_Create(llvm_FunctionType* Ty, llvm_GlobalValue_LinkageTypes Linkage, std_string* Name, llvm_Module** Module);

extern "C"
void llvm_Function_addFnAttr(llvm_Function* self, llvm_StringRef Kind, llvm_StringRef* Val);

extern "C"
int llvm_Function_cannotDuplicate(llvm_Function const* self);

extern "C"
int llvm_Function_classof(llvm_Value const* Val);

extern "C"
void llvm_Function_clearGC(llvm_Function* self);

extern "C"
void llvm_Function_copyAttributesFrom(llvm_Function* self, llvm_GlobalValue* Src);

extern "C"
void llvm_Function_delete(llvm_Function* self);

extern "C"
void llvm_Function_deleteBody(llvm_Function* self);

extern "C"
int llvm_Function_doesNotAccessMemory(llvm_Function const* self);

extern "C"
int llvm_Function_doesNotAccessMemoryParam(llvm_Function const* self, unsigned int n);

extern "C"
int llvm_Function_doesNotAlias(llvm_Function const* self, unsigned int n);

extern "C"
int llvm_Function_doesNotCapture(llvm_Function const* self, unsigned int n);

extern "C"
int llvm_Function_doesNotReturn(llvm_Function const* self);

extern "C"
int llvm_Function_doesNotThrow(llvm_Function const* self);

extern "C"
void llvm_Function_eraseFromParent(llvm_Function* self);

extern "C"
llvm_iplist_llvm_Argument const* llvm_Function_getArgumentList(llvm_Function const* self);

extern "C"
llvm_iplist_llvm_Argument* llvm_Function_getArgumentListMut(llvm_Function* self);

extern "C"
llvm_CallingConv_ID llvm_Function_getCallingConv(llvm_Function const* self);

extern "C"
llvm_LLVMContext* llvm_Function_getContext(llvm_Function const* self);

extern "C"
uint64_t llvm_Function_getDereferenceableBytes(llvm_Function const* self, unsigned int idx);

extern "C"
llvm_BasicBlock const* llvm_Function_getEntryBlock(llvm_Function const* self);

extern "C"
llvm_BasicBlock* llvm_Function_getEntryBlockMut(llvm_Function* self);

extern "C"
llvm_Argument const* llvm_Function_getFirstArg(llvm_Function const* self);

extern "C"
llvm_Argument* llvm_Function_getFirstArgMut(llvm_Function* self);

extern "C"
llvm_FunctionType* llvm_Function_getFunctionType(llvm_Function const* self);

extern "C"
unsigned int llvm_Function_getIntrinsicID(llvm_Function const* self);

extern "C"
unsigned int llvm_Function_getParamAlignment(llvm_Function const* self, unsigned int idx);

extern "C"
llvm_Type* llvm_Function_getReturnType(llvm_Function const* self);

extern "C"
llvm_ValueSymbolTable const* llvm_Function_getValueSymbolTable(llvm_Function const* self);

extern "C"
llvm_ValueSymbolTable* llvm_Function_getValueSymbolTableMut(llvm_Function* self);

extern "C"
int llvm_Function_hasFnAttr(llvm_Function const* self, llvm_StringRef Kind);

extern "C"
int llvm_Function_hasGC(llvm_Function const* self);

extern "C"
int llvm_Function_hasStructRetAttr(llvm_Function const* self);

extern "C"
int llvm_Function_hasUWTable(llvm_Function const* self);

extern "C"
int llvm_Function_isIntrinsic(llvm_Function const* self);

extern "C"
int llvm_Function_isVarArg(llvm_Function const* self);

extern "C"
int llvm_Function_needsUnwindTableEntry(llvm_Function const* self);

extern "C"
int llvm_Function_onlyReadsMemory(llvm_Function const* self);

extern "C"
int llvm_Function_onlyReadsMemoryParam(llvm_Function const* self, unsigned int n);

extern "C"
void llvm_Function_removeFromParent(llvm_Function* self);

extern "C"
void llvm_Function_setCallingConv(llvm_Function* self, llvm_CallingConv_ID CC);

extern "C"
void llvm_Function_setCannotDuplicate(llvm_Function* self);

extern "C"
void llvm_Function_setDoesNotAccessMemory(llvm_Function* self);

extern "C"
void llvm_Function_setDoesNotAccessMemoryParam(llvm_Function* self, unsigned int n);

extern "C"
void llvm_Function_setDoesNotAlias(llvm_Function* self, unsigned int n);

extern "C"
void llvm_Function_setDoesNotCapture(llvm_Function* self, unsigned int n);

extern "C"
void llvm_Function_setDoesNotReturn(llvm_Function* self);

extern "C"
void llvm_Function_setDoesNotThrow(llvm_Function* self);

extern "C"
void llvm_Function_setHasUWTable(llvm_Function* self);

extern "C"
void llvm_Function_setOnlyReadsMemory(llvm_Function* self);

extern "C"
void llvm_Function_setOnlyReadsMemoryParam(llvm_Function* self, unsigned int n);

extern "C"
void llvm_GlobalObject_setSection(llvm_GlobalObject* self, llvm_StringRef S);

extern "C"
void llvm_GlobalValue_copyAttributesFrom(llvm_GlobalValue* self, llvm_GlobalValue* Src);

extern "C"
void llvm_GlobalValue_delete(llvm_GlobalValue* self);

extern "C"
void llvm_GlobalValue_destroyConstant(llvm_GlobalValue* self);

extern "C"
void llvm_GlobalValue_eraseFromParent(llvm_GlobalValue* self);

extern "C"
unsigned int llvm_GlobalValue_getAlignment(llvm_GlobalValue const* self);

extern "C"
llvm_DataLayout const* llvm_GlobalValue_getDataLayout(llvm_GlobalValue const* self);

extern "C"
llvm_Module const* llvm_GlobalValue_getParent(llvm_GlobalValue const* self);

extern "C"
llvm_Module* llvm_GlobalValue_getParentMut(llvm_GlobalValue* self);

extern "C"
llvm_PointerType* llvm_GlobalValue_getType(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasAppendingLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasAvailableExternallyLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasCommonLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasDLLExportStorageClass(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasDLLImportStorageClass(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasDefaultVisibility(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasExternalLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasExternalWeakLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasHiddenVisibility(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasInternalLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasLinkOnceLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasLocalLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasPrivateLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasProtectedVisibility(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasSection(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasUnnamedAddr(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasWeakAnyLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasWeakLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_hasWeakODRLinkage(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_isDeclaration(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_isDiscardableIfUnused(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_isThreadLocal(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_isWeakForLinker(llvm_GlobalValue const* self);

extern "C"
int llvm_GlobalValue_mayBeOverridden(llvm_GlobalValue const* self);

extern "C"
void llvm_GlobalValue_removeFromParent(llvm_GlobalValue* self);

extern "C"
void llvm_GlobalValue_setThreadLocal(llvm_GlobalValue* self, int Val);

extern "C"
void llvm_GlobalValue_setUnnamedAddr(llvm_GlobalValue* self, int Val);

extern "C"
void llvm_GlobalVariable_copyAttributesFrom(llvm_GlobalVariable* self, llvm_GlobalValue* Src);

extern "C"
void llvm_GlobalVariable_delete(llvm_GlobalVariable* self);

extern "C"
void llvm_GlobalVariable_eraseFromParent(llvm_GlobalVariable* self);

extern "C"
llvm_Constant const* llvm_GlobalVariable_getInitializer(llvm_GlobalVariable const* self);

extern "C"
llvm_Constant* llvm_GlobalVariable_getInitializerMut(llvm_GlobalVariable* self);

extern "C"
int llvm_GlobalVariable_hasDefinitiveInitializer(llvm_GlobalVariable const* self);

extern "C"
int llvm_GlobalVariable_hasInitializer(llvm_GlobalVariable const* self);

extern "C"
int llvm_GlobalVariable_hasUniqueInitializer(llvm_GlobalVariable const* self);

extern "C"
int llvm_GlobalVariable_isConstant(llvm_GlobalVariable const* self);

extern "C"
int llvm_GlobalVariable_isExternallyInitialized(llvm_GlobalVariable const* self);

extern "C"
llvm_GlobalVariable* llvm_GlobalVariable_new(llvm_Type* Ty, int isConstant, llvm_GlobalValue_LinkageTypes Linkage);

extern "C"
llvm_GlobalVariable* llvm_GlobalVariable_newWithModule(llvm_Module* Module, llvm_Type* Ty, int isConstant, llvm_GlobalValue_LinkageTypes Linkage, llvm_Constant* Initializer);

extern "C"
void llvm_GlobalVariable_removeFromParent(llvm_GlobalVariable* self);

extern "C"
void llvm_GlobalVariable_setConstant(llvm_GlobalVariable* self, int Val);

extern "C"
void llvm_GlobalVariable_setExternallyInitialized(llvm_GlobalVariable* self, int Val);

extern "C"
void llvm_GlobalVariable_setInitializer(llvm_GlobalVariable* self, llvm_Constant* InitVal);

extern "C"
void llvm_IRBuilderBase_ClearInsertionPoint(llvm_IRBuilderBase* self);

extern "C"
llvm_Value* llvm_IRBuilderBase_CreateGlobalString(llvm_IRBuilderBase* self, llvm_StringRef Str, std_string* Name);

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateLifetimeEnd(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_ConstantInt** Size);

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateLifetimeStart(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_ConstantInt** Size);

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemCpy(llvm_IRBuilderBase* self, llvm_Value* Dst, llvm_Value* Src, llvm_Value* Size, unsigned int Align, int* isVolatile);

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemMove(llvm_IRBuilderBase* self, llvm_Value* Dst, llvm_Value* Src, llvm_Value* Size, unsigned int Align, int* isVolatile);

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemSet(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_Value* Value, llvm_Value* Size, unsigned int Align, int* isVolatile);

extern "C"
llvm_BasicBlock* llvm_IRBuilderBase_GetInsertBlock(llvm_IRBuilderBase const* self);

extern "C"
void llvm_IRBuilderBase_SetCurrentDebugLocation(llvm_IRBuilderBase* self, llvm_DebugLoc const* Loc);

extern "C"
void llvm_IRBuilderBase_SetDefaultFPMathTag(llvm_IRBuilderBase* self, llvm_MDNode* FPMathTag);

extern "C"
void llvm_IRBuilderBase_SetInsertPoint(llvm_IRBuilderBase* self, llvm_BasicBlock* BB);

extern "C"
void llvm_IRBuilderBase_SetInsertPointAtInst(llvm_IRBuilderBase* self, llvm_Instruction* Inst);

extern "C"
void llvm_IRBuilderBase_SetInstDebugLocation(llvm_IRBuilderBase const* self, llvm_Instruction* Inst);

extern "C"
llvm_LLVMContext* llvm_IRBuilderBase_getContext(llvm_IRBuilderBase const* self);

extern "C"
llvm_Type* llvm_IRBuilderBase_getCurrentFunctionReturnType(llvm_IRBuilderBase const* self);

extern "C"
llvm_MDNode* llvm_IRBuilderBase_getDefaultFPMathTag(llvm_IRBuilderBase const* self);

extern "C"
llvm_Type* llvm_IRBuilderBase_getDoubleTy(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getFalse(llvm_IRBuilderBase* self);

extern "C"
llvm_Type* llvm_IRBuilderBase_getFloatTy(llvm_IRBuilderBase* self);

extern "C"
llvm_Type* llvm_IRBuilderBase_getHalfTy(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt(llvm_IRBuilderBase* self, llvm_APInt Value);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt1(llvm_IRBuilderBase* self, int Value);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt16(llvm_IRBuilderBase* self, uint16_t Value);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt16Ty(llvm_IRBuilderBase* self);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt1Ty(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt32(llvm_IRBuilderBase* self, uint32_t Value);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt32Ty(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt64(llvm_IRBuilderBase* self, uint64_t Value);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt64Ty(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt8(llvm_IRBuilderBase* self, uint8_t Value);

extern "C"
llvm_PointerType* llvm_IRBuilderBase_getInt8PtrTy(llvm_IRBuilderBase* self, unsigned int* AddrSpace);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt8Ty(llvm_IRBuilderBase* self);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getIntN(llvm_IRBuilderBase* self, unsigned int NumBits, uint64_t Value);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getIntNTy(llvm_IRBuilderBase* self, unsigned int NumBits);

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getIntPtrTy(llvm_IRBuilderBase* self, llvm_DataLayout const* DL, unsigned int* AddrSpace);

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getTrue(llvm_IRBuilderBase* self);

extern "C"
llvm_Type* llvm_IRBuilderBase_getVoidTy(llvm_IRBuilderBase* self);

extern "C"
llvm_IRBuilderBase* llvm_IRBuilderBase_new(llvm_LLVMContext* Context);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAShr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAShrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAddrSpaceCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateAlignedLoad(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Align, std_string* Name);

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateAlignedLoadVolatile(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Align, int isVolatile, std_string* Name);

extern "C"
llvm_StoreInst* llvm_IRBuilder_CreateAlignedStore(llvm_IRBuilder* self, llvm_Value* Value, llvm_Value* Ptr, unsigned int Align, int* isVolatile);

extern "C"
llvm_AllocaInst* llvm_IRBuilder_CreateAlloca(llvm_IRBuilder* self, llvm_Type* Ty, llvm_Value** ArraySize, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAnd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateAndByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateBinOp(llvm_IRBuilder* self, llvm_Instruction_BinaryOps Opcode, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_BranchInst* llvm_IRBuilder_CreateBr(llvm_IRBuilder* self, llvm_BasicBlock* Dest);

extern "C"
llvm_CallInst* llvm_IRBuilder_CreateCall(llvm_IRBuilder* self, llvm_Value* Callee, llvm_ArrayRef_ptr_llvm_Value Args, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateCast(llvm_IRBuilder* self, llvm_Instruction_CastOps Opcode, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_BranchInst* llvm_IRBuilder_CreateCondBr(llvm_IRBuilder* self, llvm_Value* Cond, llvm_BasicBlock* TrueBlock, llvm_BasicBlock* FalseBlock);

extern "C"
llvm_Value* llvm_IRBuilder_CreateExactSDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateExactUDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractElement(llvm_IRBuilder* self, llvm_Value* Vec, llvm_Value* Idx, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractInteger(llvm_IRBuilder* self, llvm_DataLayout const* DL, llvm_Value* From, llvm_IntegerType* ExtractedTy, uint64_t Offset, std_string Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractValue(llvm_IRBuilder* self, llvm_Value* Agg, llvm_ArrayRef_uint Indexes, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmp(llvm_IRBuilder* self, llvm_CmpInst_Predicate Pred, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOLE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOLT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpONE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpORD(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpULE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpULT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUNE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUNO(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPToSI(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPToUI(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFRem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateFSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_FenceInst* llvm_IRBuilder_CreateFence(llvm_IRBuilder* self, llvm_AtomicOrdering Ordering, llvm_SynchronizationScope* SynchScope, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateGEP(llvm_IRBuilder* self, llvm_Value* Ptr, llvm_ArrayRef_ptr_llvm_Value Indexes, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateGlobalStringPtr(llvm_IRBuilder* self, llvm_StringRef Str, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmp(llvm_IRBuilder* self, llvm_CmpInst_Predicate Pred, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpNE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSLE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSLT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpUGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpUGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpULE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpULT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateInBoundsGEP(llvm_IRBuilder* self, llvm_Value* Ptr, llvm_ArrayRef_ptr_llvm_Value Indexes, std_string* Name);

extern "C"
llvm_IndirectBrInst* llvm_IRBuilder_CreateIndirectBr(llvm_IRBuilder* self, llvm_Value* Addr, unsigned int* NumCases);

extern "C"
llvm_Value* llvm_IRBuilder_CreateInsertElement(llvm_IRBuilder* self, llvm_Value* Vec, llvm_Value* NewElt, llvm_Value* Idx, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateInsertValue(llvm_IRBuilder* self, llvm_Value* Agg, llvm_Value* Value, llvm_ArrayRef_uint Indexes, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateIntCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, int isSigned, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateIntToPtr(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_InvokeInst* llvm_IRBuilder_CreateInvoke(llvm_IRBuilder* self, llvm_Value* Callee, llvm_BasicBlock* NormalDest, llvm_BasicBlock* UnwindDest, llvm_ArrayRef_ptr_llvm_Value Args, std_string_const* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateIsNotNull(llvm_IRBuilder* self, llvm_Value* Arg, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateIsNull(llvm_IRBuilder* self, llvm_Value* Arg, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateLShr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateLShrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_LandingPadInst* llvm_IRBuilder_CreateLandingPad(llvm_IRBuilder* self, llvm_Type* Ty, llvm_Value* PersFn, unsigned int NumClauses, std_string* Name);

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateLoad(llvm_IRBuilder* self, llvm_Value* Ptr, std_string* Name);

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateLoadVolatile(llvm_IRBuilder* self, llvm_Value* Ptr, int isVolatile, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateNot(llvm_IRBuilder* self, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateOr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateOrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_PHINode* llvm_IRBuilder_CreatePHI(llvm_IRBuilder* self, llvm_Type* Ty, unsigned int NumReservedValues, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreatePointerCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreatePtrDiff(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreatePtrToInt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_ResumeInst* llvm_IRBuilder_CreateResume(llvm_IRBuilder* self, llvm_Value* Exn);

extern "C"
llvm_ReturnInst* llvm_IRBuilder_CreateRet(llvm_IRBuilder* self, llvm_Value* Value);

extern "C"
llvm_ReturnInst* llvm_IRBuilder_CreateRetVoid(llvm_IRBuilder* self);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExtOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExtOrTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSIToFP(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSRem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSelect(llvm_IRBuilder* self, llvm_Value* C, llvm_Value* TrueValue, llvm_Value* FalseValue, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateShl(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateShlByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateShuffleVector(llvm_IRBuilder* self, llvm_Value* V1, llvm_Value* P2, llvm_Value* Mask, std_string* Name);

extern "C"
llvm_StoreInst* llvm_IRBuilder_CreateStore(llvm_IRBuilder* self, llvm_Value* Value, llvm_Value* Ptr, int* isVolatile);

extern "C"
llvm_Value* llvm_IRBuilder_CreateStructGEP(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Index, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_SwitchInst* llvm_IRBuilder_CreateSwitch(llvm_IRBuilder* self, llvm_Value* Value, llvm_BasicBlock* Dest, unsigned int* NumCases);

extern "C"
llvm_Value* llvm_IRBuilder_CreateTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateTruncOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateUDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateUIToFP(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateURem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_UnreachableInst* llvm_IRBuilder_CreateUnreachable(llvm_IRBuilder* self);

extern "C"
llvm_VAArgInst* llvm_IRBuilder_CreateVAArg(llvm_IRBuilder* self, llvm_Value* List, llvm_Type* Ty, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateVectorSplat(llvm_IRBuilder* self, unsigned int NumElements, llvm_Value* Value, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateXor(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateXorByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExtOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExtOrTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* Name);

extern "C"
void llvm_IRBuilder_delete(llvm_IRBuilder* self);

extern "C"
int llvm_IRBuilder_isNamePreserving(llvm_IRBuilder const* self);

extern "C"
llvm_IRBuilder* llvm_IRBuilder_new(llvm_LLVMContext* Context);

extern "C"
llvm_IRBuilder* llvm_IRBuilder_new_in_block(llvm_BasicBlock* BB);

extern "C"
llvm_Instruction* llvm_Instruction_clone(llvm_Instruction const* self);

extern "C"
void llvm_Instruction_copyFastMathFlags(llvm_Instruction* self, llvm_Instruction const* Inst);

extern "C"
void llvm_Instruction_delete(llvm_Instruction* self);

extern "C"
void llvm_Instruction_dropUnknownMetadata(llvm_Instruction* self);

extern "C"
void llvm_Instruction_dropUnknownMetadataFromIDS(llvm_Instruction* self, llvm_ArrayRef_uint KnownIDs);

extern "C"
void llvm_Instruction_eraseFromParent(llvm_Instruction* self);

extern "C"
llvm_DataLayout const* llvm_Instruction_getDataLayout(llvm_Instruction const* self);

extern "C"
llvm_DebugLoc const* llvm_Instruction_getDebugLoc(llvm_Instruction const* self);

extern "C"
llvm_MDNode* llvm_Instruction_getMetadata(llvm_Instruction const* self, unsigned int KindID);

extern "C"
llvm_MDNode* llvm_Instruction_getMetadataStr(llvm_Instruction const* self, llvm_StringRef Kind);

extern "C"
unsigned int llvm_Instruction_getOpcode(llvm_Instruction const* self);

extern "C"
llvm_BasicBlock const* llvm_Instruction_getParent(llvm_Instruction const* self);

extern "C"
llvm_BasicBlock* llvm_Instruction_getParentMut(llvm_Instruction* self);

extern "C"
int llvm_Instruction_hasAllowReciprocal(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasMetadata(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasMetadataOtherThanDebugLoc(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasNoInfs(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasNoNaNs(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasNoSignedZeros(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_hasUnsafeAlgebra(llvm_Instruction const* self);

extern "C"
void llvm_Instruction_insertAfter(llvm_Instruction* self, llvm_Instruction* InsertPos);

extern "C"
void llvm_Instruction_insertBefore(llvm_Instruction* self, llvm_Instruction* InsertPos);

extern "C"
int llvm_Instruction_isArithmeticShift(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isAssociative(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isBinaryOp(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isCast(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isCommutative(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isIdempotent(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isIdenticalTo(llvm_Instruction const* self, llvm_Instruction const* Inst);

extern "C"
int llvm_Instruction_isIdenticalToWhenDefined(llvm_Instruction const* self, llvm_Instruction const* Inst);

extern "C"
int llvm_Instruction_isLogicalShift(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isNilpotent(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isSameOperationAs(llvm_Instruction const* self, llvm_Instruction const* Inst, unsigned int flags);

extern "C"
int llvm_Instruction_isShift(llvm_Instruction* self);

extern "C"
int llvm_Instruction_isTerminator(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_isUsedOutsideOfBlock(llvm_Instruction const* self, llvm_BasicBlock const* BB);

extern "C"
int llvm_Instruction_mayHaveSideEffects(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_mayReadFromMemory(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_mayReadOrWriteMemory(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_mayReturn(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_mayThrow(llvm_Instruction const* self);

extern "C"
int llvm_Instruction_mayWriteToMemory(llvm_Instruction const* self);

extern "C"
void llvm_Instruction_moveBefore(llvm_Instruction* self, llvm_Instruction* MovePos);

extern "C"
void llvm_Instruction_removeFromParent(llvm_Instruction* self);

extern "C"
void llvm_Instruction_setDebugLoc(llvm_Instruction* self, llvm_DebugLoc const* Loc);

extern "C"
void llvm_Instruction_setHasAllowReciprocal(llvm_Instruction* self, int Val);

extern "C"
void llvm_Instruction_setHasNoInfs(llvm_Instruction* self, int Val);

extern "C"
void llvm_Instruction_setHasNoNaNs(llvm_Instruction* self, int Val);

extern "C"
void llvm_Instruction_setHasNoSignedZeros(llvm_Instruction* self, int Val);

extern "C"
void llvm_Instruction_setHasUnsafeAlgebra(llvm_Instruction* self, int Val);

extern "C"
void llvm_Instruction_setMetadata(llvm_Instruction* self, unsigned int KindID, llvm_MDNode* Node);

extern "C"
void llvm_Instruction_setMetadataStr(llvm_Instruction* self, llvm_StringRef Kind, llvm_MDNode* Node);

extern "C"
llvm_Instruction const* llvm_Instruction_user_back(llvm_Instruction const* self);

extern "C"
llvm_Instruction* llvm_Instruction_user_back_mut(llvm_Instruction* self);

extern "C"
int llvm_IntegerType_classof(llvm_Type const* ty);

extern "C"
llvm_IntegerType* llvm_IntegerType_get(llvm_LLVMContext* ctx, unsigned int NumBits);

extern "C"
uint64_t llvm_IntegerType_getBitMask(llvm_IntegerType const* self);

extern "C"
unsigned int llvm_IntegerType_getBitWidth(llvm_IntegerType const* self);

extern "C"
uint64_t llvm_IntegerType_getSignBit(llvm_IntegerType const* self);

extern "C"
int llvm_IntegerType_isPowerOf2ByteWidth(llvm_IntegerType const* self);

extern "C"
llvm_LLVMContext* llvm_LLVMContext_delete();

extern "C"
llvm_LLVMContext* llvm_LLVMContext_new();

extern "C"
void llvm_Module_appendModuleInlineAsm(llvm_Module* self, llvm_StringRef Asm);

extern "C"
void llvm_Module_delete(llvm_Module* self);

extern "C"
void llvm_Module_dump(llvm_Module const* self);

extern "C"
llvm_LLVMContext* llvm_Module_getContext(llvm_Module const* self);

extern "C"
llvm_DataLayout const* llvm_Module_getDataLayout(llvm_Module const* self);

extern "C"
std_string_const llvm_Module_getDataLayoutStr(llvm_Module const* self);

extern "C"
llvm_Function* llvm_Module_getFunction(llvm_Module const* self, llvm_StringRef Name);

extern "C"
unsigned int llvm_Module_getMDKindID(llvm_Module const* self, llvm_StringRef Name);

extern "C"
std_string_const llvm_Module_getModuleIdentifier(llvm_Module const* self);

extern "C"
std_string_const llvm_Module_getModuleInlineAsm(llvm_Module const* self);

extern "C"
llvm_GlobalValue* llvm_Module_getNamedValue(llvm_Module const* self, llvm_StringRef Name);

extern "C"
llvm_Constant* llvm_Module_getOrInsertFunction(llvm_Module* self, llvm_StringRef Name, llvm_FunctionType* ty);

extern "C"
std_string_const llvm_Module_getTargetTriple(llvm_Module const* self);

extern "C"
llvm_StructType* llvm_Module_getTypeByName(llvm_Module const* self, llvm_StringRef Name);

extern "C"
llvm_Module* llvm_Module_new(llvm_StringRef ModuleID, llvm_LLVMContext* Context);

extern "C"
void llvm_Module_setDataLayout(llvm_Module* self, llvm_DataLayout const* Other);

extern "C"
void llvm_Module_setDataLayoutStr(llvm_Module* self, llvm_StringRef Desc);

extern "C"
void llvm_Module_setModuleIdentifier(llvm_Module* self, llvm_StringRef ID);

extern "C"
void llvm_Module_setModuleInlineAsm(llvm_Module* self, llvm_StringRef Asm);

extern "C"
void llvm_Module_setTargetTriple(llvm_Module* self, llvm_StringRef Triple);

extern "C"
unsigned int llvm_Operator_getOpcode(llvm_Operator const* self);

extern "C"
void llvm_PHINode_addIncoming(llvm_PHINode* self, llvm_Value* V, llvm_BasicBlock* BB);

extern "C"
void llvm_PHINode_delete(llvm_PHINode* self);

extern "C"
llvm_BasicBlock* llvm_PHINode_getIncomingBlock(llvm_PHINode const* self, unsigned int i);

extern "C"
llvm_Value* llvm_PHINode_getIncomingValue(llvm_PHINode const* self, unsigned int i);

extern "C"
unsigned int llvm_PHINode_getNumIncomingValues(llvm_PHINode const* self);

extern "C"
void llvm_PHINode_setIncomingBlock(llvm_PHINode* self, unsigned int i, llvm_BasicBlock* BB);

extern "C"
void llvm_PHINode_setIncomingValue(llvm_PHINode* self, unsigned int i, llvm_Value* V);

extern "C"
void llvm_PassManager_add(llvm_PassManager* self, llvm_Pass* Pass);

extern "C"
llvm_PassManager* llvm_PassManager_new();

extern "C"
void llvm_PassManager_run(llvm_PassManager* self, llvm_Module* Module);

extern "C"
void llvm_Pass_delete(llvm_Pass* self);

extern "C"
int llvm_Pass_doFinalization(llvm_Pass* self, llvm_Module* Module);

extern "C"
int llvm_Pass_doInitialization(llvm_Pass* self, llvm_Module* Module);

extern "C"
void llvm_Pass_dump(llvm_Pass const* self);

extern "C"
llvm_PassKind llvm_Pass_getPassKind(llvm_Pass const* self);

extern "C"
int llvm_PointerType_classof(llvm_Type const* ty);

extern "C"
llvm_PointerType* llvm_PointerType_get(llvm_Type* ElementType, unsigned int AddressSpace);

extern "C"
unsigned int llvm_PointerType_getAddressSpace(llvm_PointerType const* self);

extern "C"
llvm_PointerType* llvm_PointerType_getUnqual(llvm_Type* ElementType);

extern "C"
int llvm_PointerType_isValidElementType(llvm_Type* ty);

extern "C"
int llvm_SequentialType_classof(llvm_Type const* ty);

extern "C"
llvm_Type* llvm_SequentialType_getElementType(llvm_SequentialType const* self);

extern "C"
int llvm_StructType_classof(llvm_Type const* ty);

extern "C"
llvm_StructType* llvm_StructType_create(llvm_LLVMContext* ctx, llvm_ArrayRef_ptr_llvm_Type Elements, llvm_StringRef Name);

extern "C"
llvm_StructType* llvm_StructType_createPacked(llvm_LLVMContext* ctx, llvm_ArrayRef_ptr_llvm_Type Elements, llvm_StringRef Name, int isPacked);

extern "C"
llvm_Type* llvm_StructType_getElementType(llvm_StructType const* self, unsigned int idx);

extern "C"
llvm_StringRef llvm_StructType_getName(llvm_StructType const* self);

extern "C"
unsigned int llvm_StructType_getNumElements(llvm_StructType const* self);

extern "C"
int llvm_StructType_hasName(llvm_StructType const* self);

extern "C"
int llvm_StructType_isLayoutIdentical(llvm_StructType const* self, llvm_StructType* Other);

extern "C"
int llvm_StructType_isLiteral(llvm_StructType const* self);

extern "C"
int llvm_StructType_isOpaque(llvm_StructType const* self);

extern "C"
int llvm_StructType_isPacked(llvm_StructType const* self);

extern "C"
int llvm_StructType_isSized(llvm_StructType const* self);

extern "C"
int llvm_StructType_isValidElementType(llvm_Type* ty);

extern "C"
void llvm_StructType_setBody(llvm_StructType* self, llvm_ArrayRef_ptr_llvm_Type Elements);

extern "C"
void llvm_StructType_setBodyPacked(llvm_StructType* self, llvm_ArrayRef_ptr_llvm_Type Elements, int isPacked);

extern "C"
void llvm_StructType_setName(llvm_StructType* self, llvm_StringRef Name);

extern "C"
void llvm_SwitchInst_addCase(llvm_SwitchInst* self, llvm_ConstantInt* OnVal, llvm_BasicBlock* Dest);

extern "C"
void llvm_SwitchInst_delete(llvm_SwitchInst* self);

extern "C"
llvm_Value* llvm_SwitchInst_getCondition(llvm_SwitchInst const* self);

extern "C"
llvm_BasicBlock* llvm_SwitchInst_getDefaultDest(llvm_SwitchInst const* self);

extern "C"
unsigned int llvm_SwitchInst_getNumCases(llvm_SwitchInst const* self);

extern "C"
void llvm_SwitchInst_setCondition(llvm_SwitchInst* self, llvm_Value* V);

extern "C"
void llvm_SwitchInst_setDefaultDest(llvm_SwitchInst* self, llvm_BasicBlock* DefaultCase);

extern "C"
void llvm_Type_dump(llvm_Type const* self);

extern "C"
llvm_Type* llvm_Type_getContainedType(llvm_Type const* self, unsigned int idx);

extern "C"
llvm_LLVMContext* llvm_Type_getContext(llvm_Type const* self);

extern "C"
llvm_PointerType* llvm_Type_getDoublePtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getDoubleTy(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getFP128PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getFP128Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getFloatPtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getFloatTy(llvm_LLVMContext* ctx);

extern "C"
unsigned int llvm_Type_getFunctionNumParams(llvm_Type const* self);

extern "C"
llvm_Type* llvm_Type_getFunctionParamType(llvm_Type const* self, unsigned int idx);

extern "C"
llvm_PointerType* llvm_Type_getHalfPtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getHalfTy(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getInt16PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_IntegerType* llvm_Type_getInt16Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getInt1PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_IntegerType* llvm_Type_getInt1Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getInt32PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_IntegerType* llvm_Type_getInt32Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getInt64PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_IntegerType* llvm_Type_getInt64Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getInt8PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_IntegerType* llvm_Type_getInt8Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getIntNPtrTy(llvm_LLVMContext* ctx, unsigned int size);

extern "C"
llvm_IntegerType* llvm_Type_getIntNTy(llvm_LLVMContext* ctx, unsigned int size);

extern "C"
llvm_Type* llvm_Type_getLabelTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getMetadataTy(llvm_LLVMContext* ctx);

extern "C"
unsigned int llvm_Type_getNumContainedTypes(llvm_Type const* self);

extern "C"
llvm_PointerType* llvm_Type_getPPC_FP128PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getPPC_FP128Ty(llvm_LLVMContext* ctx);

extern "C"
unsigned int llvm_Type_getPointerAddressSpace(llvm_Type const* self);

extern "C"
llvm_Type* llvm_Type_getPointerElementType(llvm_Type const* self);

extern "C"
llvm_PointerType* llvm_Type_getPointerTo(llvm_Type* self, unsigned int idx);

extern "C"
llvm_Type* llvm_Type_getSequentialElementType(llvm_Type const* self);

extern "C"
llvm_Type* llvm_Type_getStructElementType(llvm_Type const* self, unsigned int idx);

extern "C"
llvm_StringRef llvm_Type_getStructName(llvm_Type const* self);

extern "C"
unsigned int llvm_Type_getStructNumElements(llvm_Type const* self);

extern "C"
llvm_Type_TypeID llvm_Type_getTypeID(llvm_Type const* self);

extern "C"
llvm_Type* llvm_Type_getVoidTy(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getX86_FP80PtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getX86_FP80Ty(llvm_LLVMContext* ctx);

extern "C"
llvm_PointerType* llvm_Type_getX86_MMXPtrTy(llvm_LLVMContext* ctx);

extern "C"
llvm_Type* llvm_Type_getX86_MMXTy(llvm_LLVMContext* ctx);

extern "C"
int llvm_Type_isAggregateType(llvm_Type const* self);

extern "C"
int llvm_Type_isArrayTy(llvm_Type const* self);

extern "C"
int llvm_Type_isDoubleTy(llvm_Type const* self);

extern "C"
int llvm_Type_isEmptyTy(llvm_Type const* self);

extern "C"
int llvm_Type_isFP128Ty(llvm_Type const* self);

extern "C"
int llvm_Type_isFPOrFPVectorTy(llvm_Type const* self);

extern "C"
int llvm_Type_isFirstClassType(llvm_Type const* self);

extern "C"
int llvm_Type_isFloatTy(llvm_Type const* self);

extern "C"
int llvm_Type_isFloatingPointTy(llvm_Type const* self);

extern "C"
int llvm_Type_isFunctionTy(llvm_Type const* self);

extern "C"
int llvm_Type_isFunctionVarArg(llvm_Type const* self);

extern "C"
int llvm_Type_isHalfTy(llvm_Type const* self);

extern "C"
int llvm_Type_isIntOrIntVectorTy(llvm_Type const* self);

extern "C"
int llvm_Type_isIntegerTy(llvm_Type const* self);

extern "C"
int llvm_Type_isLabelTy(llvm_Type const* self);

extern "C"
int llvm_Type_isMetadataTy(llvm_Type const* self);

extern "C"
int llvm_Type_isPPC_FP128Ty(llvm_Type const* self);

extern "C"
int llvm_Type_isPointerTy(llvm_Type const* self);

extern "C"
int llvm_Type_isPtrOrPtrVectorTy(llvm_Type const* self);

extern "C"
int llvm_Type_isSingleValueType(llvm_Type const* self);

extern "C"
int llvm_Type_isSized(llvm_Type const* self);

extern "C"
int llvm_Type_isStructTy(llvm_Type const* self);

extern "C"
int llvm_Type_isVectorTy(llvm_Type const* self);

extern "C"
int llvm_Type_isVoidTy(llvm_Type const* self);

extern "C"
int llvm_Type_isX86_FP80Ty(llvm_Type const* self);

extern "C"
int llvm_Type_isX86_MMXTy(llvm_Type const* self);

extern "C"
llvm_Value* llvm_Use_get(llvm_Use const* self);

extern "C"
llvm_Use* llvm_Use_getNext(llvm_Use const* self);

extern "C"
unsigned int llvm_Use_getOperandNo(llvm_Use const* self);

extern "C"
llvm_User* llvm_Use_getUser(llvm_Use const* self);

extern "C"
llvm_Use* llvm_Use_initTags(llvm_Use* Start, llvm_Use* Stop);

extern "C"
void llvm_Use_set(llvm_Use* self, llvm_Value* Val);

extern "C"
void llvm_Use_swap(llvm_Use* self, llvm_Use* RHS);

extern "C"
int llvm_User_classof(llvm_Value* V);

extern "C"
void llvm_User_delete(llvm_User* self);

extern "C"
void llvm_User_dropAllReferences(llvm_User* self);

extern "C"
unsigned int llvm_User_getNumOperands(llvm_User const* self);

extern "C"
llvm_Value* llvm_User_getOperand(llvm_User const* self, unsigned int idx);

extern "C"
void llvm_User_replaceUsesOfWith(llvm_User* self, llvm_Value* From, llvm_Value* To);

extern "C"
void llvm_User_setOperand(llvm_User* self, unsigned int idx, llvm_Value* Val);

extern "C"
void llvm_ValueSymbolTable_delete(llvm_ValueSymbolTable* self);

extern "C"
void llvm_ValueSymbolTable_dump(llvm_ValueSymbolTable const* self);

extern "C"
int llvm_ValueSymbolTable_empty(llvm_ValueSymbolTable const* self);

extern "C"
llvm_Value* llvm_ValueSymbolTable_lookup(llvm_ValueSymbolTable const* self, llvm_StringRef Name);

extern "C"
llvm_ValueSymbolTable* llvm_ValueSymbolTable_new();

extern "C"
unsigned int llvm_ValueSymbolTable_size(llvm_ValueSymbolTable const* self);

extern "C"
void llvm_Value_delete(llvm_Value* self);

extern "C"
void llvm_Value_dump(llvm_Value const* self);

extern "C"
llvm_LLVMContext* llvm_Value_getContext(llvm_Value const* self);

extern "C"
llvm_StringRef llvm_Value_getName(llvm_Value const* self);

extern "C"
unsigned int llvm_Value_getNumUses(llvm_Value const* self);

extern "C"
llvm_Type* llvm_Value_getType(llvm_Value const* self);

extern "C"
unsigned int llvm_Value_getValueID(llvm_Value const* self);

extern "C"
int llvm_Value_hasNUses(llvm_Value const* self, unsigned int N);

extern "C"
int llvm_Value_hasNUsesOrMore(llvm_Value const* self, unsigned int N);

extern "C"
int llvm_Value_hasName(llvm_Value const* self);

extern "C"
int llvm_Value_hasOneUse(llvm_Value const* self);

extern "C"
int llvm_Value_isUsedInBasicBlock(llvm_Value const* self, llvm_BasicBlock const* BB);

extern "C"
void llvm_Value_mutateType(llvm_Value* self, llvm_Type* ty);

extern "C"
void llvm_Value_replaceAllUsesWith(llvm_Value* self, llvm_Value* Value);

extern "C"
void llvm_Value_setName(llvm_Value* self, llvm_StringRef Name);

extern "C"
void llvm_Value_takeName(llvm_Value* self, llvm_Value* Value);

extern "C"
int llvm_VectorType_classof(llvm_Type const* ty);

extern "C"
llvm_VectorType* llvm_VectorType_get(llvm_Type* ty, unsigned int NumElements);

extern "C"
unsigned int llvm_VectorType_getBitWidth(llvm_VectorType const* self);

extern "C"
llvm_VectorType* llvm_VectorType_getDoubleElementsVectorType(llvm_VectorType* ty);

extern "C"
llvm_VectorType* llvm_VectorType_getExtendedElementVectorType(llvm_VectorType* ty);

extern "C"
llvm_VectorType* llvm_VectorType_getHalfElementsVectorType(llvm_VectorType* ty);

extern "C"
llvm_VectorType* llvm_VectorType_getInteger(llvm_VectorType* ty);

extern "C"
unsigned int llvm_VectorType_getNumElements(llvm_VectorType const* self);

extern "C"
llvm_VectorType* llvm_VectorType_getTruncatedElementVectorType(llvm_VectorType* ty);

extern "C"
int llvm_VectorType_isValidElementType(llvm_Type* ty);

extern "C"
void llvm_iplist_llvm_Argument_clear(llvm_iplist_llvm_Argument* self);

extern "C"
void llvm_iplist_llvm_Argument_delete(llvm_iplist_llvm_Argument* self);

extern "C"
llvm_Argument const* llvm_iplist_llvm_Argument_first(llvm_iplist_llvm_Argument const* self);

extern "C"
llvm_Argument* llvm_iplist_llvm_Argument_firstMut(llvm_iplist_llvm_Argument* self);

extern "C"
llvm_Argument const* llvm_iplist_llvm_Argument_last(llvm_iplist_llvm_Argument const* self);

extern "C"
llvm_Argument* llvm_iplist_llvm_Argument_lastMut(llvm_iplist_llvm_Argument* self);

extern "C"
size_t llvm_iplist_llvm_Argument_max_size(llvm_iplist_llvm_Argument const* self);

extern "C"
llvm_iplist_llvm_Argument* llvm_iplist_llvm_Argument_new();

extern "C"
size_t llvm_iplist_llvm_Argument_size(llvm_iplist_llvm_Argument const* self);

#endif /* FFI_H_ */
