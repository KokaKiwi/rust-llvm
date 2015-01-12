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
pub struct llvm_FunctionType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GetElementPtrInst;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_GlobalValue;
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
pub struct llvm_VectorType;
#[repr(C)]
#[derive(Copy)]
pub struct llvm_StringRef {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
#[repr(C)]
#[derive(Copy)]
pub struct llvm_ArrayRef_llvm_Type_ptr {
    pub data: *const *mut llvm_Type,
    pub length: ::libc::size_t,
}
#[repr(C)]
#[derive(Copy)]
pub struct std_string_const {
    pub data: *const ::libc::c_char,
    pub length: ::libc::size_t,
}
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

mod raw {
    extern "C" {
        pub fn llvm_Module_appendModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_ArrayType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_CompositeType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_FunctionType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_IntegerType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_PointerType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_SequentialType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_VectorType_classof(ty: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_create(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_StructType_createPacked(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_llvm_Type_ptr, Name: super::llvm_StringRef, isPacked: ::libc::c_int) -> *mut super::llvm_StructType;
        pub fn llvm_Module_delete(inst: *mut super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Value_delete(inst: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_Module_dump(inst: *const super::llvm_Module) -> ::libc::c_void;
        pub fn llvm_Type_dump(inst: *const super::llvm_Type) -> ::libc::c_void;
        pub fn llvm_Value_dump(inst: *const super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_ArrayType_get(ElementType: *mut super::llvm_Type, NumElements: ::libc::uint64_t) -> *mut super::llvm_ArrayType;
        pub fn llvm_FunctionType_get(Result: *mut super::llvm_Type, Params: super::llvm_ArrayRef_llvm_Type_ptr, isVarArg: ::libc::c_int) -> *mut super::llvm_FunctionType;
        pub fn llvm_IntegerType_get(ctx: *mut super::llvm_LLVMContext, NumBits: ::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_PointerType_get(ElementType: *mut super::llvm_Type, AddressSpace: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_VectorType_get(ty: *mut super::llvm_Type, NumElements: ::libc::c_uint) -> *mut super::llvm_VectorType;
        pub fn llvm_PointerType_getAddressSpace(inst: *const super::llvm_PointerType) -> ::libc::c_uint;
        pub fn llvm_IntegerType_getBitMask(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_IntegerType_getBitWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getBitWidth(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_Type_getContainedType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Module_getContext(inst: *const super::llvm_Module) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Type_getContext(inst: *const super::llvm_Type) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Value_getContext(inst: *const super::llvm_Value) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_getDataLayout(inst: *const super::llvm_Module) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayoutStr(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_VectorType_getDoubleElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getDoublePtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getDoubleTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_SequentialType_getElementType(inst: *const super::llvm_SequentialType) -> *mut super::llvm_Type;
        pub fn llvm_StructType_getElementType(inst: *const super::llvm_StructType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_VectorType_getExtendedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getFP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFloatPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFloatTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFunctionNumParams(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getFunctionParamType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_getGlobalContext() -> *mut super::llvm_LLVMContext;
        pub fn llvm_VectorType_getHalfElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_Type_getHalfPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getHalfTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
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
        pub fn llvm_Type_getLabelTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Module_getMDKindID(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> ::libc::c_uint;
        pub fn llvm_Type_getMetadataTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Module_getModuleIdentifier(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getModuleInlineAsm(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_StructType_getName(inst: *const super::llvm_StructType) -> super::llvm_StringRef;
        pub fn llvm_Value_getName(inst: *const super::llvm_Value) -> super::llvm_StringRef;
        pub fn llvm_Module_getNamedValue(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_GlobalValue;
        pub fn llvm_Type_getNumContainedTypes(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_ArrayType_getNumElements(inst: *const super::llvm_ArrayType) -> ::libc::uint64_t;
        pub fn llvm_StructType_getNumElements(inst: *const super::llvm_StructType) -> ::libc::c_uint;
        pub fn llvm_VectorType_getNumElements(inst: *const super::llvm_VectorType) -> ::libc::c_uint;
        pub fn llvm_FunctionType_getNumParams(inst: *const super::llvm_FunctionType) -> ::libc::c_uint;
        pub fn llvm_Value_getNumUses(inst: *const super::llvm_Value) -> ::libc::c_uint;
        pub fn llvm_Module_getOrInsertFunction(inst: *mut super::llvm_Module, Name: super::llvm_StringRef, ty: *mut super::llvm_FunctionType) -> *mut super::llvm_Constant;
        pub fn llvm_Type_getPPC_FP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getPPC_FP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_FunctionType_getParamType(inst: *const super::llvm_FunctionType, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerAddressSpace(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Type_getPointerElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerTo(inst: *mut super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_FunctionType_getReturnType(inst: *const super::llvm_FunctionType) -> *mut super::llvm_Type;
        pub fn llvm_Type_getSequentialElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_IntegerType_getSignBit(inst: *const super::llvm_IntegerType) -> ::libc::uint64_t;
        pub fn llvm_Type_getStructElementType(inst: *const super::llvm_Type, idx: ::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getStructName(inst: *const super::llvm_Type) -> super::llvm_StringRef;
        pub fn llvm_Type_getStructNumElements(inst: *const super::llvm_Type) -> ::libc::c_uint;
        pub fn llvm_Module_getTargetTriple(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_VectorType_getTruncatedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
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
        pub fn llvm_Value_hasNUses(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Value_hasNUsesOrMore(inst: *const super::llvm_Value, N: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_StructType_hasName(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Value_hasName(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_Value_hasOneUse(inst: *const super::llvm_Value) -> ::libc::c_int;
        pub fn llvm_CompositeType_indexValid(inst: *const super::llvm_CompositeType, idx: ::libc::c_uint) -> ::libc::c_int;
        pub fn llvm_Type_isAggregateType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isArrayTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isDoubleTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isEmptyTy(inst: *const super::llvm_Type) -> ::libc::c_int;
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
        pub fn llvm_Type_isLabelTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isLayoutIdentical(inst: *const super::llvm_StructType, Other: *mut super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_StructType_isLiteral(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isMetadataTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isOpaque(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPPC_FP128Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isPacked(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isPointerTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_IntegerType_isPowerOf2ByteWidth(inst: *const super::llvm_IntegerType) -> ::libc::c_int;
        pub fn llvm_Type_isPtrOrPtrVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isSingleValueType(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isSized(inst: *const super::llvm_StructType) -> ::libc::c_int;
        pub fn llvm_Type_isSized(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isStructTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Value_isUsedInBasicBlock(inst: *const super::llvm_Value, BB: *const super::llvm_BasicBlock) -> ::libc::c_int;
        pub fn llvm_FunctionType_isValidArgumentType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_ArrayType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_PointerType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_StructType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_VectorType_isValidElementType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_FunctionType_isValidReturnType(ty: *mut super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_FunctionType_isVarArg(inst: *const super::llvm_FunctionType) -> ::libc::c_int;
        pub fn llvm_Type_isVectorTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isVoidTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isX86_FP80Ty(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Type_isX86_MMXTy(inst: *const super::llvm_Type) -> ::libc::c_int;
        pub fn llvm_Module_new(ModuleID: super::llvm_StringRef, Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_Module;
        pub fn llvm_Value_replaceAllUsesWith(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
        pub fn llvm_StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr) -> ::libc::c_void;
        pub fn llvm_StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_llvm_Type_ptr, isPacked: ::libc::c_int) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayout(inst: *mut super::llvm_Module, Other: *const super::llvm_DataLayout) -> ::libc::c_void;
        pub fn llvm_Module_setDataLayoutStr(inst: *mut super::llvm_Module, Desc: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_setModuleIdentifier(inst: *mut super::llvm_Module, ID: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_setModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_StructType_setName(inst: *mut super::llvm_StructType, Name: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Value_setName(inst: *mut super::llvm_Value, Name: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Module_setTargetTriple(inst: *mut super::llvm_Module, Triple: super::llvm_StringRef) -> ::libc::c_void;
        pub fn llvm_Value_takeName(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value) -> ::libc::c_void;
    }
}

pub mod llvm {
    use super::raw;

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
