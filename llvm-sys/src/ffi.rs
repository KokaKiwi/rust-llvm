#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

#[repr(C)]
pub struct llvm_Value;
#[repr(C)]
pub struct llvm_Type;
#[repr(C)]
pub struct llvm_LLVMContext;
#[repr(C)]
pub struct llvm_PointerType;
#[repr(C)]
pub struct llvm_IntegerType;
#[repr(C)]
pub struct llvm_BasicBlock;
#[repr(C)]
pub struct llvm_Function;
#[repr(C)]
pub struct llvm_GlobalValue;
#[repr(C)]
pub struct llvm_Module;
#[repr(C)]
pub struct llvm_DataLayout;
#[repr(C)]
pub struct llvm_Constant;
#[repr(C)]
pub struct llvm_Use;
#[repr(C)]
pub struct llvm_User;
#[repr(C)]
pub struct llvm_FunctionType;
#[repr(C)]
pub struct llvm_StructType;
#[repr(C)]
pub struct llvm_Instruction;
#[repr(C)]
pub struct llvm_MDNode;
#[repr(C)]
pub struct llvm_DebugLoc;
#[repr(C)]
pub struct llvm_ValueSymbolTable;
#[repr(C)]
pub struct llvm_LandingPadInst;
#[repr(C)]
pub struct llvm_TerminatorInst;
#[repr(C)]
pub struct llvm_Argument;
#[repr(C)]
pub struct llvm_InlineAsm;
#[repr(C)]
pub struct llvm_MDString;
#[repr(C)]
pub struct llvm_Operator;
#[repr(C)]
pub struct llvm_BlockAddress;
#[repr(C)]
pub struct llvm_ConstantAggregateZero;
#[repr(C)]
pub struct llvm_ConstantArray;
#[repr(C)]
pub struct llvm_ArrayType;
#[repr(C)]
pub struct llvm_ConstantDataSequential;
#[repr(C)]
pub struct llvm_ConstantExpr;
#[repr(C)]
pub struct llvm_ConstantFP;
#[repr(C)]
pub struct llvm_ConstantInt;
#[repr(C)]
pub struct llvm_ConstantPointerNull;
#[repr(C)]
pub struct llvm_ConstantStruct;
#[repr(C)]
pub struct llvm_ConstantVector;
#[repr(C)]
pub struct llvm_UndefValue;
#[repr(C)]
pub struct llvm_ConstantDataArray;
#[repr(C)]
pub struct llvm_ConstantDataVector;
#[repr(C)]
pub struct llvm_GlobalAlias;
#[repr(C)]
pub struct llvm_GlobalObject;
#[repr(C)]
pub struct llvm_GlobalVariable;
#[repr(C)]
pub struct llvm_AtomicCmpXchgInst;
#[repr(C)]
pub struct llvm_AtomicRMWInst;
#[repr(C)]
pub struct llvm_BinaryOperator;
#[repr(C)]
pub struct llvm_CallInst;
#[repr(C)]
pub struct llvm_CmpInst;
#[repr(C)]
pub struct llvm_ExtractElementInst;
#[repr(C)]
pub struct llvm_FenceInst;
#[repr(C)]
pub struct llvm_GetElementPtrInst;
#[repr(C)]
pub struct llvm_InsertElementInst;
#[repr(C)]
pub struct llvm_InsertValueInst;
#[repr(C)]
pub struct llvm_PHINode;
#[repr(C)]
pub struct llvm_SelectInst;
#[repr(C)]
pub struct llvm_ShuffleVectorInst;
#[repr(C)]
pub struct llvm_StoreInst;
#[repr(C)]
pub struct llvm_UnaryInstruction;
#[repr(C)]
pub struct llvm_BranchInst;
#[repr(C)]
pub struct llvm_IndirectBrInst;
#[repr(C)]
pub struct llvm_InvokeInst;
#[repr(C)]
pub struct llvm_ResumeInst;
#[repr(C)]
pub struct llvm_ReturnInst;
#[repr(C)]
pub struct llvm_SwitchInst;
#[repr(C)]
pub struct llvm_UnreachableInst;
#[repr(C)]
pub struct llvm_AllocaInst;
#[repr(C)]
pub struct llvm_CastInst;
#[repr(C)]
pub struct llvm_ExtractValueInst;
#[repr(C)]
pub struct llvm_LoadInst;
#[repr(C)]
pub struct llvm_VAArgInst;
#[repr(C)]
pub struct llvm_AddrSpaceCastInst;
#[repr(C)]
pub struct llvm_BitCastInst;
#[repr(C)]
pub struct llvm_FPExtInst;
#[repr(C)]
pub struct llvm_FPToSIInst;
#[repr(C)]
pub struct llvm_CompositeType;
#[repr(C)]
pub struct llvm_SequentialType;
#[repr(C)]
pub struct llvm_VectorType;
#[repr(C)]
pub struct llvm_Pass;
#[repr(C)]
pub struct llvm_BasicBlockPass;
#[repr(C)]
pub struct llvm_CallGraphSCCPass;
#[repr(C)]
pub struct llvm_FunctionPass;
#[repr(C)]
pub struct llvm_LoopPass;
#[repr(C)]
pub struct llvm_ModulePass;
#[repr(C)]
pub struct llvm_RegionPass;
#[repr(C)]
pub struct llvm_FunctionPassManager;
#[repr(C)]
pub struct llvm_PassManager;
#[repr(C)]
pub struct llvm_IRBuilderBase;
#[repr(C)]
pub struct llvm_IRBuilder;
pub type llvm_CallingConv_ID = libc::c_int;
pub type llvm_Value_ValueTy = libc::c_int;
pub type llvm_Type_TypeID = libc::c_int;
pub type llvm_GlobalValue_LinkageTypes = libc::c_int;
pub type llvm_Instruction_CastOps = libc::c_int;
pub type llvm_Instruction_OtherOps = libc::c_int;
pub type llvm_Instruction_MemoryOps = libc::c_int;
pub type llvm_Instruction_BinaryOps = libc::c_int;
pub type llvm_Instruction_TermOps = libc::c_int;
pub type llvm_CmpInst_Predicate = libc::c_int;
pub type llvm_PassKind = libc::c_int;
pub type llvm_AtomicOrdering = libc::c_int;
pub type llvm_SynchronizationScope = libc::c_int;
pub type llvm_PassManagerType = libc::c_int;

#[repr(C)]
pub struct llvm_StringRef {
    pub data: *const libc::c_char,
    pub length: libc::size_t,
}

#[repr(C)]
pub struct std_string_const {
    pub data: *const libc::c_char,
    pub length: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_uint64 {
    pub data: *const libc::uint64_t,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_APInt {
    pub numbits: libc::c_uint,
    pub data: llvm_ArrayRef_uint64,
}

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Type {
    pub data: *const *mut llvm_Type,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct std_string {
    pub data: *mut libc::c_char,
    pub length: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_uint {
    pub data: *const libc::c_uint,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Constant {
    pub data: *const *mut llvm_Constant,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Value {
    pub data: *const *mut llvm_Value,
    pub size: libc::size_t,
}

pub mod raw {
    #[link(name = "LLVMLTO")]
    #[link(name = "LLVMObjCARCOpts")]
    #[link(name = "LLVMLinker")]
    #[link(name = "LLVMipo")]
    #[link(name = "LLVMVectorize")]
    #[link(name = "LLVMBitWriter")]
    #[link(name = "LLVMIRReader")]
    #[link(name = "LLVMAsmParser")]
    #[link(name = "LLVMR600CodeGen")]
    #[link(name = "LLVMR600Desc")]
    #[link(name = "LLVMR600Info")]
    #[link(name = "LLVMR600AsmPrinter")]
    #[link(name = "LLVMSystemZDisassembler")]
    #[link(name = "LLVMSystemZCodeGen")]
    #[link(name = "LLVMSystemZAsmParser")]
    #[link(name = "LLVMSystemZDesc")]
    #[link(name = "LLVMSystemZInfo")]
    #[link(name = "LLVMSystemZAsmPrinter")]
    #[link(name = "LLVMHexagonCodeGen")]
    #[link(name = "LLVMHexagonAsmPrinter")]
    #[link(name = "LLVMHexagonDesc")]
    #[link(name = "LLVMHexagonInfo")]
    #[link(name = "LLVMNVPTXCodeGen")]
    #[link(name = "LLVMNVPTXDesc")]
    #[link(name = "LLVMNVPTXInfo")]
    #[link(name = "LLVMNVPTXAsmPrinter")]
    #[link(name = "LLVMCppBackendCodeGen")]
    #[link(name = "LLVMCppBackendInfo")]
    #[link(name = "LLVMMSP430CodeGen")]
    #[link(name = "LLVMMSP430Desc")]
    #[link(name = "LLVMMSP430Info")]
    #[link(name = "LLVMMSP430AsmPrinter")]
    #[link(name = "LLVMXCoreDisassembler")]
    #[link(name = "LLVMXCoreCodeGen")]
    #[link(name = "LLVMXCoreDesc")]
    #[link(name = "LLVMXCoreInfo")]
    #[link(name = "LLVMXCoreAsmPrinter")]
    #[link(name = "LLVMMipsDisassembler")]
    #[link(name = "LLVMMipsCodeGen")]
    #[link(name = "LLVMMipsAsmParser")]
    #[link(name = "LLVMMipsDesc")]
    #[link(name = "LLVMMipsInfo")]
    #[link(name = "LLVMMipsAsmPrinter")]
    #[link(name = "LLVMAArch64Disassembler")]
    #[link(name = "LLVMAArch64CodeGen")]
    #[link(name = "LLVMAArch64AsmParser")]
    #[link(name = "LLVMAArch64Desc")]
    #[link(name = "LLVMAArch64Info")]
    #[link(name = "LLVMAArch64AsmPrinter")]
    #[link(name = "LLVMAArch64Utils")]
    #[link(name = "LLVMARMDisassembler")]
    #[link(name = "LLVMARMCodeGen")]
    #[link(name = "LLVMARMAsmParser")]
    #[link(name = "LLVMARMDesc")]
    #[link(name = "LLVMARMInfo")]
    #[link(name = "LLVMARMAsmPrinter")]
    #[link(name = "LLVMPowerPCDisassembler")]
    #[link(name = "LLVMPowerPCCodeGen")]
    #[link(name = "LLVMPowerPCAsmParser")]
    #[link(name = "LLVMPowerPCDesc")]
    #[link(name = "LLVMPowerPCInfo")]
    #[link(name = "LLVMPowerPCAsmPrinter")]
    #[link(name = "LLVMSparcDisassembler")]
    #[link(name = "LLVMSparcCodeGen")]
    #[link(name = "LLVMSparcAsmParser")]
    #[link(name = "LLVMSparcDesc")]
    #[link(name = "LLVMSparcInfo")]
    #[link(name = "LLVMSparcAsmPrinter")]
    #[link(name = "LLVMTableGen")]
    #[link(name = "LLVMDebugInfo")]
    #[link(name = "LLVMOption")]
    #[link(name = "LLVMX86Disassembler")]
    #[link(name = "LLVMX86AsmParser")]
    #[link(name = "LLVMX86CodeGen")]
    #[link(name = "LLVMSelectionDAG")]
    #[link(name = "LLVMAsmPrinter")]
    #[link(name = "LLVMX86Desc")]
    #[link(name = "LLVMX86Info")]
    #[link(name = "LLVMX86AsmPrinter")]
    #[link(name = "LLVMX86Utils")]
    #[link(name = "LLVMJIT")]
    #[link(name = "LLVMLineEditor")]
    #[link(name = "LLVMMCAnalysis")]
    #[link(name = "LLVMMCDisassembler")]
    #[link(name = "LLVMInstrumentation")]
    #[link(name = "LLVMInterpreter")]
    #[link(name = "LLVMCodeGen")]
    #[link(name = "LLVMScalarOpts")]
    #[link(name = "LLVMInstCombine")]
    #[link(name = "LLVMTransformUtils")]
    #[link(name = "LLVMipa")]
    #[link(name = "LLVMAnalysis")]
    #[link(name = "LLVMProfileData")]
    #[link(name = "LLVMMCJIT")]
    #[link(name = "LLVMTarget")]
    #[link(name = "LLVMRuntimeDyld")]
    #[link(name = "LLVMObject")]
    #[link(name = "LLVMMCParser")]
    #[link(name = "LLVMBitReader")]
    #[link(name = "LLVMExecutionEngine")]
    #[link(name = "LLVMMC")]
    #[link(name = "LLVMCore")]
    #[link(name = "LLVMSupport")]
    #[link(name = "z")]
    #[link(name = "pthread")]
    #[link(name = "ffi")]
    #[link(name = "curses")]
    #[link(name = "dl")]
    #[link(name = "m")]
    #[link(name = "stdc++")]
    extern "C" {
        pub fn llvm_createAddDiscriminatorsPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAddressSanitizerFunctionPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAddressSanitizerModulePass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createAggressiveDCEPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createAlwaysInlinerPass(InsertLifetime: *const super::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createArgumentPromotionPass(maxElements: *const super::libc::c_uint) -> *mut super::llvm_Pass;
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
        pub fn llvm_createFunctionInliningPassWithOptLevel(OptLevel: super::libc::c_uint, SizeOptLevel: super::libc::c_uint) -> *mut super::llvm_Pass;
        pub fn llvm_createFunctionInliningPass() -> *mut super::llvm_Pass;
        pub fn llvm_createGCOVProfilerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createGVNPass(NoLoads: *const super::libc::c_int) -> *mut super::llvm_FunctionPass;
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
        pub fn llvm_createLoopRotatePass(MaxHeaderSize: *const super::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createLoopSimplifyPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopStrengthReducePass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopUnrollPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLoopUnswitchPass(OptimizeForSize: *const super::libc::c_int) -> *mut super::llvm_Pass;
        pub fn llvm_createLowerAtomicPass() -> *mut super::llvm_Pass;
        pub fn llvm_createLowerExpectIntrinsicPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createLowerInvokePass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createLowerSwitchPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMemCpyOptPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMemorySanitizerPass(TrackOrigins: *const super::libc::c_int) -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMergeFunctionsPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createMergedLoadStoreMotionPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createMetaRenamerPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createObjCARCAPElimPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCContractPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCExpandPass() -> *mut super::llvm_Pass;
        pub fn llvm_createObjCARCOptPass() -> *mut super::llvm_Pass;
        pub fn llvm_createPartialInliningPass() -> *mut super::llvm_ModulePass;
        pub fn llvm_createPartiallyInlineLibCallsPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createPromoteMemoryToRegisterPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createPruneEHPass() -> *mut super::llvm_Pass;
        pub fn llvm_createReassociatePass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSCCPPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createSROAPass(RequiresDomTree: *const super::libc::c_int) -> *mut super::llvm_FunctionPass;
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
        pub fn llvm_createStripSymbolsPass(OnlyDebugInfo: *const super::libc::c_int) -> *mut super::llvm_ModulePass;
        pub fn llvm_createStructurizeCFGPass() -> *mut super::llvm_Pass;
        pub fn llvm_createTailCallEliminationPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_createThreadSanitizerPass() -> *mut super::llvm_FunctionPass;
        pub fn llvm_getGlobalContext() -> *mut super::llvm_LLVMContext;
        pub fn llvm_verifyFunction(Function: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_verifyModule(Module: *const super::llvm_Module) -> super::libc::c_int;
        pub fn llvm_ArrayType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_ArrayType_get(ElementType: *mut super::llvm_Type, NumElements: super::libc::uint64_t) -> *mut super::llvm_ArrayType;
        pub fn llvm_ArrayType_getNumElements(inst: *const super::llvm_ArrayType) -> super::libc::uint64_t;
        pub fn llvm_ArrayType_isValidElementType(ty: *mut super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_BasicBlock_Create(Context: *mut super::llvm_LLVMContext, Name: *const super::std_string, Parent: *const *mut super::llvm_Function, InsertBefore: *const *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_classof(Val: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_BasicBlock_delete(inst: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_dropAllReferences(inst: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_eraseFromParent(inst: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_getDataLayout(inst: *const super::llvm_BasicBlock) -> *const super::llvm_DataLayout;
        pub fn llvm_BasicBlock_getFirstNonPHIMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHI(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbg(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getLandingPadInstMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_LandingPadInst;
        pub fn llvm_BasicBlock_getLandingPadInst(inst: *const super::llvm_BasicBlock) -> *const super::llvm_LandingPadInst;
        pub fn llvm_BasicBlock_getParent(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Function;
        pub fn llvm_BasicBlock_getParentMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Function;
        pub fn llvm_BasicBlock_getSinglePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getSinglePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getTerminator(inst: *const super::llvm_BasicBlock) -> *const super::llvm_TerminatorInst;
        pub fn llvm_BasicBlock_getTerminatorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_TerminatorInst;
        pub fn llvm_BasicBlock_getUniquePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getUniquePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getValueSymbolTable(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_BasicBlock_hasAddressTaken(inst: *const super::llvm_BasicBlock) -> super::libc::c_int;
        pub fn llvm_BasicBlock_isLandingPad(inst: *const super::llvm_BasicBlock) -> super::libc::c_int;
        pub fn llvm_BasicBlock_moveAfter(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_moveBefore(inst: *mut super::llvm_BasicBlock, MovePos: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_removeFromParent(inst: *mut super::llvm_BasicBlock);
        pub fn llvm_BasicBlock_removePredecessor(inst: *mut super::llvm_BasicBlock, Pred: *mut super::llvm_BasicBlock, DontDeleteUselessPHIs: *const super::libc::c_int);
        pub fn llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst: *mut super::llvm_BasicBlock, New: *mut super::llvm_BasicBlock);
        pub fn llvm_BlockAddress_destroyConstant(inst: *mut super::llvm_BlockAddress);
        pub fn llvm_BlockAddress_getBasicBlock(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_BasicBlock;
        pub fn llvm_BlockAddress_getFunction(inst: *const super::llvm_BlockAddress) -> *mut super::llvm_Function;
        pub fn llvm_CompositeType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_CompositeType_getTypeAtIndex(inst: *mut super::llvm_CompositeType, idx: super::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_CompositeType_indexValid(inst: *const super::llvm_CompositeType, idx: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Constant_canTrap(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_classof(V: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_Constant_destroyConstant(inst: *mut super::llvm_Constant);
        pub fn llvm_Constant_getAggregateElement(inst: *const super::llvm_Constant, Elt: super::libc::c_uint) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getAggregateElementConstant(inst: *const super::llvm_Constant, Elt: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getAllOnesValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getIntegerValue(Ty: *mut super::llvm_Type, Value: super::llvm_APInt) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getNullValue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_getSplatValue(inst: *const super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_Constant_isAllOnesValue(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isConstantUsed(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isDLLImportDependent(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isMinSignedValue(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isNegativeZeroValue(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isNullValue(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isThreadDependent(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_isZeroValue(inst: *const super::llvm_Constant) -> super::libc::c_int;
        pub fn llvm_Constant_removeDeadConstantUsers(inst: *const super::llvm_Constant);
        pub fn llvm_Constant_replaceUsesOfWithOnConstant(inst: *mut super::llvm_Constant, arg_1: *mut super::llvm_Value, arg_2: *mut super::llvm_Value, arg_3: *mut super::llvm_Use);
        pub fn llvm_Constant_stripPointerCasts(inst: *const super::llvm_Constant) -> *const super::llvm_Constant;
        pub fn llvm_Constant_stripPointerCastsMut(inst: *mut super::llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantArray_classof(V: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantArray_get(Ty: *mut super::llvm_ArrayType, Values: super::llvm_ArrayRef_ptr_llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantArray_getType(inst: *const super::llvm_ConstantArray) -> *mut super::llvm_Type;
        pub fn llvm_ConstantFP_classof(V: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantFP_get(Ty: *mut super::llvm_Type, Val: super::libc::c_double) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_fromStr(Ty: *mut super::llvm_Type, Val: super::llvm_StringRef) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_getInfinity(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_getNegativeZero(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_getZeroValueForNegation(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_isExactlyValueFloat(inst: *const super::llvm_ConstantFP, Val: super::libc::c_double) -> super::libc::c_int;
        pub fn llvm_ConstantFP_isNaN(inst: *const super::llvm_ConstantFP) -> super::libc::c_int;
        pub fn llvm_ConstantFP_isNegative(inst: *const super::llvm_ConstantFP) -> super::libc::c_int;
        pub fn llvm_ConstantFP_isZero(inst: *const super::llvm_ConstantFP) -> super::libc::c_int;
        pub fn llvm_ConstantInt_classof(Val: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantInt_equalsInt(inst: *const super::llvm_ConstantInt, Val: super::libc::uint64_t) -> super::libc::c_int;
        pub fn llvm_ConstantInt_fromAPInt(Context: *mut super::llvm_LLVMContext, Val: super::llvm_APInt) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_get(Ty: *mut super::llvm_IntegerType, Value: super::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_fromStr(Ty: *mut super::llvm_IntegerType, Str: super::llvm_StringRef, radix: super::libc::uint8_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getSigned(Ty: *mut super::llvm_IntegerType, Value: super::libc::uint64_t, isSigned: super::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getBitWidth(inst: *const super::llvm_ConstantInt) -> super::libc::c_uint;
        pub fn llvm_ConstantInt_getFalse(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_getFalseWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getSExtValue(inst: *const super::llvm_ConstantInt) -> super::libc::int64_t;
        pub fn llvm_ConstantInt_getTrue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_getTrueWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getType(inst: *const super::llvm_ConstantInt) -> *mut super::llvm_IntegerType;
        pub fn llvm_ConstantInt_getZExtValue(inst: *const super::llvm_ConstantInt) -> super::libc::uint64_t;
        pub fn llvm_ConstantInt_isMaxValue(inst: *const super::llvm_ConstantInt, isSigned: super::libc::c_int) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isMinValue(inst: *const super::llvm_ConstantInt, isSigned: super::libc::c_int) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isMinusOne(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isNegative(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isOne(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isValueValidForType(Ty: *mut super::llvm_Type, Val: super::libc::uint64_t) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isSignedValueValidForType(Ty: *mut super::llvm_Type, Val: super::libc::int64_t) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isZero(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_uge(inst: *const super::llvm_ConstantInt, Num: super::libc::uint64_t) -> super::libc::c_int;
        pub fn llvm_ConstantPointerNull_classof(Val: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantPointerNull_destroyConstant(inst: *mut super::llvm_ConstantPointerNull);
        pub fn llvm_ConstantPointerNull_get(Ty: *mut super::llvm_PointerType) -> *mut super::llvm_ConstantPointerNull;
        pub fn llvm_ConstantPointerNull_getType(inst: *const super::llvm_ConstantPointerNull) -> *mut super::llvm_PointerType;
        pub fn llvm_Function_Create(Ty: *mut super::llvm_FunctionType, Linkage: super::llvm_GlobalValue_LinkageTypes, Name: *const super::std_string, Module: *const *mut super::llvm_Module) -> *mut super::llvm_Function;
        pub fn llvm_Function_addFnAttr(inst: *mut super::llvm_Function, Kind: super::llvm_StringRef, Val: *const super::llvm_StringRef);
        pub fn llvm_Function_cannotDuplicate(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_classof(Val: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_Function_clearGC(inst: *mut super::llvm_Function);
        pub fn llvm_Function_copyAttributesFrom(inst: *mut super::llvm_Function, Src: *mut super::llvm_GlobalValue);
        pub fn llvm_Function_delete(inst: *mut super::llvm_Function);
        pub fn llvm_Function_deleteBody(inst: *mut super::llvm_Function);
        pub fn llvm_Function_doesNotAccessMemory(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_doesNotAccessMemoryParam(inst: *const super::llvm_Function, n: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Function_doesNotAlias(inst: *const super::llvm_Function, n: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Function_doesNotCapture(inst: *const super::llvm_Function, n: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Function_doesNotReturn(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_doesNotThrow(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_eraseFromParent(inst: *mut super::llvm_Function);
        pub fn llvm_Function_getCallingConv(inst: *const super::llvm_Function) -> super::llvm_CallingConv_ID;
        pub fn llvm_Function_getContext(inst: *const super::llvm_Function) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Function_getDereferenceableBytes(inst: *const super::llvm_Function, idx: super::libc::c_uint) -> super::libc::uint64_t;
        pub fn llvm_Function_getFunctionType(inst: *const super::llvm_Function) -> *mut super::llvm_FunctionType;
        pub fn llvm_Function_getIntrinsicID(inst: *const super::llvm_Function) -> super::libc::c_uint;
        pub fn llvm_Function_getParamAlignment(inst: *const super::llvm_Function, idx: super::libc::c_uint) -> super::libc::c_uint;
        pub fn llvm_Function_getReturnType(inst: *const super::llvm_Function) -> *mut super::llvm_Type;
        pub fn llvm_Function_hasFnAttr(inst: *const super::llvm_Function, Kind: super::llvm_StringRef) -> super::libc::c_int;
        pub fn llvm_Function_hasGC(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_hasStructRetAttr(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_hasUWTable(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_isIntrinsic(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_isVarArg(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_needsUnwindTableEntry(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_onlyReadsMemoryParam(inst: *const super::llvm_Function, n: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Function_onlyReadsMemory(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_removeFromParent(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setCallingConv(inst: *mut super::llvm_Function, CC: super::llvm_CallingConv_ID);
        pub fn llvm_Function_setCannotDuplicate(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setDoesNotAccessMemory(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setDoesNotAccessMemoryParam(inst: *mut super::llvm_Function, n: super::libc::c_uint);
        pub fn llvm_Function_setDoesNotAlias(inst: *mut super::llvm_Function, n: super::libc::c_uint);
        pub fn llvm_Function_setDoesNotCapture(inst: *mut super::llvm_Function, n: super::libc::c_uint);
        pub fn llvm_Function_setDoesNotReturn(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setDoesNotThrow(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setHasUWTable(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setOnlyReadsMemory(inst: *mut super::llvm_Function);
        pub fn llvm_Function_setOnlyReadsMemoryParam(inst: *mut super::llvm_Function, n: super::libc::c_uint);
        pub fn llvm_FunctionPassManager_add(inst: *mut super::llvm_FunctionPassManager, Pass: *mut super::llvm_FunctionPass);
        pub fn llvm_FunctionPassManager_doFinalization(inst: *mut super::llvm_FunctionPassManager) -> super::libc::c_int;
        pub fn llvm_FunctionPassManager_doInitialization(inst: *mut super::llvm_FunctionPassManager) -> super::libc::c_int;
        pub fn llvm_FunctionPassManager_new(Module: *mut super::llvm_Module) -> *mut super::llvm_FunctionPassManager;
        pub fn llvm_FunctionPassManager_run(inst: *mut super::llvm_FunctionPassManager, Function: *mut super::llvm_Function);
        pub fn llvm_FunctionType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_FunctionType_get(Result: *mut super::llvm_Type, Params: super::llvm_ArrayRef_ptr_llvm_Type, isVarArg: super::libc::c_int) -> *mut super::llvm_FunctionType;
        pub fn llvm_FunctionType_getNumParams(inst: *const super::llvm_FunctionType) -> super::libc::c_uint;
        pub fn llvm_FunctionType_getParamType(inst: *const super::llvm_FunctionType, idx: super::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_FunctionType_getReturnType(inst: *const super::llvm_FunctionType) -> *mut super::llvm_Type;
        pub fn llvm_FunctionType_isValidArgumentType(ty: *mut super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_FunctionType_isValidReturnType(ty: *mut super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_FunctionType_isVarArg(inst: *const super::llvm_FunctionType) -> super::libc::c_int;
        pub fn llvm_GlobalObject_setSection(inst: *mut super::llvm_GlobalObject, S: super::llvm_StringRef);
        pub fn llvm_GlobalValue_copyAttributesFrom(inst: *mut super::llvm_GlobalValue, Src: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalValue_delete(inst: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalValue_destroyConstant(inst: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalValue_eraseFromParent(inst: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalValue_getAlignment(inst: *const super::llvm_GlobalValue) -> super::libc::c_uint;
        pub fn llvm_GlobalValue_getDataLayout(inst: *const super::llvm_GlobalValue) -> *const super::llvm_DataLayout;
        pub fn llvm_GlobalValue_getParent(inst: *const super::llvm_GlobalValue) -> *const super::llvm_Module;
        pub fn llvm_GlobalValue_getParentMut(inst: *mut super::llvm_GlobalValue) -> *mut super::llvm_Module;
        pub fn llvm_GlobalValue_getType(inst: *const super::llvm_GlobalValue) -> *mut super::llvm_PointerType;
        pub fn llvm_GlobalValue_hasAppendingLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasAvailableExternallyLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasCommonLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasDLLExportStorageClass(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasDLLImportStorageClass(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasDefaultVisibility(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasExternalLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasExternalWeakLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasHiddenVisibility(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasInternalLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasLinkOnceLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasLocalLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasPrivateLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasProtectedVisibility(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasSection(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasUnnamedAddr(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakAnyLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_hasWeakODRLinkage(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_isDeclaration(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_isDiscardableIfUnused(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_isThreadLocal(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_isWeakForLinker(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_mayBeOverridden(inst: *const super::llvm_GlobalValue) -> super::libc::c_int;
        pub fn llvm_GlobalValue_removeFromParent(inst: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalValue_setThreadLocal(inst: *mut super::llvm_GlobalValue, Val: super::libc::c_int);
        pub fn llvm_GlobalValue_setUnnamedAddr(inst: *mut super::llvm_GlobalValue, Val: super::libc::c_int);
        pub fn llvm_GlobalVariable_copyAttributesFrom(inst: *mut super::llvm_GlobalVariable, Src: *mut super::llvm_GlobalValue);
        pub fn llvm_GlobalVariable_delete(inst: *mut super::llvm_GlobalVariable);
        pub fn llvm_GlobalVariable_eraseFromParent(inst: *mut super::llvm_GlobalVariable);
        pub fn llvm_GlobalVariable_getInitializer(inst: *const super::llvm_GlobalVariable) -> *const super::llvm_Constant;
        pub fn llvm_GlobalVariable_getInitializerMut(inst: *mut super::llvm_GlobalVariable) -> *mut super::llvm_Constant;
        pub fn llvm_GlobalVariable_hasDefinitiveInitializer(inst: *const super::llvm_GlobalVariable) -> super::libc::c_int;
        pub fn llvm_GlobalVariable_hasInitializer(inst: *const super::llvm_GlobalVariable) -> super::libc::c_int;
        pub fn llvm_GlobalVariable_hasUniqueInitializer(inst: *const super::llvm_GlobalVariable) -> super::libc::c_int;
        pub fn llvm_GlobalVariable_isConstant(inst: *const super::llvm_GlobalVariable) -> super::libc::c_int;
        pub fn llvm_GlobalVariable_isExternallyInitialized(inst: *const super::llvm_GlobalVariable) -> super::libc::c_int;
        pub fn llvm_GlobalVariable_new(Ty: *mut super::llvm_Type, isConstant: super::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_GlobalVariable_newWithModule(Module: *mut super::llvm_Module, Ty: *mut super::llvm_Type, isConstant: super::libc::c_int, Linkage: super::llvm_GlobalValue_LinkageTypes, Initializer: *mut super::llvm_Constant) -> *mut super::llvm_GlobalVariable;
        pub fn llvm_GlobalVariable_removeFromParent(inst: *mut super::llvm_GlobalVariable);
        pub fn llvm_GlobalVariable_setConstant(inst: *mut super::llvm_GlobalVariable, Val: super::libc::c_int);
        pub fn llvm_GlobalVariable_setExternallyInitialized(inst: *mut super::llvm_GlobalVariable, Val: super::libc::c_int);
        pub fn llvm_GlobalVariable_setInitializer(inst: *mut super::llvm_GlobalVariable, InitVal: *mut super::llvm_Constant);
        pub fn llvm_IRBuilder_CreateAShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAlignedLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: super::libc::c_int, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: *const super::libc::c_int) -> *mut super::llvm_StoreInst;
        pub fn llvm_IRBuilder_CreateAlloca(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, ArraySize: *const *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_AllocaInst;
        pub fn llvm_IRBuilder_CreateAndByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAnd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBinOp(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_BinaryOps, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateBr(inst: *mut super::llvm_IRBuilder, Dest: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst;
        pub fn llvm_IRBuilder_CreateCall(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, Args: super::llvm_ArrayRef_ptr_llvm_Value, Name: *const super::std_string) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilder_CreateCast(inst: *mut super::llvm_IRBuilder, Opcode: super::llvm_Instruction_CastOps, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateCondBr(inst: *mut super::llvm_IRBuilder, Cond: *mut super::llvm_Value, TrueBlock: *mut super::llvm_BasicBlock, FalseBlock: *mut super::llvm_BasicBlock) -> *mut super::llvm_BranchInst;
        pub fn llvm_IRBuilder_CreateExactSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExactUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractInteger(inst: *mut super::llvm_IRBuilder, DL: *const super::llvm_DataLayout, From: *mut super::llvm_Value, ExtractedTy: *mut super::llvm_IntegerType, Offset: super::libc::uint64_t, Name: super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateExtractValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_uint, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpOLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpONE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpORD(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFCmpUNO(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPToSI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPToUI(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFPTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateFence(inst: *mut super::llvm_IRBuilder, Ordering: super::llvm_AtomicOrdering, SynchScope: *const super::llvm_SynchronizationScope, Name: *const super::std_string) -> *mut super::llvm_FenceInst;
        pub fn llvm_IRBuilder_CreateGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_ptr_llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateGlobalStringPtr(inst: *mut super::llvm_IRBuilder, Str: super::llvm_StringRef, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmp(inst: *mut super::llvm_IRBuilder, Pred: super::llvm_CmpInst_Predicate, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpEQ(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpNE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSLE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpSLT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpUGE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpUGT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpULE(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateICmpULT(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInBoundsGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_ptr_llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIndirectBr(inst: *mut super::llvm_IRBuilder, Addr: *mut super::llvm_Value, NumCases: *const super::libc::c_uint) -> *mut super::llvm_IndirectBrInst;
        pub fn llvm_IRBuilder_CreateInsertElement(inst: *mut super::llvm_IRBuilder, Vec: *mut super::llvm_Value, NewElt: *mut super::llvm_Value, Idx: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInsertValue(inst: *mut super::llvm_IRBuilder, Agg: *mut super::llvm_Value, Value: *mut super::llvm_Value, Indexes: super::llvm_ArrayRef_uint, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIntCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, isSigned: super::libc::c_int, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIntToPtr(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateInvoke(inst: *mut super::llvm_IRBuilder, Callee: *mut super::llvm_Value, NormalDest: *mut super::llvm_BasicBlock, UnwindDest: *mut super::llvm_BasicBlock, Args: super::llvm_ArrayRef_ptr_llvm_Value, Name: *const super::std_string_const) -> *mut super::llvm_InvokeInst;
        pub fn llvm_IRBuilder_CreateIsNotNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateIsNull(inst: *mut super::llvm_IRBuilder, Arg: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateLandingPad(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, PersFn: *mut super::llvm_Value, NumClauses: super::libc::c_uint, Name: *const super::std_string) -> *mut super::llvm_LandingPadInst;
        pub fn llvm_IRBuilder_CreateLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, isVolatile: super::libc::c_int, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNSWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWMul(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNUWSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNeg(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateNot(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateOrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateOr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePHI(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, NumReservedValues: super::libc::c_uint, Name: *const super::std_string) -> *mut super::llvm_PHINode;
        pub fn llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePointerCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePtrDiff(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreatePtrToInt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateResume(inst: *mut super::llvm_IRBuilder, Exn: *mut super::llvm_Value) -> *mut super::llvm_ResumeInst;
        pub fn llvm_IRBuilder_CreateRet(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value) -> *mut super::llvm_ReturnInst;
        pub fn llvm_IRBuilder_CreateRetVoid(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_ReturnInst;
        pub fn llvm_IRBuilder_CreateSDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSRem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSelect(inst: *mut super::llvm_IRBuilder, C: *mut super::llvm_Value, TrueValue: *mut super::llvm_Value, FalseValue: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShlByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShl(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShuffleVector(inst: *mut super::llvm_IRBuilder, V1: *mut super::llvm_Value, P2: *mut super::llvm_Value, Mask: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, isVolatile: *const super::libc::c_int) -> *mut super::llvm_StoreInst;
        pub fn llvm_IRBuilder_CreateStructGEP(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Index: super::libc::c_uint, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSub(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateSwitch(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Dest: *mut super::llvm_BasicBlock, NumCases: *const super::libc::c_uint) -> *mut super::llvm_SwitchInst;
        pub fn llvm_IRBuilder_CreateTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateTruncOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUDiv(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUIToFP(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateURem(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateUnreachable(inst: *mut super::llvm_IRBuilder) -> *mut super::llvm_UnreachableInst;
        pub fn llvm_IRBuilder_CreateVAArg(inst: *mut super::llvm_IRBuilder, List: *mut super::llvm_Value, Ty: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_VAArgInst;
        pub fn llvm_IRBuilder_CreateVectorSplat(inst: *mut super::llvm_IRBuilder, NumElements: super::libc::c_uint, Value: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateXor(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateXorByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExt(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExtOrBitCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateZExtOrTrunc(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_delete(inst: *mut super::llvm_IRBuilder);
        pub fn llvm_IRBuilder_isNamePreserving(inst: *const super::llvm_IRBuilder) -> super::libc::c_int;
        pub fn llvm_IRBuilder_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilder;
        pub fn llvm_IRBuilder_new_in_block(BB: *mut super::llvm_BasicBlock) -> *mut super::llvm_IRBuilder;
        pub fn llvm_IRBuilderBase_ClearInsertionPoint(inst: *mut super::llvm_IRBuilderBase);
        pub fn llvm_IRBuilderBase_CreateGlobalString(inst: *mut super::llvm_IRBuilderBase, Str: super::llvm_StringRef, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilderBase_CreateLifetimeEnd(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *const *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateLifetimeStart(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Size: *const *mut super::llvm_ConstantInt) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateMemCpy(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: *const super::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateMemMove(inst: *mut super::llvm_IRBuilderBase, Dst: *mut super::llvm_Value, Src: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: *const super::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_CreateMemSet(inst: *mut super::llvm_IRBuilderBase, Ptr: *mut super::llvm_Value, Value: *mut super::llvm_Value, Size: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: *const super::libc::c_int) -> *mut super::llvm_CallInst;
        pub fn llvm_IRBuilderBase_GetInsertBlock(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_BasicBlock;
        pub fn llvm_IRBuilderBase_SetCurrentDebugLocation(inst: *mut super::llvm_IRBuilderBase, Loc: *const super::llvm_DebugLoc);
        pub fn llvm_IRBuilderBase_SetDefaultFPMathTag(inst: *mut super::llvm_IRBuilderBase, FPMathTag: *mut super::llvm_MDNode);
        pub fn llvm_IRBuilderBase_SetInsertPointAtInst(inst: *mut super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction);
        pub fn llvm_IRBuilderBase_SetInsertPoint(inst: *mut super::llvm_IRBuilderBase, BB: *mut super::llvm_BasicBlock);
        pub fn llvm_IRBuilderBase_SetInstDebugLocation(inst: *const super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction);
        pub fn llvm_IRBuilderBase_getContext(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_LLVMContext;
        pub fn llvm_IRBuilderBase_getCurrentFunctionReturnType(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_IRBuilderBase_getDefaultFPMathTag(inst: *const super::llvm_IRBuilderBase) -> *mut super::llvm_MDNode;
        pub fn llvm_IRBuilderBase_getDoubleTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_IRBuilderBase_getFalse(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getFloatTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_IRBuilderBase_getHalfTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_IRBuilderBase_getInt(inst: *mut super::llvm_IRBuilderBase, Value: super::llvm_APInt) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt1(inst: *mut super::llvm_IRBuilderBase, Value: super::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt16(inst: *mut super::llvm_IRBuilderBase, Value: super::libc::uint16_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt16Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt1Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt32(inst: *mut super::llvm_IRBuilderBase, Value: super::libc::uint32_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt32Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt64(inst: *mut super::llvm_IRBuilderBase, Value: super::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt64Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getInt8(inst: *mut super::llvm_IRBuilderBase, Value: super::libc::uint8_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getInt8PtrTy(inst: *mut super::llvm_IRBuilderBase, AddrSpace: *const super::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_IRBuilderBase_getInt8Ty(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getIntN(inst: *mut super::llvm_IRBuilderBase, NumBits: super::libc::c_uint, Value: super::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getIntNTy(inst: *mut super::llvm_IRBuilderBase, NumBits: super::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getIntPtrTy(inst: *mut super::llvm_IRBuilderBase, DL: *const super::llvm_DataLayout, AddrSpace: *const super::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_IRBuilderBase_getTrue(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_ConstantInt;
        pub fn llvm_IRBuilderBase_getVoidTy(inst: *mut super::llvm_IRBuilderBase) -> *mut super::llvm_Type;
        pub fn llvm_IRBuilderBase_new(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_IRBuilderBase;
        pub fn llvm_Instruction_clone(inst: *const super::llvm_Instruction) -> *mut super::llvm_Instruction;
        pub fn llvm_Instruction_copyFastMathFlags(inst: *mut super::llvm_Instruction, Inst: *const super::llvm_Instruction);
        pub fn llvm_Instruction_delete(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_dropUnknownMetadataFromIDS(inst: *mut super::llvm_Instruction, KnownIDs: super::llvm_ArrayRef_uint);
        pub fn llvm_Instruction_dropUnknownMetadata(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_eraseFromParent(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_getDataLayout(inst: *const super::llvm_Instruction) -> *const super::llvm_DataLayout;
        pub fn llvm_Instruction_getDebugLoc(inst: *const super::llvm_Instruction) -> *const super::llvm_DebugLoc;
        pub fn llvm_Instruction_getMetadataStr(inst: *const super::llvm_Instruction, Kind: super::llvm_StringRef) -> *mut super::llvm_MDNode;
        pub fn llvm_Instruction_getMetadata(inst: *const super::llvm_Instruction, KindID: super::libc::c_uint) -> *mut super::llvm_MDNode;
        pub fn llvm_Instruction_getOpcode(inst: *const super::llvm_Instruction) -> super::libc::c_uint;
        pub fn llvm_Instruction_getParentMut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_BasicBlock;
        pub fn llvm_Instruction_getParent(inst: *const super::llvm_Instruction) -> *const super::llvm_BasicBlock;
        pub fn llvm_Instruction_hasAllowReciprocal(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasMetadata(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasMetadataOtherThanDebugLoc(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasNoInfs(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasNoNaNs(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasNoSignedZeros(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_hasUnsafeAlgebra(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_insertAfter(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_insertBefore(inst: *mut super::llvm_Instruction, InsertPos: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_isArithmeticShift(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isAssociative(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isBinaryOp(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isCast(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isCommutative(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isIdempotent(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isIdenticalTo(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isIdenticalToWhenDefined(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isLogicalShift(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isNilpotent(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isSameOperationAs(inst: *const super::llvm_Instruction, Inst: *const super::llvm_Instruction, flags: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Instruction_isShift(inst: *mut super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isTerminator(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_isUsedOutsideOfBlock(inst: *const super::llvm_Instruction, BB: *const super::llvm_BasicBlock) -> super::libc::c_int;
        pub fn llvm_Instruction_mayHaveSideEffects(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_mayReadFromMemory(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_mayReadOrWriteMemory(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_mayReturn(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_mayThrow(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_mayWriteToMemory(inst: *const super::llvm_Instruction) -> super::libc::c_int;
        pub fn llvm_Instruction_moveBefore(inst: *mut super::llvm_Instruction, MovePos: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_removeFromParent(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_setDebugLoc(inst: *mut super::llvm_Instruction, Loc: *const super::llvm_DebugLoc);
        pub fn llvm_Instruction_setHasAllowReciprocal(inst: *mut super::llvm_Instruction, Val: super::libc::c_int);
        pub fn llvm_Instruction_setHasNoInfs(inst: *mut super::llvm_Instruction, Val: super::libc::c_int);
        pub fn llvm_Instruction_setHasNoNaNs(inst: *mut super::llvm_Instruction, Val: super::libc::c_int);
        pub fn llvm_Instruction_setHasNoSignedZeros(inst: *mut super::llvm_Instruction, Val: super::libc::c_int);
        pub fn llvm_Instruction_setHasUnsafeAlgebra(inst: *mut super::llvm_Instruction, Val: super::libc::c_int);
        pub fn llvm_Instruction_setMetadata(inst: *mut super::llvm_Instruction, KindID: super::libc::c_uint, Node: *mut super::llvm_MDNode);
        pub fn llvm_Instruction_setMetadataStr(inst: *mut super::llvm_Instruction, Kind: super::llvm_StringRef, Node: *mut super::llvm_MDNode);
        pub fn llvm_Instruction_user_back(inst: *const super::llvm_Instruction) -> *const super::llvm_Instruction;
        pub fn llvm_Instruction_user_back_mut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_Instruction;
        pub fn llvm_IntegerType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_IntegerType_get(ctx: *mut super::llvm_LLVMContext, NumBits: super::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_IntegerType_getBitMask(inst: *const super::llvm_IntegerType) -> super::libc::uint64_t;
        pub fn llvm_IntegerType_getBitWidth(inst: *const super::llvm_IntegerType) -> super::libc::c_uint;
        pub fn llvm_IntegerType_getSignBit(inst: *const super::llvm_IntegerType) -> super::libc::uint64_t;
        pub fn llvm_IntegerType_isPowerOf2ByteWidth(inst: *const super::llvm_IntegerType) -> super::libc::c_int;
        pub fn llvm_LLVMContext_delete() -> *mut super::llvm_LLVMContext;
        pub fn llvm_LLVMContext_new() -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_appendModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef);
        pub fn llvm_Module_delete(inst: *mut super::llvm_Module);
        pub fn llvm_Module_dump(inst: *const super::llvm_Module);
        pub fn llvm_Module_getContext(inst: *const super::llvm_Module) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Module_getDataLayout(inst: *const super::llvm_Module) -> *const super::llvm_DataLayout;
        pub fn llvm_Module_getDataLayoutStr(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getFunction(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_Function;
        pub fn llvm_Module_getMDKindID(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> super::libc::c_uint;
        pub fn llvm_Module_getModuleIdentifier(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getModuleInlineAsm(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getNamedValue(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_GlobalValue;
        pub fn llvm_Module_getOrInsertFunction(inst: *mut super::llvm_Module, Name: super::llvm_StringRef, ty: *mut super::llvm_FunctionType) -> *mut super::llvm_Constant;
        pub fn llvm_Module_getTargetTriple(inst: *const super::llvm_Module) -> super::std_string_const;
        pub fn llvm_Module_getTypeByName(inst: *const super::llvm_Module, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_Module_new(ModuleID: super::llvm_StringRef, Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_Module;
        pub fn llvm_Module_setDataLayout(inst: *mut super::llvm_Module, Other: *const super::llvm_DataLayout);
        pub fn llvm_Module_setDataLayoutStr(inst: *mut super::llvm_Module, Desc: super::llvm_StringRef);
        pub fn llvm_Module_setModuleIdentifier(inst: *mut super::llvm_Module, ID: super::llvm_StringRef);
        pub fn llvm_Module_setModuleInlineAsm(inst: *mut super::llvm_Module, Asm: super::llvm_StringRef);
        pub fn llvm_Module_setTargetTriple(inst: *mut super::llvm_Module, Triple: super::llvm_StringRef);
        pub fn llvm_Operator_getOpcode(inst: *const super::llvm_Operator) -> super::libc::c_uint;
        pub fn llvm_Pass_delete(inst: *mut super::llvm_Pass);
        pub fn llvm_Pass_doFinalization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> super::libc::c_int;
        pub fn llvm_Pass_doInitialization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> super::libc::c_int;
        pub fn llvm_Pass_dump(inst: *const super::llvm_Pass);
        pub fn llvm_Pass_getPassKind(inst: *const super::llvm_Pass) -> super::llvm_PassKind;
        pub fn llvm_PassManager_add(inst: *mut super::llvm_PassManager, Pass: *mut super::llvm_Pass);
        pub fn llvm_PassManager_new() -> *mut super::llvm_PassManager;
        pub fn llvm_PassManager_run(inst: *mut super::llvm_PassManager, Module: *mut super::llvm_Module);
        pub fn llvm_PointerType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_PointerType_get(ElementType: *mut super::llvm_Type, AddressSpace: super::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_PointerType_getAddressSpace(inst: *const super::llvm_PointerType) -> super::libc::c_uint;
        pub fn llvm_PointerType_getUnqual(ElementType: *mut super::llvm_Type) -> *mut super::llvm_PointerType;
        pub fn llvm_PointerType_isValidElementType(ty: *mut super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_SequentialType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_SequentialType_getElementType(inst: *const super::llvm_SequentialType) -> *mut super::llvm_Type;
        pub fn llvm_StructType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_StructType_create(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_ptr_llvm_Type, Name: super::llvm_StringRef) -> *mut super::llvm_StructType;
        pub fn llvm_StructType_createPacked(ctx: *mut super::llvm_LLVMContext, Elements: super::llvm_ArrayRef_ptr_llvm_Type, Name: super::llvm_StringRef, isPacked: super::libc::c_int) -> *mut super::llvm_StructType;
        pub fn llvm_StructType_getElementType(inst: *const super::llvm_StructType, idx: super::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_StructType_getName(inst: *const super::llvm_StructType) -> super::llvm_StringRef;
        pub fn llvm_StructType_getNumElements(inst: *const super::llvm_StructType) -> super::libc::c_uint;
        pub fn llvm_StructType_hasName(inst: *const super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isLayoutIdentical(inst: *const super::llvm_StructType, Other: *mut super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isLiteral(inst: *const super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isOpaque(inst: *const super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isPacked(inst: *const super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isSized(inst: *const super::llvm_StructType) -> super::libc::c_int;
        pub fn llvm_StructType_isValidElementType(ty: *mut super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_ptr_llvm_Type, isPacked: super::libc::c_int);
        pub fn llvm_StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_ptr_llvm_Type);
        pub fn llvm_StructType_setName(inst: *mut super::llvm_StructType, Name: super::llvm_StringRef);
        pub fn llvm_Type_dump(inst: *const super::llvm_Type);
        pub fn llvm_Type_getContainedType(inst: *const super::llvm_Type, idx: super::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getContext(inst: *const super::llvm_Type) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Type_getDoublePtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getDoubleTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFloatPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getFloatTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getFunctionNumParams(inst: *const super::llvm_Type) -> super::libc::c_uint;
        pub fn llvm_Type_getFunctionParamType(inst: *const super::llvm_Type, idx: super::libc::c_uint) -> *mut super::llvm_Type;
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
        pub fn llvm_Type_getIntNPtrTy(ctx: *mut super::llvm_LLVMContext, size: super::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getIntNTy(ctx: *mut super::llvm_LLVMContext, size: super::libc::c_uint) -> *mut super::llvm_IntegerType;
        pub fn llvm_Type_getLabelTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getMetadataTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getNumContainedTypes(inst: *const super::llvm_Type) -> super::libc::c_uint;
        pub fn llvm_Type_getPPC_FP128PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getPPC_FP128Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerAddressSpace(inst: *const super::llvm_Type) -> super::libc::c_uint;
        pub fn llvm_Type_getPointerElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_Type_getPointerTo(inst: *mut super::llvm_Type, idx: super::libc::c_uint) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getSequentialElementType(inst: *const super::llvm_Type) -> *mut super::llvm_Type;
        pub fn llvm_Type_getStructElementType(inst: *const super::llvm_Type, idx: super::libc::c_uint) -> *mut super::llvm_Type;
        pub fn llvm_Type_getStructName(inst: *const super::llvm_Type) -> super::llvm_StringRef;
        pub fn llvm_Type_getStructNumElements(inst: *const super::llvm_Type) -> super::libc::c_uint;
        pub fn llvm_Type_getTypeID(inst: *const super::llvm_Type) -> super::llvm_Type_TypeID;
        pub fn llvm_Type_getVoidTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_FP80PtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_FP80Ty(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_getX86_MMXPtrTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_PointerType;
        pub fn llvm_Type_getX86_MMXTy(ctx: *mut super::llvm_LLVMContext) -> *mut super::llvm_Type;
        pub fn llvm_Type_isAggregateType(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isArrayTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isDoubleTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isEmptyTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFP128Ty(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFPOrFPVectorTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFirstClassType(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFloatTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFloatingPointTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFunctionTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isFunctionVarArg(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isHalfTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isIntOrIntVectorTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isIntegerTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isLabelTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isMetadataTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isPPC_FP128Ty(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isPointerTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isPtrOrPtrVectorTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isSingleValueType(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isSized(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isStructTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isVectorTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isVoidTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isX86_FP80Ty(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Type_isX86_MMXTy(inst: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_Use_get(inst: *const super::llvm_Use) -> *mut super::llvm_Value;
        pub fn llvm_Use_getNext(inst: *const super::llvm_Use) -> *mut super::llvm_Use;
        pub fn llvm_Use_getOperandNo(inst: *const super::llvm_Use) -> super::libc::c_uint;
        pub fn llvm_Use_getUser(inst: *const super::llvm_Use) -> *mut super::llvm_User;
        pub fn llvm_Use_initTags(Start: *mut super::llvm_Use, Stop: *mut super::llvm_Use) -> *mut super::llvm_Use;
        pub fn llvm_Use_set(inst: *mut super::llvm_Use, Val: *mut super::llvm_Value);
        pub fn llvm_Use_swap(inst: *mut super::llvm_Use, RHS: *mut super::llvm_Use);
        pub fn llvm_User_classof(V: *mut super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_User_delete(inst: *mut super::llvm_User);
        pub fn llvm_User_dropAllReferences(inst: *mut super::llvm_User);
        pub fn llvm_User_getNumOperands(inst: *const super::llvm_User) -> super::libc::c_uint;
        pub fn llvm_User_getOperand(inst: *const super::llvm_User, idx: super::libc::c_uint) -> *mut super::llvm_Value;
        pub fn llvm_User_replaceUsesOfWith(inst: *mut super::llvm_User, From: *mut super::llvm_Value, To: *mut super::llvm_Value);
        pub fn llvm_User_setOperand(inst: *mut super::llvm_User, idx: super::libc::c_uint, Val: *mut super::llvm_Value);
        pub fn llvm_Value_delete(inst: *mut super::llvm_Value);
        pub fn llvm_Value_dump(inst: *const super::llvm_Value);
        pub fn llvm_Value_getContext(inst: *const super::llvm_Value) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Value_getName(inst: *const super::llvm_Value) -> super::llvm_StringRef;
        pub fn llvm_Value_getNumUses(inst: *const super::llvm_Value) -> super::libc::c_uint;
        pub fn llvm_Value_getType(inst: *const super::llvm_Value) -> *mut super::llvm_Type;
        pub fn llvm_Value_getValueID(inst: *const super::llvm_Value) -> super::libc::c_uint;
        pub fn llvm_Value_hasNUses(inst: *const super::llvm_Value, N: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Value_hasNUsesOrMore(inst: *const super::llvm_Value, N: super::libc::c_uint) -> super::libc::c_int;
        pub fn llvm_Value_hasName(inst: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_Value_hasOneUse(inst: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_Value_isUsedInBasicBlock(inst: *const super::llvm_Value, BB: *const super::llvm_BasicBlock) -> super::libc::c_int;
        pub fn llvm_Value_mutateType(inst: *mut super::llvm_Value, ty: *mut super::llvm_Type);
        pub fn llvm_Value_replaceAllUsesWith(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value);
        pub fn llvm_Value_setName(inst: *mut super::llvm_Value, Name: super::llvm_StringRef);
        pub fn llvm_Value_takeName(inst: *mut super::llvm_Value, Value: *mut super::llvm_Value);
        pub fn llvm_ValueSymbolTable_delete() -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_ValueSymbolTable_dump(inst: *const super::llvm_ValueSymbolTable);
        pub fn llvm_ValueSymbolTable_empty(inst: *const super::llvm_ValueSymbolTable) -> super::libc::c_int;
        pub fn llvm_ValueSymbolTable_lookup(inst: *const super::llvm_ValueSymbolTable, Name: super::llvm_StringRef) -> *mut super::llvm_Value;
        pub fn llvm_ValueSymbolTable_new() -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_ValueSymbolTable_size(inst: *const super::llvm_ValueSymbolTable) -> super::libc::c_uint;
        pub fn llvm_VectorType_classof(ty: *const super::llvm_Type) -> super::libc::c_int;
        pub fn llvm_VectorType_get(ty: *mut super::llvm_Type, NumElements: super::libc::c_uint) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_getBitWidth(inst: *const super::llvm_VectorType) -> super::libc::c_uint;
        pub fn llvm_VectorType_getDoubleElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_getExtendedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_getHalfElementsVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_getInteger(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_getNumElements(inst: *const super::llvm_VectorType) -> super::libc::c_uint;
        pub fn llvm_VectorType_getTruncatedElementVectorType(ty: *mut super::llvm_VectorType) -> *mut super::llvm_VectorType;
        pub fn llvm_VectorType_isValidElementType(ty: *mut super::llvm_Type) -> super::libc::c_int;
    }
}


pub fn llvm_createAddDiscriminatorsPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createAddDiscriminatorsPass()
    }
}

pub fn llvm_createAddressSanitizerFunctionPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createAddressSanitizerFunctionPass()
    }
}

pub fn llvm_createAddressSanitizerModulePass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createAddressSanitizerModulePass()
    }
}

pub fn llvm_createAggressiveDCEPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createAggressiveDCEPass()
    }
}

pub fn llvm_createAlwaysInlinerPass(InsertLifetime: Option<bool>) -> *mut llvm_Pass {
    let InsertLifetime = match InsertLifetime.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createAlwaysInlinerPass(InsertLifetime)
    }
}

pub fn llvm_createArgumentPromotionPass(maxElements: Option<usize>) -> *mut llvm_Pass {
    let maxElements = match maxElements.map(|value| value as libc::c_uint) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createArgumentPromotionPass(maxElements)
    }
}

pub fn llvm_createBarrierNoopPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createBarrierNoopPass()
    }
}

pub fn llvm_createBlockExtractorPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createBlockExtractorPass()
    }
}

pub fn llvm_createBoundsCheckingPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createBoundsCheckingPass()
    }
}

pub fn llvm_createBreakCriticalEdgesPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createBreakCriticalEdgesPass()
    }
}

pub fn llvm_createCFGSimplificationPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createCFGSimplificationPass()
    }
}

pub fn llvm_createConstantHoistingPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createConstantHoistingPass()
    }
}

pub fn llvm_createConstantMergePass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createConstantMergePass()
    }
}

pub fn llvm_createConstantPropagationPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createConstantPropagationPass()
    }
}

pub fn llvm_createCorrelatedValuePropagationPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createCorrelatedValuePropagationPass()
    }
}

pub fn llvm_createDataFlowSanitizerPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createDataFlowSanitizerPass()
    }
}

pub fn llvm_createDeadArgEliminationPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createDeadArgEliminationPass()
    }
}

pub fn llvm_createDeadArgHackingPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createDeadArgHackingPass()
    }
}

pub fn llvm_createDeadCodeEliminationPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createDeadCodeEliminationPass()
    }
}

pub fn llvm_createDeadInstEliminationPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createDeadInstEliminationPass()
    }
}

pub fn llvm_createDeadStoreEliminationPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createDeadStoreEliminationPass()
    }
}

pub fn llvm_createDebugIRPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createDebugIRPass()
    }
}

pub fn llvm_createDemoteRegisterToMemoryPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createDemoteRegisterToMemoryPass()
    }
}

pub fn llvm_createEarlyCSEPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createEarlyCSEPass()
    }
}

pub fn llvm_createFlattenCFGPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createFlattenCFGPass()
    }
}

pub fn llvm_createFunctionAttrsPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createFunctionAttrsPass()
    }
}

pub fn llvm_createFunctionInliningPassWithOptLevel(OptLevel: usize, SizeOptLevel: usize) -> *mut llvm_Pass {
    let OptLevel = OptLevel as libc::c_uint;
    let SizeOptLevel = SizeOptLevel as libc::c_uint;
    unsafe {
        raw::llvm_createFunctionInliningPassWithOptLevel(OptLevel, SizeOptLevel)
    }
}

pub fn llvm_createFunctionInliningPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createFunctionInliningPass()
    }
}

pub fn llvm_createGCOVProfilerPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createGCOVProfilerPass()
    }
}

pub fn llvm_createGVNPass(NoLoads: Option<bool>) -> *mut llvm_FunctionPass {
    let NoLoads = match NoLoads.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createGVNPass(NoLoads)
    }
}

pub fn llvm_createGlobalDCEPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createGlobalDCEPass()
    }
}

pub fn llvm_createGlobalMergePass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createGlobalMergePass()
    }
}

pub fn llvm_createGlobalOptimizerPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createGlobalOptimizerPass()
    }
}

pub fn llvm_createIPConstantPropagationPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createIPConstantPropagationPass()
    }
}

pub fn llvm_createIPSCCPPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createIPSCCPPass()
    }
}

pub fn llvm_createIndVarSimplifyPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createIndVarSimplifyPass()
    }
}

pub fn llvm_createInstructionCombiningPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createInstructionCombiningPass()
    }
}

pub fn llvm_createInstructionNamerPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createInstructionNamerPass()
    }
}

pub fn llvm_createInstructionSimplifierPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createInstructionSimplifierPass()
    }
}

pub fn llvm_createInternalizePass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createInternalizePass()
    }
}

pub fn llvm_createJumpThreadingPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createJumpThreadingPass()
    }
}

pub fn llvm_createLCSSAPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLCSSAPass()
    }
}

pub fn llvm_createLICMPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLICMPass()
    }
}

pub fn llvm_createLoadCombinePass() -> *mut llvm_BasicBlockPass {
    unsafe {
        raw::llvm_createLoadCombinePass()
    }
}

pub fn llvm_createLoopDeletionPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopDeletionPass()
    }
}

pub fn llvm_createLoopExtractorPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopExtractorPass()
    }
}

pub fn llvm_createLoopIdiomPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopIdiomPass()
    }
}

pub fn llvm_createLoopInstSimplifyPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopInstSimplifyPass()
    }
}

pub fn llvm_createLoopRerollPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopRerollPass()
    }
}

pub fn llvm_createLoopRotatePass(MaxHeaderSize: Option<isize>) -> *mut llvm_Pass {
    let MaxHeaderSize = match MaxHeaderSize.map(|value| value as libc::c_int) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createLoopRotatePass(MaxHeaderSize)
    }
}

pub fn llvm_createLoopSimplifyPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopSimplifyPass()
    }
}

pub fn llvm_createLoopStrengthReducePass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopStrengthReducePass()
    }
}

pub fn llvm_createLoopUnrollPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLoopUnrollPass()
    }
}

pub fn llvm_createLoopUnswitchPass(OptimizeForSize: Option<bool>) -> *mut llvm_Pass {
    let OptimizeForSize = match OptimizeForSize.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createLoopUnswitchPass(OptimizeForSize)
    }
}

pub fn llvm_createLowerAtomicPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createLowerAtomicPass()
    }
}

pub fn llvm_createLowerExpectIntrinsicPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createLowerExpectIntrinsicPass()
    }
}

pub fn llvm_createLowerInvokePass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createLowerInvokePass()
    }
}

pub fn llvm_createLowerSwitchPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createLowerSwitchPass()
    }
}

pub fn llvm_createMemCpyOptPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createMemCpyOptPass()
    }
}

pub fn llvm_createMemorySanitizerPass(TrackOrigins: Option<isize>) -> *mut llvm_FunctionPass {
    let TrackOrigins = match TrackOrigins.map(|value| value as libc::c_int) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createMemorySanitizerPass(TrackOrigins)
    }
}

pub fn llvm_createMergeFunctionsPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createMergeFunctionsPass()
    }
}

pub fn llvm_createMergedLoadStoreMotionPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createMergedLoadStoreMotionPass()
    }
}

pub fn llvm_createMetaRenamerPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createMetaRenamerPass()
    }
}

pub fn llvm_createObjCARCAPElimPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createObjCARCAPElimPass()
    }
}

pub fn llvm_createObjCARCContractPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createObjCARCContractPass()
    }
}

pub fn llvm_createObjCARCExpandPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createObjCARCExpandPass()
    }
}

pub fn llvm_createObjCARCOptPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createObjCARCOptPass()
    }
}

pub fn llvm_createPartialInliningPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createPartialInliningPass()
    }
}

pub fn llvm_createPartiallyInlineLibCallsPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createPartiallyInlineLibCallsPass()
    }
}

pub fn llvm_createPromoteMemoryToRegisterPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createPromoteMemoryToRegisterPass()
    }
}

pub fn llvm_createPruneEHPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createPruneEHPass()
    }
}

pub fn llvm_createReassociatePass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createReassociatePass()
    }
}

pub fn llvm_createSCCPPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createSCCPPass()
    }
}

pub fn llvm_createSROAPass(RequiresDomTree: Option<bool>) -> *mut llvm_FunctionPass {
    let RequiresDomTree = match RequiresDomTree.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createSROAPass(RequiresDomTree)
    }
}

pub fn llvm_createSampleProfileLoaderPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createSampleProfileLoaderPass()
    }
}

pub fn llvm_createScalarReplAggregatesPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createScalarReplAggregatesPass()
    }
}

pub fn llvm_createScalarizerPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createScalarizerPass()
    }
}

pub fn llvm_createSeparateConstOffsetFromGEPPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createSeparateConstOffsetFromGEPPass()
    }
}

pub fn llvm_createSimpleLoopUnrollPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createSimpleLoopUnrollPass()
    }
}

pub fn llvm_createSingleLoopExtractorPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createSingleLoopExtractorPass()
    }
}

pub fn llvm_createSinkingPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createSinkingPass()
    }
}

pub fn llvm_createStripDeadDebugInfoPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createStripDeadDebugInfoPass()
    }
}

pub fn llvm_createStripDeadPrototypesPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createStripDeadPrototypesPass()
    }
}

pub fn llvm_createStripDebugDeclarePass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createStripDebugDeclarePass()
    }
}

pub fn llvm_createStripNonDebugSymbolsPass() -> *mut llvm_ModulePass {
    unsafe {
        raw::llvm_createStripNonDebugSymbolsPass()
    }
}

pub fn llvm_createStripSymbolsPass(OnlyDebugInfo: Option<bool>) -> *mut llvm_ModulePass {
    let OnlyDebugInfo = match OnlyDebugInfo.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_createStripSymbolsPass(OnlyDebugInfo)
    }
}

pub fn llvm_createStructurizeCFGPass() -> *mut llvm_Pass {
    unsafe {
        raw::llvm_createStructurizeCFGPass()
    }
}

pub fn llvm_createTailCallEliminationPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createTailCallEliminationPass()
    }
}

pub fn llvm_createThreadSanitizerPass() -> *mut llvm_FunctionPass {
    unsafe {
        raw::llvm_createThreadSanitizerPass()
    }
}

pub fn llvm_getGlobalContext() -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_getGlobalContext()
    }
}

pub fn llvm_verifyFunction(Function: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_verifyFunction(Function) != 0
    }
}

pub fn llvm_verifyModule(Module: *const llvm_Module) -> bool {
    unsafe {
        raw::llvm_verifyModule(Module) != 0
    }
}

pub fn llvm_ArrayType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_ArrayType_classof(ty) != 0
    }
}

pub fn llvm_ArrayType_get(ElementType: *mut llvm_Type, NumElements: u64) -> *mut llvm_ArrayType {
    let NumElements = NumElements as libc::uint64_t;
    unsafe {
        raw::llvm_ArrayType_get(ElementType, NumElements)
    }
}

pub fn llvm_ArrayType_getNumElements(inst: *const llvm_ArrayType) -> u64 {
    unsafe {
        raw::llvm_ArrayType_getNumElements(inst) as u64
    }
}

pub fn llvm_ArrayType_isValidElementType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_ArrayType_isValidElementType(ty) != 0
    }
}

pub fn llvm_BasicBlock_Create(Context: *mut llvm_LLVMContext, Name: Option<&str>, Parent: Option<*mut llvm_Function>, InsertBefore: Option<*mut llvm_BasicBlock>) -> *mut llvm_BasicBlock {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    let Parent = match Parent.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    let InsertBefore = match InsertBefore.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_BasicBlock_Create(Context, Name, Parent, InsertBefore)
    }
}

pub fn llvm_BasicBlock_classof(Val: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_BasicBlock_classof(Val) != 0
    }
}

pub fn llvm_BasicBlock_delete(inst: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_delete(inst)
    }
}

pub fn llvm_BasicBlock_dropAllReferences(inst: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_dropAllReferences(inst)
    }
}

pub fn llvm_BasicBlock_eraseFromParent(inst: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_eraseFromParent(inst)
    }
}

pub fn llvm_BasicBlock_getDataLayout(inst: *const llvm_BasicBlock) -> *const llvm_DataLayout {
    unsafe {
        raw::llvm_BasicBlock_getDataLayout(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHIMut(inst: *mut llvm_BasicBlock) -> *mut llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHIMut(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHI(inst: *const llvm_BasicBlock) -> *const llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHI(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst: *mut llvm_BasicBlock) -> *mut llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHIOrDbg(inst: *const llvm_BasicBlock) -> *const llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbg(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut llvm_BasicBlock) -> *mut llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst)
    }
}

pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst: *const llvm_BasicBlock) -> *const llvm_Instruction {
    unsafe {
        raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst)
    }
}

pub fn llvm_BasicBlock_getLandingPadInstMut(inst: *mut llvm_BasicBlock) -> *mut llvm_LandingPadInst {
    unsafe {
        raw::llvm_BasicBlock_getLandingPadInstMut(inst)
    }
}

pub fn llvm_BasicBlock_getLandingPadInst(inst: *const llvm_BasicBlock) -> *const llvm_LandingPadInst {
    unsafe {
        raw::llvm_BasicBlock_getLandingPadInst(inst)
    }
}

pub fn llvm_BasicBlock_getParent(inst: *const llvm_BasicBlock) -> *const llvm_Function {
    unsafe {
        raw::llvm_BasicBlock_getParent(inst)
    }
}

pub fn llvm_BasicBlock_getParentMut(inst: *mut llvm_BasicBlock) -> *mut llvm_Function {
    unsafe {
        raw::llvm_BasicBlock_getParentMut(inst)
    }
}

pub fn llvm_BasicBlock_getSinglePredecessor(inst: *const llvm_BasicBlock) -> *const llvm_BasicBlock {
    unsafe {
        raw::llvm_BasicBlock_getSinglePredecessor(inst)
    }
}

pub fn llvm_BasicBlock_getSinglePredecessorMut(inst: *mut llvm_BasicBlock) -> *mut llvm_BasicBlock {
    unsafe {
        raw::llvm_BasicBlock_getSinglePredecessorMut(inst)
    }
}

pub fn llvm_BasicBlock_getTerminator(inst: *const llvm_BasicBlock) -> *const llvm_TerminatorInst {
    unsafe {
        raw::llvm_BasicBlock_getTerminator(inst)
    }
}

pub fn llvm_BasicBlock_getTerminatorMut(inst: *mut llvm_BasicBlock) -> *mut llvm_TerminatorInst {
    unsafe {
        raw::llvm_BasicBlock_getTerminatorMut(inst)
    }
}

pub fn llvm_BasicBlock_getUniquePredecessorMut(inst: *mut llvm_BasicBlock) -> *mut llvm_BasicBlock {
    unsafe {
        raw::llvm_BasicBlock_getUniquePredecessorMut(inst)
    }
}

pub fn llvm_BasicBlock_getUniquePredecessor(inst: *const llvm_BasicBlock) -> *const llvm_BasicBlock {
    unsafe {
        raw::llvm_BasicBlock_getUniquePredecessor(inst)
    }
}

pub fn llvm_BasicBlock_getValueSymbolTable(inst: *mut llvm_BasicBlock) -> *mut llvm_ValueSymbolTable {
    unsafe {
        raw::llvm_BasicBlock_getValueSymbolTable(inst)
    }
}

pub fn llvm_BasicBlock_hasAddressTaken(inst: *const llvm_BasicBlock) -> bool {
    unsafe {
        raw::llvm_BasicBlock_hasAddressTaken(inst) != 0
    }
}

pub fn llvm_BasicBlock_isLandingPad(inst: *const llvm_BasicBlock) -> bool {
    unsafe {
        raw::llvm_BasicBlock_isLandingPad(inst) != 0
    }
}

pub fn llvm_BasicBlock_moveAfter(inst: *mut llvm_BasicBlock, MovePos: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_moveAfter(inst, MovePos)
    }
}

pub fn llvm_BasicBlock_moveBefore(inst: *mut llvm_BasicBlock, MovePos: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_moveBefore(inst, MovePos)
    }
}

pub fn llvm_BasicBlock_removeFromParent(inst: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_removeFromParent(inst)
    }
}

pub fn llvm_BasicBlock_removePredecessor(inst: *mut llvm_BasicBlock, Pred: *mut llvm_BasicBlock, DontDeleteUselessPHIs: Option<bool>) {
    let DontDeleteUselessPHIs = match DontDeleteUselessPHIs.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_BasicBlock_removePredecessor(inst, Pred, DontDeleteUselessPHIs)
    }
}

pub fn llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst: *mut llvm_BasicBlock, New: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst, New)
    }
}

pub fn llvm_BlockAddress_destroyConstant(inst: *mut llvm_BlockAddress) {
    unsafe {
        raw::llvm_BlockAddress_destroyConstant(inst)
    }
}

pub fn llvm_BlockAddress_getBasicBlock(inst: *const llvm_BlockAddress) -> *mut llvm_BasicBlock {
    unsafe {
        raw::llvm_BlockAddress_getBasicBlock(inst)
    }
}

pub fn llvm_BlockAddress_getFunction(inst: *const llvm_BlockAddress) -> *mut llvm_Function {
    unsafe {
        raw::llvm_BlockAddress_getFunction(inst)
    }
}

pub fn llvm_CompositeType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_CompositeType_classof(ty) != 0
    }
}

pub fn llvm_CompositeType_getTypeAtIndex(inst: *mut llvm_CompositeType, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_CompositeType_getTypeAtIndex(inst, idx)
    }
}

pub fn llvm_CompositeType_indexValid(inst: *const llvm_CompositeType, idx: usize) -> bool {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_CompositeType_indexValid(inst, idx) != 0
    }
}

pub fn llvm_Constant_canTrap(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_canTrap(inst) != 0
    }
}

pub fn llvm_Constant_classof(V: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_Constant_classof(V) != 0
    }
}

pub fn llvm_Constant_destroyConstant(inst: *mut llvm_Constant) {
    unsafe {
        raw::llvm_Constant_destroyConstant(inst)
    }
}

pub fn llvm_Constant_getAggregateElement(inst: *const llvm_Constant, Elt: usize) -> *mut llvm_Constant {
    let Elt = Elt as libc::c_uint;
    unsafe {
        raw::llvm_Constant_getAggregateElement(inst, Elt)
    }
}

pub fn llvm_Constant_getAggregateElementConstant(inst: *const llvm_Constant, Elt: *mut llvm_Constant) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_Constant_getAggregateElementConstant(inst, Elt)
    }
}

pub fn llvm_Constant_getAllOnesValue(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_Constant_getAllOnesValue(Ty)
    }
}

pub fn llvm_Constant_getIntegerValue(Ty: *mut llvm_Type, Value: (&[u64], usize)) -> *mut llvm_Constant {
    let Value = llvm_APInt {
        data: llvm_ArrayRef_uint64 {
            data: Value.0.as_ptr(),
            size: Value.0.len() as libc::size_t,
        },
        numbits: Value.1 as libc::c_uint,
    };
    unsafe {
        raw::llvm_Constant_getIntegerValue(Ty, Value)
    }
}

pub fn llvm_Constant_getNullValue(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_Constant_getNullValue(Ty)
    }
}

pub fn llvm_Constant_getSplatValue(inst: *const llvm_Constant) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_Constant_getSplatValue(inst)
    }
}

pub fn llvm_Constant_isAllOnesValue(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isAllOnesValue(inst) != 0
    }
}

pub fn llvm_Constant_isConstantUsed(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isConstantUsed(inst) != 0
    }
}

pub fn llvm_Constant_isDLLImportDependent(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isDLLImportDependent(inst) != 0
    }
}

pub fn llvm_Constant_isMinSignedValue(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isMinSignedValue(inst) != 0
    }
}

pub fn llvm_Constant_isNegativeZeroValue(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isNegativeZeroValue(inst) != 0
    }
}

pub fn llvm_Constant_isNullValue(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isNullValue(inst) != 0
    }
}

pub fn llvm_Constant_isThreadDependent(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isThreadDependent(inst) != 0
    }
}

pub fn llvm_Constant_isZeroValue(inst: *const llvm_Constant) -> bool {
    unsafe {
        raw::llvm_Constant_isZeroValue(inst) != 0
    }
}

pub fn llvm_Constant_removeDeadConstantUsers(inst: *const llvm_Constant) {
    unsafe {
        raw::llvm_Constant_removeDeadConstantUsers(inst)
    }
}

pub fn llvm_Constant_replaceUsesOfWithOnConstant(inst: *mut llvm_Constant, arg_1: *mut llvm_Value, arg_2: *mut llvm_Value, arg_3: *mut llvm_Use) {
    unsafe {
        raw::llvm_Constant_replaceUsesOfWithOnConstant(inst, arg_1, arg_2, arg_3)
    }
}

pub fn llvm_Constant_stripPointerCasts(inst: *const llvm_Constant) -> *const llvm_Constant {
    unsafe {
        raw::llvm_Constant_stripPointerCasts(inst)
    }
}

pub fn llvm_Constant_stripPointerCastsMut(inst: *mut llvm_Constant) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_Constant_stripPointerCastsMut(inst)
    }
}

pub fn llvm_ConstantArray_classof(V: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_ConstantArray_classof(V) != 0
    }
}

pub fn llvm_ConstantArray_get(Ty: *mut llvm_ArrayType, Values: &[*mut llvm_Constant]) -> *mut llvm_Constant {
    let Values = llvm_ArrayRef_ptr_llvm_Constant {
        data: Values.as_ptr(),
        size: Values.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_ConstantArray_get(Ty, Values)
    }
}

pub fn llvm_ConstantArray_getType(inst: *const llvm_ConstantArray) -> *mut llvm_Type {
    unsafe {
        raw::llvm_ConstantArray_getType(inst)
    }
}

pub fn llvm_ConstantFP_classof(V: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_ConstantFP_classof(V) != 0
    }
}

pub fn llvm_ConstantFP_get(Ty: *mut llvm_Type, Val: f64) -> *mut llvm_Constant {
    let Val = Val as libc::c_double;
    unsafe {
        raw::llvm_ConstantFP_get(Ty, Val)
    }
}

pub fn llvm_ConstantFP_fromStr(Ty: *mut llvm_Type, Val: &str) -> *mut llvm_Constant {
    let Val = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Val.as_ptr()) },
        length: Val.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_ConstantFP_fromStr(Ty, Val)
    }
}

pub fn llvm_ConstantFP_getInfinity(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_ConstantFP_getInfinity(Ty)
    }
}

pub fn llvm_ConstantFP_getNegativeZero(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_ConstantFP_getNegativeZero(Ty)
    }
}

pub fn llvm_ConstantFP_getZeroValueForNegation(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_ConstantFP_getZeroValueForNegation(Ty)
    }
}

pub fn llvm_ConstantFP_isExactlyValueFloat(inst: *const llvm_ConstantFP, Val: f64) -> bool {
    let Val = Val as libc::c_double;
    unsafe {
        raw::llvm_ConstantFP_isExactlyValueFloat(inst, Val) != 0
    }
}

pub fn llvm_ConstantFP_isNaN(inst: *const llvm_ConstantFP) -> bool {
    unsafe {
        raw::llvm_ConstantFP_isNaN(inst) != 0
    }
}

pub fn llvm_ConstantFP_isNegative(inst: *const llvm_ConstantFP) -> bool {
    unsafe {
        raw::llvm_ConstantFP_isNegative(inst) != 0
    }
}

pub fn llvm_ConstantFP_isZero(inst: *const llvm_ConstantFP) -> bool {
    unsafe {
        raw::llvm_ConstantFP_isZero(inst) != 0
    }
}

pub fn llvm_ConstantInt_classof(Val: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_ConstantInt_classof(Val) != 0
    }
}

pub fn llvm_ConstantInt_equalsInt(inst: *const llvm_ConstantInt, Val: u64) -> bool {
    let Val = Val as libc::uint64_t;
    unsafe {
        raw::llvm_ConstantInt_equalsInt(inst, Val) != 0
    }
}

pub fn llvm_ConstantInt_fromAPInt(Context: *mut llvm_LLVMContext, Val: (&[u64], usize)) -> *mut llvm_ConstantInt {
    let Val = llvm_APInt {
        data: llvm_ArrayRef_uint64 {
            data: Val.0.as_ptr(),
            size: Val.0.len() as libc::size_t,
        },
        numbits: Val.1 as libc::c_uint,
    };
    unsafe {
        raw::llvm_ConstantInt_fromAPInt(Context, Val)
    }
}

pub fn llvm_ConstantInt_get(Ty: *mut llvm_IntegerType, Value: u64) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint64_t;
    unsafe {
        raw::llvm_ConstantInt_get(Ty, Value)
    }
}

pub fn llvm_ConstantInt_fromStr(Ty: *mut llvm_IntegerType, Str: &str, radix: u8) -> *mut llvm_ConstantInt {
    let Str = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
        length: Str.len() as libc::size_t,
    };
    let radix = radix as libc::uint8_t;
    unsafe {
        raw::llvm_ConstantInt_fromStr(Ty, Str, radix)
    }
}

pub fn llvm_ConstantInt_getSigned(Ty: *mut llvm_IntegerType, Value: u64, isSigned: bool) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint64_t;
    let isSigned = if isSigned { 1 } else { 0 };
    unsafe {
        raw::llvm_ConstantInt_getSigned(Ty, Value, isSigned)
    }
}

pub fn llvm_ConstantInt_getBitWidth(inst: *const llvm_ConstantInt) -> usize {
    unsafe {
        raw::llvm_ConstantInt_getBitWidth(inst) as usize
    }
}

pub fn llvm_ConstantInt_getFalse(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_ConstantInt_getFalse(Ty)
    }
}

pub fn llvm_ConstantInt_getFalseWithContext(Context: *mut llvm_LLVMContext) -> *mut llvm_ConstantInt {
    unsafe {
        raw::llvm_ConstantInt_getFalseWithContext(Context)
    }
}

pub fn llvm_ConstantInt_getSExtValue(inst: *const llvm_ConstantInt) -> i64 {
    unsafe {
        raw::llvm_ConstantInt_getSExtValue(inst) as i64
    }
}

pub fn llvm_ConstantInt_getTrue(Ty: *mut llvm_Type) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_ConstantInt_getTrue(Ty)
    }
}

pub fn llvm_ConstantInt_getTrueWithContext(Context: *mut llvm_LLVMContext) -> *mut llvm_ConstantInt {
    unsafe {
        raw::llvm_ConstantInt_getTrueWithContext(Context)
    }
}

pub fn llvm_ConstantInt_getType(inst: *const llvm_ConstantInt) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_ConstantInt_getType(inst)
    }
}

pub fn llvm_ConstantInt_getZExtValue(inst: *const llvm_ConstantInt) -> u64 {
    unsafe {
        raw::llvm_ConstantInt_getZExtValue(inst) as u64
    }
}

pub fn llvm_ConstantInt_isMaxValue(inst: *const llvm_ConstantInt, isSigned: bool) -> bool {
    let isSigned = if isSigned { 1 } else { 0 };
    unsafe {
        raw::llvm_ConstantInt_isMaxValue(inst, isSigned) != 0
    }
}

pub fn llvm_ConstantInt_isMinValue(inst: *const llvm_ConstantInt, isSigned: bool) -> bool {
    let isSigned = if isSigned { 1 } else { 0 };
    unsafe {
        raw::llvm_ConstantInt_isMinValue(inst, isSigned) != 0
    }
}

pub fn llvm_ConstantInt_isMinusOne(inst: *const llvm_ConstantInt) -> bool {
    unsafe {
        raw::llvm_ConstantInt_isMinusOne(inst) != 0
    }
}

pub fn llvm_ConstantInt_isNegative(inst: *const llvm_ConstantInt) -> bool {
    unsafe {
        raw::llvm_ConstantInt_isNegative(inst) != 0
    }
}

pub fn llvm_ConstantInt_isOne(inst: *const llvm_ConstantInt) -> bool {
    unsafe {
        raw::llvm_ConstantInt_isOne(inst) != 0
    }
}

pub fn llvm_ConstantInt_isValueValidForType(Ty: *mut llvm_Type, Val: u64) -> bool {
    let Val = Val as libc::uint64_t;
    unsafe {
        raw::llvm_ConstantInt_isValueValidForType(Ty, Val) != 0
    }
}

pub fn llvm_ConstantInt_isSignedValueValidForType(Ty: *mut llvm_Type, Val: i64) -> bool {
    let Val = Val as libc::int64_t;
    unsafe {
        raw::llvm_ConstantInt_isSignedValueValidForType(Ty, Val) != 0
    }
}

pub fn llvm_ConstantInt_isZero(inst: *const llvm_ConstantInt) -> bool {
    unsafe {
        raw::llvm_ConstantInt_isZero(inst) != 0
    }
}

pub fn llvm_ConstantInt_uge(inst: *const llvm_ConstantInt, Num: u64) -> bool {
    let Num = Num as libc::uint64_t;
    unsafe {
        raw::llvm_ConstantInt_uge(inst, Num) != 0
    }
}

pub fn llvm_ConstantPointerNull_classof(Val: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_ConstantPointerNull_classof(Val) != 0
    }
}

pub fn llvm_ConstantPointerNull_destroyConstant(inst: *mut llvm_ConstantPointerNull) {
    unsafe {
        raw::llvm_ConstantPointerNull_destroyConstant(inst)
    }
}

pub fn llvm_ConstantPointerNull_get(Ty: *mut llvm_PointerType) -> *mut llvm_ConstantPointerNull {
    unsafe {
        raw::llvm_ConstantPointerNull_get(Ty)
    }
}

pub fn llvm_ConstantPointerNull_getType(inst: *const llvm_ConstantPointerNull) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_ConstantPointerNull_getType(inst)
    }
}

pub fn llvm_Function_Create(Ty: *mut llvm_FunctionType, Linkage: llvm_GlobalValue_LinkageTypes, Name: Option<&str>, Module: Option<*mut llvm_Module>) -> *mut llvm_Function {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    let Module = match Module.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_Function_Create(Ty, Linkage, Name, Module)
    }
}

pub fn llvm_Function_addFnAttr(inst: *mut llvm_Function, Kind: &str, Val: Option<&str>) {
    let Kind = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
        length: Kind.len() as libc::size_t,
    };
    let Val = match Val.map(|value| llvm_StringRef {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_Function_addFnAttr(inst, Kind, Val)
    }
}

pub fn llvm_Function_cannotDuplicate(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_cannotDuplicate(inst) != 0
    }
}

pub fn llvm_Function_classof(Val: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_Function_classof(Val) != 0
    }
}

pub fn llvm_Function_clearGC(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_clearGC(inst)
    }
}

pub fn llvm_Function_copyAttributesFrom(inst: *mut llvm_Function, Src: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_Function_copyAttributesFrom(inst, Src)
    }
}

pub fn llvm_Function_delete(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_delete(inst)
    }
}

pub fn llvm_Function_deleteBody(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_deleteBody(inst)
    }
}

pub fn llvm_Function_doesNotAccessMemory(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_doesNotAccessMemory(inst) != 0
    }
}

pub fn llvm_Function_doesNotAccessMemoryParam(inst: *const llvm_Function, n: usize) -> bool {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_doesNotAccessMemoryParam(inst, n) != 0
    }
}

pub fn llvm_Function_doesNotAlias(inst: *const llvm_Function, n: usize) -> bool {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_doesNotAlias(inst, n) != 0
    }
}

pub fn llvm_Function_doesNotCapture(inst: *const llvm_Function, n: usize) -> bool {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_doesNotCapture(inst, n) != 0
    }
}

pub fn llvm_Function_doesNotReturn(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_doesNotReturn(inst) != 0
    }
}

pub fn llvm_Function_doesNotThrow(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_doesNotThrow(inst) != 0
    }
}

pub fn llvm_Function_eraseFromParent(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_eraseFromParent(inst)
    }
}

pub fn llvm_Function_getCallingConv(inst: *const llvm_Function) -> llvm_CallingConv_ID {
    unsafe {
        raw::llvm_Function_getCallingConv(inst)
    }
}

pub fn llvm_Function_getContext(inst: *const llvm_Function) -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_Function_getContext(inst)
    }
}

pub fn llvm_Function_getDereferenceableBytes(inst: *const llvm_Function, idx: usize) -> u64 {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Function_getDereferenceableBytes(inst, idx) as u64
    }
}

pub fn llvm_Function_getFunctionType(inst: *const llvm_Function) -> *mut llvm_FunctionType {
    unsafe {
        raw::llvm_Function_getFunctionType(inst)
    }
}

pub fn llvm_Function_getIntrinsicID(inst: *const llvm_Function) -> usize {
    unsafe {
        raw::llvm_Function_getIntrinsicID(inst) as usize
    }
}

pub fn llvm_Function_getParamAlignment(inst: *const llvm_Function, idx: usize) -> usize {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Function_getParamAlignment(inst, idx) as usize
    }
}

pub fn llvm_Function_getReturnType(inst: *const llvm_Function) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Function_getReturnType(inst)
    }
}

pub fn llvm_Function_hasFnAttr(inst: *const llvm_Function, Kind: &str) -> bool {
    let Kind = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
        length: Kind.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Function_hasFnAttr(inst, Kind) != 0
    }
}

pub fn llvm_Function_hasGC(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_hasGC(inst) != 0
    }
}

pub fn llvm_Function_hasStructRetAttr(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_hasStructRetAttr(inst) != 0
    }
}

pub fn llvm_Function_hasUWTable(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_hasUWTable(inst) != 0
    }
}

pub fn llvm_Function_isIntrinsic(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_isIntrinsic(inst) != 0
    }
}

pub fn llvm_Function_isVarArg(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_isVarArg(inst) != 0
    }
}

pub fn llvm_Function_needsUnwindTableEntry(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_needsUnwindTableEntry(inst) != 0
    }
}

pub fn llvm_Function_onlyReadsMemoryParam(inst: *const llvm_Function, n: usize) -> bool {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_onlyReadsMemoryParam(inst, n) != 0
    }
}

pub fn llvm_Function_onlyReadsMemory(inst: *const llvm_Function) -> bool {
    unsafe {
        raw::llvm_Function_onlyReadsMemory(inst) != 0
    }
}

pub fn llvm_Function_removeFromParent(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_removeFromParent(inst)
    }
}

pub fn llvm_Function_setCallingConv(inst: *mut llvm_Function, CC: llvm_CallingConv_ID) {
    unsafe {
        raw::llvm_Function_setCallingConv(inst, CC)
    }
}

pub fn llvm_Function_setCannotDuplicate(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setCannotDuplicate(inst)
    }
}

pub fn llvm_Function_setDoesNotAccessMemory(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setDoesNotAccessMemory(inst)
    }
}

pub fn llvm_Function_setDoesNotAccessMemoryParam(inst: *mut llvm_Function, n: usize) {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_setDoesNotAccessMemoryParam(inst, n)
    }
}

pub fn llvm_Function_setDoesNotAlias(inst: *mut llvm_Function, n: usize) {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_setDoesNotAlias(inst, n)
    }
}

pub fn llvm_Function_setDoesNotCapture(inst: *mut llvm_Function, n: usize) {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_setDoesNotCapture(inst, n)
    }
}

pub fn llvm_Function_setDoesNotReturn(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setDoesNotReturn(inst)
    }
}

pub fn llvm_Function_setDoesNotThrow(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setDoesNotThrow(inst)
    }
}

pub fn llvm_Function_setHasUWTable(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setHasUWTable(inst)
    }
}

pub fn llvm_Function_setOnlyReadsMemory(inst: *mut llvm_Function) {
    unsafe {
        raw::llvm_Function_setOnlyReadsMemory(inst)
    }
}

pub fn llvm_Function_setOnlyReadsMemoryParam(inst: *mut llvm_Function, n: usize) {
    let n = n as libc::c_uint;
    unsafe {
        raw::llvm_Function_setOnlyReadsMemoryParam(inst, n)
    }
}

pub fn llvm_FunctionPassManager_add(inst: *mut llvm_FunctionPassManager, Pass: *mut llvm_FunctionPass) {
    unsafe {
        raw::llvm_FunctionPassManager_add(inst, Pass)
    }
}

pub fn llvm_FunctionPassManager_doFinalization(inst: *mut llvm_FunctionPassManager) -> bool {
    unsafe {
        raw::llvm_FunctionPassManager_doFinalization(inst) != 0
    }
}

pub fn llvm_FunctionPassManager_doInitialization(inst: *mut llvm_FunctionPassManager) -> bool {
    unsafe {
        raw::llvm_FunctionPassManager_doInitialization(inst) != 0
    }
}

pub fn llvm_FunctionPassManager_new(Module: *mut llvm_Module) -> *mut llvm_FunctionPassManager {
    unsafe {
        raw::llvm_FunctionPassManager_new(Module)
    }
}

pub fn llvm_FunctionPassManager_run(inst: *mut llvm_FunctionPassManager, Function: *mut llvm_Function) {
    unsafe {
        raw::llvm_FunctionPassManager_run(inst, Function)
    }
}

pub fn llvm_FunctionType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_FunctionType_classof(ty) != 0
    }
}

pub fn llvm_FunctionType_get(Result: *mut llvm_Type, Params: &[*mut llvm_Type], isVarArg: bool) -> *mut llvm_FunctionType {
    let Params = llvm_ArrayRef_ptr_llvm_Type {
        data: Params.as_ptr(),
        size: Params.len() as libc::size_t,
    };
    let isVarArg = if isVarArg { 1 } else { 0 };
    unsafe {
        raw::llvm_FunctionType_get(Result, Params, isVarArg)
    }
}

pub fn llvm_FunctionType_getNumParams(inst: *const llvm_FunctionType) -> usize {
    unsafe {
        raw::llvm_FunctionType_getNumParams(inst) as usize
    }
}

pub fn llvm_FunctionType_getParamType(inst: *const llvm_FunctionType, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_FunctionType_getParamType(inst, idx)
    }
}

pub fn llvm_FunctionType_getReturnType(inst: *const llvm_FunctionType) -> *mut llvm_Type {
    unsafe {
        raw::llvm_FunctionType_getReturnType(inst)
    }
}

pub fn llvm_FunctionType_isValidArgumentType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_FunctionType_isValidArgumentType(ty) != 0
    }
}

pub fn llvm_FunctionType_isValidReturnType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_FunctionType_isValidReturnType(ty) != 0
    }
}

pub fn llvm_FunctionType_isVarArg(inst: *const llvm_FunctionType) -> bool {
    unsafe {
        raw::llvm_FunctionType_isVarArg(inst) != 0
    }
}

pub fn llvm_GlobalObject_setSection(inst: *mut llvm_GlobalObject, S: &str) {
    let S = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(S.as_ptr()) },
        length: S.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_GlobalObject_setSection(inst, S)
    }
}

pub fn llvm_GlobalValue_copyAttributesFrom(inst: *mut llvm_GlobalValue, Src: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalValue_copyAttributesFrom(inst, Src)
    }
}

pub fn llvm_GlobalValue_delete(inst: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalValue_delete(inst)
    }
}

pub fn llvm_GlobalValue_destroyConstant(inst: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalValue_destroyConstant(inst)
    }
}

pub fn llvm_GlobalValue_eraseFromParent(inst: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalValue_eraseFromParent(inst)
    }
}

pub fn llvm_GlobalValue_getAlignment(inst: *const llvm_GlobalValue) -> usize {
    unsafe {
        raw::llvm_GlobalValue_getAlignment(inst) as usize
    }
}

pub fn llvm_GlobalValue_getDataLayout(inst: *const llvm_GlobalValue) -> *const llvm_DataLayout {
    unsafe {
        raw::llvm_GlobalValue_getDataLayout(inst)
    }
}

pub fn llvm_GlobalValue_getParent(inst: *const llvm_GlobalValue) -> *const llvm_Module {
    unsafe {
        raw::llvm_GlobalValue_getParent(inst)
    }
}

pub fn llvm_GlobalValue_getParentMut(inst: *mut llvm_GlobalValue) -> *mut llvm_Module {
    unsafe {
        raw::llvm_GlobalValue_getParentMut(inst)
    }
}

pub fn llvm_GlobalValue_getType(inst: *const llvm_GlobalValue) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_GlobalValue_getType(inst)
    }
}

pub fn llvm_GlobalValue_hasAppendingLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasAppendingLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasAvailableExternallyLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasAvailableExternallyLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasCommonLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasCommonLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasDLLExportStorageClass(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasDLLExportStorageClass(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasDLLImportStorageClass(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasDLLImportStorageClass(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasDefaultVisibility(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasDefaultVisibility(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasExternalLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasExternalLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasExternalWeakLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasExternalWeakLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasHiddenVisibility(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasHiddenVisibility(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasInternalLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasInternalLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasLinkOnceLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasLinkOnceLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasLocalLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasLocalLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasPrivateLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasPrivateLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasProtectedVisibility(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasProtectedVisibility(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasSection(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasSection(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasUnnamedAddr(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasUnnamedAddr(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasWeakAnyLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasWeakAnyLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasWeakLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasWeakLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_hasWeakODRLinkage(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_hasWeakODRLinkage(inst) != 0
    }
}

pub fn llvm_GlobalValue_isDeclaration(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_isDeclaration(inst) != 0
    }
}

pub fn llvm_GlobalValue_isDiscardableIfUnused(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_isDiscardableIfUnused(inst) != 0
    }
}

pub fn llvm_GlobalValue_isThreadLocal(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_isThreadLocal(inst) != 0
    }
}

pub fn llvm_GlobalValue_isWeakForLinker(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_isWeakForLinker(inst) != 0
    }
}

pub fn llvm_GlobalValue_mayBeOverridden(inst: *const llvm_GlobalValue) -> bool {
    unsafe {
        raw::llvm_GlobalValue_mayBeOverridden(inst) != 0
    }
}

pub fn llvm_GlobalValue_removeFromParent(inst: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalValue_removeFromParent(inst)
    }
}

pub fn llvm_GlobalValue_setThreadLocal(inst: *mut llvm_GlobalValue, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalValue_setThreadLocal(inst, Val)
    }
}

pub fn llvm_GlobalValue_setUnnamedAddr(inst: *mut llvm_GlobalValue, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalValue_setUnnamedAddr(inst, Val)
    }
}

pub fn llvm_GlobalVariable_copyAttributesFrom(inst: *mut llvm_GlobalVariable, Src: *mut llvm_GlobalValue) {
    unsafe {
        raw::llvm_GlobalVariable_copyAttributesFrom(inst, Src)
    }
}

pub fn llvm_GlobalVariable_delete(inst: *mut llvm_GlobalVariable) {
    unsafe {
        raw::llvm_GlobalVariable_delete(inst)
    }
}

pub fn llvm_GlobalVariable_eraseFromParent(inst: *mut llvm_GlobalVariable) {
    unsafe {
        raw::llvm_GlobalVariable_eraseFromParent(inst)
    }
}

pub fn llvm_GlobalVariable_getInitializer(inst: *const llvm_GlobalVariable) -> *const llvm_Constant {
    unsafe {
        raw::llvm_GlobalVariable_getInitializer(inst)
    }
}

pub fn llvm_GlobalVariable_getInitializerMut(inst: *mut llvm_GlobalVariable) -> *mut llvm_Constant {
    unsafe {
        raw::llvm_GlobalVariable_getInitializerMut(inst)
    }
}

pub fn llvm_GlobalVariable_hasDefinitiveInitializer(inst: *const llvm_GlobalVariable) -> bool {
    unsafe {
        raw::llvm_GlobalVariable_hasDefinitiveInitializer(inst) != 0
    }
}

pub fn llvm_GlobalVariable_hasInitializer(inst: *const llvm_GlobalVariable) -> bool {
    unsafe {
        raw::llvm_GlobalVariable_hasInitializer(inst) != 0
    }
}

pub fn llvm_GlobalVariable_hasUniqueInitializer(inst: *const llvm_GlobalVariable) -> bool {
    unsafe {
        raw::llvm_GlobalVariable_hasUniqueInitializer(inst) != 0
    }
}

pub fn llvm_GlobalVariable_isConstant(inst: *const llvm_GlobalVariable) -> bool {
    unsafe {
        raw::llvm_GlobalVariable_isConstant(inst) != 0
    }
}

pub fn llvm_GlobalVariable_isExternallyInitialized(inst: *const llvm_GlobalVariable) -> bool {
    unsafe {
        raw::llvm_GlobalVariable_isExternallyInitialized(inst) != 0
    }
}

pub fn llvm_GlobalVariable_new(Ty: *mut llvm_Type, isConstant: bool, Linkage: llvm_GlobalValue_LinkageTypes) -> *mut llvm_GlobalVariable {
    let isConstant = if isConstant { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalVariable_new(Ty, isConstant, Linkage)
    }
}

pub fn llvm_GlobalVariable_newWithModule(Module: *mut llvm_Module, Ty: *mut llvm_Type, isConstant: bool, Linkage: llvm_GlobalValue_LinkageTypes, Initializer: *mut llvm_Constant) -> *mut llvm_GlobalVariable {
    let isConstant = if isConstant { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalVariable_newWithModule(Module, Ty, isConstant, Linkage, Initializer)
    }
}

pub fn llvm_GlobalVariable_removeFromParent(inst: *mut llvm_GlobalVariable) {
    unsafe {
        raw::llvm_GlobalVariable_removeFromParent(inst)
    }
}

pub fn llvm_GlobalVariable_setConstant(inst: *mut llvm_GlobalVariable, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalVariable_setConstant(inst, Val)
    }
}

pub fn llvm_GlobalVariable_setExternallyInitialized(inst: *mut llvm_GlobalVariable, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_GlobalVariable_setExternallyInitialized(inst, Val)
    }
}

pub fn llvm_GlobalVariable_setInitializer(inst: *mut llvm_GlobalVariable, InitVal: *mut llvm_Constant) {
    unsafe {
        raw::llvm_GlobalVariable_setInitializer(inst, InitVal)
    }
}

pub fn llvm_IRBuilder_CreateAShrByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAShrByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateAShr(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAShr(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateAdd(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAdd(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateAddrSpaceCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAddrSpaceCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateAlignedLoadVolatile(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Align: usize, isVolatile: bool, Name: Option<&str>) -> *mut llvm_LoadInst {
    let Align = Align as libc::c_uint;
    let isVolatile = if isVolatile { 1 } else { 0 };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAlignedLoadVolatile(inst, Ptr, Align, isVolatile, Name)
    }
}

pub fn llvm_IRBuilder_CreateAlignedLoad(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Align: usize, Name: Option<&str>) -> *mut llvm_LoadInst {
    let Align = Align as libc::c_uint;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAlignedLoad(inst, Ptr, Align, Name)
    }
}

pub fn llvm_IRBuilder_CreateAlignedStore(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Ptr: *mut llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut llvm_StoreInst {
    let Align = Align as libc::c_uint;
    let isVolatile = match isVolatile.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAlignedStore(inst, Value, Ptr, Align, isVolatile)
    }
}

pub fn llvm_IRBuilder_CreateAlloca(inst: *mut llvm_IRBuilder, Ty: *mut llvm_Type, ArraySize: Option<*mut llvm_Value>, Name: Option<&str>) -> *mut llvm_AllocaInst {
    let ArraySize = match ArraySize.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAlloca(inst, Ty, ArraySize, Name)
    }
}

pub fn llvm_IRBuilder_CreateAndByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAndByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateAnd(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateAnd(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateBinOp(inst: *mut llvm_IRBuilder, Opcode: llvm_Instruction_BinaryOps, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateBinOp(inst, Opcode, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateBitCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateBitCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateBr(inst: *mut llvm_IRBuilder, Dest: *mut llvm_BasicBlock) -> *mut llvm_BranchInst {
    unsafe {
        raw::llvm_IRBuilder_CreateBr(inst, Dest)
    }
}

pub fn llvm_IRBuilder_CreateCall(inst: *mut llvm_IRBuilder, Callee: *mut llvm_Value, Args: &[*mut llvm_Value], Name: Option<&str>) -> *mut llvm_CallInst {
    let Args = llvm_ArrayRef_ptr_llvm_Value {
        data: Args.as_ptr(),
        size: Args.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateCall(inst, Callee, Args, Name)
    }
}

pub fn llvm_IRBuilder_CreateCast(inst: *mut llvm_IRBuilder, Opcode: llvm_Instruction_CastOps, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateCast(inst, Opcode, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateCondBr(inst: *mut llvm_IRBuilder, Cond: *mut llvm_Value, TrueBlock: *mut llvm_BasicBlock, FalseBlock: *mut llvm_BasicBlock) -> *mut llvm_BranchInst {
    unsafe {
        raw::llvm_IRBuilder_CreateCondBr(inst, Cond, TrueBlock, FalseBlock)
    }
}

pub fn llvm_IRBuilder_CreateExactSDiv(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateExactSDiv(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateExactUDiv(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateExactUDiv(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateExtractElement(inst: *mut llvm_IRBuilder, Vec: *mut llvm_Value, Idx: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateExtractElement(inst, Vec, Idx, Name)
    }
}

pub fn llvm_IRBuilder_CreateExtractInteger(inst: *mut llvm_IRBuilder, DL: *const llvm_DataLayout, From: *mut llvm_Value, ExtractedTy: *mut llvm_IntegerType, Offset: u64, Name: &str) -> *mut llvm_Value {
    let Offset = Offset as libc::uint64_t;
    let Name = std_string {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateExtractInteger(inst, DL, From, ExtractedTy, Offset, Name)
    }
}

pub fn llvm_IRBuilder_CreateExtractValue(inst: *mut llvm_IRBuilder, Agg: *mut llvm_Value, Indexes: &[libc::c_uint], Name: Option<&str>) -> *mut llvm_Value {
    let Indexes = llvm_ArrayRef_uint {
        data: Indexes.as_ptr(),
        size: Indexes.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateExtractValue(inst, Agg, Indexes, Name)
    }
}

pub fn llvm_IRBuilder_CreateFAdd(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFAdd(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmp(inst: *mut llvm_IRBuilder, Pred: llvm_CmpInst_Predicate, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmp(inst, Pred, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpOEQ(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpOEQ(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpOGE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpOGE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpOGT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpOGT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpOLE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpOLE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpOLT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpOLT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpONE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpONE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpORD(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpORD(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpUEQ(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpUEQ(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpUGE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpUGE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpUGT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpUGT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpULE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpULE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpULT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpULT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpUNE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpUNE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFCmpUNO(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFCmpUNO(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFDiv(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFDiv(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFMul(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFMul(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFNeg(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFNeg(inst, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateFPCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFPCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateFPExt(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFPExt(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateFPToSI(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFPToSI(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateFPToUI(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFPToUI(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateFPTrunc(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFPTrunc(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateFRem(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFRem(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFSub(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFSub(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateFence(inst: *mut llvm_IRBuilder, Ordering: llvm_AtomicOrdering, SynchScope: Option<llvm_SynchronizationScope>, Name: Option<&str>) -> *mut llvm_FenceInst {
    let SynchScope = match SynchScope.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateFence(inst, Ordering, SynchScope, Name)
    }
}

pub fn llvm_IRBuilder_CreateGEP(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Indexes: &[*mut llvm_Value], Name: Option<&str>) -> *mut llvm_Value {
    let Indexes = llvm_ArrayRef_ptr_llvm_Value {
        data: Indexes.as_ptr(),
        size: Indexes.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateGEP(inst, Ptr, Indexes, Name)
    }
}

pub fn llvm_IRBuilder_CreateGlobalStringPtr(inst: *mut llvm_IRBuilder, Str: &str, Name: Option<&str>) -> *mut llvm_Value {
    let Str = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
        length: Str.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateGlobalStringPtr(inst, Str, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmp(inst: *mut llvm_IRBuilder, Pred: llvm_CmpInst_Predicate, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmp(inst, Pred, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpEQ(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpEQ(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpNE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpNE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpSGE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpSGE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpSGT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpSGT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpSLE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpSLE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpSLT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpSLT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpUGE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpUGE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpUGT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpUGT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpULE(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpULE(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateICmpULT(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateICmpULT(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateInBoundsGEP(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Indexes: &[*mut llvm_Value], Name: Option<&str>) -> *mut llvm_Value {
    let Indexes = llvm_ArrayRef_ptr_llvm_Value {
        data: Indexes.as_ptr(),
        size: Indexes.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateInBoundsGEP(inst, Ptr, Indexes, Name)
    }
}

pub fn llvm_IRBuilder_CreateIndirectBr(inst: *mut llvm_IRBuilder, Addr: *mut llvm_Value, NumCases: Option<usize>) -> *mut llvm_IndirectBrInst {
    let NumCases = match NumCases.map(|value| value as libc::c_uint) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateIndirectBr(inst, Addr, NumCases)
    }
}

pub fn llvm_IRBuilder_CreateInsertElement(inst: *mut llvm_IRBuilder, Vec: *mut llvm_Value, NewElt: *mut llvm_Value, Idx: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateInsertElement(inst, Vec, NewElt, Idx, Name)
    }
}

pub fn llvm_IRBuilder_CreateInsertValue(inst: *mut llvm_IRBuilder, Agg: *mut llvm_Value, Value: *mut llvm_Value, Indexes: &[libc::c_uint], Name: Option<&str>) -> *mut llvm_Value {
    let Indexes = llvm_ArrayRef_uint {
        data: Indexes.as_ptr(),
        size: Indexes.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateInsertValue(inst, Agg, Value, Indexes, Name)
    }
}

pub fn llvm_IRBuilder_CreateIntCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, isSigned: bool, Name: Option<&str>) -> *mut llvm_Value {
    let isSigned = if isSigned { 1 } else { 0 };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateIntCast(inst, Value, DestTy, isSigned, Name)
    }
}

pub fn llvm_IRBuilder_CreateIntToPtr(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateIntToPtr(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateInvoke(inst: *mut llvm_IRBuilder, Callee: *mut llvm_Value, NormalDest: *mut llvm_BasicBlock, UnwindDest: *mut llvm_BasicBlock, Args: &[*mut llvm_Value], Name: Option<&str>) -> *mut llvm_InvokeInst {
    let Args = llvm_ArrayRef_ptr_llvm_Value {
        data: Args.as_ptr(),
        size: Args.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string_const {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateInvoke(inst, Callee, NormalDest, UnwindDest, Args, Name)
    }
}

pub fn llvm_IRBuilder_CreateIsNotNull(inst: *mut llvm_IRBuilder, Arg: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateIsNotNull(inst, Arg, Name)
    }
}

pub fn llvm_IRBuilder_CreateIsNull(inst: *mut llvm_IRBuilder, Arg: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateIsNull(inst, Arg, Name)
    }
}

pub fn llvm_IRBuilder_CreateLShr(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateLShr(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateLShrByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateLShrByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateLandingPad(inst: *mut llvm_IRBuilder, Ty: *mut llvm_Type, PersFn: *mut llvm_Value, NumClauses: usize, Name: Option<&str>) -> *mut llvm_LandingPadInst {
    let NumClauses = NumClauses as libc::c_uint;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateLandingPad(inst, Ty, PersFn, NumClauses, Name)
    }
}

pub fn llvm_IRBuilder_CreateLoadVolatile(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, isVolatile: bool, Name: Option<&str>) -> *mut llvm_LoadInst {
    let isVolatile = if isVolatile { 1 } else { 0 };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateLoadVolatile(inst, Ptr, isVolatile, Name)
    }
}

pub fn llvm_IRBuilder_CreateLoad(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_LoadInst {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateLoad(inst, Ptr, Name)
    }
}

pub fn llvm_IRBuilder_CreateMul(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateMul(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNSWAdd(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNSWAdd(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNSWMul(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNSWMul(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNSWNeg(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNSWNeg(inst, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateNSWSub(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNSWSub(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNUWAdd(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNUWAdd(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNUWMul(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNUWMul(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNUWNeg(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNUWNeg(inst, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateNUWSub(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNUWSub(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateNeg(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNeg(inst, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateNot(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateNot(inst, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateOrByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateOrByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateOr(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateOr(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreatePHI(inst: *mut llvm_IRBuilder, Ty: *mut llvm_Type, NumReservedValues: usize, Name: Option<&str>) -> *mut llvm_PHINode {
    let NumReservedValues = NumReservedValues as libc::c_uint;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreatePHI(inst, Ty, NumReservedValues, Name)
    }
}

pub fn llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreatePointerCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreatePointerCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreatePtrDiff(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreatePtrDiff(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreatePtrToInt(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreatePtrToInt(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateResume(inst: *mut llvm_IRBuilder, Exn: *mut llvm_Value) -> *mut llvm_ResumeInst {
    unsafe {
        raw::llvm_IRBuilder_CreateResume(inst, Exn)
    }
}

pub fn llvm_IRBuilder_CreateRet(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value) -> *mut llvm_ReturnInst {
    unsafe {
        raw::llvm_IRBuilder_CreateRet(inst, Value)
    }
}

pub fn llvm_IRBuilder_CreateRetVoid(inst: *mut llvm_IRBuilder) -> *mut llvm_ReturnInst {
    unsafe {
        raw::llvm_IRBuilder_CreateRetVoid(inst)
    }
}

pub fn llvm_IRBuilder_CreateSDiv(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSDiv(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateSExt(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSExt(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateSExtOrBitCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSExtOrBitCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateSExtOrTrunc(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSExtOrTrunc(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateSIToFP(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSIToFP(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateSRem(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSRem(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateSelect(inst: *mut llvm_IRBuilder, C: *mut llvm_Value, TrueValue: *mut llvm_Value, FalseValue: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSelect(inst, C, TrueValue, FalseValue, Name)
    }
}

pub fn llvm_IRBuilder_CreateShlByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateShlByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateShl(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateShl(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateShuffleVector(inst: *mut llvm_IRBuilder, V1: *mut llvm_Value, P2: *mut llvm_Value, Mask: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateShuffleVector(inst, V1, P2, Mask, Name)
    }
}

pub fn llvm_IRBuilder_CreateStore(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Ptr: *mut llvm_Value, isVolatile: Option<bool>) -> *mut llvm_StoreInst {
    let isVolatile = match isVolatile.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateStore(inst, Value, Ptr, isVolatile)
    }
}

pub fn llvm_IRBuilder_CreateStructGEP(inst: *mut llvm_IRBuilder, Ptr: *mut llvm_Value, Index: usize, Name: Option<&str>) -> *mut llvm_Value {
    let Index = Index as libc::c_uint;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateStructGEP(inst, Ptr, Index, Name)
    }
}

pub fn llvm_IRBuilder_CreateSub(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSub(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateSwitch(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, Dest: *mut llvm_BasicBlock, NumCases: Option<usize>) -> *mut llvm_SwitchInst {
    let NumCases = match NumCases.map(|value| value as libc::c_uint) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateSwitch(inst, Value, Dest, NumCases)
    }
}

pub fn llvm_IRBuilder_CreateTrunc(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateTrunc(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateTruncOrBitCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateTruncOrBitCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateUDiv(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateUDiv(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateUIToFP(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateUIToFP(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateURem(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateURem(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateUnreachable(inst: *mut llvm_IRBuilder) -> *mut llvm_UnreachableInst {
    unsafe {
        raw::llvm_IRBuilder_CreateUnreachable(inst)
    }
}

pub fn llvm_IRBuilder_CreateVAArg(inst: *mut llvm_IRBuilder, List: *mut llvm_Value, Ty: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_VAArgInst {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateVAArg(inst, List, Ty, Name)
    }
}

pub fn llvm_IRBuilder_CreateVectorSplat(inst: *mut llvm_IRBuilder, NumElements: usize, Value: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let NumElements = NumElements as libc::c_uint;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateVectorSplat(inst, NumElements, Value, Name)
    }
}

pub fn llvm_IRBuilder_CreateXor(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: *mut llvm_Value, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateXor(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateXorByValue(inst: *mut llvm_IRBuilder, LHS: *mut llvm_Value, RHS: u64, Name: Option<&str>) -> *mut llvm_Value {
    let RHS = RHS as libc::uint64_t;
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateXorByValue(inst, LHS, RHS, Name)
    }
}

pub fn llvm_IRBuilder_CreateZExt(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateZExt(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateZExtOrBitCast(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateZExtOrBitCast(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_CreateZExtOrTrunc(inst: *mut llvm_IRBuilder, Value: *mut llvm_Value, DestTy: *mut llvm_Type, Name: Option<&str>) -> *mut llvm_Value {
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilder_CreateZExtOrTrunc(inst, Value, DestTy, Name)
    }
}

pub fn llvm_IRBuilder_delete(inst: *mut llvm_IRBuilder) {
    unsafe {
        raw::llvm_IRBuilder_delete(inst)
    }
}

pub fn llvm_IRBuilder_isNamePreserving(inst: *const llvm_IRBuilder) -> bool {
    unsafe {
        raw::llvm_IRBuilder_isNamePreserving(inst) != 0
    }
}

pub fn llvm_IRBuilder_new(Context: *mut llvm_LLVMContext) -> *mut llvm_IRBuilder {
    unsafe {
        raw::llvm_IRBuilder_new(Context)
    }
}

pub fn llvm_IRBuilder_new_in_block(BB: *mut llvm_BasicBlock) -> *mut llvm_IRBuilder {
    unsafe {
        raw::llvm_IRBuilder_new_in_block(BB)
    }
}

pub fn llvm_IRBuilderBase_ClearInsertionPoint(inst: *mut llvm_IRBuilderBase) {
    unsafe {
        raw::llvm_IRBuilderBase_ClearInsertionPoint(inst)
    }
}

pub fn llvm_IRBuilderBase_CreateGlobalString(inst: *mut llvm_IRBuilderBase, Str: &str, Name: Option<&str>) -> *mut llvm_Value {
    let Str = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
        length: Str.len() as libc::size_t,
    };
    let Name = match Name.map(|value| std_string {
            data: unsafe { ::std::mem::transmute(value.as_ptr()) },
            length: value.len() as libc::size_t,
        }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateGlobalString(inst, Str, Name)
    }
}

pub fn llvm_IRBuilderBase_CreateLifetimeEnd(inst: *mut llvm_IRBuilderBase, Ptr: *mut llvm_Value, Size: Option<*mut llvm_ConstantInt>) -> *mut llvm_CallInst {
    let Size = match Size.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateLifetimeEnd(inst, Ptr, Size)
    }
}

pub fn llvm_IRBuilderBase_CreateLifetimeStart(inst: *mut llvm_IRBuilderBase, Ptr: *mut llvm_Value, Size: Option<*mut llvm_ConstantInt>) -> *mut llvm_CallInst {
    let Size = match Size.map(|value| value) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateLifetimeStart(inst, Ptr, Size)
    }
}

pub fn llvm_IRBuilderBase_CreateMemCpy(inst: *mut llvm_IRBuilderBase, Dst: *mut llvm_Value, Src: *mut llvm_Value, Size: *mut llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut llvm_CallInst {
    let Align = Align as libc::c_uint;
    let isVolatile = match isVolatile.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateMemCpy(inst, Dst, Src, Size, Align, isVolatile)
    }
}

pub fn llvm_IRBuilderBase_CreateMemMove(inst: *mut llvm_IRBuilderBase, Dst: *mut llvm_Value, Src: *mut llvm_Value, Size: *mut llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut llvm_CallInst {
    let Align = Align as libc::c_uint;
    let isVolatile = match isVolatile.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateMemMove(inst, Dst, Src, Size, Align, isVolatile)
    }
}

pub fn llvm_IRBuilderBase_CreateMemSet(inst: *mut llvm_IRBuilderBase, Ptr: *mut llvm_Value, Value: *mut llvm_Value, Size: *mut llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut llvm_CallInst {
    let Align = Align as libc::c_uint;
    let isVolatile = match isVolatile.map(|value| if value { 1 } else { 0 }) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_CreateMemSet(inst, Ptr, Value, Size, Align, isVolatile)
    }
}

pub fn llvm_IRBuilderBase_GetInsertBlock(inst: *const llvm_IRBuilderBase) -> *mut llvm_BasicBlock {
    unsafe {
        raw::llvm_IRBuilderBase_GetInsertBlock(inst)
    }
}

pub fn llvm_IRBuilderBase_SetCurrentDebugLocation(inst: *mut llvm_IRBuilderBase, Loc: *const llvm_DebugLoc) {
    unsafe {
        raw::llvm_IRBuilderBase_SetCurrentDebugLocation(inst, Loc)
    }
}

pub fn llvm_IRBuilderBase_SetDefaultFPMathTag(inst: *mut llvm_IRBuilderBase, FPMathTag: *mut llvm_MDNode) {
    unsafe {
        raw::llvm_IRBuilderBase_SetDefaultFPMathTag(inst, FPMathTag)
    }
}

pub fn llvm_IRBuilderBase_SetInsertPointAtInst(inst: *mut llvm_IRBuilderBase, Inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_IRBuilderBase_SetInsertPointAtInst(inst, Inst)
    }
}

pub fn llvm_IRBuilderBase_SetInsertPoint(inst: *mut llvm_IRBuilderBase, BB: *mut llvm_BasicBlock) {
    unsafe {
        raw::llvm_IRBuilderBase_SetInsertPoint(inst, BB)
    }
}

pub fn llvm_IRBuilderBase_SetInstDebugLocation(inst: *const llvm_IRBuilderBase, Inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_IRBuilderBase_SetInstDebugLocation(inst, Inst)
    }
}

pub fn llvm_IRBuilderBase_getContext(inst: *const llvm_IRBuilderBase) -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_IRBuilderBase_getContext(inst)
    }
}

pub fn llvm_IRBuilderBase_getCurrentFunctionReturnType(inst: *const llvm_IRBuilderBase) -> *mut llvm_Type {
    unsafe {
        raw::llvm_IRBuilderBase_getCurrentFunctionReturnType(inst)
    }
}

pub fn llvm_IRBuilderBase_getDefaultFPMathTag(inst: *const llvm_IRBuilderBase) -> *mut llvm_MDNode {
    unsafe {
        raw::llvm_IRBuilderBase_getDefaultFPMathTag(inst)
    }
}

pub fn llvm_IRBuilderBase_getDoubleTy(inst: *mut llvm_IRBuilderBase) -> *mut llvm_Type {
    unsafe {
        raw::llvm_IRBuilderBase_getDoubleTy(inst)
    }
}

pub fn llvm_IRBuilderBase_getFalse(inst: *mut llvm_IRBuilderBase) -> *mut llvm_ConstantInt {
    unsafe {
        raw::llvm_IRBuilderBase_getFalse(inst)
    }
}

pub fn llvm_IRBuilderBase_getFloatTy(inst: *mut llvm_IRBuilderBase) -> *mut llvm_Type {
    unsafe {
        raw::llvm_IRBuilderBase_getFloatTy(inst)
    }
}

pub fn llvm_IRBuilderBase_getHalfTy(inst: *mut llvm_IRBuilderBase) -> *mut llvm_Type {
    unsafe {
        raw::llvm_IRBuilderBase_getHalfTy(inst)
    }
}

pub fn llvm_IRBuilderBase_getInt(inst: *mut llvm_IRBuilderBase, Value: (&[u64], usize)) -> *mut llvm_ConstantInt {
    let Value = llvm_APInt {
        data: llvm_ArrayRef_uint64 {
            data: Value.0.as_ptr(),
            size: Value.0.len() as libc::size_t,
        },
        numbits: Value.1 as libc::c_uint,
    };
    unsafe {
        raw::llvm_IRBuilderBase_getInt(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt1(inst: *mut llvm_IRBuilderBase, Value: bool) -> *mut llvm_ConstantInt {
    let Value = if Value { 1 } else { 0 };
    unsafe {
        raw::llvm_IRBuilderBase_getInt1(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt16(inst: *mut llvm_IRBuilderBase, Value: u16) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint16_t;
    unsafe {
        raw::llvm_IRBuilderBase_getInt16(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt16Ty(inst: *mut llvm_IRBuilderBase) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_IRBuilderBase_getInt16Ty(inst)
    }
}

pub fn llvm_IRBuilderBase_getInt1Ty(inst: *mut llvm_IRBuilderBase) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_IRBuilderBase_getInt1Ty(inst)
    }
}

pub fn llvm_IRBuilderBase_getInt32(inst: *mut llvm_IRBuilderBase, Value: u32) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint32_t;
    unsafe {
        raw::llvm_IRBuilderBase_getInt32(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt32Ty(inst: *mut llvm_IRBuilderBase) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_IRBuilderBase_getInt32Ty(inst)
    }
}

pub fn llvm_IRBuilderBase_getInt64(inst: *mut llvm_IRBuilderBase, Value: u64) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint64_t;
    unsafe {
        raw::llvm_IRBuilderBase_getInt64(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt64Ty(inst: *mut llvm_IRBuilderBase) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_IRBuilderBase_getInt64Ty(inst)
    }
}

pub fn llvm_IRBuilderBase_getInt8(inst: *mut llvm_IRBuilderBase, Value: u8) -> *mut llvm_ConstantInt {
    let Value = Value as libc::uint8_t;
    unsafe {
        raw::llvm_IRBuilderBase_getInt8(inst, Value)
    }
}

pub fn llvm_IRBuilderBase_getInt8PtrTy(inst: *mut llvm_IRBuilderBase, AddrSpace: Option<usize>) -> *mut llvm_PointerType {
    let AddrSpace = match AddrSpace.map(|value| value as libc::c_uint) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_getInt8PtrTy(inst, AddrSpace)
    }
}

pub fn llvm_IRBuilderBase_getInt8Ty(inst: *mut llvm_IRBuilderBase) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_IRBuilderBase_getInt8Ty(inst)
    }
}

pub fn llvm_IRBuilderBase_getIntN(inst: *mut llvm_IRBuilderBase, NumBits: usize, Value: u64) -> *mut llvm_ConstantInt {
    let NumBits = NumBits as libc::c_uint;
    let Value = Value as libc::uint64_t;
    unsafe {
        raw::llvm_IRBuilderBase_getIntN(inst, NumBits, Value)
    }
}

pub fn llvm_IRBuilderBase_getIntNTy(inst: *mut llvm_IRBuilderBase, NumBits: usize) -> *mut llvm_IntegerType {
    let NumBits = NumBits as libc::c_uint;
    unsafe {
        raw::llvm_IRBuilderBase_getIntNTy(inst, NumBits)
    }
}

pub fn llvm_IRBuilderBase_getIntPtrTy(inst: *mut llvm_IRBuilderBase, DL: *const llvm_DataLayout, AddrSpace: Option<usize>) -> *mut llvm_IntegerType {
    let AddrSpace = match AddrSpace.map(|value| value as libc::c_uint) {
        None => ::std::ptr::null(),
        Some(ref value) => value as *const _,
    };
    unsafe {
        raw::llvm_IRBuilderBase_getIntPtrTy(inst, DL, AddrSpace)
    }
}

pub fn llvm_IRBuilderBase_getTrue(inst: *mut llvm_IRBuilderBase) -> *mut llvm_ConstantInt {
    unsafe {
        raw::llvm_IRBuilderBase_getTrue(inst)
    }
}

pub fn llvm_IRBuilderBase_getVoidTy(inst: *mut llvm_IRBuilderBase) -> *mut llvm_Type {
    unsafe {
        raw::llvm_IRBuilderBase_getVoidTy(inst)
    }
}

pub fn llvm_IRBuilderBase_new(Context: *mut llvm_LLVMContext) -> *mut llvm_IRBuilderBase {
    unsafe {
        raw::llvm_IRBuilderBase_new(Context)
    }
}

pub fn llvm_Instruction_clone(inst: *const llvm_Instruction) -> *mut llvm_Instruction {
    unsafe {
        raw::llvm_Instruction_clone(inst)
    }
}

pub fn llvm_Instruction_copyFastMathFlags(inst: *mut llvm_Instruction, Inst: *const llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_copyFastMathFlags(inst, Inst)
    }
}

pub fn llvm_Instruction_delete(inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_delete(inst)
    }
}

pub fn llvm_Instruction_dropUnknownMetadataFromIDS(inst: *mut llvm_Instruction, KnownIDs: &[libc::c_uint]) {
    let KnownIDs = llvm_ArrayRef_uint {
        data: KnownIDs.as_ptr(),
        size: KnownIDs.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Instruction_dropUnknownMetadataFromIDS(inst, KnownIDs)
    }
}

pub fn llvm_Instruction_dropUnknownMetadata(inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_dropUnknownMetadata(inst)
    }
}

pub fn llvm_Instruction_eraseFromParent(inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_eraseFromParent(inst)
    }
}

pub fn llvm_Instruction_getDataLayout(inst: *const llvm_Instruction) -> *const llvm_DataLayout {
    unsafe {
        raw::llvm_Instruction_getDataLayout(inst)
    }
}

pub fn llvm_Instruction_getDebugLoc(inst: *const llvm_Instruction) -> *const llvm_DebugLoc {
    unsafe {
        raw::llvm_Instruction_getDebugLoc(inst)
    }
}

pub fn llvm_Instruction_getMetadataStr(inst: *const llvm_Instruction, Kind: &str) -> *mut llvm_MDNode {
    let Kind = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
        length: Kind.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Instruction_getMetadataStr(inst, Kind)
    }
}

pub fn llvm_Instruction_getMetadata(inst: *const llvm_Instruction, KindID: usize) -> *mut llvm_MDNode {
    let KindID = KindID as libc::c_uint;
    unsafe {
        raw::llvm_Instruction_getMetadata(inst, KindID)
    }
}

pub fn llvm_Instruction_getOpcode(inst: *const llvm_Instruction) -> usize {
    unsafe {
        raw::llvm_Instruction_getOpcode(inst) as usize
    }
}

pub fn llvm_Instruction_getParentMut(inst: *mut llvm_Instruction) -> *mut llvm_BasicBlock {
    unsafe {
        raw::llvm_Instruction_getParentMut(inst)
    }
}

pub fn llvm_Instruction_getParent(inst: *const llvm_Instruction) -> *const llvm_BasicBlock {
    unsafe {
        raw::llvm_Instruction_getParent(inst)
    }
}

pub fn llvm_Instruction_hasAllowReciprocal(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasAllowReciprocal(inst) != 0
    }
}

pub fn llvm_Instruction_hasMetadata(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasMetadata(inst) != 0
    }
}

pub fn llvm_Instruction_hasMetadataOtherThanDebugLoc(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasMetadataOtherThanDebugLoc(inst) != 0
    }
}

pub fn llvm_Instruction_hasNoInfs(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasNoInfs(inst) != 0
    }
}

pub fn llvm_Instruction_hasNoNaNs(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasNoNaNs(inst) != 0
    }
}

pub fn llvm_Instruction_hasNoSignedZeros(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasNoSignedZeros(inst) != 0
    }
}

pub fn llvm_Instruction_hasUnsafeAlgebra(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_hasUnsafeAlgebra(inst) != 0
    }
}

pub fn llvm_Instruction_insertAfter(inst: *mut llvm_Instruction, InsertPos: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_insertAfter(inst, InsertPos)
    }
}

pub fn llvm_Instruction_insertBefore(inst: *mut llvm_Instruction, InsertPos: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_insertBefore(inst, InsertPos)
    }
}

pub fn llvm_Instruction_isArithmeticShift(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isArithmeticShift(inst) != 0
    }
}

pub fn llvm_Instruction_isAssociative(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isAssociative(inst) != 0
    }
}

pub fn llvm_Instruction_isBinaryOp(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isBinaryOp(inst) != 0
    }
}

pub fn llvm_Instruction_isCast(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isCast(inst) != 0
    }
}

pub fn llvm_Instruction_isCommutative(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isCommutative(inst) != 0
    }
}

pub fn llvm_Instruction_isIdempotent(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isIdempotent(inst) != 0
    }
}

pub fn llvm_Instruction_isIdenticalTo(inst: *const llvm_Instruction, Inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isIdenticalTo(inst, Inst) != 0
    }
}

pub fn llvm_Instruction_isIdenticalToWhenDefined(inst: *const llvm_Instruction, Inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isIdenticalToWhenDefined(inst, Inst) != 0
    }
}

pub fn llvm_Instruction_isLogicalShift(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isLogicalShift(inst) != 0
    }
}

pub fn llvm_Instruction_isNilpotent(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isNilpotent(inst) != 0
    }
}

pub fn llvm_Instruction_isSameOperationAs(inst: *const llvm_Instruction, Inst: *const llvm_Instruction, flags: usize) -> bool {
    let flags = flags as libc::c_uint;
    unsafe {
        raw::llvm_Instruction_isSameOperationAs(inst, Inst, flags) != 0
    }
}

pub fn llvm_Instruction_isShift(inst: *mut llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isShift(inst) != 0
    }
}

pub fn llvm_Instruction_isTerminator(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_isTerminator(inst) != 0
    }
}

pub fn llvm_Instruction_isUsedOutsideOfBlock(inst: *const llvm_Instruction, BB: *const llvm_BasicBlock) -> bool {
    unsafe {
        raw::llvm_Instruction_isUsedOutsideOfBlock(inst, BB) != 0
    }
}

pub fn llvm_Instruction_mayHaveSideEffects(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayHaveSideEffects(inst) != 0
    }
}

pub fn llvm_Instruction_mayReadFromMemory(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayReadFromMemory(inst) != 0
    }
}

pub fn llvm_Instruction_mayReadOrWriteMemory(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayReadOrWriteMemory(inst) != 0
    }
}

pub fn llvm_Instruction_mayReturn(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayReturn(inst) != 0
    }
}

pub fn llvm_Instruction_mayThrow(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayThrow(inst) != 0
    }
}

pub fn llvm_Instruction_mayWriteToMemory(inst: *const llvm_Instruction) -> bool {
    unsafe {
        raw::llvm_Instruction_mayWriteToMemory(inst) != 0
    }
}

pub fn llvm_Instruction_moveBefore(inst: *mut llvm_Instruction, MovePos: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_moveBefore(inst, MovePos)
    }
}

pub fn llvm_Instruction_removeFromParent(inst: *mut llvm_Instruction) {
    unsafe {
        raw::llvm_Instruction_removeFromParent(inst)
    }
}

pub fn llvm_Instruction_setDebugLoc(inst: *mut llvm_Instruction, Loc: *const llvm_DebugLoc) {
    unsafe {
        raw::llvm_Instruction_setDebugLoc(inst, Loc)
    }
}

pub fn llvm_Instruction_setHasAllowReciprocal(inst: *mut llvm_Instruction, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_Instruction_setHasAllowReciprocal(inst, Val)
    }
}

pub fn llvm_Instruction_setHasNoInfs(inst: *mut llvm_Instruction, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_Instruction_setHasNoInfs(inst, Val)
    }
}

pub fn llvm_Instruction_setHasNoNaNs(inst: *mut llvm_Instruction, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_Instruction_setHasNoNaNs(inst, Val)
    }
}

pub fn llvm_Instruction_setHasNoSignedZeros(inst: *mut llvm_Instruction, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_Instruction_setHasNoSignedZeros(inst, Val)
    }
}

pub fn llvm_Instruction_setHasUnsafeAlgebra(inst: *mut llvm_Instruction, Val: bool) {
    let Val = if Val { 1 } else { 0 };
    unsafe {
        raw::llvm_Instruction_setHasUnsafeAlgebra(inst, Val)
    }
}

pub fn llvm_Instruction_setMetadata(inst: *mut llvm_Instruction, KindID: usize, Node: *mut llvm_MDNode) {
    let KindID = KindID as libc::c_uint;
    unsafe {
        raw::llvm_Instruction_setMetadata(inst, KindID, Node)
    }
}

pub fn llvm_Instruction_setMetadataStr(inst: *mut llvm_Instruction, Kind: &str, Node: *mut llvm_MDNode) {
    let Kind = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
        length: Kind.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Instruction_setMetadataStr(inst, Kind, Node)
    }
}

pub fn llvm_Instruction_user_back(inst: *const llvm_Instruction) -> *const llvm_Instruction {
    unsafe {
        raw::llvm_Instruction_user_back(inst)
    }
}

pub fn llvm_Instruction_user_back_mut(inst: *mut llvm_Instruction) -> *mut llvm_Instruction {
    unsafe {
        raw::llvm_Instruction_user_back_mut(inst)
    }
}

pub fn llvm_IntegerType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_IntegerType_classof(ty) != 0
    }
}

pub fn llvm_IntegerType_get(ctx: *mut llvm_LLVMContext, NumBits: usize) -> *mut llvm_IntegerType {
    let NumBits = NumBits as libc::c_uint;
    unsafe {
        raw::llvm_IntegerType_get(ctx, NumBits)
    }
}

pub fn llvm_IntegerType_getBitMask(inst: *const llvm_IntegerType) -> u64 {
    unsafe {
        raw::llvm_IntegerType_getBitMask(inst) as u64
    }
}

pub fn llvm_IntegerType_getBitWidth(inst: *const llvm_IntegerType) -> usize {
    unsafe {
        raw::llvm_IntegerType_getBitWidth(inst) as usize
    }
}

pub fn llvm_IntegerType_getSignBit(inst: *const llvm_IntegerType) -> u64 {
    unsafe {
        raw::llvm_IntegerType_getSignBit(inst) as u64
    }
}

pub fn llvm_IntegerType_isPowerOf2ByteWidth(inst: *const llvm_IntegerType) -> bool {
    unsafe {
        raw::llvm_IntegerType_isPowerOf2ByteWidth(inst) != 0
    }
}

pub fn llvm_LLVMContext_delete() -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_LLVMContext_delete()
    }
}

pub fn llvm_LLVMContext_new() -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_LLVMContext_new()
    }
}

pub fn llvm_Module_appendModuleInlineAsm(inst: *mut llvm_Module, Asm: &str) {
    let Asm = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Asm.as_ptr()) },
        length: Asm.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_appendModuleInlineAsm(inst, Asm)
    }
}

pub fn llvm_Module_delete(inst: *mut llvm_Module) {
    unsafe {
        raw::llvm_Module_delete(inst)
    }
}

pub fn llvm_Module_dump(inst: *const llvm_Module) {
    unsafe {
        raw::llvm_Module_dump(inst)
    }
}

pub fn llvm_Module_getContext(inst: *const llvm_Module) -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_Module_getContext(inst)
    }
}

pub fn llvm_Module_getDataLayout(inst: *const llvm_Module) -> *const llvm_DataLayout {
    unsafe {
        raw::llvm_Module_getDataLayout(inst)
    }
}

pub fn llvm_Module_getDataLayoutStr(inst: *const llvm_Module) -> std_string_const {
    unsafe {
        raw::llvm_Module_getDataLayoutStr(inst)
    }
}

pub fn llvm_Module_getFunction(inst: *const llvm_Module, Name: &str) -> *mut llvm_Function {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_getFunction(inst, Name)
    }
}

pub fn llvm_Module_getMDKindID(inst: *const llvm_Module, Name: &str) -> usize {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_getMDKindID(inst, Name) as usize
    }
}

pub fn llvm_Module_getModuleIdentifier(inst: *const llvm_Module) -> std_string_const {
    unsafe {
        raw::llvm_Module_getModuleIdentifier(inst)
    }
}

pub fn llvm_Module_getModuleInlineAsm(inst: *const llvm_Module) -> std_string_const {
    unsafe {
        raw::llvm_Module_getModuleInlineAsm(inst)
    }
}

pub fn llvm_Module_getNamedValue(inst: *const llvm_Module, Name: &str) -> *mut llvm_GlobalValue {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_getNamedValue(inst, Name)
    }
}

pub fn llvm_Module_getOrInsertFunction(inst: *mut llvm_Module, Name: &str, ty: *mut llvm_FunctionType) -> *mut llvm_Constant {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_getOrInsertFunction(inst, Name, ty)
    }
}

pub fn llvm_Module_getTargetTriple(inst: *const llvm_Module) -> std_string_const {
    unsafe {
        raw::llvm_Module_getTargetTriple(inst)
    }
}

pub fn llvm_Module_getTypeByName(inst: *const llvm_Module, Name: &str) -> *mut llvm_StructType {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_getTypeByName(inst, Name)
    }
}

pub fn llvm_Module_new(ModuleID: &str, Context: *mut llvm_LLVMContext) -> *mut llvm_Module {
    let ModuleID = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(ModuleID.as_ptr()) },
        length: ModuleID.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_new(ModuleID, Context)
    }
}

pub fn llvm_Module_setDataLayout(inst: *mut llvm_Module, Other: *const llvm_DataLayout) {
    unsafe {
        raw::llvm_Module_setDataLayout(inst, Other)
    }
}

pub fn llvm_Module_setDataLayoutStr(inst: *mut llvm_Module, Desc: &str) {
    let Desc = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Desc.as_ptr()) },
        length: Desc.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_setDataLayoutStr(inst, Desc)
    }
}

pub fn llvm_Module_setModuleIdentifier(inst: *mut llvm_Module, ID: &str) {
    let ID = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(ID.as_ptr()) },
        length: ID.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_setModuleIdentifier(inst, ID)
    }
}

pub fn llvm_Module_setModuleInlineAsm(inst: *mut llvm_Module, Asm: &str) {
    let Asm = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Asm.as_ptr()) },
        length: Asm.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_setModuleInlineAsm(inst, Asm)
    }
}

pub fn llvm_Module_setTargetTriple(inst: *mut llvm_Module, Triple: &str) {
    let Triple = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Triple.as_ptr()) },
        length: Triple.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Module_setTargetTriple(inst, Triple)
    }
}

pub fn llvm_Operator_getOpcode(inst: *const llvm_Operator) -> usize {
    unsafe {
        raw::llvm_Operator_getOpcode(inst) as usize
    }
}

pub fn llvm_Pass_delete(inst: *mut llvm_Pass) {
    unsafe {
        raw::llvm_Pass_delete(inst)
    }
}

pub fn llvm_Pass_doFinalization(inst: *mut llvm_Pass, Module: *mut llvm_Module) -> bool {
    unsafe {
        raw::llvm_Pass_doFinalization(inst, Module) != 0
    }
}

pub fn llvm_Pass_doInitialization(inst: *mut llvm_Pass, Module: *mut llvm_Module) -> bool {
    unsafe {
        raw::llvm_Pass_doInitialization(inst, Module) != 0
    }
}

pub fn llvm_Pass_dump(inst: *const llvm_Pass) {
    unsafe {
        raw::llvm_Pass_dump(inst)
    }
}

pub fn llvm_Pass_getPassKind(inst: *const llvm_Pass) -> llvm_PassKind {
    unsafe {
        raw::llvm_Pass_getPassKind(inst)
    }
}

pub fn llvm_PassManager_add(inst: *mut llvm_PassManager, Pass: *mut llvm_Pass) {
    unsafe {
        raw::llvm_PassManager_add(inst, Pass)
    }
}

pub fn llvm_PassManager_new() -> *mut llvm_PassManager {
    unsafe {
        raw::llvm_PassManager_new()
    }
}

pub fn llvm_PassManager_run(inst: *mut llvm_PassManager, Module: *mut llvm_Module) {
    unsafe {
        raw::llvm_PassManager_run(inst, Module)
    }
}

pub fn llvm_PointerType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_PointerType_classof(ty) != 0
    }
}

pub fn llvm_PointerType_get(ElementType: *mut llvm_Type, AddressSpace: usize) -> *mut llvm_PointerType {
    let AddressSpace = AddressSpace as libc::c_uint;
    unsafe {
        raw::llvm_PointerType_get(ElementType, AddressSpace)
    }
}

pub fn llvm_PointerType_getAddressSpace(inst: *const llvm_PointerType) -> usize {
    unsafe {
        raw::llvm_PointerType_getAddressSpace(inst) as usize
    }
}

pub fn llvm_PointerType_getUnqual(ElementType: *mut llvm_Type) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_PointerType_getUnqual(ElementType)
    }
}

pub fn llvm_PointerType_isValidElementType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_PointerType_isValidElementType(ty) != 0
    }
}

pub fn llvm_SequentialType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_SequentialType_classof(ty) != 0
    }
}

pub fn llvm_SequentialType_getElementType(inst: *const llvm_SequentialType) -> *mut llvm_Type {
    unsafe {
        raw::llvm_SequentialType_getElementType(inst)
    }
}

pub fn llvm_StructType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_StructType_classof(ty) != 0
    }
}

pub fn llvm_StructType_create(ctx: *mut llvm_LLVMContext, Elements: &[*mut llvm_Type], Name: &str) -> *mut llvm_StructType {
    let Elements = llvm_ArrayRef_ptr_llvm_Type {
        data: Elements.as_ptr(),
        size: Elements.len() as libc::size_t,
    };
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_StructType_create(ctx, Elements, Name)
    }
}

pub fn llvm_StructType_createPacked(ctx: *mut llvm_LLVMContext, Elements: &[*mut llvm_Type], Name: &str, isPacked: bool) -> *mut llvm_StructType {
    let Elements = llvm_ArrayRef_ptr_llvm_Type {
        data: Elements.as_ptr(),
        size: Elements.len() as libc::size_t,
    };
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    let isPacked = if isPacked { 1 } else { 0 };
    unsafe {
        raw::llvm_StructType_createPacked(ctx, Elements, Name, isPacked)
    }
}

pub fn llvm_StructType_getElementType(inst: *const llvm_StructType, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_StructType_getElementType(inst, idx)
    }
}

pub fn llvm_StructType_getName(inst: *const llvm_StructType) -> llvm_StringRef {
    unsafe {
        raw::llvm_StructType_getName(inst)
    }
}

pub fn llvm_StructType_getNumElements(inst: *const llvm_StructType) -> usize {
    unsafe {
        raw::llvm_StructType_getNumElements(inst) as usize
    }
}

pub fn llvm_StructType_hasName(inst: *const llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_hasName(inst) != 0
    }
}

pub fn llvm_StructType_isLayoutIdentical(inst: *const llvm_StructType, Other: *mut llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_isLayoutIdentical(inst, Other) != 0
    }
}

pub fn llvm_StructType_isLiteral(inst: *const llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_isLiteral(inst) != 0
    }
}

pub fn llvm_StructType_isOpaque(inst: *const llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_isOpaque(inst) != 0
    }
}

pub fn llvm_StructType_isPacked(inst: *const llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_isPacked(inst) != 0
    }
}

pub fn llvm_StructType_isSized(inst: *const llvm_StructType) -> bool {
    unsafe {
        raw::llvm_StructType_isSized(inst) != 0
    }
}

pub fn llvm_StructType_isValidElementType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_StructType_isValidElementType(ty) != 0
    }
}

pub fn llvm_StructType_setBodyPacked(inst: *mut llvm_StructType, Elements: &[*mut llvm_Type], isPacked: bool) {
    let Elements = llvm_ArrayRef_ptr_llvm_Type {
        data: Elements.as_ptr(),
        size: Elements.len() as libc::size_t,
    };
    let isPacked = if isPacked { 1 } else { 0 };
    unsafe {
        raw::llvm_StructType_setBodyPacked(inst, Elements, isPacked)
    }
}

pub fn llvm_StructType_setBody(inst: *mut llvm_StructType, Elements: &[*mut llvm_Type]) {
    let Elements = llvm_ArrayRef_ptr_llvm_Type {
        data: Elements.as_ptr(),
        size: Elements.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_StructType_setBody(inst, Elements)
    }
}

pub fn llvm_StructType_setName(inst: *mut llvm_StructType, Name: &str) {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_StructType_setName(inst, Name)
    }
}

pub fn llvm_Type_dump(inst: *const llvm_Type) {
    unsafe {
        raw::llvm_Type_dump(inst)
    }
}

pub fn llvm_Type_getContainedType(inst: *const llvm_Type, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Type_getContainedType(inst, idx)
    }
}

pub fn llvm_Type_getContext(inst: *const llvm_Type) -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_Type_getContext(inst)
    }
}

pub fn llvm_Type_getDoublePtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getDoublePtrTy(ctx)
    }
}

pub fn llvm_Type_getDoubleTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getDoubleTy(ctx)
    }
}

pub fn llvm_Type_getFP128PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getFP128PtrTy(ctx)
    }
}

pub fn llvm_Type_getFP128Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getFP128Ty(ctx)
    }
}

pub fn llvm_Type_getFloatPtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getFloatPtrTy(ctx)
    }
}

pub fn llvm_Type_getFloatTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getFloatTy(ctx)
    }
}

pub fn llvm_Type_getFunctionNumParams(inst: *const llvm_Type) -> usize {
    unsafe {
        raw::llvm_Type_getFunctionNumParams(inst) as usize
    }
}

pub fn llvm_Type_getFunctionParamType(inst: *const llvm_Type, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Type_getFunctionParamType(inst, idx)
    }
}

pub fn llvm_Type_getHalfPtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getHalfPtrTy(ctx)
    }
}

pub fn llvm_Type_getHalfTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getHalfTy(ctx)
    }
}

pub fn llvm_Type_getInt16PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getInt16PtrTy(ctx)
    }
}

pub fn llvm_Type_getInt16Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_Type_getInt16Ty(ctx)
    }
}

pub fn llvm_Type_getInt1PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getInt1PtrTy(ctx)
    }
}

pub fn llvm_Type_getInt1Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_Type_getInt1Ty(ctx)
    }
}

pub fn llvm_Type_getInt32PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getInt32PtrTy(ctx)
    }
}

pub fn llvm_Type_getInt32Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_Type_getInt32Ty(ctx)
    }
}

pub fn llvm_Type_getInt64PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getInt64PtrTy(ctx)
    }
}

pub fn llvm_Type_getInt64Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_Type_getInt64Ty(ctx)
    }
}

pub fn llvm_Type_getInt8PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getInt8PtrTy(ctx)
    }
}

pub fn llvm_Type_getInt8Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_IntegerType {
    unsafe {
        raw::llvm_Type_getInt8Ty(ctx)
    }
}

pub fn llvm_Type_getIntNPtrTy(ctx: *mut llvm_LLVMContext, size: usize) -> *mut llvm_PointerType {
    let size = size as libc::c_uint;
    unsafe {
        raw::llvm_Type_getIntNPtrTy(ctx, size)
    }
}

pub fn llvm_Type_getIntNTy(ctx: *mut llvm_LLVMContext, size: usize) -> *mut llvm_IntegerType {
    let size = size as libc::c_uint;
    unsafe {
        raw::llvm_Type_getIntNTy(ctx, size)
    }
}

pub fn llvm_Type_getLabelTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getLabelTy(ctx)
    }
}

pub fn llvm_Type_getMetadataTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getMetadataTy(ctx)
    }
}

pub fn llvm_Type_getNumContainedTypes(inst: *const llvm_Type) -> usize {
    unsafe {
        raw::llvm_Type_getNumContainedTypes(inst) as usize
    }
}

pub fn llvm_Type_getPPC_FP128PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getPPC_FP128PtrTy(ctx)
    }
}

pub fn llvm_Type_getPPC_FP128Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getPPC_FP128Ty(ctx)
    }
}

pub fn llvm_Type_getPointerAddressSpace(inst: *const llvm_Type) -> usize {
    unsafe {
        raw::llvm_Type_getPointerAddressSpace(inst) as usize
    }
}

pub fn llvm_Type_getPointerElementType(inst: *const llvm_Type) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getPointerElementType(inst)
    }
}

pub fn llvm_Type_getPointerTo(inst: *mut llvm_Type, idx: usize) -> *mut llvm_PointerType {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Type_getPointerTo(inst, idx)
    }
}

pub fn llvm_Type_getSequentialElementType(inst: *const llvm_Type) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getSequentialElementType(inst)
    }
}

pub fn llvm_Type_getStructElementType(inst: *const llvm_Type, idx: usize) -> *mut llvm_Type {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_Type_getStructElementType(inst, idx)
    }
}

pub fn llvm_Type_getStructName(inst: *const llvm_Type) -> llvm_StringRef {
    unsafe {
        raw::llvm_Type_getStructName(inst)
    }
}

pub fn llvm_Type_getStructNumElements(inst: *const llvm_Type) -> usize {
    unsafe {
        raw::llvm_Type_getStructNumElements(inst) as usize
    }
}

pub fn llvm_Type_getTypeID(inst: *const llvm_Type) -> llvm_Type_TypeID {
    unsafe {
        raw::llvm_Type_getTypeID(inst)
    }
}

pub fn llvm_Type_getVoidTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getVoidTy(ctx)
    }
}

pub fn llvm_Type_getX86_FP80PtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getX86_FP80PtrTy(ctx)
    }
}

pub fn llvm_Type_getX86_FP80Ty(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getX86_FP80Ty(ctx)
    }
}

pub fn llvm_Type_getX86_MMXPtrTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_PointerType {
    unsafe {
        raw::llvm_Type_getX86_MMXPtrTy(ctx)
    }
}

pub fn llvm_Type_getX86_MMXTy(ctx: *mut llvm_LLVMContext) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Type_getX86_MMXTy(ctx)
    }
}

pub fn llvm_Type_isAggregateType(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isAggregateType(inst) != 0
    }
}

pub fn llvm_Type_isArrayTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isArrayTy(inst) != 0
    }
}

pub fn llvm_Type_isDoubleTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isDoubleTy(inst) != 0
    }
}

pub fn llvm_Type_isEmptyTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isEmptyTy(inst) != 0
    }
}

pub fn llvm_Type_isFP128Ty(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFP128Ty(inst) != 0
    }
}

pub fn llvm_Type_isFPOrFPVectorTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFPOrFPVectorTy(inst) != 0
    }
}

pub fn llvm_Type_isFirstClassType(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFirstClassType(inst) != 0
    }
}

pub fn llvm_Type_isFloatTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFloatTy(inst) != 0
    }
}

pub fn llvm_Type_isFloatingPointTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFloatingPointTy(inst) != 0
    }
}

pub fn llvm_Type_isFunctionTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFunctionTy(inst) != 0
    }
}

pub fn llvm_Type_isFunctionVarArg(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isFunctionVarArg(inst) != 0
    }
}

pub fn llvm_Type_isHalfTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isHalfTy(inst) != 0
    }
}

pub fn llvm_Type_isIntOrIntVectorTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isIntOrIntVectorTy(inst) != 0
    }
}

pub fn llvm_Type_isIntegerTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isIntegerTy(inst) != 0
    }
}

pub fn llvm_Type_isLabelTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isLabelTy(inst) != 0
    }
}

pub fn llvm_Type_isMetadataTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isMetadataTy(inst) != 0
    }
}

pub fn llvm_Type_isPPC_FP128Ty(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isPPC_FP128Ty(inst) != 0
    }
}

pub fn llvm_Type_isPointerTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isPointerTy(inst) != 0
    }
}

pub fn llvm_Type_isPtrOrPtrVectorTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isPtrOrPtrVectorTy(inst) != 0
    }
}

pub fn llvm_Type_isSingleValueType(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isSingleValueType(inst) != 0
    }
}

pub fn llvm_Type_isSized(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isSized(inst) != 0
    }
}

pub fn llvm_Type_isStructTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isStructTy(inst) != 0
    }
}

pub fn llvm_Type_isVectorTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isVectorTy(inst) != 0
    }
}

pub fn llvm_Type_isVoidTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isVoidTy(inst) != 0
    }
}

pub fn llvm_Type_isX86_FP80Ty(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isX86_FP80Ty(inst) != 0
    }
}

pub fn llvm_Type_isX86_MMXTy(inst: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_Type_isX86_MMXTy(inst) != 0
    }
}

pub fn llvm_Use_get(inst: *const llvm_Use) -> *mut llvm_Value {
    unsafe {
        raw::llvm_Use_get(inst)
    }
}

pub fn llvm_Use_getNext(inst: *const llvm_Use) -> *mut llvm_Use {
    unsafe {
        raw::llvm_Use_getNext(inst)
    }
}

pub fn llvm_Use_getOperandNo(inst: *const llvm_Use) -> usize {
    unsafe {
        raw::llvm_Use_getOperandNo(inst) as usize
    }
}

pub fn llvm_Use_getUser(inst: *const llvm_Use) -> *mut llvm_User {
    unsafe {
        raw::llvm_Use_getUser(inst)
    }
}

pub fn llvm_Use_initTags(Start: *mut llvm_Use, Stop: *mut llvm_Use) -> *mut llvm_Use {
    unsafe {
        raw::llvm_Use_initTags(Start, Stop)
    }
}

pub fn llvm_Use_set(inst: *mut llvm_Use, Val: *mut llvm_Value) {
    unsafe {
        raw::llvm_Use_set(inst, Val)
    }
}

pub fn llvm_Use_swap(inst: *mut llvm_Use, RHS: *mut llvm_Use) {
    unsafe {
        raw::llvm_Use_swap(inst, RHS)
    }
}

pub fn llvm_User_classof(V: *mut llvm_Value) -> bool {
    unsafe {
        raw::llvm_User_classof(V) != 0
    }
}

pub fn llvm_User_delete(inst: *mut llvm_User) {
    unsafe {
        raw::llvm_User_delete(inst)
    }
}

pub fn llvm_User_dropAllReferences(inst: *mut llvm_User) {
    unsafe {
        raw::llvm_User_dropAllReferences(inst)
    }
}

pub fn llvm_User_getNumOperands(inst: *const llvm_User) -> usize {
    unsafe {
        raw::llvm_User_getNumOperands(inst) as usize
    }
}

pub fn llvm_User_getOperand(inst: *const llvm_User, idx: usize) -> *mut llvm_Value {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_User_getOperand(inst, idx)
    }
}

pub fn llvm_User_replaceUsesOfWith(inst: *mut llvm_User, From: *mut llvm_Value, To: *mut llvm_Value) {
    unsafe {
        raw::llvm_User_replaceUsesOfWith(inst, From, To)
    }
}

pub fn llvm_User_setOperand(inst: *mut llvm_User, idx: usize, Val: *mut llvm_Value) {
    let idx = idx as libc::c_uint;
    unsafe {
        raw::llvm_User_setOperand(inst, idx, Val)
    }
}

pub fn llvm_Value_delete(inst: *mut llvm_Value) {
    unsafe {
        raw::llvm_Value_delete(inst)
    }
}

pub fn llvm_Value_dump(inst: *const llvm_Value) {
    unsafe {
        raw::llvm_Value_dump(inst)
    }
}

pub fn llvm_Value_getContext(inst: *const llvm_Value) -> *mut llvm_LLVMContext {
    unsafe {
        raw::llvm_Value_getContext(inst)
    }
}

pub fn llvm_Value_getName(inst: *const llvm_Value) -> llvm_StringRef {
    unsafe {
        raw::llvm_Value_getName(inst)
    }
}

pub fn llvm_Value_getNumUses(inst: *const llvm_Value) -> usize {
    unsafe {
        raw::llvm_Value_getNumUses(inst) as usize
    }
}

pub fn llvm_Value_getType(inst: *const llvm_Value) -> *mut llvm_Type {
    unsafe {
        raw::llvm_Value_getType(inst)
    }
}

pub fn llvm_Value_getValueID(inst: *const llvm_Value) -> usize {
    unsafe {
        raw::llvm_Value_getValueID(inst) as usize
    }
}

pub fn llvm_Value_hasNUses(inst: *const llvm_Value, N: usize) -> bool {
    let N = N as libc::c_uint;
    unsafe {
        raw::llvm_Value_hasNUses(inst, N) != 0
    }
}

pub fn llvm_Value_hasNUsesOrMore(inst: *const llvm_Value, N: usize) -> bool {
    let N = N as libc::c_uint;
    unsafe {
        raw::llvm_Value_hasNUsesOrMore(inst, N) != 0
    }
}

pub fn llvm_Value_hasName(inst: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_Value_hasName(inst) != 0
    }
}

pub fn llvm_Value_hasOneUse(inst: *const llvm_Value) -> bool {
    unsafe {
        raw::llvm_Value_hasOneUse(inst) != 0
    }
}

pub fn llvm_Value_isUsedInBasicBlock(inst: *const llvm_Value, BB: *const llvm_BasicBlock) -> bool {
    unsafe {
        raw::llvm_Value_isUsedInBasicBlock(inst, BB) != 0
    }
}

pub fn llvm_Value_mutateType(inst: *mut llvm_Value, ty: *mut llvm_Type) {
    unsafe {
        raw::llvm_Value_mutateType(inst, ty)
    }
}

pub fn llvm_Value_replaceAllUsesWith(inst: *mut llvm_Value, Value: *mut llvm_Value) {
    unsafe {
        raw::llvm_Value_replaceAllUsesWith(inst, Value)
    }
}

pub fn llvm_Value_setName(inst: *mut llvm_Value, Name: &str) {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_Value_setName(inst, Name)
    }
}

pub fn llvm_Value_takeName(inst: *mut llvm_Value, Value: *mut llvm_Value) {
    unsafe {
        raw::llvm_Value_takeName(inst, Value)
    }
}

pub fn llvm_ValueSymbolTable_delete() -> *mut llvm_ValueSymbolTable {
    unsafe {
        raw::llvm_ValueSymbolTable_delete()
    }
}

pub fn llvm_ValueSymbolTable_dump(inst: *const llvm_ValueSymbolTable) {
    unsafe {
        raw::llvm_ValueSymbolTable_dump(inst)
    }
}

pub fn llvm_ValueSymbolTable_empty(inst: *const llvm_ValueSymbolTable) -> bool {
    unsafe {
        raw::llvm_ValueSymbolTable_empty(inst) != 0
    }
}

pub fn llvm_ValueSymbolTable_lookup(inst: *const llvm_ValueSymbolTable, Name: &str) -> *mut llvm_Value {
    let Name = llvm_StringRef {
        data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
        length: Name.len() as libc::size_t,
    };
    unsafe {
        raw::llvm_ValueSymbolTable_lookup(inst, Name)
    }
}

pub fn llvm_ValueSymbolTable_new() -> *mut llvm_ValueSymbolTable {
    unsafe {
        raw::llvm_ValueSymbolTable_new()
    }
}

pub fn llvm_ValueSymbolTable_size(inst: *const llvm_ValueSymbolTable) -> usize {
    unsafe {
        raw::llvm_ValueSymbolTable_size(inst) as usize
    }
}

pub fn llvm_VectorType_classof(ty: *const llvm_Type) -> bool {
    unsafe {
        raw::llvm_VectorType_classof(ty) != 0
    }
}

pub fn llvm_VectorType_get(ty: *mut llvm_Type, NumElements: usize) -> *mut llvm_VectorType {
    let NumElements = NumElements as libc::c_uint;
    unsafe {
        raw::llvm_VectorType_get(ty, NumElements)
    }
}

pub fn llvm_VectorType_getBitWidth(inst: *const llvm_VectorType) -> usize {
    unsafe {
        raw::llvm_VectorType_getBitWidth(inst) as usize
    }
}

pub fn llvm_VectorType_getDoubleElementsVectorType(ty: *mut llvm_VectorType) -> *mut llvm_VectorType {
    unsafe {
        raw::llvm_VectorType_getDoubleElementsVectorType(ty)
    }
}

pub fn llvm_VectorType_getExtendedElementVectorType(ty: *mut llvm_VectorType) -> *mut llvm_VectorType {
    unsafe {
        raw::llvm_VectorType_getExtendedElementVectorType(ty)
    }
}

pub fn llvm_VectorType_getHalfElementsVectorType(ty: *mut llvm_VectorType) -> *mut llvm_VectorType {
    unsafe {
        raw::llvm_VectorType_getHalfElementsVectorType(ty)
    }
}

pub fn llvm_VectorType_getInteger(ty: *mut llvm_VectorType) -> *mut llvm_VectorType {
    unsafe {
        raw::llvm_VectorType_getInteger(ty)
    }
}

pub fn llvm_VectorType_getNumElements(inst: *const llvm_VectorType) -> usize {
    unsafe {
        raw::llvm_VectorType_getNumElements(inst) as usize
    }
}

pub fn llvm_VectorType_getTruncatedElementVectorType(ty: *mut llvm_VectorType) -> *mut llvm_VectorType {
    unsafe {
        raw::llvm_VectorType_getTruncatedElementVectorType(ty)
    }
}

pub fn llvm_VectorType_isValidElementType(ty: *mut llvm_Type) -> bool {
    unsafe {
        raw::llvm_VectorType_isValidElementType(ty) != 0
    }
}
