#![allow(non_camel_case_types, non_snake_case, unstable)]

#[repr(C)]
#[derive(Copy)]
pub struct llvm_AddrSpaceCastInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_AllocaInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Argument;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ArrayType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_AtomicCmpXchgInst;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_AtomicOrdering {
    NotAtomic = 0,
    Unordered = 1,
    Monotonic = 2,
    Acquire = 4,
    Release = 5,
    AcquireRelease = 6,
    SequentiallyConsistent = 7,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_AtomicRMWInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BasicBlock;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BasicBlockPass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BinaryOperator;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Instruction_BinaryOps {
    Add = 8,
    FAdd = 9,
    Sub = 10,
    FSub = 11,
    Mul = 12,
    FMul = 13,
    UDiv = 14,
    SDiv = 15,
    FDiv = 16,
    URem = 17,
    SRem = 18,
    FRem = 19,
    Shl = 20,
    LShr = 21,
    AShr = 22,
    And = 23,
    Or = 24,
    Xor = 25,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BitCastInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BlockAddress;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BranchInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CallGraphSCCPass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CallInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CastInst;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Instruction_CastOps {
    Trunc = 33,
    ZExt = 34,
    SExt = 35,
    FPToUI = 36,
    FPToSI = 37,
    UIToFP = 38,
    SIToFP = 39,
    FPTrunc = 40,
    FPExt = 41,
    PtrToInt = 42,
    IntToPtr = 43,
    BitCast = 44,
    AddrSpaceCast = 45,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CmpInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CompositeType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Constant;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantAggregateZero;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantArray;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantDataArray;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantDataSequential;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantDataVector;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantExpr;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantFP;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantInt;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantPointerNull;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantStruct;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ConstantVector;
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct std_string {
    pub data: *mut ::libc::c_char,
    pub length: ::libc::size_t,
}
impl Copy for std_string {}
#[repr(C)]
#[derive(Copy)]
pub enum llvm_GlobalValue_LinkageTypes {
    ExternalLinkage = 0,
    AvailableExternallyLinkage,
    LinkOnceAnyLinkage,
    LinkOnceODRLinkage,
    WeakAnyLinkage,
    WeakODRLinkage,
    AppendingLinkage,
    InternalLinkage,
    PrivateLinkage,
    ExternalWeakLinkage,
    CommonLinkage,
}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef_llvm_Value_ptr {
    pub data: *const *mut llvm_Value,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef_llvm_Value_ptr {}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef__libc_c_uint {
    pub data: *const ::libc::c_uint,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef__libc_c_uint {}
#[repr(C)]
#[derive(Copy)]
pub enum llvm_CmpInst_Predicate {
    FCMP_FALSE = 0,
    FCMP_OEQ = 1,
    FCMP_OGT = 2,
    FCMP_OGE = 3,
    FCMP_OLT = 4,
    FCMP_OLE = 5,
    FCMP_ONE = 6,
    FCMP_ORD = 7,
    FCMP_UNO = 8,
    FCMP_UEQ = 9,
    FCMP_UGT = 10,
    FCMP_UGE = 11,
    FCMP_ULT = 12,
    FCMP_ULE = 13,
    FCMP_UNE = 14,
    FCMP_TRUE = 15,
    BAD_FCMP_PREDICATE = 16,
    ICMP_EQ = 32,
    ICMP_NE = 33,
    ICMP_UGT = 34,
    ICMP_UGE = 35,
    ICMP_ULT = 36,
    ICMP_ULE = 37,
    ICMP_SGT = 38,
    ICMP_SGE = 39,
    ICMP_SLT = 40,
    ICMP_SLE = 41,
    BAD_ICMP_PREDICATE = 42,
}
#[repr(C)]
#[derive(Copy)]
pub enum llvm_SynchronizationScope {
    SingleThread = 0,
    CrossThread = 1,
}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_StringRef {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
impl Copy for llvm_StringRef {}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct std_string_const {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
impl Copy for std_string_const {}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_DataLayout;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_DebugLoc;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ExtractElementInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ExtractValueInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FPExtInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FPToSIInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FenceInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Function;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FunctionPass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FunctionPassManager;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_FunctionType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GetElementPtrInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GlobalAlias;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GlobalObject;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GlobalValue;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GlobalVariable;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_CallingConv_ID {
    C = 0,
    Fast = 8,
    Cold = 9,
    GHC = 10,
    HiPE = 11,
    WebKit_JS = 12,
    AnyReg = 13,
    PreserveMost = 14,
    PreserveAll = 15,
    X86_StdCall = 64,
    X86_FastCall = 65,
    ARM_APCS = 66,
    ARM_AAPCS = 67,
    ARM_AAPCS_VFP = 68,
    MSP430_INTR = 69,
    X86_ThisCall = 70,
    PTX_Kernel = 71,
    PTX_Device = 72,
    SPIR_FUNC = 75,
    SPIR_KERNEL = 76,
    Intel_OCL_BI = 77,
    X86_64_SysV = 78,
    X86_64_Win64 = 79,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_IRBuilder;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_IRBuilderBase;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_IndirectBrInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_InlineAsm;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_InsertElementInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_InsertValueInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Instruction;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_IntegerType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_InvokeInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_LLVMContext;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_LandingPadInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_LoadInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_LoopPass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_MDNode;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_MDString;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Instruction_MemoryOps {
    Alloca = 26,
    Load = 27,
    Store = 28,
    GetElementPtr = 29,
    Fence = 30,
    AtomicCmpXchg = 31,
    AtomicRMW = 32,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Module;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ModulePass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Operator;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Instruction_OtherOps {
    ICmp = 46,
    FCmp = 47,
    PHI = 48,
    Call = 49,
    Select = 50,
    UserOp1 = 51,
    UserOp2 = 52,
    VAArg = 53,
    ExtractElement = 54,
    InsertElement = 55,
    ShuffleVector = 56,
    ExtractValue = 57,
    InsertValue = 58,
    LandingPad = 59,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_PHINode;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Pass;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_PassKind {
    PT_BasicBlock = 0,
    PT_Region = 1,
    PT_Loop = 2,
    PT_Function = 3,
    PT_CallGraphSCC = 4,
    PT_Module = 5,
    PT_PassManager = 6,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_PassManager;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_PassManagerType {
    PMT_Unknown = 0,
    PMT_ModulePassManager = 1,
    PMT_CallGraphPassManager = 2,
    PMT_FunctionPassManager = 3,
    PMT_LoopPassManager = 4,
    PMT_RegionPassManager = 5,
    PMT_BasicBlockPassManager = 6,
    PMT_Last = 7,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_PointerType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_RegionPass;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ResumeInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ReturnInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_SelectInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_SequentialType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ShuffleVectorInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_StoreInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_StructType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_SwitchInst;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Instruction_TermOps {
    Ret = 1,
    Br = 2,
    Switch = 3,
    IndirectBr = 4,
    Invoke = 5,
    Resume = 6,
    Unreachable = 7,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_TerminatorInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Type;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Type_TypeID {
    VoidTyID = 0,
    HalfTyID,
    FloatTyID,
    DoubleTyID,
    X86_FP80TyID,
    FP128TyID,
    PPC_FP128TyID,
    LabelTyID,
    MetadataTyID,
    X86_MMXTyID,
    IntegerTyID,
    FunctionTyID,
    StructTyID,
    ArrayTyID,
    PointerTyID,
    VectorTyID,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_UnaryInstruction;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_UndefValue;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_UnreachableInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Use;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_User;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_VAArgInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Value;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ValueSymbolTable;
#[repr(C)]
#[derive(Copy)]
pub enum llvm_Value_ValueTy {
    ArgumentVal,
    BasicBlockVal,
    FunctionVal,
    GlobalAliasVal,
    GlobalVariableVal,
    UndefValueVal,
    BlockAddressVal,
    ConstantExprVal,
    ConstantAggregateZeroVal,
    ConstantDataArrayVal,
    ConstantDataVectorVal,
    ConstantIntVal,
    ConstantFPVal,
    ConstantArrayVal,
    ConstantStructVal,
    ConstantVectorVal,
    ConstantPointerNullVal,
    MetadataAsValueVal,
    InlineAsmVal,
    InstructionVal,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_VectorType;
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef_llvm_Type_ptr {
    pub data: *const *mut llvm_Type,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef_llvm_Type_ptr {}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_APInt {
    pub num_bits: ::libc::c_uint,
    pub value: llvm_ArrayRef__libc_uint64_t,
}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef_llvm_Constant_ptr {
    pub data: *const *mut llvm_Constant,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef_llvm_Constant_ptr {}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef__libc_uint64_t {
    pub data: *const ::libc::uint64_t,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef__libc_uint64_t {}

mod raw {
    extern "C" {
        pub fn llvm_IRBuilderBase_ClearInsertionPoint(inst: *mut super::llvm_IRBuilderBase) -> ::libc::c_void;
        pub fn llvm_BasicBlock_Create(Context: *mut super::llvm_LLVMContext, Name: super::std_string, Parent: *mut super::llvm_Function, InsertBefore: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_Function_Create(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes, Name: super::std_string, Module: *mut super::llvm_Module) -> *mut super::llvm_Function;
        pub fn llvm_IRBuilder_CreateAShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAggregateRet(inst: *mut super::llvm_IRBuilder, retVals: *const *mut super::llvm_Value, N: ::libc::c_uint) -> *mut super::llvm_ReturnInst;
        pub fn llvm_IRBuilder_CreateAlignedLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: ::libc::c_int, Name: super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: ::libc::c_int) -> *mut super::llvm_StoreInst;
        pub fn llvm_IRBuilder_CreateAlloca(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, ArraySize: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_AllocaInst;
        pub fn llvm_IRBuilder_CreateAnd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAndByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBinOp(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_BinaryOps, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBr(inst: *mut super::llvm_IRBuilder, Dest: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst;
        pub fn llvm_IRBuilder_CreateCall(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, Args: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilder_CreateCast(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_CastOps, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateCondBr(inst: *mut super::llvm_IRBuilder, Cond: *mut super::llvm_Value, TrueBlock: *mut super::llvm_BasicBlock, FalseBlock: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst;
        pub fn llvm_IRBuilder_CreateExactSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExactUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractInteger(inst: *mut super::llvm_IRBuilder, DL: *const super::llvm_DataLayout, From: *mut super::llvm_Value, ExtractedTy: *mut super::llvm_IntegerType, Offset: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef__libc_c_uint, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpONE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpORD(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUNO(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPToSI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPToUI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFence(inst: *mut super::llvm_IRBuilder, Ordering: super::llvm_AtomicOrdering, SynchScope: super::llvm_SynchronizationScope, Name: super::std_string) -> *mut super::llvm_FenceInst;
        pub fn llvm_IRBuilder_CreateGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilderBase_CreateGlobalString(inst: *mut super::llvm_IRBuilderBase, Str: super::llvm_StringRef, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateGlobalStringPtr(inst: *mut super::llvm_IRBuilder, Str: super::llvm_StringRef, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInBoundsGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIndirectBr(inst: *mut super::llvm_IRBuilder, Addr: *mut super::llvm_Value, NumCases: ::libc::c_uint) -> *mut super::llvm_IndirectBrInst;
        pub fn llvm_IRBuilder_CreateInsertElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, NewElt: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInsertValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Value: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef__libc_c_uint, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIntCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, isSigned: ::libc::c_int, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIntToPtr(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInvoke(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, NormalDest: *mut super::llvm_BasicBlock, UnwindDest: *mut super::llvm_BasicBlock, Args: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string_const) -> *mut super::llvm_InvokeInst;
        pub fn llvm_IRBuilder_CreateIsNotNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIsNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLandingPad(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, PersFn: *mut super::llvm_Value, NumClauses: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_LandingPadInst;
        pub fn llvm_IRBuilderBase_CreateLifetimeEnd(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateLifetimeStart(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilder_CreateLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, isVolatile: ::libc::c_int, Name: super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilderBase_CreateMemCpy(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: ::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateMemMove(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: ::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateMemSet(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Value: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: ::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilder_CreateMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNot(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateOr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateOrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePHI(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, NumReservedValues: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_PHINode;
        pub fn llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePointerCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePtrDiff(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePtrToInt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateResume(inst: *mut super::llvm_IRBuilder, Exn: *mut super::llvm_Value) -> *mut super::llvm_ResumeInst;
        pub fn llvm_IRBuilder_CreateRet(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value) -> *mut super::llvm_ReturnInst;
        pub fn llvm_IRBuilder_CreateRetVoid(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_ReturnInst;
        pub fn llvm_IRBuilder_CreateSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSelect(inst: *mut super::llvm_IRBuilder, C: *mut super::llvm_Value, TrueValue: *mut super::llvm_Value, FalseValue: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShl(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShlByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShuffleVector(inst: *mut super::llvm_IRBuilder, V1: *mut super::llvm_Value, P2: *mut super::llvm_Value, Mask: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, isVolatile: ::libc::c_int) -> *mut super::llvm_StoreInst;
        pub fn llvm_IRBuilder_CreateStructGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Index: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSwitch(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Dest: *mut super::llvm_BasicBlock, NumCases: ::libc::c_uint) -> *mut super::llvm_SwitchInst;
        pub fn llvm_IRBuilder_CreateTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateTruncOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateURem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUnreachable(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_UnreachableInst;
        pub fn llvm_IRBuilder_CreateVAArg(inst: *mut super::llvm_IRBuilder, List: *mut super::llvm_Value, Ty: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_VAArgInst;
        pub fn llvm_IRBuilder_CreateVectorSplat(inst: *mut super::llvm_IRBuilder, NumElements: ::libc::c_uint, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateXor(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateXorByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilderBase_GetInsertBlock(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_BasicBlock;
        pub fn llvm_IRBuilderBase_SetCurrentDebugLocation(inst: *mut super::llvm_IRBuilderBase, Loc: *const super::llvm_DebugLoc) -> ::libc::c_void;
        pub fn llvm_IRBuilderBase_SetDefaultFPMathTag(inst: *mut super::llvm_IRBuilderBase, FPMathTag: *mut super::llvm_MDNode) -> ::libc::c_void;
        pub fn llvm_IRBuilderBase_SetInsertPoint(inst: *mut super::llvm_IRBuilderBase, BB: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_IRBuilderBase_SetInsertPointAtInst(inst: *mut super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_IRBuilderBase_SetInstDebugLocation(inst: *const super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_FunctionPassManager_add(inst: *mut super::llvm_FunctionPassManager, Pass: *mut super::llvm_FunctionPass) -> ::libc::c_void;
        pub fn llvm_PassManager_add(inst: *mut super::llvm_PassManager, Pass: *mut super::llvm_Pass) -> ::libc::c_void;
        pub fn llvm_Function_addFnAttr(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Function_addFnAttrWithValue(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef, Val: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_appendModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Constant_canTrap(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Function_cannotDuplicate(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_ArrayType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_BasicBlock_classof(Val: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_CompositeType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_classof(V: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_ConstantArray_classof(V: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_ConstantFP_classof(V: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_ConstantInt_classof(Val: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_ConstantPointerNull_classof(Val: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_Function_classof(Val: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_FunctionType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_IntegerType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_PointerType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_SequentialType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_User_classof(V: *mut super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_VectorType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Function_clearGC(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Instruction_clone(inst: *const super::llvm_Instruction) -> *mut super::llvm_Instruction;
        pub fn llvm_Function_copyAttributesFrom(inst: *mut super::llvm_Function, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalValue_copyAttributesFrom(inst: *mut super::llvm_GlobalValue, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_copyAttributesFrom(inst: *mut super::llvm_GlobalVariable, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_Instruction_copyFastMathFlags(inst: *mut super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_StructType_create(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_createAddDiscriminatorsPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAddressSanitizerFunctionPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAddressSanitizerModulePass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createAggressiveDCEPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAlwaysInlinerPass(InsertLifetime: ::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createArgumentPromotionPass(maxElements: ::libc::c_uint) -> *mut super::llvm_Pass;
        pub fn llvm_createBarrierNoopPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createBlockExtractorPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createBoundsCheckingPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createBreakCriticalEdgesPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createCFGSimplificationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createConstantHoistingPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createConstantMergePass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createConstantPropagationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createCorrelatedValuePropagationPass() -> *mut super::llvm_Pass;
        pub fn llvm_createDataFlowSanitizerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createDeadArgEliminationPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createDeadArgHackingPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createDeadCodeEliminationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createDeadInstEliminationPass() -> *mut super::llvm_Pass;
        pub fn llvm_createDeadStoreEliminationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createDebugIRPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createDemoteRegisterToMemoryPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createEarlyCSEPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createFlattenCFGPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createFunctionAttrsPass() -> *mut super::llvm_Pass;
        pub fn llvm_createFunctionInliningPass() -> *mut super::llvm_Pass;
        pub fn llvm_createGCOVProfilerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createGVNPass(NoLoads: ::libc::c_int) -> *mut super::llvm_FunctionPass;
        pub fn llvm_createGlobalDCEPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createGlobalMergePass() -> *mut super::llvm_Pass;
        pub fn llvm_createGlobalOptimizerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createIPConstantPropagationPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createIPSCCPPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createIndVarSimplifyPass() -> *mut super::llvm_Pass;
        pub fn llvm_createInstructionCombiningPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createInstructionNamerPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createInstructionSimplifierPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createInternalizePass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createJumpThreadingPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createLCSSAPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLICMPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoadCombinePass() -> *mut super::llvm_BasicBlockPass;
        pub fn llvm_createLoopDeletionPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopExtractorPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopIdiomPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopInstSimplifyPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopRerollPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopRotatePass(MaxHeaderSize: ::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createLoopSimplifyPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopStrengthReducePass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopUnrollPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopUnswitchPass(OptimizeForSize: ::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createLowerAtomicPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLowerExpectIntrinsicPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createLowerInvokePass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createLowerSwitchPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMemCpyOptPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMemorySanitizerPass(TrackOrigins: ::libc::c_int) -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMergeFunctionsPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createMergedLoadStoreMotionPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMetaRenamerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createObjCARCAPElimPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCContractPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCExpandPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCOptPass() -> *mut super::llvm_Pass;
        pub fn llvm_StructType_createPacked(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef, isPacked: ::libc::c_int) -> *mut super::llvm_StructType;
        pub fn llvm_createPartialInliningPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createPartiallyInlineLibCallsPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createPromoteMemoryToRegisterPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createPruneEHPass() -> *mut super::llvm_Pass;
        pub fn llvm_createReassociatePass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSCCPPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSROAPass(RequiresDomTree: ::libc::c_int) -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSampleProfileLoaderPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createScalarReplAggregatesPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createScalarizerPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSeparateConstOffsetFromGEPPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSimpleLoopUnrollPass() -> *mut super::llvm_Pass;
        pub fn llvm_createSingleLoopExtractorPass() -> *mut super::llvm_Pass;
        pub fn llvm_createSinkingPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createStripDeadDebugInfoPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createStripDeadPrototypesPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createStripDebugDeclarePass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createStripNonDebugSymbolsPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createStripSymbolsPass(OnlyDebugInfo: ::libc::c_int) -> *mut super::llvm_ModulePass;
        pub fn llvm_createStructurizeCFGPass() -> *mut super::llvm_Pass;
        pub fn llvm_createTailCallEliminationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createThreadSanitizerPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_BasicBlock_delete(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_Function_delete(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_delete(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_delete(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
        pub fn llvm_IRBuilder_delete(inst: *mut super::llvm_IRBuilder) -> ::libc::c_void;
        pub fn llvm_Instruction_delete(inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_LLVMContext_delete() -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_delete(inst: *mut super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Pass_delete(inst: *mut super::llvm_Pass) -> ::libc::c_void;
        pub fn llvm_User_delete(inst: *mut super::llvm_User) -> ::libc::c_void;
        pub fn llvm_Value_delete(inst: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ValueSymbolTable_delete() -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_Function_deleteBody(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_BlockAddress_destroyConstant(inst: *mut super::llvm_BlockAddress) -> ::libc::c_void;
        pub fn llvm_Constant_destroyConstant(inst: *mut super::llvm_Constant) -> ::libc::c_void;
        pub fn llvm_ConstantPointerNull_destroyConstant(inst: *mut super::llvm_ConstantPointerNull) -> ::libc::c_void;
        pub fn llvm_GlobalValue_destroyConstant(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_FunctionPassManager_doFinalization(inst: *mut super::llvm_FunctionPassManager) -> ::libc::c_int;
        pub fn llvm_Pass_doFinalization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> ::libc::c_int;
        pub fn llvm_FunctionPassManager_doInitialization(inst: *mut super::llvm_FunctionPassManager) -> ::libc::c_int;
        pub fn llvm_Pass_doInitialization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> ::libc::c_int;
        pub fn llvm_Function_doesNotAccessMemory(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_doesNotAccessMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotAlias(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotCapture(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotReturn(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_doesNotThrow(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_BasicBlock_dropAllReferences(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_User_dropAllReferences(inst: *mut super::llvm_User) -> ::libc::c_void;
        pub fn llvm_Instruction_dropUnknownMetadata(inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_Instruction_dropUnknownMetadataFromIDS(inst: *mut super::llvm_Instruction, KnownIDs: super::llvm_ArrayRef__libc_c_uint) -> ::libc::c_void;
        pub fn llvm_DebugLoc_dump(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> ::libc::c_void;
        pub fn llvm_Module_dump(inst: *const super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Pass_dump(inst: *const super::llvm_Pass) -> ::libc::c_void;
        pub fn llvm_Type_dump(inst: *const super::llvm_Type) -> ::libc::c_void;
        pub fn llvm_Value_dump(inst: *const super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ValueSymbolTable_dump(inst: *const super::llvm_ValueSymbolTable) -> ::libc::c_void;
        pub fn llvm_ValueSymbolTable_empty(inst: *const super::llvm_ValueSymbolTable) -> ::libc::c_int;
        pub fn llvm_ConstantInt_equalsInt(inst: *const super::llvm_ConstantInt, Val: ::libc::uint64_t) -> ::libc::c_int;
        pub fn llvm_BasicBlock_eraseFromParent(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_Function_eraseFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_eraseFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_eraseFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
        pub fn llvm_Instruction_eraseFromParent(inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_ConstantInt_fromAPInt(Context: *mut super::llvm_LLVMContext, Val: super::llvm_APInt) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantFP_fromStr(Ty: *mut super::llvm_Type, Val: super::llvm_StringRef) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_fromStr(Ty: *mut super::llvm_IntegerType, Str: super::llvm_StringRef, radix: ::libc::uint8_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ArrayType_get(ElementType: *mut super::llvm_Type, NumElements: ::libc::uint64_t) -> *mut super::llvm_ArrayType;
        pub fn llvm_ConstantArray_get(Ty: *mut super::llvm_ArrayType, Values: super::llvm_ArrayRef_llvm_Constant_ptr) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_get(Ty: *mut super::llvm_Type, Val: ::libc::c_double) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_get(Ty: *mut super::llvm_IntegerType, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantPointerNull_get(Ty: *mut super::llvm_PointerType) -> *mut super::llvm_ConstantPointerNull;
        pub fn llvm_FunctionType_get(Result: *mut super::llvm_Type, Params: super::llvm_ArrayRef_llvm_Type_ptr, isVarArg: ::libc::c_int) -> *mut super::llvm_FunctionType;
        pub fn llvm_IntegerType_get(ctx: *mut super::llvm_LLVMContext, NumBits: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_PointerType_get(ElementType: *mut super::llvm_Type, AddressSpace: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Use_get(inst: *const super::llvm_Use) -> *mut super::llvm_Value;
        pub fn llvm_VectorType_get(ty: *mut super::llvm_Type, NumElements: ::libc::c_uint) -> *mut super::llvm_VectorType;
        pub fn llvm_PointerType_getAddressSpace(inst: *const super::llvm_PointerType) -> ::libc::c_uint;
        pub fn llvm_Constant_getAggregateElement(inst: *const super::llvm_Constant, Elt: ::libc::c_uint) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getAggregateElementConstant(inst: *const super::llvm_Constant, Elt: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_GlobalValue_getAlignment(inst: *const super::llvm_GlobalValue) -> ::libc::c_uint;
        pub fn llvm_Constant_getAllOnesValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_DebugLoc_getAsMDNode(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode;
        pub fn llvm_BlockAddress_getBasicBlock(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_BasicBlock;
        pub fn llvm_IntegerType_getBitMask(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_ConstantInt_getBitWidth(inst: *const super::llvm_ConstantInt) -> ::libc::c_uint;
        pub fn llvm_IntegerType_getBitWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getBitWidth(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_Function_getCallingConv(inst: *const super::llvm_Function) -> super::llvm_CallingConv_ID;
        pub fn llvm_DebugLoc_getCol(inst: *const super::llvm_DebugLoc) -> ::libc::c_uint;
        pub fn llvm_Type_getContainedType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Function_getContext(inst: *const super::llvm_Function) -> *mut super::llvm_LLVMContext;
        pub fn llvm_IRBuilderBase_getContext(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_getContext(inst: *const super::llvm_Module) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Type_getContext(inst: *const super::llvm_Type) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Value_getContext(inst: *const super::llvm_Value) -> *mut super::llvm_LLVMContext;
        pub fn llvm_IRBuilderBase_getCurrentFunctionReturnType(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_BasicBlock_getDataLayout(inst: *const super::llvm_BasicBlock) -> *const super::llvm_DataLayout;
        pub fn llvm_GlobalValue_getDataLayout(inst: *const super::llvm_GlobalValue) -> *const super::llvm_DataLayout;
        pub fn llvm_Instruction_getDataLayout(inst: *const super::llvm_Instruction) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayout(inst: *const super::llvm_Module) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayoutStr(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Instruction_getDebugLoc(inst: *const super::llvm_Instruction) -> *const super::llvm_DebugLoc;
        pub fn llvm_IRBuilderBase_getDefaultFPMathTag(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_MDNode;
        pub fn llvm_Function_getDereferenceableBytes(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::uint64_t;
        pub fn llvm_VectorType_getDoubleElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getDoublePtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getDoubleTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_Type_getDoubleTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_SequentialType_getElementType(inst: *const super::llvm_SequentialType) -> *mut super::llvm_Type;
        pub fn llvm_StructType_getElementType(inst: *const super::llvm_StructType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_VectorType_getExtendedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getFP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getFalse(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_IRBuilderBase_getFalse(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getFalseWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_BasicBlock_getFirstNonPHI(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbg(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_Type_getFloatPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getFloatTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFloatTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_BlockAddress_getFunction(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_Function;
        pub fn llvm_Module_getFunction(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_Function;
        pub fn llvm_Type_getFunctionNumParams(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getFunctionParamType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Function_getFunctionType(inst: *const super::llvm_Function) -> *mut super::llvm_FunctionType;
        pub fn llvm_getGlobalContext() -> *mut super::llvm_LLVMContext;
        pub fn llvm_VectorType_getHalfElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getHalfPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getHalfTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_Type_getHalfTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantFP_getInfinity(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_GlobalVariable_getInitializer(inst: *const super::llvm_GlobalVariable) -> *const super::llvm_Constant;
        pub fn llvm_GlobalVariable_getInitializerMut(inst: *mut super::llvm_GlobalVariable) -> *mut super::llvm_Constant;
        pub fn llvm_DebugLoc_getInlinedAt(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode;
        pub fn llvm_IRBuilderBase_getInt(inst: *mut super::llvm_IRBuilderBase, Value: super::llvm_APInt) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt1(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt16(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint16_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Type_getInt16PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt16Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt16Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt1PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt1Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt1Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt32(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint32_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Type_getInt32PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt32Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt32Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt64(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Type_getInt64PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt64Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt64Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt8(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint8_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt8PtrTy(inst: *mut super::llvm_IRBuilderBase, AddrSpace: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt8PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt8Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt8Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getIntN(inst: *mut super::llvm_IRBuilderBase, NumBits: ::libc::c_uint, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Type_getIntNPtrTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getIntNTy(inst: *mut super::llvm_IRBuilderBase, NumBits: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getIntNTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getIntPtrTy(inst: *mut super::llvm_IRBuilderBase, DL: *const super::llvm_DataLayout, AddrSpace: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_VectorType_getInteger(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Constant_getIntegerValue(Ty: *mut super::llvm_Type, Value: super::llvm_APInt) -> *mut super::llvm_Constant;
        pub fn llvm_Function_getIntrinsicID(inst: *const super::llvm_Function) -> ::libc::c_uint;
        pub fn llvm_Type_getLabelTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_BasicBlock_getLandingPadInst(inst: *const super::llvm_BasicBlock) -> *const super::llvm_LandingPadInst;
        pub fn llvm_BasicBlock_getLandingPadInstMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_LandingPadInst;
        pub fn llvm_DebugLoc_getLine(inst: *const super::llvm_DebugLoc) -> ::libc::c_uint;
        pub fn llvm_Module_getMDKindID(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> ::libc::c_uint;
        pub fn llvm_Instruction_getMetadata(inst: *const super::llvm_Instruction, KindID: ::libc::c_uint) -> *mut super::llvm_MDNode;
        pub fn llvm_Instruction_getMetadataStr(inst: *const super::llvm_Instruction, Kind: super::llvm_StringRef) -> *mut super::llvm_MDNode;
        pub fn llvm_Type_getMetadataTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Module_getModuleIdentifier(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getModuleInlineAsm(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_StructType_getName(inst: *const super::llvm_StructType) -> super::llvm_StringRef;
        pub fn llvm_Value_getName(inst: *const super::llvm_Value) -> super::llvm_StringRef;
        pub fn llvm_Module_getNamedValue(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_GlobalValue;
        pub fn llvm_ConstantFP_getNegativeZero(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Use_getNext(inst: *const super::llvm_Use) -> *mut super::llvm_Use;
        pub fn llvm_Constant_getNullValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getNumContainedTypes(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_ArrayType_getNumElements(inst: *const super::llvm_ArrayType) -> ::libc::uint64_t;
        pub fn llvm_StructType_getNumElements(inst: *const super::llvm_StructType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getNumElements(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_User_getNumOperands(inst: *const super::llvm_User) -> ::libc::c_uint;
        pub fn llvm_FunctionType_getNumParams(inst: *const super::llvm_FunctionType) -> ::libc::c_uint;
        pub fn llvm_Value_getNumUses(inst: *const super::llvm_Value) -> ::libc::c_uint;
        pub fn llvm_Instruction_getOpcode(inst: *const super::llvm_Instruction) -> ::libc::c_uint;
        pub fn llvm_Operator_getOpcode(inst: *const super::llvm_Operator) -> ::libc::c_uint;
        pub fn llvm_User_getOperand(inst: *const super::llvm_User, idx: ::libc::c_uint) -> *mut super::llvm_Value;
        pub fn llvm_Use_getOperandNo(inst: *const super::llvm_Use) -> ::libc::c_uint;
        pub fn llvm_Module_getOrInsertFunction(inst: *mut super::llvm_Module, Name: super::llvm_StringRef, ty: *mut super::llvm_FunctionType) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getPPC_FP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getPPC_FP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Function_getParamAlignment(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::c_uint;
        pub fn llvm_FunctionType_getParamType(inst: *const super::llvm_FunctionType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_BasicBlock_getParent(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Function;
        pub fn llvm_GlobalValue_getParent(inst: *const super::llvm_GlobalValue) -> *const super::llvm_Module;
        pub fn llvm_Instruction_getParent(inst: *const super::llvm_Instruction) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getParentMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Function;
        pub fn llvm_GlobalValue_getParentMut(inst: *mut super::llvm_GlobalValue) -> *mut super::llvm_Module;
        pub fn llvm_Instruction_getParentMut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_BasicBlock;
        pub fn llvm_Pass_getPassKind(inst: *const super::llvm_Pass) -> super::llvm_PassKind;
        pub fn llvm_Type_getPointerAddressSpace(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getPointerElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerTo(inst: *mut super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Function_getReturnType(inst: *const super::llvm_Function) -> *mut super::llvm_Type;
        pub fn llvm_FunctionType_getReturnType(inst: *const super::llvm_FunctionType) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getSExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::int64_t;
        pub fn llvm_DebugLoc_getScope(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode;
        pub fn llvm_DebugLoc_getScopeNode(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode;
        pub fn llvm_Type_getSequentialElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_IntegerType_getSignBit(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_ConstantInt_getSigned(Ty: *mut super::llvm_IntegerType, Value: ::libc::uint64_t, isSigned: ::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_BasicBlock_getSinglePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getSinglePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_Constant_getSplatValue(inst: *const super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getStructElementType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getStructName(inst: *const super::llvm_Type) -> super::llvm_StringRef;
        pub fn llvm_Type_getStructNumElements(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Module_getTargetTriple(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_BasicBlock_getTerminator(inst: *const super::llvm_BasicBlock) -> *const super::llvm_TerminatorInst;
        pub fn llvm_BasicBlock_getTerminatorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_TerminatorInst;
        pub fn llvm_ConstantInt_getTrue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_IRBuilderBase_getTrue(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getTrueWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_VectorType_getTruncatedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_ConstantArray_getType(inst: *const super::llvm_ConstantArray) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getType(inst: *const super::llvm_ConstantInt) -> *mut super::llvm_IntegerType;
        pub fn llvm_ConstantPointerNull_getType(inst: *const super::llvm_ConstantPointerNull) -> *mut super::llvm_PointerType;
        pub fn llvm_GlobalValue_getType(inst: *const super::llvm_GlobalValue) -> *mut super::llvm_PointerType;
        pub fn llvm_Value_getType(inst: *const super::llvm_Value) -> *mut super::llvm_Type;
        pub fn llvm_CompositeType_getTypeAtIndex(inst: *mut super::llvm_CompositeType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Module_getTypeByName(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_Type_getTypeID(inst: *const super::llvm_Type) -> super::llvm_Type_TypeID;
        pub fn llvm_BasicBlock_getUniquePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getUniquePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_PointerType_getUnqual(ElementType: *mut super::llvm_Type) -> *mut super::llvm_PointerType;
        pub fn llvm_Use_getUser(inst: *const super::llvm_Use) -> *mut super::llvm_User;
        pub fn llvm_Value_getValueID(inst: *const super::llvm_Value) -> ::libc::c_uint;
        pub fn llvm_BasicBlock_getValueSymbolTable(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_IRBuilderBase_getVoidTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_Type_getVoidTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_FP80PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_FP80Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_MMXPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_MMXTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getZExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::uint64_t;
        pub fn llvm_ConstantFP_getZeroValueForNegation(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_BasicBlock_hasAddressTaken(inst: *const super::llvm_BasicBlock) -> ::libc::c_int;
        pub fn llvm_Instruction_hasAllowReciprocal(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasAppendingLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasAvailableExternallyLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasCommonLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasDLLExportStorageClass(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasDLLImportStorageClass(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasDefaultVisibility(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_hasDefinitiveInitializer(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasExternalLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasExternalWeakLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Function_hasFnAttr(inst: *const super::llvm_Function, Kind: super::llvm_StringRef) -> ::libc::c_int;
        pub fn llvm_Function_hasGC(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasHiddenVisibility(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_hasInitializer(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasInternalLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasLinkOnceLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasLocalLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Instruction_hasMetadata(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_hasMetadataOtherThanDebugLoc(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Value_hasNUses(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Value_hasNUsesOrMore(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_StructType_hasName(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Value_hasName(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_Instruction_hasNoInfs(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_hasNoNaNs(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_hasNoSignedZeros(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Value_hasOneUse(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasPrivateLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasProtectedVisibility(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasSection(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Function_hasStructRetAttr(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_hasUWTable(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_hasUniqueInitializer(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasUnnamedAddr(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Instruction_hasUnsafeAlgebra(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakAnyLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakODRLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_CompositeType_indexValid(inst: *const super::llvm_CompositeType, idx: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Use_initTags(Start: *mut super::llvm_Use, Stop: *mut super::llvm_Use) -> *mut super::llvm_Use;
        pub fn llvm_Instruction_insertAfter(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_Instruction_insertBefore(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_Type_isAggregateType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_isAllOnesValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Instruction_isArithmeticShift(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Type_isArrayTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Instruction_isAssociative(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_isBinaryOp(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_isCast(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_isCommutative(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_isConstant(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_Constant_isConstantUsed(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Constant_isDLLImportDependent(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_GlobalValue_isDeclaration(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_isDiscardableIfUnused(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Type_isDoubleTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isEmptyTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isExactlyValueFloat(inst: *const super::llvm_ConstantFP, Val: ::libc::c_double) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_isExternallyInitialized(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_Type_isFP128Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFPOrFPVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFirstClassType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFloatTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFloatingPointTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFunctionTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isFunctionVarArg(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isHalfTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Instruction_isIdempotent(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_isIdenticalTo(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_isIdenticalToWhenDefined(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Type_isIntOrIntVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isIntegerTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Function_isIntrinsic(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Type_isLabelTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_BasicBlock_isLandingPad(inst: *const super::llvm_BasicBlock) -> ::libc::c_int;
        pub fn llvm_StructType_isLayoutIdentical(inst: *const super::llvm_StructType, Other: *mut super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_StructType_isLiteral(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Instruction_isLogicalShift(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMaxValue(inst: *const super::llvm_ConstantInt, isSigned: ::libc::c_int) -> ::libc::c_int;
        pub fn llvm_Type_isMetadataTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_isMinSignedValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMinValue(inst: *const super::llvm_ConstantInt, isSigned: ::libc::c_int) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMinusOne(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isNaN(inst: *const super::llvm_ConstantFP) -> ::libc::c_int;
        pub fn llvm_IRBuilder_isNamePreserving(inst: *const super::llvm_IRBuilder) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isNegative(inst: *const super::llvm_ConstantFP) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isNegative(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_Constant_isNegativeZeroValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Instruction_isNilpotent(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Constant_isNullValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isOne(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_StructType_isOpaque(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPPC_FP128Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isPacked(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPointerTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_IntegerType_isPowerOf2ByteWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_int;
        pub fn llvm_Type_isPtrOrPtrVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Instruction_isSameOperationAs(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction, flags: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Instruction_isShift(inst: *mut super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isSignedValueValidForType(Ty: *mut super::llvm_Type, Val: ::libc::int64_t) -> ::libc::c_int;
        pub fn llvm_Type_isSingleValueType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isSized(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isSized(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isStructTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Instruction_isTerminator(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Constant_isThreadDependent(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_GlobalValue_isThreadLocal(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_DebugLoc_isUnknown(inst: *const super::llvm_DebugLoc) -> ::libc::c_int;
        pub fn llvm_Value_isUsedInBasicBlock(inst: *const super::llvm_Value, BB: *const super::llvm_BasicBlock) -> ::libc::c_int;
        pub fn llvm_Instruction_isUsedOutsideOfBlock(inst: *const super::llvm_Instruction, BB: *const super::llvm_BasicBlock) -> ::libc::c_int;
        pub fn llvm_FunctionType_isValidArgumentType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ArrayType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_PointerType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_VectorType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_FunctionType_isValidReturnType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isValueValidForType(Ty: *mut super::llvm_Type, Val: ::libc::uint64_t) -> ::libc::c_int;
        pub fn llvm_Function_isVarArg(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_FunctionType_isVarArg(inst: *const super::llvm_FunctionType) -> ::libc::c_int;
        pub fn llvm_Type_isVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isVoidTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_GlobalValue_isWeakForLinker(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Type_isX86_FP80Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isX86_MMXTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isZero(inst: *const super::llvm_ConstantFP) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isZero(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_Constant_isZeroValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_ValueSymbolTable_lookup(inst: *const super::llvm_ValueSymbolTable, Name: super::llvm_StringRef) -> *mut super::llvm_Value;
        pub fn llvm_GlobalValue_mayBeOverridden(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Instruction_mayHaveSideEffects(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_mayReadFromMemory(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_mayReadOrWriteMemory(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_mayReturn(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_mayThrow(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_Instruction_mayWriteToMemory(inst: *const super::llvm_Instruction) -> ::libc::c_int;
        pub fn llvm_BasicBlock_moveAfter(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_BasicBlock_moveBefore(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_Instruction_moveBefore(inst: *mut super::llvm_Instruction, MovePos: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_Value_mutateType(inst: *mut super::llvm_Value, ty: *mut super::llvm_Type) -> ::libc::c_void;
        pub fn llvm_Function_needsUnwindTableEntry(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_DebugLoc_new() -> *mut super::llvm_DebugLoc;
        pub fn llvm_FunctionPassManager_new(Module: *mut super::llvm_Module) -> *mut super::llvm_FunctionPassManager;
        pub fn llvm_GlobalVariable_new(Ty: *mut super::llvm_Type, isConstant: ::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_IRBuilder_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilder;
        pub fn llvm_IRBuilderBase_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilderBase;
        pub fn llvm_LLVMContext_new() -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_new(ModuleID: super::llvm_StringRef, Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_Module;
        pub fn llvm_PassManager_new() -> *mut super::llvm_PassManager;
        pub fn llvm_ValueSymbolTable_new() -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_GlobalVariable_newWithModule(Module: *mut super::llvm_Module, Ty: *mut super::llvm_Type, isConstant: ::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes, Initializer: *mut super::llvm_Constant) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_IRBuilder_new_in_block(BB: *mut super::llvm_BasicBlock) -> *mut super::llvm_IRBuilder;
        pub fn llvm_Function_onlyReadsMemory(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_onlyReadsMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Constant_removeDeadConstantUsers(inst: *const super::llvm_Constant) -> ::libc::c_void;
        pub fn llvm_BasicBlock_removeFromParent(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_Function_removeFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_removeFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_removeFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
        pub fn llvm_Instruction_removeFromParent(inst: *mut super::llvm_Instruction) -> ::libc::c_void;
        pub fn llvm_BasicBlock_removePredecessor(inst: *mut super::llvm_BasicBlock, Pred: *mut super::llvm_BasicBlock, DontDeleteUselessPHIs: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Value_replaceAllUsesWith(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst: *mut super::llvm_BasicBlock, New: *mut super::llvm_BasicBlock) -> ::libc::c_void;
        pub fn llvm_User_replaceUsesOfWith(inst: *mut super::llvm_User, From: *mut super::llvm_Value, To: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_FunctionPassManager_run(inst: *mut super::llvm_FunctionPassManager, Function: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_PassManager_run(inst: *mut super::llvm_PassManager, Module: *mut super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Use_set(inst: *mut super::llvm_Use, Val: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr) -> ::libc::c_void;
        pub fn llvm_StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr, isPacked: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Function_setCallingConv(inst: *mut super::llvm_Function, CC: super::llvm_CallingConv_ID) -> ::libc::c_void;
        pub fn llvm_Function_setCannotDuplicate(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setConstant(inst: *mut super::llvm_GlobalVariable, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayout(inst: *mut super::llvm_Module, Other: *const super::llvm_DataLayout) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayoutStr(inst: *mut super::llvm_Module, Desc: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Instruction_setDebugLoc(inst: *mut super::llvm_Instruction, Loc: *const super::llvm_DebugLoc) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAccessMemory(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAccessMemoryParam(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAlias(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotCapture(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotReturn(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotThrow(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setExternallyInitialized(inst: *mut super::llvm_GlobalVariable, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Instruction_setHasAllowReciprocal(inst: *mut super::llvm_Instruction, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Instruction_setHasNoInfs(inst: *mut super::llvm_Instruction, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Instruction_setHasNoNaNs(inst: *mut super::llvm_Instruction, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Instruction_setHasNoSignedZeros(inst: *mut super::llvm_Instruction, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Function_setHasUWTable(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Instruction_setHasUnsafeAlgebra(inst: *mut super::llvm_Instruction, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setInitializer(inst: *mut super::llvm_GlobalVariable, InitVal: *mut super::llvm_Constant) -> ::libc::c_void;
        pub fn llvm_Instruction_setMetadata(inst: *mut super::llvm_Instruction, KindID: ::libc::c_uint, Node: *mut super::llvm_MDNode) -> ::libc::c_void;
        pub fn llvm_Instruction_setMetadataStr(inst: *mut super::llvm_Instruction, Kind: super::llvm_StringRef, Node: *mut super::llvm_MDNode) -> ::libc::c_void;
        pub fn llvm_Module_setModuleIdentifier(inst: *mut super::llvm_Module, ID: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_setModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_StructType_setName(inst: *mut super::llvm_StructType, Name: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Value_setName(inst: *mut super::llvm_Value, Name: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Function_setOnlyReadsMemory(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Function_setOnlyReadsMemoryParam(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_User_setOperand(inst: *mut super::llvm_User, idx: ::libc::c_uint, Val: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_GlobalObject_setSection(inst: *mut super::llvm_GlobalObject, S: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_setTargetTriple(inst: *mut super::llvm_Module, Triple: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_GlobalValue_setThreadLocal(inst: *mut super::llvm_GlobalValue, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_GlobalValue_setUnnamedAddr(inst: *mut super::llvm_GlobalValue, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_ValueSymbolTable_size(inst: *const super::llvm_ValueSymbolTable) -> ::libc::c_uint;
        pub fn llvm_Constant_stripPointerCasts(inst: *const super::llvm_Constant) -> *const super::llvm_Constant;
        pub fn llvm_Constant_stripPointerCastsMut(inst: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Use_swap(inst: *mut super::llvm_Use, RHS: *mut super::llvm_Use) -> ::libc::c_void;
        pub fn llvm_Value_takeName(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ConstantInt_uge(inst: *const super::llvm_ConstantInt, Num: ::libc::uint64_t) -> ::libc::c_int;
        pub fn llvm_Instruction_user_back(inst: *const super::llvm_Instruction) -> *const super::llvm_Instruction;
        pub fn llvm_Instruction_user_back_mut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_Instruction;
        pub fn llvm_verifyFunction(Function: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_verifyModule(Module: *const super::llvm_Module) -> ::libc::c_int;
    }
}

pub mod llvm {
    pub use super::raw;

    // ::llvm::ArrayType::classof
    #[inline(always)]
    pub unsafe fn ArrayType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_ArrayType_classof(ty) != 0
    }

    // ::llvm::ArrayType::get
    #[inline(always)]
    pub unsafe fn ArrayType_get(ElementType: *mut super::llvm_Type, NumElements: ::libc::uint64_t) -> *mut super::llvm_ArrayType {
        raw::llvm_ArrayType_get(ElementType, NumElements)
    }

    // ::llvm::ArrayType::getNumElements
    #[inline(always)]
    pub unsafe fn ArrayType_getNumElements(inst: *const super::llvm_ArrayType) -> ::libc::uint64_t {
        raw::llvm_ArrayType_getNumElements(inst)
    }

    // ::llvm::ArrayType::isValidElementType
    #[inline(always)]
    pub unsafe fn ArrayType_isValidElementType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_ArrayType_isValidElementType(ty) != 0
    }

    // ::llvm::BasicBlock::Create
    #[inline(always)]
    pub unsafe fn BasicBlock_Create(Context: *mut super::llvm_LLVMContext, Name: super::std_string, Parent: *mut super::llvm_Function, InsertBefore: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock {
        raw::llvm_BasicBlock_Create(Context, Name, Parent, InsertBefore)
    }

    // ::llvm::BasicBlock::classof
    #[inline(always)]
    pub unsafe fn BasicBlock_classof(Val: *const super::llvm_Value) -> bool {
        raw::llvm_BasicBlock_classof(Val) != 0
    }

    // ::llvm::BasicBlock::delete
    #[inline(always)]
    pub unsafe fn BasicBlock_delete(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_delete(inst)
    }

    // ::llvm::BasicBlock::dropAllReferences
    #[inline(always)]
    pub unsafe fn BasicBlock_dropAllReferences(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_dropAllReferences(inst)
    }

    // ::llvm::BasicBlock::eraseFromParent
    #[inline(always)]
    pub unsafe fn BasicBlock_eraseFromParent(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_eraseFromParent(inst)
    }

    // ::llvm::BasicBlock::getDataLayout
    #[inline(always)]
    pub unsafe fn BasicBlock_getDataLayout(inst: *const super::llvm_BasicBlock) -> *const super::llvm_DataLayout {
        raw::llvm_BasicBlock_getDataLayout(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHI
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHI(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHI(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHIMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHIMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHIMut(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHIOrDbg
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHIOrDbg(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbg(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHIOrDbgMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHIOrDbgMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHIOrDbgOrLifetime
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst)
    }

    // ::llvm::BasicBlock::getFirstNonPHIOrDbgOrLifetimeMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst)
    }

    // ::llvm::BasicBlock::getLandingPadInst
    #[inline(always)]
    pub unsafe fn BasicBlock_getLandingPadInst(inst: *const super::llvm_BasicBlock) -> *const super::llvm_LandingPadInst {
        raw::llvm_BasicBlock_getLandingPadInst(inst)
    }

    // ::llvm::BasicBlock::getLandingPadInstMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getLandingPadInstMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_LandingPadInst {
        raw::llvm_BasicBlock_getLandingPadInstMut(inst)
    }

    // ::llvm::BasicBlock::getParent
    #[inline(always)]
    pub unsafe fn BasicBlock_getParent(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Function {
        raw::llvm_BasicBlock_getParent(inst)
    }

    // ::llvm::BasicBlock::getParentMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getParentMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Function {
        raw::llvm_BasicBlock_getParentMut(inst)
    }

    // ::llvm::BasicBlock::getSinglePredecessor
    #[inline(always)]
    pub unsafe fn BasicBlock_getSinglePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock {
        raw::llvm_BasicBlock_getSinglePredecessor(inst)
    }

    // ::llvm::BasicBlock::getSinglePredecessorMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getSinglePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock {
        raw::llvm_BasicBlock_getSinglePredecessorMut(inst)
    }

    // ::llvm::BasicBlock::getTerminator
    #[inline(always)]
    pub unsafe fn BasicBlock_getTerminator(inst: *const super::llvm_BasicBlock) -> *const super::llvm_TerminatorInst {
        raw::llvm_BasicBlock_getTerminator(inst)
    }

    // ::llvm::BasicBlock::getTerminatorMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getTerminatorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_TerminatorInst {
        raw::llvm_BasicBlock_getTerminatorMut(inst)
    }

    // ::llvm::BasicBlock::getUniquePredecessor
    #[inline(always)]
    pub unsafe fn BasicBlock_getUniquePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock {
        raw::llvm_BasicBlock_getUniquePredecessor(inst)
    }

    // ::llvm::BasicBlock::getUniquePredecessorMut
    #[inline(always)]
    pub unsafe fn BasicBlock_getUniquePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock {
        raw::llvm_BasicBlock_getUniquePredecessorMut(inst)
    }

    // ::llvm::BasicBlock::getValueSymbolTable
    #[inline(always)]
    pub unsafe fn BasicBlock_getValueSymbolTable(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_ValueSymbolTable {
        raw::llvm_BasicBlock_getValueSymbolTable(inst)
    }

    // ::llvm::BasicBlock::hasAddressTaken
    #[inline(always)]
    pub unsafe fn BasicBlock_hasAddressTaken(inst: *const super::llvm_BasicBlock) -> bool {
        raw::llvm_BasicBlock_hasAddressTaken(inst) != 0
    }

    // ::llvm::BasicBlock::isLandingPad
    #[inline(always)]
    pub unsafe fn BasicBlock_isLandingPad(inst: *const super::llvm_BasicBlock) -> bool {
        raw::llvm_BasicBlock_isLandingPad(inst) != 0
    }

    // ::llvm::BasicBlock::moveAfter
    #[inline(always)]
    pub unsafe fn BasicBlock_moveAfter(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_moveAfter(inst, MovePos)
    }

    // ::llvm::BasicBlock::moveBefore
    #[inline(always)]
    pub unsafe fn BasicBlock_moveBefore(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_moveBefore(inst, MovePos)
    }

    // ::llvm::BasicBlock::removeFromParent
    #[inline(always)]
    pub unsafe fn BasicBlock_removeFromParent(inst: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_removeFromParent(inst)
    }

    // ::llvm::BasicBlock::removePredecessor
    #[inline(always)]
    pub unsafe fn BasicBlock_removePredecessor(inst: *mut super::llvm_BasicBlock, Pred: *mut super::llvm_BasicBlock, DontDeleteUselessPHIs: bool) -> ::libc::c_void {
        raw::llvm_BasicBlock_removePredecessor(inst, Pred, if DontDeleteUselessPHIs { 1 } else { 0 })
    }

    // ::llvm::BasicBlock::replaceSuccessorsPhiUsesWith
    #[inline(always)]
    pub unsafe fn BasicBlock_replaceSuccessorsPhiUsesWith(inst: *mut super::llvm_BasicBlock, New: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst, New)
    }

    // ::llvm::BlockAddress::destroyConstant
    #[inline(always)]
    pub unsafe fn BlockAddress_destroyConstant(inst: *mut super::llvm_BlockAddress) -> ::libc::c_void {
        raw::llvm_BlockAddress_destroyConstant(inst)
    }

    // ::llvm::BlockAddress::getBasicBlock
    #[inline(always)]
    pub unsafe fn BlockAddress_getBasicBlock(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_BasicBlock {
        raw::llvm_BlockAddress_getBasicBlock(inst)
    }

    // ::llvm::BlockAddress::getFunction
    #[inline(always)]
    pub unsafe fn BlockAddress_getFunction(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_Function {
        raw::llvm_BlockAddress_getFunction(inst)
    }

    pub mod CallingConv {
        pub use super::raw;
    }

    // ::llvm::CompositeType::classof
    #[inline(always)]
    pub unsafe fn CompositeType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_CompositeType_classof(ty) != 0
    }

    // ::llvm::CompositeType::getTypeAtIndex
    #[inline(always)]
    pub unsafe fn CompositeType_getTypeAtIndex(inst: *mut super::llvm_CompositeType, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_CompositeType_getTypeAtIndex(inst, idx)
    }

    // ::llvm::CompositeType::indexValid
    #[inline(always)]
    pub unsafe fn CompositeType_indexValid(inst: *const super::llvm_CompositeType, idx: ::libc::c_uint) -> bool {
        raw::llvm_CompositeType_indexValid(inst, idx) != 0
    }

    // ::llvm::Constant::canTrap
    #[inline(always)]
    pub unsafe fn Constant_canTrap(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_canTrap(inst) != 0
    }

    // ::llvm::Constant::classof
    #[inline(always)]
    pub unsafe fn Constant_classof(V: *const super::llvm_Value) -> bool {
        raw::llvm_Constant_classof(V) != 0
    }

    // ::llvm::Constant::destroyConstant
    #[inline(always)]
    pub unsafe fn Constant_destroyConstant(inst: *mut super::llvm_Constant) -> ::libc::c_void {
        raw::llvm_Constant_destroyConstant(inst)
    }

    // ::llvm::Constant::getAggregateElement
    #[inline(always)]
    pub unsafe fn Constant_getAggregateElement(inst: *const super::llvm_Constant, Elt: ::libc::c_uint) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getAggregateElement(inst, Elt)
    }

    // ::llvm::Constant::getAggregateElementConstant
    #[inline(always)]
    pub unsafe fn Constant_getAggregateElementConstant(inst: *const super::llvm_Constant, Elt: *mut super::llvm_Constant) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getAggregateElementConstant(inst, Elt)
    }

    // ::llvm::Constant::getAllOnesValue
    #[inline(always)]
    pub unsafe fn Constant_getAllOnesValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getAllOnesValue(Ty)
    }

    // ::llvm::Constant::getIntegerValue
    #[inline(always)]
    pub unsafe fn Constant_getIntegerValue(Ty: *mut super::llvm_Type, Value: super::llvm_APInt) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getIntegerValue(Ty, Value)
    }

    // ::llvm::Constant::getNullValue
    #[inline(always)]
    pub unsafe fn Constant_getNullValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getNullValue(Ty)
    }

    // ::llvm::Constant::getSplatValue
    #[inline(always)]
    pub unsafe fn Constant_getSplatValue(inst: *const super::llvm_Constant) -> *mut super::llvm_Constant {
        raw::llvm_Constant_getSplatValue(inst)
    }

    // ::llvm::Constant::isAllOnesValue
    #[inline(always)]
    pub unsafe fn Constant_isAllOnesValue(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isAllOnesValue(inst) != 0
    }

    // ::llvm::Constant::isConstantUsed
    #[inline(always)]
    pub unsafe fn Constant_isConstantUsed(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isConstantUsed(inst) != 0
    }

    // ::llvm::Constant::isDLLImportDependent
    #[inline(always)]
    pub unsafe fn Constant_isDLLImportDependent(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isDLLImportDependent(inst) != 0
    }

    // ::llvm::Constant::isMinSignedValue
    #[inline(always)]
    pub unsafe fn Constant_isMinSignedValue(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isMinSignedValue(inst) != 0
    }

    // ::llvm::Constant::isNegativeZeroValue
    #[inline(always)]
    pub unsafe fn Constant_isNegativeZeroValue(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isNegativeZeroValue(inst) != 0
    }

    // ::llvm::Constant::isNullValue
    #[inline(always)]
    pub unsafe fn Constant_isNullValue(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isNullValue(inst) != 0
    }

    // ::llvm::Constant::isThreadDependent
    #[inline(always)]
    pub unsafe fn Constant_isThreadDependent(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isThreadDependent(inst) != 0
    }

    // ::llvm::Constant::isZeroValue
    #[inline(always)]
    pub unsafe fn Constant_isZeroValue(inst: *const super::llvm_Constant) -> bool {
        raw::llvm_Constant_isZeroValue(inst) != 0
    }

    // ::llvm::Constant::removeDeadConstantUsers
    #[inline(always)]
    pub unsafe fn Constant_removeDeadConstantUsers(inst: *const super::llvm_Constant) -> ::libc::c_void {
        raw::llvm_Constant_removeDeadConstantUsers(inst)
    }

    // ::llvm::Constant::stripPointerCasts
    #[inline(always)]
    pub unsafe fn Constant_stripPointerCasts(inst: *const super::llvm_Constant) -> *const super::llvm_Constant {
        raw::llvm_Constant_stripPointerCasts(inst)
    }

    // ::llvm::Constant::stripPointerCastsMut
    #[inline(always)]
    pub unsafe fn Constant_stripPointerCastsMut(inst: *mut super::llvm_Constant) -> *mut super::llvm_Constant {
        raw::llvm_Constant_stripPointerCastsMut(inst)
    }

    // ::llvm::ConstantArray::classof
    #[inline(always)]
    pub unsafe fn ConstantArray_classof(V: *const super::llvm_Value) -> bool {
        raw::llvm_ConstantArray_classof(V) != 0
    }

    // ::llvm::ConstantArray::get
    #[inline(always)]
    pub unsafe fn ConstantArray_get(Ty: *mut super::llvm_ArrayType, Values: super::llvm_ArrayRef_llvm_Constant_ptr) -> *mut super::llvm_Constant {
        raw::llvm_ConstantArray_get(Ty, Values)
    }

    // ::llvm::ConstantArray::getType
    #[inline(always)]
    pub unsafe fn ConstantArray_getType(inst: *const super::llvm_ConstantArray) -> *mut super::llvm_Type {
        raw::llvm_ConstantArray_getType(inst)
    }

    // ::llvm::ConstantFP::classof
    #[inline(always)]
    pub unsafe fn ConstantFP_classof(V: *const super::llvm_Value) -> bool {
        raw::llvm_ConstantFP_classof(V) != 0
    }

    // ::llvm::ConstantFP::fromStr
    #[inline(always)]
    pub unsafe fn ConstantFP_fromStr(Ty: *mut super::llvm_Type, Val: super::llvm_StringRef) -> *mut super::llvm_Constant {
        raw::llvm_ConstantFP_fromStr(Ty, Val)
    }

    // ::llvm::ConstantFP::get
    #[inline(always)]
    pub unsafe fn ConstantFP_get(Ty: *mut super::llvm_Type, Val: ::libc::c_double) -> *mut super::llvm_Constant {
        raw::llvm_ConstantFP_get(Ty, Val)
    }

    // ::llvm::ConstantFP::getInfinity
    #[inline(always)]
    pub unsafe fn ConstantFP_getInfinity(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_ConstantFP_getInfinity(Ty)
    }

    // ::llvm::ConstantFP::getNegativeZero
    #[inline(always)]
    pub unsafe fn ConstantFP_getNegativeZero(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_ConstantFP_getNegativeZero(Ty)
    }

    // ::llvm::ConstantFP::getZeroValueForNegation
    #[inline(always)]
    pub unsafe fn ConstantFP_getZeroValueForNegation(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_ConstantFP_getZeroValueForNegation(Ty)
    }

    // ::llvm::ConstantFP::isExactlyValueFloat
    #[inline(always)]
    pub unsafe fn ConstantFP_isExactlyValueFloat(inst: *const super::llvm_ConstantFP, Val: ::libc::c_double) -> bool {
        raw::llvm_ConstantFP_isExactlyValueFloat(inst, Val) != 0
    }

    // ::llvm::ConstantFP::isNaN
    #[inline(always)]
    pub unsafe fn ConstantFP_isNaN(inst: *const super::llvm_ConstantFP) -> bool {
        raw::llvm_ConstantFP_isNaN(inst) != 0
    }

    // ::llvm::ConstantFP::isNegative
    #[inline(always)]
    pub unsafe fn ConstantFP_isNegative(inst: *const super::llvm_ConstantFP) -> bool {
        raw::llvm_ConstantFP_isNegative(inst) != 0
    }

    // ::llvm::ConstantFP::isZero
    #[inline(always)]
    pub unsafe fn ConstantFP_isZero(inst: *const super::llvm_ConstantFP) -> bool {
        raw::llvm_ConstantFP_isZero(inst) != 0
    }

    // ::llvm::ConstantInt::classof
    #[inline(always)]
    pub unsafe fn ConstantInt_classof(Val: *const super::llvm_Value) -> bool {
        raw::llvm_ConstantInt_classof(Val) != 0
    }

    // ::llvm::ConstantInt::equalsInt
    #[inline(always)]
    pub unsafe fn ConstantInt_equalsInt(inst: *const super::llvm_ConstantInt, Val: ::libc::uint64_t) -> bool {
        raw::llvm_ConstantInt_equalsInt(inst, Val) != 0
    }

    // ::llvm::ConstantInt::fromAPInt
    #[inline(always)]
    pub unsafe fn ConstantInt_fromAPInt(Context: *mut super::llvm_LLVMContext, Val: super::llvm_APInt) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_fromAPInt(Context, Val)
    }

    // ::llvm::ConstantInt::fromStr
    #[inline(always)]
    pub unsafe fn ConstantInt_fromStr(Ty: *mut super::llvm_IntegerType, Str: super::llvm_StringRef, radix: ::libc::uint8_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_fromStr(Ty, Str, radix)
    }

    // ::llvm::ConstantInt::get
    #[inline(always)]
    pub unsafe fn ConstantInt_get(Ty: *mut super::llvm_IntegerType, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_get(Ty, Value)
    }

    // ::llvm::ConstantInt::getBitWidth
    #[inline(always)]
    pub unsafe fn ConstantInt_getBitWidth(inst: *const super::llvm_ConstantInt) -> ::libc::c_uint {
        raw::llvm_ConstantInt_getBitWidth(inst)
    }

    // ::llvm::ConstantInt::getFalse
    #[inline(always)]
    pub unsafe fn ConstantInt_getFalse(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_ConstantInt_getFalse(Ty)
    }

    // ::llvm::ConstantInt::getFalseWithContext
    #[inline(always)]
    pub unsafe fn ConstantInt_getFalseWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_getFalseWithContext(Context)
    }

    // ::llvm::ConstantInt::getSExtValue
    #[inline(always)]
    pub unsafe fn ConstantInt_getSExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::int64_t {
        raw::llvm_ConstantInt_getSExtValue(inst)
    }

    // ::llvm::ConstantInt::getSigned
    #[inline(always)]
    pub unsafe fn ConstantInt_getSigned(Ty: *mut super::llvm_IntegerType, Value: ::libc::uint64_t, isSigned: bool) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_getSigned(Ty, Value, if isSigned { 1 } else { 0 })
    }

    // ::llvm::ConstantInt::getTrue
    #[inline(always)]
    pub unsafe fn ConstantInt_getTrue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant {
        raw::llvm_ConstantInt_getTrue(Ty)
    }

    // ::llvm::ConstantInt::getTrueWithContext
    #[inline(always)]
    pub unsafe fn ConstantInt_getTrueWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt {
        raw::llvm_ConstantInt_getTrueWithContext(Context)
    }

    // ::llvm::ConstantInt::getType
    #[inline(always)]
    pub unsafe fn ConstantInt_getType(inst: *const super::llvm_ConstantInt) -> *mut super::llvm_IntegerType {
        raw::llvm_ConstantInt_getType(inst)
    }

    // ::llvm::ConstantInt::getZExtValue
    #[inline(always)]
    pub unsafe fn ConstantInt_getZExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::uint64_t {
        raw::llvm_ConstantInt_getZExtValue(inst)
    }

    // ::llvm::ConstantInt::isMaxValue
    #[inline(always)]
    pub unsafe fn ConstantInt_isMaxValue(inst: *const super::llvm_ConstantInt, isSigned: bool) -> bool {
        raw::llvm_ConstantInt_isMaxValue(inst, if isSigned { 1 } else { 0 }) != 0
    }

    // ::llvm::ConstantInt::isMinValue
    #[inline(always)]
    pub unsafe fn ConstantInt_isMinValue(inst: *const super::llvm_ConstantInt, isSigned: bool) -> bool {
        raw::llvm_ConstantInt_isMinValue(inst, if isSigned { 1 } else { 0 }) != 0
    }

    // ::llvm::ConstantInt::isMinusOne
    #[inline(always)]
    pub unsafe fn ConstantInt_isMinusOne(inst: *const super::llvm_ConstantInt) -> bool {
        raw::llvm_ConstantInt_isMinusOne(inst) != 0
    }

    // ::llvm::ConstantInt::isNegative
    #[inline(always)]
    pub unsafe fn ConstantInt_isNegative(inst: *const super::llvm_ConstantInt) -> bool {
        raw::llvm_ConstantInt_isNegative(inst) != 0
    }

    // ::llvm::ConstantInt::isOne
    #[inline(always)]
    pub unsafe fn ConstantInt_isOne(inst: *const super::llvm_ConstantInt) -> bool {
        raw::llvm_ConstantInt_isOne(inst) != 0
    }

    // ::llvm::ConstantInt::isSignedValueValidForType
    #[inline(always)]
    pub unsafe fn ConstantInt_isSignedValueValidForType(Ty: *mut super::llvm_Type, Val: ::libc::int64_t) -> bool {
        raw::llvm_ConstantInt_isSignedValueValidForType(Ty, Val) != 0
    }

    // ::llvm::ConstantInt::isValueValidForType
    #[inline(always)]
    pub unsafe fn ConstantInt_isValueValidForType(Ty: *mut super::llvm_Type, Val: ::libc::uint64_t) -> bool {
        raw::llvm_ConstantInt_isValueValidForType(Ty, Val) != 0
    }

    // ::llvm::ConstantInt::isZero
    #[inline(always)]
    pub unsafe fn ConstantInt_isZero(inst: *const super::llvm_ConstantInt) -> bool {
        raw::llvm_ConstantInt_isZero(inst) != 0
    }

    // ::llvm::ConstantInt::uge
    #[inline(always)]
    pub unsafe fn ConstantInt_uge(inst: *const super::llvm_ConstantInt, Num: ::libc::uint64_t) -> bool {
        raw::llvm_ConstantInt_uge(inst, Num) != 0
    }

    // ::llvm::ConstantPointerNull::classof
    #[inline(always)]
    pub unsafe fn ConstantPointerNull_classof(Val: *const super::llvm_Value) -> bool {
        raw::llvm_ConstantPointerNull_classof(Val) != 0
    }

    // ::llvm::ConstantPointerNull::destroyConstant
    #[inline(always)]
    pub unsafe fn ConstantPointerNull_destroyConstant(inst: *mut super::llvm_ConstantPointerNull) -> ::libc::c_void {
        raw::llvm_ConstantPointerNull_destroyConstant(inst)
    }

    // ::llvm::ConstantPointerNull::get
    #[inline(always)]
    pub unsafe fn ConstantPointerNull_get(Ty: *mut super::llvm_PointerType) -> *mut super::llvm_ConstantPointerNull {
        raw::llvm_ConstantPointerNull_get(Ty)
    }

    // ::llvm::ConstantPointerNull::getType
    #[inline(always)]
    pub unsafe fn ConstantPointerNull_getType(inst: *const super::llvm_ConstantPointerNull) -> *mut super::llvm_PointerType {
        raw::llvm_ConstantPointerNull_getType(inst)
    }

    // ::llvm::DebugLoc::dump
    #[inline(always)]
    pub unsafe fn DebugLoc_dump(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> ::libc::c_void {
        raw::llvm_DebugLoc_dump(inst, Ctx)
    }

    // ::llvm::DebugLoc::getAsMDNode
    #[inline(always)]
    pub unsafe fn DebugLoc_getAsMDNode(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode {
        raw::llvm_DebugLoc_getAsMDNode(inst, Ctx)
    }

    // ::llvm::DebugLoc::getCol
    #[inline(always)]
    pub unsafe fn DebugLoc_getCol(inst: *const super::llvm_DebugLoc) -> ::libc::c_uint {
        raw::llvm_DebugLoc_getCol(inst)
    }

    // ::llvm::DebugLoc::getInlinedAt
    #[inline(always)]
    pub unsafe fn DebugLoc_getInlinedAt(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode {
        raw::llvm_DebugLoc_getInlinedAt(inst, Ctx)
    }

    // ::llvm::DebugLoc::getLine
    #[inline(always)]
    pub unsafe fn DebugLoc_getLine(inst: *const super::llvm_DebugLoc) -> ::libc::c_uint {
        raw::llvm_DebugLoc_getLine(inst)
    }

    // ::llvm::DebugLoc::getScope
    #[inline(always)]
    pub unsafe fn DebugLoc_getScope(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode {
        raw::llvm_DebugLoc_getScope(inst, Ctx)
    }

    // ::llvm::DebugLoc::getScopeNode
    #[inline(always)]
    pub unsafe fn DebugLoc_getScopeNode(inst: *const super::llvm_DebugLoc, Ctx: *const super::llvm_LLVMContext) -> *mut super::llvm_MDNode {
        raw::llvm_DebugLoc_getScopeNode(inst, Ctx)
    }

    // ::llvm::DebugLoc::isUnknown
    #[inline(always)]
    pub unsafe fn DebugLoc_isUnknown(inst: *const super::llvm_DebugLoc) -> bool {
        raw::llvm_DebugLoc_isUnknown(inst) != 0
    }

    // ::llvm::DebugLoc::new
    #[inline(always)]
    pub unsafe fn DebugLoc_new() -> *mut super::llvm_DebugLoc {
        raw::llvm_DebugLoc_new()
    }

    // ::llvm::Function::Create
    #[inline(always)]
    pub unsafe fn Function_Create(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes, Name: super::std_string, Module: *mut super::llvm_Module) -> *mut super::llvm_Function {
        raw::llvm_Function_Create(Ty, Linkage, Name, Module)
    }

    // ::llvm::Function::addFnAttr
    #[inline(always)]
    pub unsafe fn Function_addFnAttr(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Function_addFnAttr(inst, Kind)
    }

    // ::llvm::Function::addFnAttrWithValue
    #[inline(always)]
    pub unsafe fn Function_addFnAttrWithValue(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef, Val: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Function_addFnAttrWithValue(inst, Kind, Val)
    }

    // ::llvm::Function::cannotDuplicate
    #[inline(always)]
    pub unsafe fn Function_cannotDuplicate(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_cannotDuplicate(inst) != 0
    }

    // ::llvm::Function::classof
    #[inline(always)]
    pub unsafe fn Function_classof(Val: *const super::llvm_Value) -> bool {
        raw::llvm_Function_classof(Val) != 0
    }

    // ::llvm::Function::clearGC
    #[inline(always)]
    pub unsafe fn Function_clearGC(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_clearGC(inst)
    }

    // ::llvm::Function::copyAttributesFrom
    #[inline(always)]
    pub unsafe fn Function_copyAttributesFrom(inst: *mut super::llvm_Function, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_Function_copyAttributesFrom(inst, Src)
    }

    // ::llvm::Function::delete
    #[inline(always)]
    pub unsafe fn Function_delete(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_delete(inst)
    }

    // ::llvm::Function::deleteBody
    #[inline(always)]
    pub unsafe fn Function_deleteBody(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_deleteBody(inst)
    }

    // ::llvm::Function::doesNotAccessMemory
    #[inline(always)]
    pub unsafe fn Function_doesNotAccessMemory(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_doesNotAccessMemory(inst) != 0
    }

    // ::llvm::Function::doesNotAccessMemoryParam
    #[inline(always)]
    pub unsafe fn Function_doesNotAccessMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> bool {
        raw::llvm_Function_doesNotAccessMemoryParam(inst, n) != 0
    }

    // ::llvm::Function::doesNotAlias
    #[inline(always)]
    pub unsafe fn Function_doesNotAlias(inst: *const super::llvm_Function, n: ::libc::c_uint) -> bool {
        raw::llvm_Function_doesNotAlias(inst, n) != 0
    }

    // ::llvm::Function::doesNotCapture
    #[inline(always)]
    pub unsafe fn Function_doesNotCapture(inst: *const super::llvm_Function, n: ::libc::c_uint) -> bool {
        raw::llvm_Function_doesNotCapture(inst, n) != 0
    }

    // ::llvm::Function::doesNotReturn
    #[inline(always)]
    pub unsafe fn Function_doesNotReturn(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_doesNotReturn(inst) != 0
    }

    // ::llvm::Function::doesNotThrow
    #[inline(always)]
    pub unsafe fn Function_doesNotThrow(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_doesNotThrow(inst) != 0
    }

    // ::llvm::Function::eraseFromParent
    #[inline(always)]
    pub unsafe fn Function_eraseFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_eraseFromParent(inst)
    }

    // ::llvm::Function::getCallingConv
    #[inline(always)]
    pub unsafe fn Function_getCallingConv(inst: *const super::llvm_Function) -> super::llvm_CallingConv_ID {
        raw::llvm_Function_getCallingConv(inst)
    }

    // ::llvm::Function::getContext
    #[inline(always)]
    pub unsafe fn Function_getContext(inst: *const super::llvm_Function) -> *mut super::llvm_LLVMContext {
        raw::llvm_Function_getContext(inst)
    }

    // ::llvm::Function::getDereferenceableBytes
    #[inline(always)]
    pub unsafe fn Function_getDereferenceableBytes(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::uint64_t {
        raw::llvm_Function_getDereferenceableBytes(inst, idx)
    }

    // ::llvm::Function::getFunctionType
    #[inline(always)]
    pub unsafe fn Function_getFunctionType(inst: *const super::llvm_Function) -> *mut super::llvm_FunctionType {
        raw::llvm_Function_getFunctionType(inst)
    }

    // ::llvm::Function::getIntrinsicID
    #[inline(always)]
    pub unsafe fn Function_getIntrinsicID(inst: *const super::llvm_Function) -> ::libc::c_uint {
        raw::llvm_Function_getIntrinsicID(inst)
    }

    // ::llvm::Function::getParamAlignment
    #[inline(always)]
    pub unsafe fn Function_getParamAlignment(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::c_uint {
        raw::llvm_Function_getParamAlignment(inst, idx)
    }

    // ::llvm::Function::getReturnType
    #[inline(always)]
    pub unsafe fn Function_getReturnType(inst: *const super::llvm_Function) -> *mut super::llvm_Type {
        raw::llvm_Function_getReturnType(inst)
    }

    // ::llvm::Function::hasFnAttr
    #[inline(always)]
    pub unsafe fn Function_hasFnAttr(inst: *const super::llvm_Function, Kind: super::llvm_StringRef) -> bool {
        raw::llvm_Function_hasFnAttr(inst, Kind) != 0
    }

    // ::llvm::Function::hasGC
    #[inline(always)]
    pub unsafe fn Function_hasGC(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_hasGC(inst) != 0
    }

    // ::llvm::Function::hasStructRetAttr
    #[inline(always)]
    pub unsafe fn Function_hasStructRetAttr(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_hasStructRetAttr(inst) != 0
    }

    // ::llvm::Function::hasUWTable
    #[inline(always)]
    pub unsafe fn Function_hasUWTable(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_hasUWTable(inst) != 0
    }

    // ::llvm::Function::isIntrinsic
    #[inline(always)]
    pub unsafe fn Function_isIntrinsic(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_isIntrinsic(inst) != 0
    }

    // ::llvm::Function::isVarArg
    #[inline(always)]
    pub unsafe fn Function_isVarArg(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_isVarArg(inst) != 0
    }

    // ::llvm::Function::needsUnwindTableEntry
    #[inline(always)]
    pub unsafe fn Function_needsUnwindTableEntry(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_needsUnwindTableEntry(inst) != 0
    }

    // ::llvm::Function::onlyReadsMemory
    #[inline(always)]
    pub unsafe fn Function_onlyReadsMemory(inst: *const super::llvm_Function) -> bool {
        raw::llvm_Function_onlyReadsMemory(inst) != 0
    }

    // ::llvm::Function::onlyReadsMemoryParam
    #[inline(always)]
    pub unsafe fn Function_onlyReadsMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> bool {
        raw::llvm_Function_onlyReadsMemoryParam(inst, n) != 0
    }

    // ::llvm::Function::removeFromParent
    #[inline(always)]
    pub unsafe fn Function_removeFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_removeFromParent(inst)
    }

    // ::llvm::Function::setCallingConv
    #[inline(always)]
    pub unsafe fn Function_setCallingConv(inst: *mut super::llvm_Function, CC: super::llvm_CallingConv_ID) -> ::libc::c_void {
        raw::llvm_Function_setCallingConv(inst, CC)
    }

    // ::llvm::Function::setCannotDuplicate
    #[inline(always)]
    pub unsafe fn Function_setCannotDuplicate(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setCannotDuplicate(inst)
    }

    // ::llvm::Function::setDoesNotAccessMemory
    #[inline(always)]
    pub unsafe fn Function_setDoesNotAccessMemory(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotAccessMemory(inst)
    }

    // ::llvm::Function::setDoesNotAccessMemoryParam
    #[inline(always)]
    pub unsafe fn Function_setDoesNotAccessMemoryParam(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotAccessMemoryParam(inst, n)
    }

    // ::llvm::Function::setDoesNotAlias
    #[inline(always)]
    pub unsafe fn Function_setDoesNotAlias(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotAlias(inst, n)
    }

    // ::llvm::Function::setDoesNotCapture
    #[inline(always)]
    pub unsafe fn Function_setDoesNotCapture(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotCapture(inst, n)
    }

    // ::llvm::Function::setDoesNotReturn
    #[inline(always)]
    pub unsafe fn Function_setDoesNotReturn(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotReturn(inst)
    }

    // ::llvm::Function::setDoesNotThrow
    #[inline(always)]
    pub unsafe fn Function_setDoesNotThrow(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setDoesNotThrow(inst)
    }

    // ::llvm::Function::setHasUWTable
    #[inline(always)]
    pub unsafe fn Function_setHasUWTable(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setHasUWTable(inst)
    }

    // ::llvm::Function::setOnlyReadsMemory
    #[inline(always)]
    pub unsafe fn Function_setOnlyReadsMemory(inst: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_Function_setOnlyReadsMemory(inst)
    }

    // ::llvm::Function::setOnlyReadsMemoryParam
    #[inline(always)]
    pub unsafe fn Function_setOnlyReadsMemoryParam(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void {
        raw::llvm_Function_setOnlyReadsMemoryParam(inst, n)
    }

    // ::llvm::FunctionPassManager::add
    #[inline(always)]
    pub unsafe fn FunctionPassManager_add(inst: *mut super::llvm_FunctionPassManager, Pass: *mut super::llvm_FunctionPass) -> ::libc::c_void {
        raw::llvm_FunctionPassManager_add(inst, Pass)
    }

    // ::llvm::FunctionPassManager::doFinalization
    #[inline(always)]
    pub unsafe fn FunctionPassManager_doFinalization(inst: *mut super::llvm_FunctionPassManager) -> bool {
        raw::llvm_FunctionPassManager_doFinalization(inst) != 0
    }

    // ::llvm::FunctionPassManager::doInitialization
    #[inline(always)]
    pub unsafe fn FunctionPassManager_doInitialization(inst: *mut super::llvm_FunctionPassManager) -> bool {
        raw::llvm_FunctionPassManager_doInitialization(inst) != 0
    }

    // ::llvm::FunctionPassManager::new
    #[inline(always)]
    pub unsafe fn FunctionPassManager_new(Module: *mut super::llvm_Module) -> *mut super::llvm_FunctionPassManager {
        raw::llvm_FunctionPassManager_new(Module)
    }

    // ::llvm::FunctionPassManager::run
    #[inline(always)]
    pub unsafe fn FunctionPassManager_run(inst: *mut super::llvm_FunctionPassManager, Function: *mut super::llvm_Function) -> ::libc::c_void {
        raw::llvm_FunctionPassManager_run(inst, Function)
    }

    // ::llvm::FunctionType::classof
    #[inline(always)]
    pub unsafe fn FunctionType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_FunctionType_classof(ty) != 0
    }

    // ::llvm::FunctionType::get
    #[inline(always)]
    pub unsafe fn FunctionType_get(Result: *mut super::llvm_Type, Params: super::llvm_ArrayRef_llvm_Type_ptr, isVarArg: bool) -> *mut super::llvm_FunctionType {
        raw::llvm_FunctionType_get(Result, Params, if isVarArg { 1 } else { 0 })
    }

    // ::llvm::FunctionType::getNumParams
    #[inline(always)]
    pub unsafe fn FunctionType_getNumParams(inst: *const super::llvm_FunctionType) -> ::libc::c_uint {
        raw::llvm_FunctionType_getNumParams(inst)
    }

    // ::llvm::FunctionType::getParamType
    #[inline(always)]
    pub unsafe fn FunctionType_getParamType(inst: *const super::llvm_FunctionType, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_FunctionType_getParamType(inst, idx)
    }

    // ::llvm::FunctionType::getReturnType
    #[inline(always)]
    pub unsafe fn FunctionType_getReturnType(inst: *const super::llvm_FunctionType) -> *mut super::llvm_Type {
        raw::llvm_FunctionType_getReturnType(inst)
    }

    // ::llvm::FunctionType::isValidArgumentType
    #[inline(always)]
    pub unsafe fn FunctionType_isValidArgumentType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_FunctionType_isValidArgumentType(ty) != 0
    }

    // ::llvm::FunctionType::isValidReturnType
    #[inline(always)]
    pub unsafe fn FunctionType_isValidReturnType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_FunctionType_isValidReturnType(ty) != 0
    }

    // ::llvm::FunctionType::isVarArg
    #[inline(always)]
    pub unsafe fn FunctionType_isVarArg(inst: *const super::llvm_FunctionType) -> bool {
        raw::llvm_FunctionType_isVarArg(inst) != 0
    }

    // ::llvm::GlobalObject::setSection
    #[inline(always)]
    pub unsafe fn GlobalObject_setSection(inst: *mut super::llvm_GlobalObject, S: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_GlobalObject_setSection(inst, S)
    }

    // ::llvm::GlobalValue::copyAttributesFrom
    #[inline(always)]
    pub unsafe fn GlobalValue_copyAttributesFrom(inst: *mut super::llvm_GlobalValue, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalValue_copyAttributesFrom(inst, Src)
    }

    // ::llvm::GlobalValue::delete
    #[inline(always)]
    pub unsafe fn GlobalValue_delete(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalValue_delete(inst)
    }

    // ::llvm::GlobalValue::destroyConstant
    #[inline(always)]
    pub unsafe fn GlobalValue_destroyConstant(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalValue_destroyConstant(inst)
    }

    // ::llvm::GlobalValue::eraseFromParent
    #[inline(always)]
    pub unsafe fn GlobalValue_eraseFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalValue_eraseFromParent(inst)
    }

    // ::llvm::GlobalValue::getAlignment
    #[inline(always)]
    pub unsafe fn GlobalValue_getAlignment(inst: *const super::llvm_GlobalValue) -> ::libc::c_uint {
        raw::llvm_GlobalValue_getAlignment(inst)
    }

    // ::llvm::GlobalValue::getDataLayout
    #[inline(always)]
    pub unsafe fn GlobalValue_getDataLayout(inst: *const super::llvm_GlobalValue) -> *const super::llvm_DataLayout {
        raw::llvm_GlobalValue_getDataLayout(inst)
    }

    // ::llvm::GlobalValue::getParent
    #[inline(always)]
    pub unsafe fn GlobalValue_getParent(inst: *const super::llvm_GlobalValue) -> *const super::llvm_Module {
        raw::llvm_GlobalValue_getParent(inst)
    }

    // ::llvm::GlobalValue::getParentMut
    #[inline(always)]
    pub unsafe fn GlobalValue_getParentMut(inst: *mut super::llvm_GlobalValue) -> *mut super::llvm_Module {
        raw::llvm_GlobalValue_getParentMut(inst)
    }

    // ::llvm::GlobalValue::getType
    #[inline(always)]
    pub unsafe fn GlobalValue_getType(inst: *const super::llvm_GlobalValue) -> *mut super::llvm_PointerType {
        raw::llvm_GlobalValue_getType(inst)
    }

    // ::llvm::GlobalValue::hasAppendingLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasAppendingLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasAppendingLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasAvailableExternallyLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasAvailableExternallyLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasAvailableExternallyLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasCommonLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasCommonLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasCommonLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasDLLExportStorageClass
    #[inline(always)]
    pub unsafe fn GlobalValue_hasDLLExportStorageClass(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasDLLExportStorageClass(inst) != 0
    }

    // ::llvm::GlobalValue::hasDLLImportStorageClass
    #[inline(always)]
    pub unsafe fn GlobalValue_hasDLLImportStorageClass(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasDLLImportStorageClass(inst) != 0
    }

    // ::llvm::GlobalValue::hasDefaultVisibility
    #[inline(always)]
    pub unsafe fn GlobalValue_hasDefaultVisibility(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasDefaultVisibility(inst) != 0
    }

    // ::llvm::GlobalValue::hasExternalLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasExternalLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasExternalLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasExternalWeakLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasExternalWeakLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasExternalWeakLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasHiddenVisibility
    #[inline(always)]
    pub unsafe fn GlobalValue_hasHiddenVisibility(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasHiddenVisibility(inst) != 0
    }

    // ::llvm::GlobalValue::hasInternalLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasInternalLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasInternalLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasLinkOnceLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasLinkOnceLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasLinkOnceLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasLocalLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasLocalLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasLocalLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasPrivateLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasPrivateLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasPrivateLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasProtectedVisibility
    #[inline(always)]
    pub unsafe fn GlobalValue_hasProtectedVisibility(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasProtectedVisibility(inst) != 0
    }

    // ::llvm::GlobalValue::hasSection
    #[inline(always)]
    pub unsafe fn GlobalValue_hasSection(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasSection(inst) != 0
    }

    // ::llvm::GlobalValue::hasUnnamedAddr
    #[inline(always)]
    pub unsafe fn GlobalValue_hasUnnamedAddr(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasUnnamedAddr(inst) != 0
    }

    // ::llvm::GlobalValue::hasWeakAnyLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasWeakAnyLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasWeakAnyLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasWeakLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasWeakLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasWeakLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::hasWeakODRLinkage
    #[inline(always)]
    pub unsafe fn GlobalValue_hasWeakODRLinkage(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_hasWeakODRLinkage(inst) != 0
    }

    // ::llvm::GlobalValue::isDeclaration
    #[inline(always)]
    pub unsafe fn GlobalValue_isDeclaration(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_isDeclaration(inst) != 0
    }

    // ::llvm::GlobalValue::isDiscardableIfUnused
    #[inline(always)]
    pub unsafe fn GlobalValue_isDiscardableIfUnused(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_isDiscardableIfUnused(inst) != 0
    }

    // ::llvm::GlobalValue::isThreadLocal
    #[inline(always)]
    pub unsafe fn GlobalValue_isThreadLocal(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_isThreadLocal(inst) != 0
    }

    // ::llvm::GlobalValue::isWeakForLinker
    #[inline(always)]
    pub unsafe fn GlobalValue_isWeakForLinker(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_isWeakForLinker(inst) != 0
    }

    // ::llvm::GlobalValue::mayBeOverridden
    #[inline(always)]
    pub unsafe fn GlobalValue_mayBeOverridden(inst: *const super::llvm_GlobalValue) -> bool {
        raw::llvm_GlobalValue_mayBeOverridden(inst) != 0
    }

    // ::llvm::GlobalValue::removeFromParent
    #[inline(always)]
    pub unsafe fn GlobalValue_removeFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalValue_removeFromParent(inst)
    }

    // ::llvm::GlobalValue::setThreadLocal
    #[inline(always)]
    pub unsafe fn GlobalValue_setThreadLocal(inst: *mut super::llvm_GlobalValue, Val: bool) -> ::libc::c_void {
        raw::llvm_GlobalValue_setThreadLocal(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::GlobalValue::setUnnamedAddr
    #[inline(always)]
    pub unsafe fn GlobalValue_setUnnamedAddr(inst: *mut super::llvm_GlobalValue, Val: bool) -> ::libc::c_void {
        raw::llvm_GlobalValue_setUnnamedAddr(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::GlobalVariable::copyAttributesFrom
    #[inline(always)]
    pub unsafe fn GlobalVariable_copyAttributesFrom(inst: *mut super::llvm_GlobalVariable, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void {
        raw::llvm_GlobalVariable_copyAttributesFrom(inst, Src)
    }

    // ::llvm::GlobalVariable::delete
    #[inline(always)]
    pub unsafe fn GlobalVariable_delete(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void {
        raw::llvm_GlobalVariable_delete(inst)
    }

    // ::llvm::GlobalVariable::eraseFromParent
    #[inline(always)]
    pub unsafe fn GlobalVariable_eraseFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void {
        raw::llvm_GlobalVariable_eraseFromParent(inst)
    }

    // ::llvm::GlobalVariable::getInitializer
    #[inline(always)]
    pub unsafe fn GlobalVariable_getInitializer(inst: *const super::llvm_GlobalVariable) -> *const super::llvm_Constant {
        raw::llvm_GlobalVariable_getInitializer(inst)
    }

    // ::llvm::GlobalVariable::getInitializerMut
    #[inline(always)]
    pub unsafe fn GlobalVariable_getInitializerMut(inst: *mut super::llvm_GlobalVariable) -> *mut super::llvm_Constant {
        raw::llvm_GlobalVariable_getInitializerMut(inst)
    }

    // ::llvm::GlobalVariable::hasDefinitiveInitializer
    #[inline(always)]
    pub unsafe fn GlobalVariable_hasDefinitiveInitializer(inst: *const super::llvm_GlobalVariable) -> bool {
        raw::llvm_GlobalVariable_hasDefinitiveInitializer(inst) != 0
    }

    // ::llvm::GlobalVariable::hasInitializer
    #[inline(always)]
    pub unsafe fn GlobalVariable_hasInitializer(inst: *const super::llvm_GlobalVariable) -> bool {
        raw::llvm_GlobalVariable_hasInitializer(inst) != 0
    }

    // ::llvm::GlobalVariable::hasUniqueInitializer
    #[inline(always)]
    pub unsafe fn GlobalVariable_hasUniqueInitializer(inst: *const super::llvm_GlobalVariable) -> bool {
        raw::llvm_GlobalVariable_hasUniqueInitializer(inst) != 0
    }

    // ::llvm::GlobalVariable::isConstant
    #[inline(always)]
    pub unsafe fn GlobalVariable_isConstant(inst: *const super::llvm_GlobalVariable) -> bool {
        raw::llvm_GlobalVariable_isConstant(inst) != 0
    }

    // ::llvm::GlobalVariable::isExternallyInitialized
    #[inline(always)]
    pub unsafe fn GlobalVariable_isExternallyInitialized(inst: *const super::llvm_GlobalVariable) -> bool {
        raw::llvm_GlobalVariable_isExternallyInitialized(inst) != 0
    }

    // ::llvm::GlobalVariable::new
    #[inline(always)]
    pub unsafe fn GlobalVariable_new(Ty: *mut super::llvm_Type, isConstant: bool, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_GlobalVariable {
        raw::llvm_GlobalVariable_new(Ty, if isConstant { 1 } else { 0 }, Linkage)
    }

    // ::llvm::GlobalVariable::newWithModule
    #[inline(always)]
    pub unsafe fn GlobalVariable_newWithModule(Module: *mut super::llvm_Module, Ty: *mut super::llvm_Type, isConstant: bool, Linkage: super::llvm_GlobalValue_LinkageTypes, Initializer: *mut super::llvm_Constant) -> *mut super::llvm_GlobalVariable {
        raw::llvm_GlobalVariable_newWithModule(Module, Ty, if isConstant { 1 } else { 0 }, Linkage, Initializer)
    }

    // ::llvm::GlobalVariable::removeFromParent
    #[inline(always)]
    pub unsafe fn GlobalVariable_removeFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void {
        raw::llvm_GlobalVariable_removeFromParent(inst)
    }

    // ::llvm::GlobalVariable::setConstant
    #[inline(always)]
    pub unsafe fn GlobalVariable_setConstant(inst: *mut super::llvm_GlobalVariable, Val: bool) -> ::libc::c_void {
        raw::llvm_GlobalVariable_setConstant(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::GlobalVariable::setExternallyInitialized
    #[inline(always)]
    pub unsafe fn GlobalVariable_setExternallyInitialized(inst: *mut super::llvm_GlobalVariable, Val: bool) -> ::libc::c_void {
        raw::llvm_GlobalVariable_setExternallyInitialized(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::GlobalVariable::setInitializer
    #[inline(always)]
    pub unsafe fn GlobalVariable_setInitializer(inst: *mut super::llvm_GlobalVariable, InitVal: *mut super::llvm_Constant) -> ::libc::c_void {
        raw::llvm_GlobalVariable_setInitializer(inst, InitVal)
    }

    // ::llvm::IRBuilder::CreateAShr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAShr(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateAShrByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAShrByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateAdd
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAdd(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateAddrSpaceCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAddrSpaceCast(inst, Value, DestTy, Name)
    }

    pub unsafe fn IRBuilder_CreateAggregateRet(inst: *mut super::llvm_IRBuilder, values: &[*mut super::llvm_Value]) -> *mut super::llvm_ReturnInst {
        raw::llvm_IRBuilder_CreateAggregateRet(inst, values.as_ptr(), values.len() as ::libc::c_uint)
    }

    // ::llvm::IRBuilder::CreateAlignedLoad
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAlignedLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_LoadInst {
        raw::llvm_IRBuilder_CreateAlignedLoad(inst, Ptr, Align, Name)
    }

    // ::llvm::IRBuilder::CreateAlignedLoadVolatile
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAlignedLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: bool, Name: super::std_string) -> *mut super::llvm_LoadInst {
        raw::llvm_IRBuilder_CreateAlignedLoadVolatile(inst, Ptr, Align, if isVolatile { 1 } else { 0 }, Name)
    }

    // ::llvm::IRBuilder::CreateAlignedStore
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAlignedStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: bool) -> *mut super::llvm_StoreInst {
        raw::llvm_IRBuilder_CreateAlignedStore(inst, Value, Ptr, Align, if isVolatile { 1 } else { 0 })
    }

    // ::llvm::IRBuilder::CreateAlloca
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAlloca(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, ArraySize: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_AllocaInst {
        raw::llvm_IRBuilder_CreateAlloca(inst, Ty, ArraySize, Name)
    }

    // ::llvm::IRBuilder::CreateAnd
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAnd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAnd(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateAndByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateAndByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateAndByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateBinOp
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateBinOp(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_BinaryOps, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateBinOp(inst, Opcode, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateBitCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateBitCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateBr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateBr(inst: *mut super::llvm_IRBuilder, Dest: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst {
        raw::llvm_IRBuilder_CreateBr(inst, Dest)
    }

    // ::llvm::IRBuilder::CreateCall
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateCall(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, Args: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilder_CreateCall(inst, Callee, Args, Name)
    }

    // ::llvm::IRBuilder::CreateCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateCast(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_CastOps, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateCast(inst, Opcode, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateCondBr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateCondBr(inst: *mut super::llvm_IRBuilder, Cond: *mut super::llvm_Value, TrueBlock: *mut super::llvm_BasicBlock, FalseBlock: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst {
        raw::llvm_IRBuilder_CreateCondBr(inst, Cond, TrueBlock, FalseBlock)
    }

    // ::llvm::IRBuilder::CreateExactSDiv
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateExactSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateExactSDiv(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateExactUDiv
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateExactUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateExactUDiv(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateExtractElement
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateExtractElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateExtractElement(inst, Vec, Idx, Name)
    }

    // ::llvm::IRBuilder::CreateExtractInteger
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateExtractInteger(inst: *mut super::llvm_IRBuilder, DL: *const super::llvm_DataLayout, From: *mut super::llvm_Value, ExtractedTy: *mut super::llvm_IntegerType, Offset: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateExtractInteger(inst, DL, From, ExtractedTy, Offset, Name)
    }

    // ::llvm::IRBuilder::CreateExtractValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateExtractValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef__libc_c_uint, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateExtractValue(inst, Agg, Indexes, Name)
    }

    // ::llvm::IRBuilder::CreateFAdd
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFAdd(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmp
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmp(inst, Pred, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpOEQ
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpOEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpOEQ(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpOGE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpOGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpOGE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpOGT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpOGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpOGT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpOLE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpOLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpOLE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpOLT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpOLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpOLT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpONE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpONE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpONE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpORD
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpORD(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpORD(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpUEQ
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpUEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpUEQ(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpUGE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpUGE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpUGT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpUGT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpULE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpULE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpULT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpULT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpUNE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpUNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpUNE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFCmpUNO
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFCmpUNO(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFCmpUNO(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFDiv
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFDiv(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFMul
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFMul(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFNeg
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFNeg(inst, Value, Name)
    }

    // ::llvm::IRBuilder::CreateFPCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFPCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFPCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateFPExt
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFPExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFPExt(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateFPToSI
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFPToSI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFPToSI(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateFPToUI
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFPToUI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFPToUI(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateFPTrunc
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFPTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFPTrunc(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateFRem
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFRem(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFSub
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateFSub(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateFence
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateFence(inst: *mut super::llvm_IRBuilder, Ordering: super::llvm_AtomicOrdering, SynchScope: super::llvm_SynchronizationScope, Name: super::std_string) -> *mut super::llvm_FenceInst {
        raw::llvm_IRBuilder_CreateFence(inst, Ordering, SynchScope, Name)
    }

    // ::llvm::IRBuilder::CreateGEP
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateGEP(inst, Ptr, Indexes, Name)
    }

    // ::llvm::IRBuilder::CreateGlobalStringPtr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateGlobalStringPtr(inst: *mut super::llvm_IRBuilder, Str: super::llvm_StringRef, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateGlobalStringPtr(inst, Str, Name)
    }

    // ::llvm::IRBuilder::CreateICmp
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmp(inst, Pred, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpEQ
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpEQ(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpNE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpNE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpSGE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpSGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpSGE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpSGT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpSGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpSGT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpSLE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpSLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpSLE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpSLT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpSLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpSLT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpUGE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpUGE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpUGT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpUGT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpULE
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpULE(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateICmpULT
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateICmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateICmpULT(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateInBoundsGEP
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateInBoundsGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateInBoundsGEP(inst, Ptr, Indexes, Name)
    }

    // ::llvm::IRBuilder::CreateIndirectBr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateIndirectBr(inst: *mut super::llvm_IRBuilder, Addr: *mut super::llvm_Value, NumCases: ::libc::c_uint) -> *mut super::llvm_IndirectBrInst {
        raw::llvm_IRBuilder_CreateIndirectBr(inst, Addr, NumCases)
    }

    // ::llvm::IRBuilder::CreateInsertElement
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateInsertElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, NewElt: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateInsertElement(inst, Vec, NewElt, Idx, Name)
    }

    // ::llvm::IRBuilder::CreateInsertValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateInsertValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Value: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef__libc_c_uint, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateInsertValue(inst, Agg, Value, Indexes, Name)
    }

    // ::llvm::IRBuilder::CreateIntCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateIntCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, isSigned: bool, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateIntCast(inst, Value, DestTy, if isSigned { 1 } else { 0 }, Name)
    }

    // ::llvm::IRBuilder::CreateIntToPtr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateIntToPtr(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateIntToPtr(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateInvoke
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateInvoke(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, NormalDest: *mut super::llvm_BasicBlock, UnwindDest: *mut super::llvm_BasicBlock, Args: super::llvm_ArrayRef_llvm_Value_ptr, Name: super::std_string_const) -> *mut super::llvm_InvokeInst {
        raw::llvm_IRBuilder_CreateInvoke(inst, Callee, NormalDest, UnwindDest, Args, Name)
    }

    // ::llvm::IRBuilder::CreateIsNotNull
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateIsNotNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateIsNotNull(inst, Arg, Name)
    }

    // ::llvm::IRBuilder::CreateIsNull
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateIsNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateIsNull(inst, Arg, Name)
    }

    // ::llvm::IRBuilder::CreateLShr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateLShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateLShr(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateLShrByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateLShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateLShrByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateLandingPad
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateLandingPad(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, PersFn: *mut super::llvm_Value, NumClauses: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_LandingPadInst {
        raw::llvm_IRBuilder_CreateLandingPad(inst, Ty, PersFn, NumClauses, Name)
    }

    // ::llvm::IRBuilder::CreateLoad
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_LoadInst {
        raw::llvm_IRBuilder_CreateLoad(inst, Ptr, Name)
    }

    // ::llvm::IRBuilder::CreateLoadVolatile
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, isVolatile: bool, Name: super::std_string) -> *mut super::llvm_LoadInst {
        raw::llvm_IRBuilder_CreateLoadVolatile(inst, Ptr, if isVolatile { 1 } else { 0 }, Name)
    }

    // ::llvm::IRBuilder::CreateMul
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateMul(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNSWAdd
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNSWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNSWAdd(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNSWMul
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNSWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNSWMul(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNSWNeg
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNSWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNSWNeg(inst, Value, Name)
    }

    // ::llvm::IRBuilder::CreateNSWSub
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNSWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNSWSub(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNUWAdd
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNUWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNUWAdd(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNUWMul
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNUWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNUWMul(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNUWNeg
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNUWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNUWNeg(inst, Value, Name)
    }

    // ::llvm::IRBuilder::CreateNUWSub
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNUWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNUWSub(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateNeg
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNeg(inst, Value, Name)
    }

    // ::llvm::IRBuilder::CreateNot
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateNot(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateNot(inst, Value, Name)
    }

    // ::llvm::IRBuilder::CreateOr
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateOr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateOr(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateOrByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateOrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateOrByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreatePHI
    #[inline(always)]
    pub unsafe fn IRBuilder_CreatePHI(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, NumReservedValues: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_PHINode {
        raw::llvm_IRBuilder_CreatePHI(inst, Ty, NumReservedValues, Name)
    }

    // ::llvm::IRBuilder::CreatePointerBitCastOrAddrSpaceCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreatePointerCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreatePointerCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreatePointerCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreatePtrDiff
    #[inline(always)]
    pub unsafe fn IRBuilder_CreatePtrDiff(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreatePtrDiff(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreatePtrToInt
    #[inline(always)]
    pub unsafe fn IRBuilder_CreatePtrToInt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreatePtrToInt(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateResume
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateResume(inst: *mut super::llvm_IRBuilder, Exn: *mut super::llvm_Value) -> *mut super::llvm_ResumeInst {
        raw::llvm_IRBuilder_CreateResume(inst, Exn)
    }

    // ::llvm::IRBuilder::CreateRet
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateRet(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value) -> *mut super::llvm_ReturnInst {
        raw::llvm_IRBuilder_CreateRet(inst, Value)
    }

    // ::llvm::IRBuilder::CreateRetVoid
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateRetVoid(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_ReturnInst {
        raw::llvm_IRBuilder_CreateRetVoid(inst)
    }

    // ::llvm::IRBuilder::CreateSDiv
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSDiv(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateSExt
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSExt(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateSExtOrBitCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSExtOrBitCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateSExtOrTrunc
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSExtOrTrunc(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateSIToFP
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSIToFP(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateSRem
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSRem(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateSelect
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSelect(inst: *mut super::llvm_IRBuilder, C: *mut super::llvm_Value, TrueValue: *mut super::llvm_Value, FalseValue: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSelect(inst, C, TrueValue, FalseValue, Name)
    }

    // ::llvm::IRBuilder::CreateShl
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateShl(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateShl(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateShlByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateShlByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateShlByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateShuffleVector
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateShuffleVector(inst: *mut super::llvm_IRBuilder, V1: *mut super::llvm_Value, P2: *mut super::llvm_Value, Mask: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateShuffleVector(inst, V1, P2, Mask, Name)
    }

    // ::llvm::IRBuilder::CreateStore
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, isVolatile: bool) -> *mut super::llvm_StoreInst {
        raw::llvm_IRBuilder_CreateStore(inst, Value, Ptr, if isVolatile { 1 } else { 0 })
    }

    // ::llvm::IRBuilder::CreateStructGEP
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateStructGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Index: ::libc::c_uint, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateStructGEP(inst, Ptr, Index, Name)
    }

    // ::llvm::IRBuilder::CreateSub
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateSub(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateSwitch
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateSwitch(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Dest: *mut super::llvm_BasicBlock, NumCases: ::libc::c_uint) -> *mut super::llvm_SwitchInst {
        raw::llvm_IRBuilder_CreateSwitch(inst, Value, Dest, NumCases)
    }

    // ::llvm::IRBuilder::CreateTrunc
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateTrunc(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateTruncOrBitCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateTruncOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateTruncOrBitCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateUDiv
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateUDiv(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateUIToFP
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateUIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateUIToFP(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateURem
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateURem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateURem(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateUnreachable
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateUnreachable(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_UnreachableInst {
        raw::llvm_IRBuilder_CreateUnreachable(inst)
    }

    // ::llvm::IRBuilder::CreateVAArg
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateVAArg(inst: *mut super::llvm_IRBuilder, List: *mut super::llvm_Value, Ty: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_VAArgInst {
        raw::llvm_IRBuilder_CreateVAArg(inst, List, Ty, Name)
    }

    // ::llvm::IRBuilder::CreateVectorSplat
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateVectorSplat(inst: *mut super::llvm_IRBuilder, NumElements: ::libc::c_uint, Value: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateVectorSplat(inst, NumElements, Value, Name)
    }

    // ::llvm::IRBuilder::CreateXor
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateXor(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateXor(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateXorByValue
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateXorByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: ::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateXorByValue(inst, LHS, RHS, Name)
    }

    // ::llvm::IRBuilder::CreateZExt
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateZExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateZExt(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateZExtOrBitCast
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateZExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateZExtOrBitCast(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::CreateZExtOrTrunc
    #[inline(always)]
    pub unsafe fn IRBuilder_CreateZExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilder_CreateZExtOrTrunc(inst, Value, DestTy, Name)
    }

    // ::llvm::IRBuilder::delete
    #[inline(always)]
    pub unsafe fn IRBuilder_delete(inst: *mut super::llvm_IRBuilder) -> ::libc::c_void {
        raw::llvm_IRBuilder_delete(inst)
    }

    // ::llvm::IRBuilder::isNamePreserving
    #[inline(always)]
    pub unsafe fn IRBuilder_isNamePreserving(inst: *const super::llvm_IRBuilder) -> bool {
        raw::llvm_IRBuilder_isNamePreserving(inst) != 0
    }

    // ::llvm::IRBuilder::new
    #[inline(always)]
    pub unsafe fn IRBuilder_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilder {
        raw::llvm_IRBuilder_new(Context)
    }

    // ::llvm::IRBuilder::new_in_block
    #[inline(always)]
    pub unsafe fn IRBuilder_new_in_block(BB: *mut super::llvm_BasicBlock) -> *mut super::llvm_IRBuilder {
        raw::llvm_IRBuilder_new_in_block(BB)
    }

    // ::llvm::IRBuilderBase::ClearInsertionPoint
    #[inline(always)]
    pub unsafe fn IRBuilderBase_ClearInsertionPoint(inst: *mut super::llvm_IRBuilderBase) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_ClearInsertionPoint(inst)
    }

    // ::llvm::IRBuilderBase::CreateGlobalString
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateGlobalString(inst: *mut super::llvm_IRBuilderBase, Str: super::llvm_StringRef, Name: super::std_string) -> *mut super::llvm_Value {
        raw::llvm_IRBuilderBase_CreateGlobalString(inst, Str, Name)
    }

    // ::llvm::IRBuilderBase::CreateLifetimeEnd
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateLifetimeEnd(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilderBase_CreateLifetimeEnd(inst, Ptr, Size)
    }

    // ::llvm::IRBuilderBase::CreateLifetimeStart
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateLifetimeStart(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilderBase_CreateLifetimeStart(inst, Ptr, Size)
    }

    // ::llvm::IRBuilderBase::CreateMemCpy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateMemCpy(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: bool) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilderBase_CreateMemCpy(inst, Dst, Src, Size, Align, if isVolatile { 1 } else { 0 })
    }

    // ::llvm::IRBuilderBase::CreateMemMove
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateMemMove(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: bool) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilderBase_CreateMemMove(inst, Dst, Src, Size, Align, if isVolatile { 1 } else { 0 })
    }

    // ::llvm::IRBuilderBase::CreateMemSet
    #[inline(always)]
    pub unsafe fn IRBuilderBase_CreateMemSet(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Value: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: ::libc::c_uint, isVolatile: bool) -> *mut super::llvm_CallInst {
        raw::llvm_IRBuilderBase_CreateMemSet(inst, Ptr, Value, Size, Align, if isVolatile { 1 } else { 0 })
    }

    // ::llvm::IRBuilderBase::GetInsertBlock
    #[inline(always)]
    pub unsafe fn IRBuilderBase_GetInsertBlock(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_BasicBlock {
        raw::llvm_IRBuilderBase_GetInsertBlock(inst)
    }

    // ::llvm::IRBuilderBase::SetCurrentDebugLocation
    #[inline(always)]
    pub unsafe fn IRBuilderBase_SetCurrentDebugLocation(inst: *mut super::llvm_IRBuilderBase, Loc: *const super::llvm_DebugLoc) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_SetCurrentDebugLocation(inst, Loc)
    }

    // ::llvm::IRBuilderBase::SetDefaultFPMathTag
    #[inline(always)]
    pub unsafe fn IRBuilderBase_SetDefaultFPMathTag(inst: *mut super::llvm_IRBuilderBase, FPMathTag: *mut super::llvm_MDNode) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_SetDefaultFPMathTag(inst, FPMathTag)
    }

    // ::llvm::IRBuilderBase::SetInsertPoint
    #[inline(always)]
    pub unsafe fn IRBuilderBase_SetInsertPoint(inst: *mut super::llvm_IRBuilderBase, BB: *mut super::llvm_BasicBlock) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_SetInsertPoint(inst, BB)
    }

    // ::llvm::IRBuilderBase::SetInsertPointAtInst
    #[inline(always)]
    pub unsafe fn IRBuilderBase_SetInsertPointAtInst(inst: *mut super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_SetInsertPointAtInst(inst, Inst)
    }

    // ::llvm::IRBuilderBase::SetInstDebugLocation
    #[inline(always)]
    pub unsafe fn IRBuilderBase_SetInstDebugLocation(inst: *const super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_IRBuilderBase_SetInstDebugLocation(inst, Inst)
    }

    // ::llvm::IRBuilderBase::getContext
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getContext(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_LLVMContext {
        raw::llvm_IRBuilderBase_getContext(inst)
    }

    // ::llvm::IRBuilderBase::getCurrentFunctionReturnType
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getCurrentFunctionReturnType(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_Type {
        raw::llvm_IRBuilderBase_getCurrentFunctionReturnType(inst)
    }

    // ::llvm::IRBuilderBase::getDefaultFPMathTag
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getDefaultFPMathTag(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_MDNode {
        raw::llvm_IRBuilderBase_getDefaultFPMathTag(inst)
    }

    // ::llvm::IRBuilderBase::getDoubleTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getDoubleTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type {
        raw::llvm_IRBuilderBase_getDoubleTy(inst)
    }

    // ::llvm::IRBuilderBase::getFalse
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getFalse(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getFalse(inst)
    }

    // ::llvm::IRBuilderBase::getFloatTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getFloatTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type {
        raw::llvm_IRBuilderBase_getFloatTy(inst)
    }

    // ::llvm::IRBuilderBase::getHalfTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getHalfTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type {
        raw::llvm_IRBuilderBase_getHalfTy(inst)
    }

    // ::llvm::IRBuilderBase::getInt
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt(inst: *mut super::llvm_IRBuilderBase, Value: super::llvm_APInt) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt(inst, Value)
    }

    // ::llvm::IRBuilderBase::getInt1
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt1(inst: *mut super::llvm_IRBuilderBase, Value: bool) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt1(inst, if Value { 1 } else { 0 })
    }

    // ::llvm::IRBuilderBase::getInt16
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt16(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint16_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt16(inst, Value)
    }

    // ::llvm::IRBuilderBase::getInt16Ty
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt16Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getInt16Ty(inst)
    }

    // ::llvm::IRBuilderBase::getInt1Ty
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt1Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getInt1Ty(inst)
    }

    // ::llvm::IRBuilderBase::getInt32
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt32(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint32_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt32(inst, Value)
    }

    // ::llvm::IRBuilderBase::getInt32Ty
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt32Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getInt32Ty(inst)
    }

    // ::llvm::IRBuilderBase::getInt64
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt64(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt64(inst, Value)
    }

    // ::llvm::IRBuilderBase::getInt64Ty
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt64Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getInt64Ty(inst)
    }

    // ::llvm::IRBuilderBase::getInt8
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt8(inst: *mut super::llvm_IRBuilderBase, Value: ::libc::uint8_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getInt8(inst, Value)
    }

    // ::llvm::IRBuilderBase::getInt8PtrTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt8PtrTy(inst: *mut super::llvm_IRBuilderBase, AddrSpace: ::libc::c_uint) -> *mut super::llvm_PointerType {
        raw::llvm_IRBuilderBase_getInt8PtrTy(inst, AddrSpace)
    }

    // ::llvm::IRBuilderBase::getInt8Ty
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getInt8Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getInt8Ty(inst)
    }

    // ::llvm::IRBuilderBase::getIntN
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getIntN(inst: *mut super::llvm_IRBuilderBase, NumBits: ::libc::c_uint, Value: ::libc::uint64_t) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getIntN(inst, NumBits, Value)
    }

    // ::llvm::IRBuilderBase::getIntNTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getIntNTy(inst: *mut super::llvm_IRBuilderBase, NumBits: ::libc::c_uint) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getIntNTy(inst, NumBits)
    }

    // ::llvm::IRBuilderBase::getIntPtrTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getIntPtrTy(inst: *mut super::llvm_IRBuilderBase, DL: *const super::llvm_DataLayout, AddrSpace: ::libc::c_uint) -> *mut super::llvm_IntegerType {
        raw::llvm_IRBuilderBase_getIntPtrTy(inst, DL, AddrSpace)
    }

    // ::llvm::IRBuilderBase::getTrue
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getTrue(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt {
        raw::llvm_IRBuilderBase_getTrue(inst)
    }

    // ::llvm::IRBuilderBase::getVoidTy
    #[inline(always)]
    pub unsafe fn IRBuilderBase_getVoidTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type {
        raw::llvm_IRBuilderBase_getVoidTy(inst)
    }

    // ::llvm::IRBuilderBase::new
    #[inline(always)]
    pub unsafe fn IRBuilderBase_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilderBase {
        raw::llvm_IRBuilderBase_new(Context)
    }

    // ::llvm::Instruction::clone
    #[inline(always)]
    pub unsafe fn Instruction_clone(inst: *const super::llvm_Instruction) -> *mut super::llvm_Instruction {
        raw::llvm_Instruction_clone(inst)
    }

    // ::llvm::Instruction::copyFastMathFlags
    #[inline(always)]
    pub unsafe fn Instruction_copyFastMathFlags(inst: *mut super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_copyFastMathFlags(inst, Inst)
    }

    // ::llvm::Instruction::delete
    #[inline(always)]
    pub unsafe fn Instruction_delete(inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_delete(inst)
    }

    // ::llvm::Instruction::dropUnknownMetadata
    #[inline(always)]
    pub unsafe fn Instruction_dropUnknownMetadata(inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_dropUnknownMetadata(inst)
    }

    // ::llvm::Instruction::dropUnknownMetadataFromIDS
    #[inline(always)]
    pub unsafe fn Instruction_dropUnknownMetadataFromIDS(inst: *mut super::llvm_Instruction, KnownIDs: super::llvm_ArrayRef__libc_c_uint) -> ::libc::c_void {
        raw::llvm_Instruction_dropUnknownMetadataFromIDS(inst, KnownIDs)
    }

    // ::llvm::Instruction::eraseFromParent
    #[inline(always)]
    pub unsafe fn Instruction_eraseFromParent(inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_eraseFromParent(inst)
    }

    // ::llvm::Instruction::getDataLayout
    #[inline(always)]
    pub unsafe fn Instruction_getDataLayout(inst: *const super::llvm_Instruction) -> *const super::llvm_DataLayout {
        raw::llvm_Instruction_getDataLayout(inst)
    }

    // ::llvm::Instruction::getDebugLoc
    #[inline(always)]
    pub unsafe fn Instruction_getDebugLoc(inst: *const super::llvm_Instruction) -> *const super::llvm_DebugLoc {
        raw::llvm_Instruction_getDebugLoc(inst)
    }

    // ::llvm::Instruction::getMetadata
    #[inline(always)]
    pub unsafe fn Instruction_getMetadata(inst: *const super::llvm_Instruction, KindID: ::libc::c_uint) -> *mut super::llvm_MDNode {
        raw::llvm_Instruction_getMetadata(inst, KindID)
    }

    // ::llvm::Instruction::getMetadataStr
    #[inline(always)]
    pub unsafe fn Instruction_getMetadataStr(inst: *const super::llvm_Instruction, Kind: super::llvm_StringRef) -> *mut super::llvm_MDNode {
        raw::llvm_Instruction_getMetadataStr(inst, Kind)
    }

    // ::llvm::Instruction::getOpcode
    #[inline(always)]
    pub unsafe fn Instruction_getOpcode(inst: *const super::llvm_Instruction) -> ::libc::c_uint {
        raw::llvm_Instruction_getOpcode(inst)
    }

    // ::llvm::Instruction::getParent
    #[inline(always)]
    pub unsafe fn Instruction_getParent(inst: *const super::llvm_Instruction) -> *const super::llvm_BasicBlock {
        raw::llvm_Instruction_getParent(inst)
    }

    // ::llvm::Instruction::getParentMut
    #[inline(always)]
    pub unsafe fn Instruction_getParentMut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_BasicBlock {
        raw::llvm_Instruction_getParentMut(inst)
    }

    // ::llvm::Instruction::hasAllowReciprocal
    #[inline(always)]
    pub unsafe fn Instruction_hasAllowReciprocal(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasAllowReciprocal(inst) != 0
    }

    // ::llvm::Instruction::hasMetadata
    #[inline(always)]
    pub unsafe fn Instruction_hasMetadata(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasMetadata(inst) != 0
    }

    // ::llvm::Instruction::hasMetadataOtherThanDebugLoc
    #[inline(always)]
    pub unsafe fn Instruction_hasMetadataOtherThanDebugLoc(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasMetadataOtherThanDebugLoc(inst) != 0
    }

    // ::llvm::Instruction::hasNoInfs
    #[inline(always)]
    pub unsafe fn Instruction_hasNoInfs(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasNoInfs(inst) != 0
    }

    // ::llvm::Instruction::hasNoNaNs
    #[inline(always)]
    pub unsafe fn Instruction_hasNoNaNs(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasNoNaNs(inst) != 0
    }

    // ::llvm::Instruction::hasNoSignedZeros
    #[inline(always)]
    pub unsafe fn Instruction_hasNoSignedZeros(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasNoSignedZeros(inst) != 0
    }

    // ::llvm::Instruction::hasUnsafeAlgebra
    #[inline(always)]
    pub unsafe fn Instruction_hasUnsafeAlgebra(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_hasUnsafeAlgebra(inst) != 0
    }

    // ::llvm::Instruction::insertAfter
    #[inline(always)]
    pub unsafe fn Instruction_insertAfter(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_insertAfter(inst, InsertPos)
    }

    // ::llvm::Instruction::insertBefore
    #[inline(always)]
    pub unsafe fn Instruction_insertBefore(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_insertBefore(inst, InsertPos)
    }

    // ::llvm::Instruction::isArithmeticShift
    #[inline(always)]
    pub unsafe fn Instruction_isArithmeticShift(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isArithmeticShift(inst) != 0
    }

    // ::llvm::Instruction::isAssociative
    #[inline(always)]
    pub unsafe fn Instruction_isAssociative(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isAssociative(inst) != 0
    }

    // ::llvm::Instruction::isBinaryOp
    #[inline(always)]
    pub unsafe fn Instruction_isBinaryOp(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isBinaryOp(inst) != 0
    }

    // ::llvm::Instruction::isCast
    #[inline(always)]
    pub unsafe fn Instruction_isCast(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isCast(inst) != 0
    }

    // ::llvm::Instruction::isCommutative
    #[inline(always)]
    pub unsafe fn Instruction_isCommutative(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isCommutative(inst) != 0
    }

    // ::llvm::Instruction::isIdempotent
    #[inline(always)]
    pub unsafe fn Instruction_isIdempotent(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isIdempotent(inst) != 0
    }

    // ::llvm::Instruction::isIdenticalTo
    #[inline(always)]
    pub unsafe fn Instruction_isIdenticalTo(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isIdenticalTo(inst, Inst) != 0
    }

    // ::llvm::Instruction::isIdenticalToWhenDefined
    #[inline(always)]
    pub unsafe fn Instruction_isIdenticalToWhenDefined(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isIdenticalToWhenDefined(inst, Inst) != 0
    }

    // ::llvm::Instruction::isLogicalShift
    #[inline(always)]
    pub unsafe fn Instruction_isLogicalShift(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isLogicalShift(inst) != 0
    }

    // ::llvm::Instruction::isNilpotent
    #[inline(always)]
    pub unsafe fn Instruction_isNilpotent(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isNilpotent(inst) != 0
    }

    // ::llvm::Instruction::isSameOperationAs
    #[inline(always)]
    pub unsafe fn Instruction_isSameOperationAs(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction, flags: ::libc::c_uint) -> bool {
        raw::llvm_Instruction_isSameOperationAs(inst, Inst, flags) != 0
    }

    // ::llvm::Instruction::isShift
    #[inline(always)]
    pub unsafe fn Instruction_isShift(inst: *mut super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isShift(inst) != 0
    }

    // ::llvm::Instruction::isTerminator
    #[inline(always)]
    pub unsafe fn Instruction_isTerminator(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_isTerminator(inst) != 0
    }

    // ::llvm::Instruction::isUsedOutsideOfBlock
    #[inline(always)]
    pub unsafe fn Instruction_isUsedOutsideOfBlock(inst: *const super::llvm_Instruction, BB: *const super::llvm_BasicBlock) -> bool {
        raw::llvm_Instruction_isUsedOutsideOfBlock(inst, BB) != 0
    }

    // ::llvm::Instruction::mayHaveSideEffects
    #[inline(always)]
    pub unsafe fn Instruction_mayHaveSideEffects(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayHaveSideEffects(inst) != 0
    }

    // ::llvm::Instruction::mayReadFromMemory
    #[inline(always)]
    pub unsafe fn Instruction_mayReadFromMemory(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayReadFromMemory(inst) != 0
    }

    // ::llvm::Instruction::mayReadOrWriteMemory
    #[inline(always)]
    pub unsafe fn Instruction_mayReadOrWriteMemory(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayReadOrWriteMemory(inst) != 0
    }

    // ::llvm::Instruction::mayReturn
    #[inline(always)]
    pub unsafe fn Instruction_mayReturn(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayReturn(inst) != 0
    }

    // ::llvm::Instruction::mayThrow
    #[inline(always)]
    pub unsafe fn Instruction_mayThrow(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayThrow(inst) != 0
    }

    // ::llvm::Instruction::mayWriteToMemory
    #[inline(always)]
    pub unsafe fn Instruction_mayWriteToMemory(inst: *const super::llvm_Instruction) -> bool {
        raw::llvm_Instruction_mayWriteToMemory(inst) != 0
    }

    // ::llvm::Instruction::moveBefore
    #[inline(always)]
    pub unsafe fn Instruction_moveBefore(inst: *mut super::llvm_Instruction, MovePos: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_moveBefore(inst, MovePos)
    }

    // ::llvm::Instruction::removeFromParent
    #[inline(always)]
    pub unsafe fn Instruction_removeFromParent(inst: *mut super::llvm_Instruction) -> ::libc::c_void {
        raw::llvm_Instruction_removeFromParent(inst)
    }

    // ::llvm::Instruction::setDebugLoc
    #[inline(always)]
    pub unsafe fn Instruction_setDebugLoc(inst: *mut super::llvm_Instruction, Loc: *const super::llvm_DebugLoc) -> ::libc::c_void {
        raw::llvm_Instruction_setDebugLoc(inst, Loc)
    }

    // ::llvm::Instruction::setHasAllowReciprocal
    #[inline(always)]
    pub unsafe fn Instruction_setHasAllowReciprocal(inst: *mut super::llvm_Instruction, Val: bool) -> ::libc::c_void {
        raw::llvm_Instruction_setHasAllowReciprocal(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::Instruction::setHasNoInfs
    #[inline(always)]
    pub unsafe fn Instruction_setHasNoInfs(inst: *mut super::llvm_Instruction, Val: bool) -> ::libc::c_void {
        raw::llvm_Instruction_setHasNoInfs(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::Instruction::setHasNoNaNs
    #[inline(always)]
    pub unsafe fn Instruction_setHasNoNaNs(inst: *mut super::llvm_Instruction, Val: bool) -> ::libc::c_void {
        raw::llvm_Instruction_setHasNoNaNs(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::Instruction::setHasNoSignedZeros
    #[inline(always)]
    pub unsafe fn Instruction_setHasNoSignedZeros(inst: *mut super::llvm_Instruction, Val: bool) -> ::libc::c_void {
        raw::llvm_Instruction_setHasNoSignedZeros(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::Instruction::setHasUnsafeAlgebra
    #[inline(always)]
    pub unsafe fn Instruction_setHasUnsafeAlgebra(inst: *mut super::llvm_Instruction, Val: bool) -> ::libc::c_void {
        raw::llvm_Instruction_setHasUnsafeAlgebra(inst, if Val { 1 } else { 0 })
    }

    // ::llvm::Instruction::setMetadata
    #[inline(always)]
    pub unsafe fn Instruction_setMetadata(inst: *mut super::llvm_Instruction, KindID: ::libc::c_uint, Node: *mut super::llvm_MDNode) -> ::libc::c_void {
        raw::llvm_Instruction_setMetadata(inst, KindID, Node)
    }

    // ::llvm::Instruction::setMetadataStr
    #[inline(always)]
    pub unsafe fn Instruction_setMetadataStr(inst: *mut super::llvm_Instruction, Kind: super::llvm_StringRef, Node: *mut super::llvm_MDNode) -> ::libc::c_void {
        raw::llvm_Instruction_setMetadataStr(inst, Kind, Node)
    }

    // ::llvm::Instruction::user_back
    #[inline(always)]
    pub unsafe fn Instruction_user_back(inst: *const super::llvm_Instruction) -> *const super::llvm_Instruction {
        raw::llvm_Instruction_user_back(inst)
    }

    // ::llvm::Instruction::user_back_mut
    #[inline(always)]
    pub unsafe fn Instruction_user_back_mut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_Instruction {
        raw::llvm_Instruction_user_back_mut(inst)
    }

    // ::llvm::IntegerType::classof
    #[inline(always)]
    pub unsafe fn IntegerType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_IntegerType_classof(ty) != 0
    }

    // ::llvm::IntegerType::get
    #[inline(always)]
    pub unsafe fn IntegerType_get(ctx: *mut super::llvm_LLVMContext, NumBits: ::libc::c_uint) -> *mut super::llvm_IntegerType {
        raw::llvm_IntegerType_get(ctx, NumBits)
    }

    // ::llvm::IntegerType::getBitMask
    #[inline(always)]
    pub unsafe fn IntegerType_getBitMask(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t {
        raw::llvm_IntegerType_getBitMask(inst)
    }

    // ::llvm::IntegerType::getBitWidth
    #[inline(always)]
    pub unsafe fn IntegerType_getBitWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_uint {
        raw::llvm_IntegerType_getBitWidth(inst)
    }

    // ::llvm::IntegerType::getSignBit
    #[inline(always)]
    pub unsafe fn IntegerType_getSignBit(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t {
        raw::llvm_IntegerType_getSignBit(inst)
    }

    // ::llvm::IntegerType::isPowerOf2ByteWidth
    #[inline(always)]
    pub unsafe fn IntegerType_isPowerOf2ByteWidth(inst: *const super::llvm_IntegerType) -> bool {
        raw::llvm_IntegerType_isPowerOf2ByteWidth(inst) != 0
    }

    // ::llvm::LLVMContext::delete
    #[inline(always)]
    pub unsafe fn LLVMContext_delete() -> *mut super::llvm_LLVMContext {
        raw::llvm_LLVMContext_delete()
    }

    // ::llvm::LLVMContext::new
    #[inline(always)]
    pub unsafe fn LLVMContext_new() -> *mut super::llvm_LLVMContext {
        raw::llvm_LLVMContext_new()
    }

    // ::llvm::Module::appendModuleInlineAsm
    #[inline(always)]
    pub unsafe fn Module_appendModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Module_appendModuleInlineAsm(inst, Asm)
    }

    // ::llvm::Module::delete
    #[inline(always)]
    pub unsafe fn Module_delete(inst: *mut super::llvm_Module) -> ::libc::c_void {
        raw::llvm_Module_delete(inst)
    }

    // ::llvm::Module::dump
    #[inline(always)]
    pub unsafe fn Module_dump(inst: *const super::llvm_Module) -> ::libc::c_void {
        raw::llvm_Module_dump(inst)
    }

    // ::llvm::Module::getContext
    #[inline(always)]
    pub unsafe fn Module_getContext(inst: *const super::llvm_Module) -> *mut super::llvm_LLVMContext {
        raw::llvm_Module_getContext(inst)
    }

    // ::llvm::Module::getDataLayout
    #[inline(always)]
    pub unsafe fn Module_getDataLayout(inst: *const super::llvm_Module) -> *const super::llvm_DataLayout {
        raw::llvm_Module_getDataLayout(inst)
    }

    // ::llvm::Module::getDataLayoutStr
    #[inline(always)]
    pub unsafe fn Module_getDataLayoutStr(inst: *const super::llvm_Module) -> super::std_string_const {
        raw::llvm_Module_getDataLayoutStr(inst)
    }

    // ::llvm::Module::getFunction
    #[inline(always)]
    pub unsafe fn Module_getFunction(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_Function {
        raw::llvm_Module_getFunction(inst, Name)
    }

    // ::llvm::Module::getMDKindID
    #[inline(always)]
    pub unsafe fn Module_getMDKindID(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> ::libc::c_uint {
        raw::llvm_Module_getMDKindID(inst, Name)
    }

    // ::llvm::Module::getModuleIdentifier
    #[inline(always)]
    pub unsafe fn Module_getModuleIdentifier(inst: *const super::llvm_Module) -> super::std_string_const {
        raw::llvm_Module_getModuleIdentifier(inst)
    }

    // ::llvm::Module::getModuleInlineAsm
    #[inline(always)]
    pub unsafe fn Module_getModuleInlineAsm(inst: *const super::llvm_Module) -> super::std_string_const {
        raw::llvm_Module_getModuleInlineAsm(inst)
    }

    // ::llvm::Module::getNamedValue
    #[inline(always)]
    pub unsafe fn Module_getNamedValue(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_GlobalValue {
        raw::llvm_Module_getNamedValue(inst, Name)
    }

    // ::llvm::Module::getOrInsertFunction
    #[inline(always)]
    pub unsafe fn Module_getOrInsertFunction(inst: *mut super::llvm_Module, Name: super::llvm_StringRef, ty: *mut super::llvm_FunctionType) -> *mut super::llvm_Constant {
        raw::llvm_Module_getOrInsertFunction(inst, Name, ty)
    }

    // ::llvm::Module::getTargetTriple
    #[inline(always)]
    pub unsafe fn Module_getTargetTriple(inst: *const super::llvm_Module) -> super::std_string_const {
        raw::llvm_Module_getTargetTriple(inst)
    }

    // ::llvm::Module::getTypeByName
    #[inline(always)]
    pub unsafe fn Module_getTypeByName(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_StructType {
        raw::llvm_Module_getTypeByName(inst, Name)
    }

    // ::llvm::Module::new
    #[inline(always)]
    pub unsafe fn Module_new(ModuleID: super::llvm_StringRef, Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_Module {
        raw::llvm_Module_new(ModuleID, Context)
    }

    // ::llvm::Module::setDataLayout
    #[inline(always)]
    pub unsafe fn Module_setDataLayout(inst: *mut super::llvm_Module, Other: *const super::llvm_DataLayout) -> ::libc::c_void {
        raw::llvm_Module_setDataLayout(inst, Other)
    }

    // ::llvm::Module::setDataLayoutStr
    #[inline(always)]
    pub unsafe fn Module_setDataLayoutStr(inst: *mut super::llvm_Module, Desc: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Module_setDataLayoutStr(inst, Desc)
    }

    // ::llvm::Module::setModuleIdentifier
    #[inline(always)]
    pub unsafe fn Module_setModuleIdentifier(inst: *mut super::llvm_Module, ID: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Module_setModuleIdentifier(inst, ID)
    }

    // ::llvm::Module::setModuleInlineAsm
    #[inline(always)]
    pub unsafe fn Module_setModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Module_setModuleInlineAsm(inst, Asm)
    }

    // ::llvm::Module::setTargetTriple
    #[inline(always)]
    pub unsafe fn Module_setTargetTriple(inst: *mut super::llvm_Module, Triple: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Module_setTargetTriple(inst, Triple)
    }

    // ::llvm::Operator::getOpcode
    #[inline(always)]
    pub unsafe fn Operator_getOpcode(inst: *const super::llvm_Operator) -> ::libc::c_uint {
        raw::llvm_Operator_getOpcode(inst)
    }

    // ::llvm::Pass::delete
    #[inline(always)]
    pub unsafe fn Pass_delete(inst: *mut super::llvm_Pass) -> ::libc::c_void {
        raw::llvm_Pass_delete(inst)
    }

    // ::llvm::Pass::doFinalization
    #[inline(always)]
    pub unsafe fn Pass_doFinalization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> bool {
        raw::llvm_Pass_doFinalization(inst, Module) != 0
    }

    // ::llvm::Pass::doInitialization
    #[inline(always)]
    pub unsafe fn Pass_doInitialization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> bool {
        raw::llvm_Pass_doInitialization(inst, Module) != 0
    }

    // ::llvm::Pass::dump
    #[inline(always)]
    pub unsafe fn Pass_dump(inst: *const super::llvm_Pass) -> ::libc::c_void {
        raw::llvm_Pass_dump(inst)
    }

    // ::llvm::Pass::getPassKind
    #[inline(always)]
    pub unsafe fn Pass_getPassKind(inst: *const super::llvm_Pass) -> super::llvm_PassKind {
        raw::llvm_Pass_getPassKind(inst)
    }

    // ::llvm::PassManager::add
    #[inline(always)]
    pub unsafe fn PassManager_add(inst: *mut super::llvm_PassManager, Pass: *mut super::llvm_Pass) -> ::libc::c_void {
        raw::llvm_PassManager_add(inst, Pass)
    }

    // ::llvm::PassManager::new
    #[inline(always)]
    pub unsafe fn PassManager_new() -> *mut super::llvm_PassManager {
        raw::llvm_PassManager_new()
    }

    // ::llvm::PassManager::run
    #[inline(always)]
    pub unsafe fn PassManager_run(inst: *mut super::llvm_PassManager, Module: *mut super::llvm_Module) -> ::libc::c_void {
        raw::llvm_PassManager_run(inst, Module)
    }

    // ::llvm::PointerType::classof
    #[inline(always)]
    pub unsafe fn PointerType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_PointerType_classof(ty) != 0
    }

    // ::llvm::PointerType::get
    #[inline(always)]
    pub unsafe fn PointerType_get(ElementType: *mut super::llvm_Type, AddressSpace: ::libc::c_uint) -> *mut super::llvm_PointerType {
        raw::llvm_PointerType_get(ElementType, AddressSpace)
    }

    // ::llvm::PointerType::getAddressSpace
    #[inline(always)]
    pub unsafe fn PointerType_getAddressSpace(inst: *const super::llvm_PointerType) -> ::libc::c_uint {
        raw::llvm_PointerType_getAddressSpace(inst)
    }

    // ::llvm::PointerType::getUnqual
    #[inline(always)]
    pub unsafe fn PointerType_getUnqual(ElementType: *mut super::llvm_Type) -> *mut super::llvm_PointerType {
        raw::llvm_PointerType_getUnqual(ElementType)
    }

    // ::llvm::PointerType::isValidElementType
    #[inline(always)]
    pub unsafe fn PointerType_isValidElementType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_PointerType_isValidElementType(ty) != 0
    }

    // ::llvm::SequentialType::classof
    #[inline(always)]
    pub unsafe fn SequentialType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_SequentialType_classof(ty) != 0
    }

    // ::llvm::SequentialType::getElementType
    #[inline(always)]
    pub unsafe fn SequentialType_getElementType(inst: *const super::llvm_SequentialType) -> *mut super::llvm_Type {
        raw::llvm_SequentialType_getElementType(inst)
    }

    // ::llvm::StructType::classof
    #[inline(always)]
    pub unsafe fn StructType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_StructType_classof(ty) != 0
    }

    // ::llvm::StructType::create
    #[inline(always)]
    pub unsafe fn StructType_create(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef) -> *mut super::llvm_StructType {
        raw::llvm_StructType_create(ctx, Elements, Name)
    }

    // ::llvm::StructType::createPacked
    #[inline(always)]
    pub unsafe fn StructType_createPacked(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef, isPacked: bool) -> *mut super::llvm_StructType {
        raw::llvm_StructType_createPacked(ctx, Elements, Name, if isPacked { 1 } else { 0 })
    }

    // ::llvm::StructType::getElementType
    #[inline(always)]
    pub unsafe fn StructType_getElementType(inst: *const super::llvm_StructType, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_StructType_getElementType(inst, idx)
    }

    // ::llvm::StructType::getName
    #[inline(always)]
    pub unsafe fn StructType_getName(inst: *const super::llvm_StructType) -> super::llvm_StringRef {
        raw::llvm_StructType_getName(inst)
    }

    // ::llvm::StructType::getNumElements
    #[inline(always)]
    pub unsafe fn StructType_getNumElements(inst: *const super::llvm_StructType) -> ::libc::c_uint {
        raw::llvm_StructType_getNumElements(inst)
    }

    // ::llvm::StructType::hasName
    #[inline(always)]
    pub unsafe fn StructType_hasName(inst: *const super::llvm_StructType) -> bool {
        raw::llvm_StructType_hasName(inst) != 0
    }

    // ::llvm::StructType::isLayoutIdentical
    #[inline(always)]
    pub unsafe fn StructType_isLayoutIdentical(inst: *const super::llvm_StructType, Other: *mut super::llvm_StructType) -> bool {
        raw::llvm_StructType_isLayoutIdentical(inst, Other) != 0
    }

    // ::llvm::StructType::isLiteral
    #[inline(always)]
    pub unsafe fn StructType_isLiteral(inst: *const super::llvm_StructType) -> bool {
        raw::llvm_StructType_isLiteral(inst) != 0
    }

    // ::llvm::StructType::isOpaque
    #[inline(always)]
    pub unsafe fn StructType_isOpaque(inst: *const super::llvm_StructType) -> bool {
        raw::llvm_StructType_isOpaque(inst) != 0
    }

    // ::llvm::StructType::isPacked
    #[inline(always)]
    pub unsafe fn StructType_isPacked(inst: *const super::llvm_StructType) -> bool {
        raw::llvm_StructType_isPacked(inst) != 0
    }

    // ::llvm::StructType::isSized
    #[inline(always)]
    pub unsafe fn StructType_isSized(inst: *const super::llvm_StructType) -> bool {
        raw::llvm_StructType_isSized(inst) != 0
    }

    // ::llvm::StructType::isValidElementType
    #[inline(always)]
    pub unsafe fn StructType_isValidElementType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_StructType_isValidElementType(ty) != 0
    }

    // ::llvm::StructType::setBody
    #[inline(always)]
    pub unsafe fn StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr) -> ::libc::c_void {
        raw::llvm_StructType_setBody(inst, Elements)
    }

    // ::llvm::StructType::setBodyPacked
    #[inline(always)]
    pub unsafe fn StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr, isPacked: bool) -> ::libc::c_void {
        raw::llvm_StructType_setBodyPacked(inst, Elements, if isPacked { 1 } else { 0 })
    }

    // ::llvm::StructType::setName
    #[inline(always)]
    pub unsafe fn StructType_setName(inst: *mut super::llvm_StructType, Name: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_StructType_setName(inst, Name)
    }

    // ::llvm::Type::dump
    #[inline(always)]
    pub unsafe fn Type_dump(inst: *const super::llvm_Type) -> ::libc::c_void {
        raw::llvm_Type_dump(inst)
    }

    // ::llvm::Type::getContainedType
    #[inline(always)]
    pub unsafe fn Type_getContainedType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_Type_getContainedType(inst, idx)
    }

    // ::llvm::Type::getContext
    #[inline(always)]
    pub unsafe fn Type_getContext(inst: *const super::llvm_Type) -> *mut super::llvm_LLVMContext {
        raw::llvm_Type_getContext(inst)
    }

    // ::llvm::Type::getDoublePtrTy
    #[inline(always)]
    pub unsafe fn Type_getDoublePtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getDoublePtrTy(ctx)
    }

    // ::llvm::Type::getDoubleTy
    #[inline(always)]
    pub unsafe fn Type_getDoubleTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getDoubleTy(ctx)
    }

    // ::llvm::Type::getFP128PtrTy
    #[inline(always)]
    pub unsafe fn Type_getFP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getFP128PtrTy(ctx)
    }

    // ::llvm::Type::getFP128Ty
    #[inline(always)]
    pub unsafe fn Type_getFP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getFP128Ty(ctx)
    }

    // ::llvm::Type::getFloatPtrTy
    #[inline(always)]
    pub unsafe fn Type_getFloatPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getFloatPtrTy(ctx)
    }

    // ::llvm::Type::getFloatTy
    #[inline(always)]
    pub unsafe fn Type_getFloatTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getFloatTy(ctx)
    }

    // ::llvm::Type::getFunctionNumParams
    #[inline(always)]
    pub unsafe fn Type_getFunctionNumParams(inst: *const super::llvm_Type) -> ::libc::c_uint {
        raw::llvm_Type_getFunctionNumParams(inst)
    }

    // ::llvm::Type::getFunctionParamType
    #[inline(always)]
    pub unsafe fn Type_getFunctionParamType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_Type_getFunctionParamType(inst, idx)
    }

    // ::llvm::Type::getHalfPtrTy
    #[inline(always)]
    pub unsafe fn Type_getHalfPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getHalfPtrTy(ctx)
    }

    // ::llvm::Type::getHalfTy
    #[inline(always)]
    pub unsafe fn Type_getHalfTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getHalfTy(ctx)
    }

    // ::llvm::Type::getInt16PtrTy
    #[inline(always)]
    pub unsafe fn Type_getInt16PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getInt16PtrTy(ctx)
    }

    // ::llvm::Type::getInt16Ty
    #[inline(always)]
    pub unsafe fn Type_getInt16Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getInt16Ty(ctx)
    }

    // ::llvm::Type::getInt1PtrTy
    #[inline(always)]
    pub unsafe fn Type_getInt1PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getInt1PtrTy(ctx)
    }

    // ::llvm::Type::getInt1Ty
    #[inline(always)]
    pub unsafe fn Type_getInt1Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getInt1Ty(ctx)
    }

    // ::llvm::Type::getInt32PtrTy
    #[inline(always)]
    pub unsafe fn Type_getInt32PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getInt32PtrTy(ctx)
    }

    // ::llvm::Type::getInt32Ty
    #[inline(always)]
    pub unsafe fn Type_getInt32Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getInt32Ty(ctx)
    }

    // ::llvm::Type::getInt64PtrTy
    #[inline(always)]
    pub unsafe fn Type_getInt64PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getInt64PtrTy(ctx)
    }

    // ::llvm::Type::getInt64Ty
    #[inline(always)]
    pub unsafe fn Type_getInt64Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getInt64Ty(ctx)
    }

    // ::llvm::Type::getInt8PtrTy
    #[inline(always)]
    pub unsafe fn Type_getInt8PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getInt8PtrTy(ctx)
    }

    // ::llvm::Type::getInt8Ty
    #[inline(always)]
    pub unsafe fn Type_getInt8Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getInt8Ty(ctx)
    }

    // ::llvm::Type::getIntNPtrTy
    #[inline(always)]
    pub unsafe fn Type_getIntNPtrTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getIntNPtrTy(ctx, size)
    }

    // ::llvm::Type::getIntNTy
    #[inline(always)]
    pub unsafe fn Type_getIntNTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_IntegerType {
        raw::llvm_Type_getIntNTy(ctx, size)
    }

    // ::llvm::Type::getLabelTy
    #[inline(always)]
    pub unsafe fn Type_getLabelTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getLabelTy(ctx)
    }

    // ::llvm::Type::getMetadataTy
    #[inline(always)]
    pub unsafe fn Type_getMetadataTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getMetadataTy(ctx)
    }

    // ::llvm::Type::getNumContainedTypes
    #[inline(always)]
    pub unsafe fn Type_getNumContainedTypes(inst: *const super::llvm_Type) -> ::libc::c_uint {
        raw::llvm_Type_getNumContainedTypes(inst)
    }

    // ::llvm::Type::getPPC_FP128PtrTy
    #[inline(always)]
    pub unsafe fn Type_getPPC_FP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getPPC_FP128PtrTy(ctx)
    }

    // ::llvm::Type::getPPC_FP128Ty
    #[inline(always)]
    pub unsafe fn Type_getPPC_FP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getPPC_FP128Ty(ctx)
    }

    // ::llvm::Type::getPointerAddressSpace
    #[inline(always)]
    pub unsafe fn Type_getPointerAddressSpace(inst: *const super::llvm_Type) -> ::libc::c_uint {
        raw::llvm_Type_getPointerAddressSpace(inst)
    }

    // ::llvm::Type::getPointerElementType
    #[inline(always)]
    pub unsafe fn Type_getPointerElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type {
        raw::llvm_Type_getPointerElementType(inst)
    }

    // ::llvm::Type::getPointerTo
    #[inline(always)]
    pub unsafe fn Type_getPointerTo(inst: *mut super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getPointerTo(inst, idx)
    }

    // ::llvm::Type::getSequentialElementType
    #[inline(always)]
    pub unsafe fn Type_getSequentialElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type {
        raw::llvm_Type_getSequentialElementType(inst)
    }

    // ::llvm::Type::getStructElementType
    #[inline(always)]
    pub unsafe fn Type_getStructElementType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type {
        raw::llvm_Type_getStructElementType(inst, idx)
    }

    // ::llvm::Type::getStructName
    #[inline(always)]
    pub unsafe fn Type_getStructName(inst: *const super::llvm_Type) -> super::llvm_StringRef {
        raw::llvm_Type_getStructName(inst)
    }

    // ::llvm::Type::getStructNumElements
    #[inline(always)]
    pub unsafe fn Type_getStructNumElements(inst: *const super::llvm_Type) -> ::libc::c_uint {
        raw::llvm_Type_getStructNumElements(inst)
    }

    // ::llvm::Type::getTypeID
    #[inline(always)]
    pub unsafe fn Type_getTypeID(inst: *const super::llvm_Type) -> super::llvm_Type_TypeID {
        raw::llvm_Type_getTypeID(inst)
    }

    // ::llvm::Type::getVoidTy
    #[inline(always)]
    pub unsafe fn Type_getVoidTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getVoidTy(ctx)
    }

    // ::llvm::Type::getX86_FP80PtrTy
    #[inline(always)]
    pub unsafe fn Type_getX86_FP80PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getX86_FP80PtrTy(ctx)
    }

    // ::llvm::Type::getX86_FP80Ty
    #[inline(always)]
    pub unsafe fn Type_getX86_FP80Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getX86_FP80Ty(ctx)
    }

    // ::llvm::Type::getX86_MMXPtrTy
    #[inline(always)]
    pub unsafe fn Type_getX86_MMXPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType {
        raw::llvm_Type_getX86_MMXPtrTy(ctx)
    }

    // ::llvm::Type::getX86_MMXTy
    #[inline(always)]
    pub unsafe fn Type_getX86_MMXTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type {
        raw::llvm_Type_getX86_MMXTy(ctx)
    }

    // ::llvm::Type::isAggregateType
    #[inline(always)]
    pub unsafe fn Type_isAggregateType(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isAggregateType(inst) != 0
    }

    // ::llvm::Type::isArrayTy
    #[inline(always)]
    pub unsafe fn Type_isArrayTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isArrayTy(inst) != 0
    }

    // ::llvm::Type::isDoubleTy
    #[inline(always)]
    pub unsafe fn Type_isDoubleTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isDoubleTy(inst) != 0
    }

    // ::llvm::Type::isEmptyTy
    #[inline(always)]
    pub unsafe fn Type_isEmptyTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isEmptyTy(inst) != 0
    }

    // ::llvm::Type::isFP128Ty
    #[inline(always)]
    pub unsafe fn Type_isFP128Ty(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFP128Ty(inst) != 0
    }

    // ::llvm::Type::isFPOrFPVectorTy
    #[inline(always)]
    pub unsafe fn Type_isFPOrFPVectorTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFPOrFPVectorTy(inst) != 0
    }

    // ::llvm::Type::isFirstClassType
    #[inline(always)]
    pub unsafe fn Type_isFirstClassType(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFirstClassType(inst) != 0
    }

    // ::llvm::Type::isFloatTy
    #[inline(always)]
    pub unsafe fn Type_isFloatTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFloatTy(inst) != 0
    }

    // ::llvm::Type::isFloatingPointTy
    #[inline(always)]
    pub unsafe fn Type_isFloatingPointTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFloatingPointTy(inst) != 0
    }

    // ::llvm::Type::isFunctionTy
    #[inline(always)]
    pub unsafe fn Type_isFunctionTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFunctionTy(inst) != 0
    }

    // ::llvm::Type::isFunctionVarArg
    #[inline(always)]
    pub unsafe fn Type_isFunctionVarArg(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isFunctionVarArg(inst) != 0
    }

    // ::llvm::Type::isHalfTy
    #[inline(always)]
    pub unsafe fn Type_isHalfTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isHalfTy(inst) != 0
    }

    // ::llvm::Type::isIntOrIntVectorTy
    #[inline(always)]
    pub unsafe fn Type_isIntOrIntVectorTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isIntOrIntVectorTy(inst) != 0
    }

    // ::llvm::Type::isIntegerTy
    #[inline(always)]
    pub unsafe fn Type_isIntegerTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isIntegerTy(inst) != 0
    }

    // ::llvm::Type::isLabelTy
    #[inline(always)]
    pub unsafe fn Type_isLabelTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isLabelTy(inst) != 0
    }

    // ::llvm::Type::isMetadataTy
    #[inline(always)]
    pub unsafe fn Type_isMetadataTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isMetadataTy(inst) != 0
    }

    // ::llvm::Type::isPPC_FP128Ty
    #[inline(always)]
    pub unsafe fn Type_isPPC_FP128Ty(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isPPC_FP128Ty(inst) != 0
    }

    // ::llvm::Type::isPointerTy
    #[inline(always)]
    pub unsafe fn Type_isPointerTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isPointerTy(inst) != 0
    }

    // ::llvm::Type::isPtrOrPtrVectorTy
    #[inline(always)]
    pub unsafe fn Type_isPtrOrPtrVectorTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isPtrOrPtrVectorTy(inst) != 0
    }

    // ::llvm::Type::isSingleValueType
    #[inline(always)]
    pub unsafe fn Type_isSingleValueType(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isSingleValueType(inst) != 0
    }

    // ::llvm::Type::isSized
    #[inline(always)]
    pub unsafe fn Type_isSized(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isSized(inst) != 0
    }

    // ::llvm::Type::isStructTy
    #[inline(always)]
    pub unsafe fn Type_isStructTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isStructTy(inst) != 0
    }

    // ::llvm::Type::isVectorTy
    #[inline(always)]
    pub unsafe fn Type_isVectorTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isVectorTy(inst) != 0
    }

    // ::llvm::Type::isVoidTy
    #[inline(always)]
    pub unsafe fn Type_isVoidTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isVoidTy(inst) != 0
    }

    // ::llvm::Type::isX86_FP80Ty
    #[inline(always)]
    pub unsafe fn Type_isX86_FP80Ty(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isX86_FP80Ty(inst) != 0
    }

    // ::llvm::Type::isX86_MMXTy
    #[inline(always)]
    pub unsafe fn Type_isX86_MMXTy(inst: *const super::llvm_Type) -> bool {
        raw::llvm_Type_isX86_MMXTy(inst) != 0
    }

    // ::llvm::Use::get
    #[inline(always)]
    pub unsafe fn Use_get(inst: *const super::llvm_Use) -> *mut super::llvm_Value {
        raw::llvm_Use_get(inst)
    }

    // ::llvm::Use::getNext
    #[inline(always)]
    pub unsafe fn Use_getNext(inst: *const super::llvm_Use) -> *mut super::llvm_Use {
        raw::llvm_Use_getNext(inst)
    }

    // ::llvm::Use::getOperandNo
    #[inline(always)]
    pub unsafe fn Use_getOperandNo(inst: *const super::llvm_Use) -> ::libc::c_uint {
        raw::llvm_Use_getOperandNo(inst)
    }

    // ::llvm::Use::getUser
    #[inline(always)]
    pub unsafe fn Use_getUser(inst: *const super::llvm_Use) -> *mut super::llvm_User {
        raw::llvm_Use_getUser(inst)
    }

    // ::llvm::Use::initTags
    #[inline(always)]
    pub unsafe fn Use_initTags(Start: *mut super::llvm_Use, Stop: *mut super::llvm_Use) -> *mut super::llvm_Use {
        raw::llvm_Use_initTags(Start, Stop)
    }

    // ::llvm::Use::set
    #[inline(always)]
    pub unsafe fn Use_set(inst: *mut super::llvm_Use, Val: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_Use_set(inst, Val)
    }

    // ::llvm::Use::swap
    #[inline(always)]
    pub unsafe fn Use_swap(inst: *mut super::llvm_Use, RHS: *mut super::llvm_Use) -> ::libc::c_void {
        raw::llvm_Use_swap(inst, RHS)
    }

    // ::llvm::User::classof
    #[inline(always)]
    pub unsafe fn User_classof(V: *mut super::llvm_Value) -> bool {
        raw::llvm_User_classof(V) != 0
    }

    // ::llvm::User::delete
    #[inline(always)]
    pub unsafe fn User_delete(inst: *mut super::llvm_User) -> ::libc::c_void {
        raw::llvm_User_delete(inst)
    }

    // ::llvm::User::dropAllReferences
    #[inline(always)]
    pub unsafe fn User_dropAllReferences(inst: *mut super::llvm_User) -> ::libc::c_void {
        raw::llvm_User_dropAllReferences(inst)
    }

    // ::llvm::User::getNumOperands
    #[inline(always)]
    pub unsafe fn User_getNumOperands(inst: *const super::llvm_User) -> ::libc::c_uint {
        raw::llvm_User_getNumOperands(inst)
    }

    // ::llvm::User::getOperand
    #[inline(always)]
    pub unsafe fn User_getOperand(inst: *const super::llvm_User, idx: ::libc::c_uint) -> *mut super::llvm_Value {
        raw::llvm_User_getOperand(inst, idx)
    }

    // ::llvm::User::replaceUsesOfWith
    #[inline(always)]
    pub unsafe fn User_replaceUsesOfWith(inst: *mut super::llvm_User, From: *mut super::llvm_Value, To: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_User_replaceUsesOfWith(inst, From, To)
    }

    // ::llvm::User::setOperand
    #[inline(always)]
    pub unsafe fn User_setOperand(inst: *mut super::llvm_User, idx: ::libc::c_uint, Val: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_User_setOperand(inst, idx, Val)
    }

    // ::llvm::Value::delete
    #[inline(always)]
    pub unsafe fn Value_delete(inst: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_Value_delete(inst)
    }

    // ::llvm::Value::dump
    #[inline(always)]
    pub unsafe fn Value_dump(inst: *const super::llvm_Value) -> ::libc::c_void {
        raw::llvm_Value_dump(inst)
    }

    // ::llvm::Value::getContext
    #[inline(always)]
    pub unsafe fn Value_getContext(inst: *const super::llvm_Value) -> *mut super::llvm_LLVMContext {
        raw::llvm_Value_getContext(inst)
    }

    // ::llvm::Value::getName
    #[inline(always)]
    pub unsafe fn Value_getName(inst: *const super::llvm_Value) -> super::llvm_StringRef {
        raw::llvm_Value_getName(inst)
    }

    // ::llvm::Value::getNumUses
    #[inline(always)]
    pub unsafe fn Value_getNumUses(inst: *const super::llvm_Value) -> ::libc::c_uint {
        raw::llvm_Value_getNumUses(inst)
    }

    // ::llvm::Value::getType
    #[inline(always)]
    pub unsafe fn Value_getType(inst: *const super::llvm_Value) -> *mut super::llvm_Type {
        raw::llvm_Value_getType(inst)
    }

    // ::llvm::Value::getValueID
    #[inline(always)]
    pub unsafe fn Value_getValueID(inst: *const super::llvm_Value) -> ::libc::c_uint {
        raw::llvm_Value_getValueID(inst)
    }

    // ::llvm::Value::hasNUses
    #[inline(always)]
    pub unsafe fn Value_hasNUses(inst: *const super::llvm_Value, N: ::libc::c_uint) -> bool {
        raw::llvm_Value_hasNUses(inst, N) != 0
    }

    // ::llvm::Value::hasNUsesOrMore
    #[inline(always)]
    pub unsafe fn Value_hasNUsesOrMore(inst: *const super::llvm_Value, N: ::libc::c_uint) -> bool {
        raw::llvm_Value_hasNUsesOrMore(inst, N) != 0
    }

    // ::llvm::Value::hasName
    #[inline(always)]
    pub unsafe fn Value_hasName(inst: *const super::llvm_Value) -> bool {
        raw::llvm_Value_hasName(inst) != 0
    }

    // ::llvm::Value::hasOneUse
    #[inline(always)]
    pub unsafe fn Value_hasOneUse(inst: *const super::llvm_Value) -> bool {
        raw::llvm_Value_hasOneUse(inst) != 0
    }

    // ::llvm::Value::isUsedInBasicBlock
    #[inline(always)]
    pub unsafe fn Value_isUsedInBasicBlock(inst: *const super::llvm_Value, BB: *const super::llvm_BasicBlock) -> bool {
        raw::llvm_Value_isUsedInBasicBlock(inst, BB) != 0
    }

    // ::llvm::Value::mutateType
    #[inline(always)]
    pub unsafe fn Value_mutateType(inst: *mut super::llvm_Value, ty: *mut super::llvm_Type) -> ::libc::c_void {
        raw::llvm_Value_mutateType(inst, ty)
    }

    // ::llvm::Value::replaceAllUsesWith
    #[inline(always)]
    pub unsafe fn Value_replaceAllUsesWith(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_Value_replaceAllUsesWith(inst, Value)
    }

    // ::llvm::Value::setName
    #[inline(always)]
    pub unsafe fn Value_setName(inst: *mut super::llvm_Value, Name: super::llvm_StringRef) -> ::libc::c_void {
        raw::llvm_Value_setName(inst, Name)
    }

    // ::llvm::Value::takeName
    #[inline(always)]
    pub unsafe fn Value_takeName(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void {
        raw::llvm_Value_takeName(inst, Value)
    }

    // ::llvm::ValueSymbolTable::delete
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_delete() -> *mut super::llvm_ValueSymbolTable {
        raw::llvm_ValueSymbolTable_delete()
    }

    // ::llvm::ValueSymbolTable::dump
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_dump(inst: *const super::llvm_ValueSymbolTable) -> ::libc::c_void {
        raw::llvm_ValueSymbolTable_dump(inst)
    }

    // ::llvm::ValueSymbolTable::empty
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_empty(inst: *const super::llvm_ValueSymbolTable) -> bool {
        raw::llvm_ValueSymbolTable_empty(inst) != 0
    }

    // ::llvm::ValueSymbolTable::lookup
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_lookup(inst: *const super::llvm_ValueSymbolTable, Name: super::llvm_StringRef) -> *mut super::llvm_Value {
        raw::llvm_ValueSymbolTable_lookup(inst, Name)
    }

    // ::llvm::ValueSymbolTable::new
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_new() -> *mut super::llvm_ValueSymbolTable {
        raw::llvm_ValueSymbolTable_new()
    }

    // ::llvm::ValueSymbolTable::size
    #[inline(always)]
    pub unsafe fn ValueSymbolTable_size(inst: *const super::llvm_ValueSymbolTable) -> ::libc::c_uint {
        raw::llvm_ValueSymbolTable_size(inst)
    }

    // ::llvm::VectorType::classof
    #[inline(always)]
    pub unsafe fn VectorType_classof(ty: *const super::llvm_Type) -> bool {
        raw::llvm_VectorType_classof(ty) != 0
    }

    // ::llvm::VectorType::get
    #[inline(always)]
    pub unsafe fn VectorType_get(ty: *mut super::llvm_Type, NumElements: ::libc::c_uint) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_get(ty, NumElements)
    }

    // ::llvm::VectorType::getBitWidth
    #[inline(always)]
    pub unsafe fn VectorType_getBitWidth(inst: *const super::llvm_VectorType) -> ::libc::c_uint {
        raw::llvm_VectorType_getBitWidth(inst)
    }

    // ::llvm::VectorType::getDoubleElementsVectorType
    #[inline(always)]
    pub unsafe fn VectorType_getDoubleElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_getDoubleElementsVectorType(ty)
    }

    // ::llvm::VectorType::getExtendedElementVectorType
    #[inline(always)]
    pub unsafe fn VectorType_getExtendedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_getExtendedElementVectorType(ty)
    }

    // ::llvm::VectorType::getHalfElementsVectorType
    #[inline(always)]
    pub unsafe fn VectorType_getHalfElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_getHalfElementsVectorType(ty)
    }

    // ::llvm::VectorType::getInteger
    #[inline(always)]
    pub unsafe fn VectorType_getInteger(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_getInteger(ty)
    }

    // ::llvm::VectorType::getNumElements
    #[inline(always)]
    pub unsafe fn VectorType_getNumElements(inst: *const super::llvm_VectorType) -> ::libc::c_uint {
        raw::llvm_VectorType_getNumElements(inst)
    }

    // ::llvm::VectorType::getTruncatedElementVectorType
    #[inline(always)]
    pub unsafe fn VectorType_getTruncatedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType {
        raw::llvm_VectorType_getTruncatedElementVectorType(ty)
    }

    // ::llvm::VectorType::isValidElementType
    #[inline(always)]
    pub unsafe fn VectorType_isValidElementType(ty: *mut super::llvm_Type) -> bool {
        raw::llvm_VectorType_isValidElementType(ty) != 0
    }

    // ::llvm::createAddDiscriminatorsPass
    #[inline(always)]
    pub unsafe fn createAddDiscriminatorsPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createAddDiscriminatorsPass()
    }

    // ::llvm::createAddressSanitizerFunctionPass
    #[inline(always)]
    pub unsafe fn createAddressSanitizerFunctionPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createAddressSanitizerFunctionPass()
    }

    // ::llvm::createAddressSanitizerModulePass
    #[inline(always)]
    pub unsafe fn createAddressSanitizerModulePass() -> *mut super::llvm_ModulePass {
        raw::llvm_createAddressSanitizerModulePass()
    }

    // ::llvm::createAggressiveDCEPass
    #[inline(always)]
    pub unsafe fn createAggressiveDCEPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createAggressiveDCEPass()
    }

    // ::llvm::createAlwaysInlinerPass
    #[inline(always)]
    pub unsafe fn createAlwaysInlinerPass(InsertLifetime: bool) -> *mut super::llvm_Pass {
        raw::llvm_createAlwaysInlinerPass(if InsertLifetime { 1 } else { 0 })
    }

    // ::llvm::createArgumentPromotionPass
    #[inline(always)]
    pub unsafe fn createArgumentPromotionPass(maxElements: ::libc::c_uint) -> *mut super::llvm_Pass {
        raw::llvm_createArgumentPromotionPass(maxElements)
    }

    // ::llvm::createBarrierNoopPass
    #[inline(always)]
    pub unsafe fn createBarrierNoopPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createBarrierNoopPass()
    }

    // ::llvm::createBlockExtractorPass
    #[inline(always)]
    pub unsafe fn createBlockExtractorPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createBlockExtractorPass()
    }

    // ::llvm::createBoundsCheckingPass
    #[inline(always)]
    pub unsafe fn createBoundsCheckingPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createBoundsCheckingPass()
    }

    // ::llvm::createBreakCriticalEdgesPass
    #[inline(always)]
    pub unsafe fn createBreakCriticalEdgesPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createBreakCriticalEdgesPass()
    }

    // ::llvm::createCFGSimplificationPass
    #[inline(always)]
    pub unsafe fn createCFGSimplificationPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createCFGSimplificationPass()
    }

    // ::llvm::createConstantHoistingPass
    #[inline(always)]
    pub unsafe fn createConstantHoistingPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createConstantHoistingPass()
    }

    // ::llvm::createConstantMergePass
    #[inline(always)]
    pub unsafe fn createConstantMergePass() -> *mut super::llvm_ModulePass {
        raw::llvm_createConstantMergePass()
    }

    // ::llvm::createConstantPropagationPass
    #[inline(always)]
    pub unsafe fn createConstantPropagationPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createConstantPropagationPass()
    }

    // ::llvm::createCorrelatedValuePropagationPass
    #[inline(always)]
    pub unsafe fn createCorrelatedValuePropagationPass() -> *mut super::llvm_Pass {
        raw::llvm_createCorrelatedValuePropagationPass()
    }

    // ::llvm::createDataFlowSanitizerPass
    #[inline(always)]
    pub unsafe fn createDataFlowSanitizerPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createDataFlowSanitizerPass()
    }

    // ::llvm::createDeadArgEliminationPass
    #[inline(always)]
    pub unsafe fn createDeadArgEliminationPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createDeadArgEliminationPass()
    }

    // ::llvm::createDeadArgHackingPass
    #[inline(always)]
    pub unsafe fn createDeadArgHackingPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createDeadArgHackingPass()
    }

    // ::llvm::createDeadCodeEliminationPass
    #[inline(always)]
    pub unsafe fn createDeadCodeEliminationPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createDeadCodeEliminationPass()
    }

    // ::llvm::createDeadInstEliminationPass
    #[inline(always)]
    pub unsafe fn createDeadInstEliminationPass() -> *mut super::llvm_Pass {
        raw::llvm_createDeadInstEliminationPass()
    }

    // ::llvm::createDeadStoreEliminationPass
    #[inline(always)]
    pub unsafe fn createDeadStoreEliminationPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createDeadStoreEliminationPass()
    }

    // ::llvm::createDebugIRPass
    #[inline(always)]
    pub unsafe fn createDebugIRPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createDebugIRPass()
    }

    // ::llvm::createDemoteRegisterToMemoryPass
    #[inline(always)]
    pub unsafe fn createDemoteRegisterToMemoryPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createDemoteRegisterToMemoryPass()
    }

    // ::llvm::createEarlyCSEPass
    #[inline(always)]
    pub unsafe fn createEarlyCSEPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createEarlyCSEPass()
    }

    // ::llvm::createFlattenCFGPass
    #[inline(always)]
    pub unsafe fn createFlattenCFGPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createFlattenCFGPass()
    }

    // ::llvm::createFunctionAttrsPass
    #[inline(always)]
    pub unsafe fn createFunctionAttrsPass() -> *mut super::llvm_Pass {
        raw::llvm_createFunctionAttrsPass()
    }

    // ::llvm::createFunctionInliningPass
    #[inline(always)]
    pub unsafe fn createFunctionInliningPass() -> *mut super::llvm_Pass {
        raw::llvm_createFunctionInliningPass()
    }

    // ::llvm::createGCOVProfilerPass
    #[inline(always)]
    pub unsafe fn createGCOVProfilerPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createGCOVProfilerPass()
    }

    // ::llvm::createGVNPass
    #[inline(always)]
    pub unsafe fn createGVNPass(NoLoads: bool) -> *mut super::llvm_FunctionPass {
        raw::llvm_createGVNPass(if NoLoads { 1 } else { 0 })
    }

    // ::llvm::createGlobalDCEPass
    #[inline(always)]
    pub unsafe fn createGlobalDCEPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createGlobalDCEPass()
    }

    // ::llvm::createGlobalMergePass
    #[inline(always)]
    pub unsafe fn createGlobalMergePass() -> *mut super::llvm_Pass {
        raw::llvm_createGlobalMergePass()
    }

    // ::llvm::createGlobalOptimizerPass
    #[inline(always)]
    pub unsafe fn createGlobalOptimizerPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createGlobalOptimizerPass()
    }

    // ::llvm::createIPConstantPropagationPass
    #[inline(always)]
    pub unsafe fn createIPConstantPropagationPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createIPConstantPropagationPass()
    }

    // ::llvm::createIPSCCPPass
    #[inline(always)]
    pub unsafe fn createIPSCCPPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createIPSCCPPass()
    }

    // ::llvm::createIndVarSimplifyPass
    #[inline(always)]
    pub unsafe fn createIndVarSimplifyPass() -> *mut super::llvm_Pass {
        raw::llvm_createIndVarSimplifyPass()
    }

    // ::llvm::createInstructionCombiningPass
    #[inline(always)]
    pub unsafe fn createInstructionCombiningPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createInstructionCombiningPass()
    }

    // ::llvm::createInstructionNamerPass
    #[inline(always)]
    pub unsafe fn createInstructionNamerPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createInstructionNamerPass()
    }

    // ::llvm::createInstructionSimplifierPass
    #[inline(always)]
    pub unsafe fn createInstructionSimplifierPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createInstructionSimplifierPass()
    }

    // ::llvm::createInternalizePass
    #[inline(always)]
    pub unsafe fn createInternalizePass() -> *mut super::llvm_ModulePass {
        raw::llvm_createInternalizePass()
    }

    // ::llvm::createJumpThreadingPass
    #[inline(always)]
    pub unsafe fn createJumpThreadingPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createJumpThreadingPass()
    }

    // ::llvm::createLCSSAPass
    #[inline(always)]
    pub unsafe fn createLCSSAPass() -> *mut super::llvm_Pass {
        raw::llvm_createLCSSAPass()
    }

    // ::llvm::createLICMPass
    #[inline(always)]
    pub unsafe fn createLICMPass() -> *mut super::llvm_Pass {
        raw::llvm_createLICMPass()
    }

    // ::llvm::createLoadCombinePass
    #[inline(always)]
    pub unsafe fn createLoadCombinePass() -> *mut super::llvm_BasicBlockPass {
        raw::llvm_createLoadCombinePass()
    }

    // ::llvm::createLoopDeletionPass
    #[inline(always)]
    pub unsafe fn createLoopDeletionPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopDeletionPass()
    }

    // ::llvm::createLoopExtractorPass
    #[inline(always)]
    pub unsafe fn createLoopExtractorPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopExtractorPass()
    }

    // ::llvm::createLoopIdiomPass
    #[inline(always)]
    pub unsafe fn createLoopIdiomPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopIdiomPass()
    }

    // ::llvm::createLoopInstSimplifyPass
    #[inline(always)]
    pub unsafe fn createLoopInstSimplifyPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopInstSimplifyPass()
    }

    // ::llvm::createLoopRerollPass
    #[inline(always)]
    pub unsafe fn createLoopRerollPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopRerollPass()
    }

    // ::llvm::createLoopRotatePass
    #[inline(always)]
    pub unsafe fn createLoopRotatePass(MaxHeaderSize: ::libc::c_int) -> *mut super::llvm_Pass {
        raw::llvm_createLoopRotatePass(MaxHeaderSize)
    }

    // ::llvm::createLoopSimplifyPass
    #[inline(always)]
    pub unsafe fn createLoopSimplifyPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopSimplifyPass()
    }

    // ::llvm::createLoopStrengthReducePass
    #[inline(always)]
    pub unsafe fn createLoopStrengthReducePass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopStrengthReducePass()
    }

    // ::llvm::createLoopUnrollPass
    #[inline(always)]
    pub unsafe fn createLoopUnrollPass() -> *mut super::llvm_Pass {
        raw::llvm_createLoopUnrollPass()
    }

    // ::llvm::createLoopUnswitchPass
    #[inline(always)]
    pub unsafe fn createLoopUnswitchPass(OptimizeForSize: bool) -> *mut super::llvm_Pass {
        raw::llvm_createLoopUnswitchPass(if OptimizeForSize { 1 } else { 0 })
    }

    // ::llvm::createLowerAtomicPass
    #[inline(always)]
    pub unsafe fn createLowerAtomicPass() -> *mut super::llvm_Pass {
        raw::llvm_createLowerAtomicPass()
    }

    // ::llvm::createLowerExpectIntrinsicPass
    #[inline(always)]
    pub unsafe fn createLowerExpectIntrinsicPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createLowerExpectIntrinsicPass()
    }

    // ::llvm::createLowerInvokePass
    #[inline(always)]
    pub unsafe fn createLowerInvokePass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createLowerInvokePass()
    }

    // ::llvm::createLowerSwitchPass
    #[inline(always)]
    pub unsafe fn createLowerSwitchPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createLowerSwitchPass()
    }

    // ::llvm::createMemCpyOptPass
    #[inline(always)]
    pub unsafe fn createMemCpyOptPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createMemCpyOptPass()
    }

    // ::llvm::createMemorySanitizerPass
    #[inline(always)]
    pub unsafe fn createMemorySanitizerPass(TrackOrigins: ::libc::c_int) -> *mut super::llvm_FunctionPass {
        raw::llvm_createMemorySanitizerPass(TrackOrigins)
    }

    // ::llvm::createMergeFunctionsPass
    #[inline(always)]
    pub unsafe fn createMergeFunctionsPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createMergeFunctionsPass()
    }

    // ::llvm::createMergedLoadStoreMotionPass
    #[inline(always)]
    pub unsafe fn createMergedLoadStoreMotionPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createMergedLoadStoreMotionPass()
    }

    // ::llvm::createMetaRenamerPass
    #[inline(always)]
    pub unsafe fn createMetaRenamerPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createMetaRenamerPass()
    }

    // ::llvm::createObjCARCAPElimPass
    #[inline(always)]
    pub unsafe fn createObjCARCAPElimPass() -> *mut super::llvm_Pass {
        raw::llvm_createObjCARCAPElimPass()
    }

    // ::llvm::createObjCARCContractPass
    #[inline(always)]
    pub unsafe fn createObjCARCContractPass() -> *mut super::llvm_Pass {
        raw::llvm_createObjCARCContractPass()
    }

    // ::llvm::createObjCARCExpandPass
    #[inline(always)]
    pub unsafe fn createObjCARCExpandPass() -> *mut super::llvm_Pass {
        raw::llvm_createObjCARCExpandPass()
    }

    // ::llvm::createObjCARCOptPass
    #[inline(always)]
    pub unsafe fn createObjCARCOptPass() -> *mut super::llvm_Pass {
        raw::llvm_createObjCARCOptPass()
    }

    // ::llvm::createPartialInliningPass
    #[inline(always)]
    pub unsafe fn createPartialInliningPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createPartialInliningPass()
    }

    // ::llvm::createPartiallyInlineLibCallsPass
    #[inline(always)]
    pub unsafe fn createPartiallyInlineLibCallsPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createPartiallyInlineLibCallsPass()
    }

    // ::llvm::createPromoteMemoryToRegisterPass
    #[inline(always)]
    pub unsafe fn createPromoteMemoryToRegisterPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createPromoteMemoryToRegisterPass()
    }

    // ::llvm::createPruneEHPass
    #[inline(always)]
    pub unsafe fn createPruneEHPass() -> *mut super::llvm_Pass {
        raw::llvm_createPruneEHPass()
    }

    // ::llvm::createReassociatePass
    #[inline(always)]
    pub unsafe fn createReassociatePass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createReassociatePass()
    }

    // ::llvm::createSCCPPass
    #[inline(always)]
    pub unsafe fn createSCCPPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createSCCPPass()
    }

    // ::llvm::createSROAPass
    #[inline(always)]
    pub unsafe fn createSROAPass(RequiresDomTree: bool) -> *mut super::llvm_FunctionPass {
        raw::llvm_createSROAPass(if RequiresDomTree { 1 } else { 0 })
    }

    // ::llvm::createSampleProfileLoaderPass
    #[inline(always)]
    pub unsafe fn createSampleProfileLoaderPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createSampleProfileLoaderPass()
    }

    // ::llvm::createScalarReplAggregatesPass
    #[inline(always)]
    pub unsafe fn createScalarReplAggregatesPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createScalarReplAggregatesPass()
    }

    // ::llvm::createScalarizerPass
    #[inline(always)]
    pub unsafe fn createScalarizerPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createScalarizerPass()
    }

    // ::llvm::createSeparateConstOffsetFromGEPPass
    #[inline(always)]
    pub unsafe fn createSeparateConstOffsetFromGEPPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createSeparateConstOffsetFromGEPPass()
    }

    // ::llvm::createSimpleLoopUnrollPass
    #[inline(always)]
    pub unsafe fn createSimpleLoopUnrollPass() -> *mut super::llvm_Pass {
        raw::llvm_createSimpleLoopUnrollPass()
    }

    // ::llvm::createSingleLoopExtractorPass
    #[inline(always)]
    pub unsafe fn createSingleLoopExtractorPass() -> *mut super::llvm_Pass {
        raw::llvm_createSingleLoopExtractorPass()
    }

    // ::llvm::createSinkingPass
    #[inline(always)]
    pub unsafe fn createSinkingPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createSinkingPass()
    }

    // ::llvm::createStripDeadDebugInfoPass
    #[inline(always)]
    pub unsafe fn createStripDeadDebugInfoPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createStripDeadDebugInfoPass()
    }

    // ::llvm::createStripDeadPrototypesPass
    #[inline(always)]
    pub unsafe fn createStripDeadPrototypesPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createStripDeadPrototypesPass()
    }

    // ::llvm::createStripDebugDeclarePass
    #[inline(always)]
    pub unsafe fn createStripDebugDeclarePass() -> *mut super::llvm_ModulePass {
        raw::llvm_createStripDebugDeclarePass()
    }

    // ::llvm::createStripNonDebugSymbolsPass
    #[inline(always)]
    pub unsafe fn createStripNonDebugSymbolsPass() -> *mut super::llvm_ModulePass {
        raw::llvm_createStripNonDebugSymbolsPass()
    }

    // ::llvm::createStripSymbolsPass
    #[inline(always)]
    pub unsafe fn createStripSymbolsPass(OnlyDebugInfo: bool) -> *mut super::llvm_ModulePass {
        raw::llvm_createStripSymbolsPass(if OnlyDebugInfo { 1 } else { 0 })
    }

    // ::llvm::createStructurizeCFGPass
    #[inline(always)]
    pub unsafe fn createStructurizeCFGPass() -> *mut super::llvm_Pass {
        raw::llvm_createStructurizeCFGPass()
    }

    // ::llvm::createTailCallEliminationPass
    #[inline(always)]
    pub unsafe fn createTailCallEliminationPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createTailCallEliminationPass()
    }

    // ::llvm::createThreadSanitizerPass
    #[inline(always)]
    pub unsafe fn createThreadSanitizerPass() -> *mut super::llvm_FunctionPass {
        raw::llvm_createThreadSanitizerPass()
    }

    // ::llvm::getGlobalContext
    #[inline(always)]
    pub unsafe fn getGlobalContext() -> *mut super::llvm_LLVMContext {
        raw::llvm_getGlobalContext()
    }

    // ::llvm::verifyFunction
    #[inline(always)]
    pub unsafe fn verifyFunction(Function: *const super::llvm_Function) -> bool {
        raw::llvm_verifyFunction(Function) != 0
    }

    // ::llvm::verifyModule
    #[inline(always)]
    pub unsafe fn verifyModule(Module: *const super::llvm_Module) -> bool {
        raw::llvm_verifyModule(Module) != 0
    }
}
