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
pub struct llvm_AtomicRMWInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BasicBlock;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_BinaryOperator;
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
pub struct llvm_CallInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_CastInst;
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
pub struct llvm_StringRef {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
impl Copy for llvm_StringRef {}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_DataLayout;
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
pub struct llvm_MetadataAsValue;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Module;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Operator;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_PHINode;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_PointerType;
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
pub struct llvm_User;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_VAArgInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_Value;
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
pub struct std_string_const {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
impl Copy for std_string_const {}
#[repr(C)]
#[allow(raw_pointer_derive)]
pub struct llvm_ArrayRef__libc_uint64_t {
    pub data: *const ::libc::uint64_t,
    pub length: ::libc::size_t,
}
impl Copy for llvm_ArrayRef__libc_uint64_t {}

mod raw {
    extern "C" {
        pub fn llvm_Function_Create(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_Function;
        pub fn llvm_Function_CreateWithName(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes, Name: super::llvm_StringRef) -> *mut super::llvm_Function;
        pub fn llvm_Function_addFnAttr(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Function_addFnAttrWithValue(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef, Val: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_appendModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Constant_canTrap(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Function_cannotDuplicate(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_ArrayType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
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
        pub fn llvm_Function_copyAttributesFrom(inst: *mut super::llvm_Function, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalValue_copyAttributesFrom(inst: *mut super::llvm_GlobalValue, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_copyAttributesFrom(inst: *mut super::llvm_GlobalVariable, Src: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_StructType_create(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_StructType_createPacked(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef, isPacked: ::libc::c_int) -> *mut super::llvm_StructType;
        pub fn llvm_Function_delete(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_delete(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_delete(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
        pub fn llvm_Module_delete(inst: *mut super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_User_delete(inst: *mut super::llvm_User) -> ::libc::c_void;
        pub fn llvm_Value_delete(inst: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_Function_deleteBody(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_BlockAddress_destroyConstant(inst: *mut super::llvm_BlockAddress) -> ::libc::c_void;
        pub fn llvm_Constant_destroyConstant(inst: *mut super::llvm_Constant) -> ::libc::c_void;
        pub fn llvm_ConstantPointerNull_destroyConstant(inst: *mut super::llvm_ConstantPointerNull) -> ::libc::c_void;
        pub fn llvm_GlobalValue_destroyConstant(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_Function_doesNotAccessMemory(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_doesNotAccessMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotAlias(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotCapture(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Function_doesNotReturn(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_doesNotThrow(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_User_dropAllReferences(inst: *mut super::llvm_User) -> ::libc::c_void;
        pub fn llvm_Module_dump(inst: *const super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Type_dump(inst: *const super::llvm_Type) -> ::libc::c_void;
        pub fn llvm_Value_dump(inst: *const super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ConstantInt_equalsInt(inst: *const super::llvm_ConstantInt, Val: ::libc::uint64_t) -> ::libc::c_int;
        pub fn llvm_Function_eraseFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_eraseFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_eraseFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
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
        pub fn llvm_VectorType_get(ty: *mut super::llvm_Type, NumElements: ::libc::c_uint) -> *mut super::llvm_VectorType;
        pub fn llvm_PointerType_getAddressSpace(inst: *const super::llvm_PointerType) -> ::libc::c_uint;
        pub fn llvm_Constant_getAggregateElement(inst: *const super::llvm_Constant, Elt: ::libc::c_uint) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getAggregateElementConstant(inst: *const super::llvm_Constant, Elt: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_GlobalValue_getAlignment(inst: *const super::llvm_GlobalValue) -> ::libc::c_uint;
        pub fn llvm_Constant_getAllOnesValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_BlockAddress_getBasicBlock(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_BasicBlock;
        pub fn llvm_IntegerType_getBitMask(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_ConstantInt_getBitWidth(inst: *const super::llvm_ConstantInt) -> ::libc::c_uint;
        pub fn llvm_IntegerType_getBitWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getBitWidth(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_Function_getCallingConv(inst: *const super::llvm_Function) -> super::llvm_CallingConv_ID;
        pub fn llvm_Type_getContainedType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Function_getContext(inst: *const super::llvm_Function) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_getContext(inst: *const super::llvm_Module) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Type_getContext(inst: *const super::llvm_Type) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Value_getContext(inst: *const super::llvm_Value) -> *mut super::llvm_LLVMContext;
        pub fn llvm_GlobalValue_getDataLayout(inst: *const super::llvm_GlobalValue) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayout(inst: *const super::llvm_Module) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayoutStr(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Function_getDereferenceableBytes(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::uint64_t;
        pub fn llvm_VectorType_getDoubleElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getDoublePtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getDoubleTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_SequentialType_getElementType(inst: *const super::llvm_SequentialType) -> *mut super::llvm_Type;
        pub fn llvm_StructType_getElementType(inst: *const super::llvm_StructType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_VectorType_getExtendedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getFP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getFalse(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_getFalseWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Type_getFloatPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFloatTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_BlockAddress_getFunction(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_Function;
        pub fn llvm_Type_getFunctionNumParams(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getFunctionParamType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Function_getFunctionType(inst: *const super::llvm_Function) -> *mut super::llvm_FunctionType;
        pub fn llvm_getGlobalContext() -> *mut super::llvm_LLVMContext;
        pub fn llvm_VectorType_getHalfElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getHalfPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getHalfTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantFP_getInfinity(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_GlobalVariable_getInitializer(inst: *const super::llvm_GlobalVariable) -> *const super::llvm_Constant;
        pub fn llvm_GlobalVariable_getInitializerMut(inst: *mut super::llvm_GlobalVariable) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getInt16PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt16Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt1PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt1Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt32PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt32Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt64PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt64Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getInt8PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getInt8Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getIntNPtrTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getIntNTy(ctx: *mut super::llvm_LLVMContext, size: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_VectorType_getInteger(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Constant_getIntegerValue(Ty: *mut super::llvm_Type, Value: super::llvm_APInt) -> *mut super::llvm_Constant;
        pub fn llvm_Function_getIntrinsicID(inst: *const super::llvm_Function) -> ::libc::c_uint;
        pub fn llvm_Type_getLabelTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Module_getMDKindID(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> ::libc::c_uint;
        pub fn llvm_Type_getMetadataTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Module_getModuleIdentifier(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getModuleInlineAsm(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_StructType_getName(inst: *const super::llvm_StructType) -> super::llvm_StringRef;
        pub fn llvm_Value_getName(inst: *const super::llvm_Value) -> super::llvm_StringRef;
        pub fn llvm_Module_getNamedValue(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_GlobalValue;
        pub fn llvm_ConstantFP_getNegativeZero(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getNullValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getNumContainedTypes(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_ArrayType_getNumElements(inst: *const super::llvm_ArrayType) -> ::libc::uint64_t;
        pub fn llvm_StructType_getNumElements(inst: *const super::llvm_StructType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getNumElements(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_User_getNumOperands(inst: *const super::llvm_User) -> ::libc::c_uint;
        pub fn llvm_FunctionType_getNumParams(inst: *const super::llvm_FunctionType) -> ::libc::c_uint;
        pub fn llvm_Value_getNumUses(inst: *const super::llvm_Value) -> ::libc::c_uint;
        pub fn llvm_User_getOperand(inst: *const super::llvm_User, idx: ::libc::c_uint) -> *mut super::llvm_Value;
        pub fn llvm_Module_getOrInsertFunction(inst: *mut super::llvm_Module, Name: super::llvm_StringRef, ty: *mut super::llvm_FunctionType) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getPPC_FP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getPPC_FP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Function_getParamAlignment(inst: *const super::llvm_Function, idx: ::libc::c_uint) -> ::libc::c_uint;
        pub fn llvm_FunctionType_getParamType(inst: *const super::llvm_FunctionType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_GlobalValue_getParent(inst: *const super::llvm_GlobalValue) -> *const super::llvm_Module;
        pub fn llvm_GlobalValue_getParentMut(inst: *mut super::llvm_GlobalValue) -> *mut super::llvm_Module;
        pub fn llvm_Type_getPointerAddressSpace(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getPointerElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerTo(inst: *mut super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Function_getReturnType(inst: *const super::llvm_Function) -> *mut super::llvm_Type;
        pub fn llvm_FunctionType_getReturnType(inst: *const super::llvm_FunctionType) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getSExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::int64_t;
        pub fn llvm_Type_getSequentialElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_IntegerType_getSignBit(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_ConstantInt_getSigned(Ty: *mut super::llvm_IntegerType, Value: ::libc::uint64_t, isSigned: ::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_Constant_getSplatValue(inst: *const super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getStructElementType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getStructName(inst: *const super::llvm_Type) -> super::llvm_StringRef;
        pub fn llvm_Type_getStructNumElements(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Module_getTargetTriple(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_ConstantInt_getTrue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
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
        pub fn llvm_PointerType_getUnqual(ElementType: *mut super::llvm_Type) -> *mut super::llvm_PointerType;
        pub fn llvm_Value_getValueID(inst: *const super::llvm_Value) -> ::libc::c_uint;
        pub fn llvm_Type_getVoidTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_FP80PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_FP80Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_MMXPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_MMXTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_ConstantInt_getZExtValue(inst: *const super::llvm_ConstantInt) -> ::libc::uint64_t;
        pub fn llvm_ConstantFP_getZeroValueForNegation(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
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
        pub fn llvm_Value_hasNUses(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Value_hasNUsesOrMore(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_StructType_hasName(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Value_hasName(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_Value_hasOneUse(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasPrivateLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasProtectedVisibility(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasSection(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Function_hasStructRetAttr(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_hasUWTable(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_hasUniqueInitializer(inst: *const super::llvm_GlobalVariable) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasUnnamedAddr(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakAnyLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakODRLinkage(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_CompositeType_indexValid(inst: *const super::llvm_CompositeType, idx: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Type_isAggregateType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_isAllOnesValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Type_isArrayTy(inst: *const super::llvm_Type) -> ::libc::c_int;
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
        pub fn llvm_Type_isIntOrIntVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isIntegerTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Function_isIntrinsic(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Type_isLabelTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isLayoutIdentical(inst: *const super::llvm_StructType, Other: *mut super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_StructType_isLiteral(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMaxValue(inst: *const super::llvm_ConstantInt, isSigned: ::libc::c_int) -> ::libc::c_int;
        pub fn llvm_Type_isMetadataTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_isMinSignedValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMinValue(inst: *const super::llvm_ConstantInt, isSigned: ::libc::c_int) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isMinusOne(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isNaN(inst: *const super::llvm_ConstantFP) -> ::libc::c_int;
        pub fn llvm_ConstantFP_isNegative(inst: *const super::llvm_ConstantFP) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isNegative(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_Constant_isNegativeZeroValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_Constant_isNullValue(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isOne(inst: *const super::llvm_ConstantInt) -> ::libc::c_int;
        pub fn llvm_StructType_isOpaque(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPPC_FP128Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isPacked(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPointerTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_IntegerType_isPowerOf2ByteWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_int;
        pub fn llvm_Type_isPtrOrPtrVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ConstantInt_isSignedValueValidForType(Ty: *mut super::llvm_Type, Val: ::libc::int64_t) -> ::libc::c_int;
        pub fn llvm_Type_isSingleValueType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isSized(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isSized(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isStructTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Constant_isThreadDependent(inst: *const super::llvm_Constant) -> ::libc::c_int;
        pub fn llvm_GlobalValue_isThreadLocal(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Value_isUsedInBasicBlock(inst: *const super::llvm_Value, BB: *const super::llvm_BasicBlock) -> ::libc::c_int;
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
        pub fn llvm_GlobalValue_mayBeOverridden(inst: *const super::llvm_GlobalValue) -> ::libc::c_int;
        pub fn llvm_Value_mutateType(inst: *mut super::llvm_Value, ty: *mut super::llvm_Type) -> ::libc::c_void;
        pub fn llvm_Function_needsUnwindTableEntry(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_GlobalVariable_new(Ty: *mut super::llvm_Type, isConstant: ::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_Module_new(ModuleID: super::llvm_StringRef, Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_Module;
        pub fn llvm_GlobalVariable_newWithModule(Module: *mut super::llvm_Module, Ty: *mut super::llvm_Type, isConstant: ::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes, Initializer: *mut super::llvm_Constant) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_Function_onlyReadsMemory(inst: *const super::llvm_Function) -> ::libc::c_int;
        pub fn llvm_Function_onlyReadsMemoryParam(inst: *const super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Constant_removeDeadConstantUsers(inst: *const super::llvm_Constant) -> ::libc::c_void;
        pub fn llvm_Function_removeFromParent(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalValue_removeFromParent(inst: *mut super::llvm_GlobalValue) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_removeFromParent(inst: *mut super::llvm_GlobalVariable) -> ::libc::c_void;
        pub fn llvm_Value_replaceAllUsesWith(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_User_replaceUsesOfWith(inst: *mut super::llvm_User, From: *mut super::llvm_Value, To: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr) -> ::libc::c_void;
        pub fn llvm_StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr, isPacked: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Function_setCallingConv(inst: *mut super::llvm_Function, CC: super::llvm_CallingConv_ID) -> ::libc::c_void;
        pub fn llvm_Function_setCannotDuplicate(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setConstant(inst: *mut super::llvm_GlobalVariable, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayout(inst: *mut super::llvm_Module, Other: *const super::llvm_DataLayout) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayoutStr(inst: *mut super::llvm_Module, Desc: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAccessMemory(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAccessMemoryParam(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotAlias(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotCapture(inst: *mut super::llvm_Function, n: ::libc::c_uint) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotReturn(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_Function_setDoesNotThrow(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setExternallyInitialized(inst: *mut super::llvm_GlobalVariable, Val: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Function_setHasUWTable(inst: *mut super::llvm_Function) -> ::libc::c_void;
        pub fn llvm_GlobalVariable_setInitializer(inst: *mut super::llvm_GlobalVariable, InitVal: *mut super::llvm_Constant) -> ::libc::c_void;
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
        pub fn llvm_Constant_stripPointerCasts(inst: *const super::llvm_Constant) -> *const super::llvm_Constant;
        pub fn llvm_Constant_stripPointerCastsMut(inst: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Value_takeName(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ConstantInt_uge(inst: *const super::llvm_ConstantInt, Num: ::libc::uint64_t) -> ::libc::c_int;
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

    // ::llvm::Function::Create
    #[inline(always)]
    pub unsafe fn Function_Create(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_Function {
        raw::llvm_Function_Create(Ty, Linkage)
    }

    // ::llvm::Function::CreateWithName
    #[inline(always)]
    pub unsafe fn Function_CreateWithName(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes, Name: super::llvm_StringRef) -> *mut super::llvm_Function {
        raw::llvm_Function_CreateWithName(Ty, Linkage, Name)
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

    // ::llvm::getGlobalContext
    #[inline(always)]
    pub unsafe fn getGlobalContext() -> *mut super::llvm_LLVMContext {
        raw::llvm_getGlobalContext()
    }
}
