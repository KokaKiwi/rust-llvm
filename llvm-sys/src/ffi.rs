#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

#[repr(C)]
pub struct llvm_AddrSpaceCastInst;
#[repr(C)]
pub struct llvm_AllocaInst;
#[repr(C)]
pub struct llvm_Argument;
#[repr(C)]
pub struct llvm_ArrayType;
#[repr(C)]
pub struct llvm_AtomicCmpXchgInst;
#[repr(C)]
pub struct llvm_AtomicRMWInst;
#[repr(C)]
pub struct llvm_BasicBlock;
#[repr(C)]
pub struct llvm_BasicBlockPass;
#[repr(C)]
pub struct llvm_BinaryOperator;
#[repr(C)]
pub struct llvm_BitCastInst;
#[repr(C)]
pub struct llvm_BlockAddress;
#[repr(C)]
pub struct llvm_BranchInst;
#[repr(C)]
pub struct llvm_CallGraphSCCPass;
#[repr(C)]
pub struct llvm_CallInst;
#[repr(C)]
pub struct llvm_CastInst;
#[repr(C)]
pub struct llvm_CmpInst;
#[repr(C)]
pub struct llvm_CompositeType;
#[repr(C)]
pub struct llvm_Constant;
#[repr(C)]
pub struct llvm_ConstantAggregateZero;
#[repr(C)]
pub struct llvm_ConstantArray;
#[repr(C)]
pub struct llvm_ConstantDataArray;
#[repr(C)]
pub struct llvm_ConstantDataSequential;
#[repr(C)]
pub struct llvm_ConstantDataVector;
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
pub struct llvm_DataLayout;
#[repr(C)]
pub struct llvm_DebugLoc;
#[repr(C)]
pub struct llvm_ExtractElementInst;
#[repr(C)]
pub struct llvm_ExtractValueInst;
#[repr(C)]
pub struct llvm_FPExtInst;
#[repr(C)]
pub struct llvm_FPToSIInst;
#[repr(C)]
pub struct llvm_FenceInst;
#[repr(C)]
pub struct llvm_Function;
#[repr(C)]
pub struct llvm_FunctionPass;
#[repr(C)]
pub struct llvm_FunctionPassManager;
#[repr(C)]
pub struct llvm_FunctionType;
#[repr(C)]
pub struct llvm_GetElementPtrInst;
#[repr(C)]
pub struct llvm_GlobalAlias;
#[repr(C)]
pub struct llvm_GlobalObject;
#[repr(C)]
pub struct llvm_GlobalValue;
#[repr(C)]
pub struct llvm_GlobalVariable;
#[repr(C)]
pub struct llvm_IRBuilder;
#[repr(C)]
pub struct llvm_IRBuilderBase;
#[repr(C)]
pub struct llvm_IndirectBrInst;
#[repr(C)]
pub struct llvm_InlineAsm;
#[repr(C)]
pub struct llvm_InsertElementInst;
#[repr(C)]
pub struct llvm_InsertValueInst;
#[repr(C)]
pub struct llvm_Instruction;
#[repr(C)]
pub struct llvm_IntegerType;
#[repr(C)]
pub struct llvm_InvokeInst;
#[repr(C)]
pub struct llvm_LLVMContext;
#[repr(C)]
pub struct llvm_LandingPadInst;
#[repr(C)]
pub struct llvm_LoadInst;
#[repr(C)]
pub struct llvm_LoopPass;
#[repr(C)]
pub struct llvm_MDNode;
#[repr(C)]
pub struct llvm_MDString;
#[repr(C)]
pub struct llvm_Module;
#[repr(C)]
pub struct llvm_ModulePass;
#[repr(C)]
pub struct llvm_Operator;
#[repr(C)]
pub struct llvm_PHINode;
#[repr(C)]
pub struct llvm_Pass;
#[repr(C)]
pub struct llvm_PassManager;
#[repr(C)]
pub struct llvm_PointerType;
#[repr(C)]
pub struct llvm_RegionPass;
#[repr(C)]
pub struct llvm_ResumeInst;
#[repr(C)]
pub struct llvm_ReturnInst;
#[repr(C)]
pub struct llvm_SelectInst;
#[repr(C)]
pub struct llvm_SequentialType;
#[repr(C)]
pub struct llvm_ShuffleVectorInst;
#[repr(C)]
pub struct llvm_StoreInst;
#[repr(C)]
pub struct llvm_StructType;
#[repr(C)]
pub struct llvm_SwitchInst;
#[repr(C)]
pub struct llvm_TerminatorInst;
#[repr(C)]
pub struct llvm_Type;
#[repr(C)]
pub struct llvm_UnaryInstruction;
#[repr(C)]
pub struct llvm_UndefValue;
#[repr(C)]
pub struct llvm_UnreachableInst;
#[repr(C)]
pub struct llvm_Use;
#[repr(C)]
pub struct llvm_User;
#[repr(C)]
pub struct llvm_VAArgInst;
#[repr(C)]
pub struct llvm_Value;
#[repr(C)]
pub struct llvm_ValueSymbolTable;
#[repr(C)]
pub struct llvm_VectorType;
#[repr(C)]
pub struct llvm_iplist_llvm_Argument;
pub type llvm_AtomicOrdering = libc::c_int;
pub type llvm_Instruction_BinaryOps = libc::c_int;
pub type llvm_Instruction_CastOps = libc::c_int;
pub type llvm_CallingConv_ID = libc::c_int;
pub type llvm_GlobalValue_LinkageTypes = libc::c_int;
pub type llvm_Instruction_MemoryOps = libc::c_int;
pub type llvm_Instruction_OtherOps = libc::c_int;
pub type llvm_PassKind = libc::c_int;
pub type llvm_PassManagerType = libc::c_int;
pub type llvm_CmpInst_Predicate = libc::c_int;
pub type llvm_SynchronizationScope = libc::c_int;
pub type llvm_Instruction_TermOps = libc::c_int;
pub type llvm_Type_TypeID = libc::c_int;
pub type llvm_Value_ValueTy = libc::c_int;

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Constant {
    pub data: *const *mut llvm_Constant,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Type {
    pub data: *const *mut llvm_Type,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_ptr_llvm_Value {
    pub data: *const *mut llvm_Value,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_uint64 {
    pub data: *const libc::uint64_t,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_ArrayRef_uint {
    pub data: *const libc::c_uint,
    pub size: libc::size_t,
}

#[repr(C)]
pub struct llvm_APInt {
    pub numbits: libc::c_uint,
    pub data: llvm_ArrayRef_uint64,
}

#[repr(C)]
pub struct llvm_StringRef {
    pub data: *const libc::c_char,
    pub length: libc::size_t,
}

#[repr(C)]
pub struct std_string {
    pub data: *mut libc::c_char,
    pub length: libc::size_t,
}

#[repr(C)]
pub struct std_string_const {
    pub data: *const libc::c_char,
    pub length: libc::size_t,
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
        pub fn llvm_createFunctionInliningPass() -> *mut super::llvm_Pass;
        pub fn llvm_createFunctionInliningPassWithOptLevel(OptLevel: super::libc::c_uint, SizeOptLevel: super::libc::c_uint) -> *mut super::llvm_Pass;
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
        pub fn llvm_Argument_getArgNo(inst: *const super::llvm_Argument) -> super::libc::c_uint;
        pub fn llvm_Argument_getParent(inst: *const super::llvm_Argument) -> *const super::llvm_Function;
        pub fn llvm_Argument_getParentMut(inst: *mut super::llvm_Argument) -> *mut super::llvm_Function;
        pub fn llvm_Argument_new(Ty: *mut super::llvm_Type, Name: *const super::std_string_const, F: *const *mut super::llvm_Function) -> *mut super::llvm_Argument;
        pub fn llvm_Argument_next(inst: *const super::llvm_Argument) -> *const super::llvm_Argument;
        pub fn llvm_Argument_nextMut(inst: *mut super::llvm_Argument) -> *mut super::llvm_Argument;
        pub fn llvm_Argument_prev(inst: *const super::llvm_Argument) -> *const super::llvm_Argument;
        pub fn llvm_Argument_prevMut(inst: *mut super::llvm_Argument) -> *mut super::llvm_Argument;
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
        pub fn llvm_BasicBlock_getFirstNonPHI(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbg(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Instruction;
        pub fn llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Instruction;
        pub fn llvm_BasicBlock_getLandingPadInst(inst: *const super::llvm_BasicBlock) -> *const super::llvm_LandingPadInst;
        pub fn llvm_BasicBlock_getLandingPadInstMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_LandingPadInst;
        pub fn llvm_BasicBlock_getParent(inst: *const super::llvm_BasicBlock) -> *const super::llvm_Function;
        pub fn llvm_BasicBlock_getParentMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_Function;
        pub fn llvm_BasicBlock_getSinglePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getSinglePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getTerminator(inst: *const super::llvm_BasicBlock) -> *const super::llvm_TerminatorInst;
        pub fn llvm_BasicBlock_getTerminatorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_TerminatorInst;
        pub fn llvm_BasicBlock_getUniquePredecessor(inst: *const super::llvm_BasicBlock) -> *const super::llvm_BasicBlock;
        pub fn llvm_BasicBlock_getUniquePredecessorMut(inst: *mut super::llvm_BasicBlock) -> *mut super::llvm_BasicBlock;
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
        pub fn llvm_ConstantArray_classof(V: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantArray_get(Ty: *mut super::llvm_ArrayType, Values: super::llvm_ArrayRef_ptr_llvm_Constant) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantArray_getType(inst: *const super::llvm_ConstantArray) -> *mut super::llvm_Type;
        pub fn llvm_ConstantFP_classof(V: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantFP_fromStr(Ty: *mut super::llvm_Type, Val: super::llvm_StringRef) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantFP_get(Ty: *mut super::llvm_Type, Val: super::libc::c_double) -> *mut super::llvm_Constant;
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
        pub fn llvm_ConstantInt_fromStr(Ty: *mut super::llvm_IntegerType, Str: super::llvm_StringRef, radix: super::libc::uint8_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_get(Ty: *mut super::llvm_IntegerType, Value: super::libc::uint64_t) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getBitWidth(inst: *const super::llvm_ConstantInt) -> super::libc::c_uint;
        pub fn llvm_ConstantInt_getFalse(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_getFalseWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getSExtValue(inst: *const super::llvm_ConstantInt) -> super::libc::int64_t;
        pub fn llvm_ConstantInt_getSigned(Ty: *mut super::llvm_IntegerType, Value: super::libc::uint64_t, isSigned: super::libc::c_int) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getTrue(Ty: *mut super::llvm_Type) -> *mut super::llvm_Constant;
        pub fn llvm_ConstantInt_getTrueWithContext(Context: *mut super::llvm_LLVMContext) -> *mut super::llvm_ConstantInt;
        pub fn llvm_ConstantInt_getType(inst: *const super::llvm_ConstantInt) -> *mut super::llvm_IntegerType;
        pub fn llvm_ConstantInt_getZExtValue(inst: *const super::llvm_ConstantInt) -> super::libc::uint64_t;
        pub fn llvm_ConstantInt_isMaxValue(inst: *const super::llvm_ConstantInt, isSigned: super::libc::c_int) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isMinValue(inst: *const super::llvm_ConstantInt, isSigned: super::libc::c_int) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isMinusOne(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isNegative(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isOne(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isSignedValueValidForType(Ty: *mut super::llvm_Type, Val: super::libc::int64_t) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isValueValidForType(Ty: *mut super::llvm_Type, Val: super::libc::uint64_t) -> super::libc::c_int;
        pub fn llvm_ConstantInt_isZero(inst: *const super::llvm_ConstantInt) -> super::libc::c_int;
        pub fn llvm_ConstantInt_uge(inst: *const super::llvm_ConstantInt, Num: super::libc::uint64_t) -> super::libc::c_int;
        pub fn llvm_ConstantPointerNull_classof(Val: *const super::llvm_Value) -> super::libc::c_int;
        pub fn llvm_ConstantPointerNull_destroyConstant(inst: *mut super::llvm_ConstantPointerNull);
        pub fn llvm_ConstantPointerNull_get(Ty: *mut super::llvm_PointerType) -> *mut super::llvm_ConstantPointerNull;
        pub fn llvm_ConstantPointerNull_getType(inst: *const super::llvm_ConstantPointerNull) -> *mut super::llvm_PointerType;
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
        pub fn llvm_Function_getArgumentList(inst: *const super::llvm_Function) -> *const super::llvm_iplist_llvm_Argument;
        pub fn llvm_Function_getArgumentListMut(inst: *mut super::llvm_Function) -> *mut super::llvm_iplist_llvm_Argument;
        pub fn llvm_Function_getCallingConv(inst: *const super::llvm_Function) -> super::llvm_CallingConv_ID;
        pub fn llvm_Function_getContext(inst: *const super::llvm_Function) -> *mut super::llvm_LLVMContext;
        pub fn llvm_Function_getDereferenceableBytes(inst: *const super::llvm_Function, idx: super::libc::c_uint) -> super::libc::uint64_t;
        pub fn llvm_Function_getEntryBlock(inst: *const super::llvm_Function) -> *const super::llvm_BasicBlock;
        pub fn llvm_Function_getEntryBlockMut(inst: *mut super::llvm_Function) -> *mut super::llvm_BasicBlock;
        pub fn llvm_Function_getFirstArg(inst: *const super::llvm_Function) -> *const super::llvm_Argument;
        pub fn llvm_Function_getFirstArgMut(inst: *mut super::llvm_Function) -> *mut super::llvm_Argument;
        pub fn llvm_Function_getFunctionType(inst: *const super::llvm_Function) -> *mut super::llvm_FunctionType;
        pub fn llvm_Function_getIntrinsicID(inst: *const super::llvm_Function) -> super::libc::c_uint;
        pub fn llvm_Function_getParamAlignment(inst: *const super::llvm_Function, idx: super::libc::c_uint) -> super::libc::c_uint;
        pub fn llvm_Function_getReturnType(inst: *const super::llvm_Function) -> *mut super::llvm_Type;
        pub fn llvm_Function_getValueSymbolTable(inst: *const super::llvm_Function) -> *const super::llvm_ValueSymbolTable;
        pub fn llvm_Function_getValueSymbolTableMut(inst: *mut super::llvm_Function) -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_Function_hasFnAttr(inst: *const super::llvm_Function, Kind: super::llvm_StringRef) -> super::libc::c_int;
        pub fn llvm_Function_hasGC(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_hasStructRetAttr(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_hasUWTable(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_isIntrinsic(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_isVarArg(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_needsUnwindTableEntry(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_onlyReadsMemory(inst: *const super::llvm_Function) -> super::libc::c_int;
        pub fn llvm_Function_onlyReadsMemoryParam(inst: *const super::llvm_Function, n: super::libc::c_uint) -> super::libc::c_int;
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
        pub fn llvm_IRBuilderBase_SetInsertPoint(inst: *mut super::llvm_IRBuilderBase, BB: *mut super::llvm_BasicBlock);
        pub fn llvm_IRBuilderBase_SetInsertPointAtInst(inst: *mut super::llvm_IRBuilderBase, Inst: *mut super::llvm_Instruction);
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
        pub fn llvm_IRBuilder_CreateAShr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAShrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAdd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAddrSpaceCast(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, DestTy: *mut super::llvm_Type, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAlignedLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: super::libc::c_int, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateAlignedStore(inst: *mut super::llvm_IRBuilder, Value: *mut super::llvm_Value, Ptr: *mut super::llvm_Value, Align: super::libc::c_uint, isVolatile: *const super::libc::c_int) -> *mut super::llvm_StoreInst;
        pub fn llvm_IRBuilder_CreateAlloca(inst: *mut super::llvm_IRBuilder, Ty: *mut super::llvm_Type, ArraySize: *const *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_AllocaInst;
        pub fn llvm_IRBuilder_CreateAnd(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateAndByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
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
        pub fn llvm_IRBuilder_CreateLoad(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
        pub fn llvm_IRBuilder_CreateLoadVolatile(inst: *mut super::llvm_IRBuilder, Ptr: *mut super::llvm_Value, isVolatile: super::libc::c_int, Name: *const super::std_string) -> *mut super::llvm_LoadInst;
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
        pub fn llvm_IRBuilder_CreateOr(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateOrByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
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
        pub fn llvm_IRBuilder_CreateShl(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: *mut super::llvm_Value, Name: *const super::std_string) -> *mut super::llvm_Value;
        pub fn llvm_IRBuilder_CreateShlByValue(inst: *mut super::llvm_IRBuilder, LHS: *mut super::llvm_Value, RHS: super::libc::uint64_t, Name: *const super::std_string) -> *mut super::llvm_Value;
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
        pub fn llvm_Instruction_clone(inst: *const super::llvm_Instruction) -> *mut super::llvm_Instruction;
        pub fn llvm_Instruction_copyFastMathFlags(inst: *mut super::llvm_Instruction, Inst: *const super::llvm_Instruction);
        pub fn llvm_Instruction_delete(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_dropUnknownMetadata(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_dropUnknownMetadataFromIDS(inst: *mut super::llvm_Instruction, KnownIDs: super::llvm_ArrayRef_uint);
        pub fn llvm_Instruction_eraseFromParent(inst: *mut super::llvm_Instruction);
        pub fn llvm_Instruction_getDataLayout(inst: *const super::llvm_Instruction) -> *const super::llvm_DataLayout;
        pub fn llvm_Instruction_getDebugLoc(inst: *const super::llvm_Instruction) -> *const super::llvm_DebugLoc;
        pub fn llvm_Instruction_getMetadata(inst: *const super::llvm_Instruction, KindID: super::libc::c_uint) -> *mut super::llvm_MDNode;
        pub fn llvm_Instruction_getMetadataStr(inst: *const super::llvm_Instruction, Kind: super::llvm_StringRef) -> *mut super::llvm_MDNode;
        pub fn llvm_Instruction_getOpcode(inst: *const super::llvm_Instruction) -> super::libc::c_uint;
        pub fn llvm_Instruction_getParent(inst: *const super::llvm_Instruction) -> *const super::llvm_BasicBlock;
        pub fn llvm_Instruction_getParentMut(inst: *mut super::llvm_Instruction) -> *mut super::llvm_BasicBlock;
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
        pub fn llvm_PHINode_addIncoming(inst: *mut super::llvm_PHINode, V: *mut super::llvm_Value, BB: *mut super::llvm_BasicBlock);
        pub fn llvm_PHINode_delete(inst: *mut super::llvm_PHINode);
        pub fn llvm_PHINode_getIncomingBlock(inst: *const super::llvm_PHINode, i: super::libc::c_uint) -> *mut super::llvm_BasicBlock;
        pub fn llvm_PHINode_getIncomingValue(inst: *const super::llvm_PHINode, i: super::libc::c_uint) -> *mut super::llvm_Value;
        pub fn llvm_PHINode_getNumIncomingValues(inst: *const super::llvm_PHINode) -> super::libc::c_uint;
        pub fn llvm_PHINode_setIncomingBlock(inst: *mut super::llvm_PHINode, i: super::libc::c_uint, BB: *mut super::llvm_BasicBlock);
        pub fn llvm_PHINode_setIncomingValue(inst: *mut super::llvm_PHINode, i: super::libc::c_uint, V: *mut super::llvm_Value);
        pub fn llvm_PassManager_add(inst: *mut super::llvm_PassManager, Pass: *mut super::llvm_Pass);
        pub fn llvm_PassManager_new() -> *mut super::llvm_PassManager;
        pub fn llvm_PassManager_run(inst: *mut super::llvm_PassManager, Module: *mut super::llvm_Module);
        pub fn llvm_Pass_delete(inst: *mut super::llvm_Pass);
        pub fn llvm_Pass_doFinalization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> super::libc::c_int;
        pub fn llvm_Pass_doInitialization(inst: *mut super::llvm_Pass, Module: *mut super::llvm_Module) -> super::libc::c_int;
        pub fn llvm_Pass_dump(inst: *const super::llvm_Pass);
        pub fn llvm_Pass_getPassKind(inst: *const super::llvm_Pass) -> super::llvm_PassKind;
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
        pub fn llvm_StructType_setBody(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_ptr_llvm_Type);
        pub fn llvm_StructType_setBodyPacked(inst: *mut super::llvm_StructType, Elements: super::llvm_ArrayRef_ptr_llvm_Type, isPacked: super::libc::c_int);
        pub fn llvm_StructType_setName(inst: *mut super::llvm_StructType, Name: super::llvm_StringRef);
        pub fn llvm_SwitchInst_addCase(inst: *mut super::llvm_SwitchInst, OnVal: *mut super::llvm_ConstantInt, Dest: *mut super::llvm_BasicBlock);
        pub fn llvm_SwitchInst_delete(inst: *mut super::llvm_SwitchInst);
        pub fn llvm_SwitchInst_getCondition(inst: *const super::llvm_SwitchInst) -> *mut super::llvm_Value;
        pub fn llvm_SwitchInst_getDefaultDest(inst: *const super::llvm_SwitchInst) -> *mut super::llvm_BasicBlock;
        pub fn llvm_SwitchInst_getNumCases(inst: *const super::llvm_SwitchInst) -> super::libc::c_uint;
        pub fn llvm_SwitchInst_setCondition(inst: *mut super::llvm_SwitchInst, V: *mut super::llvm_Value);
        pub fn llvm_SwitchInst_setDefaultDest(inst: *mut super::llvm_SwitchInst, DefaultCase: *mut super::llvm_BasicBlock);
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
        pub fn llvm_ValueSymbolTable_delete(inst: *mut super::llvm_ValueSymbolTable);
        pub fn llvm_ValueSymbolTable_dump(inst: *const super::llvm_ValueSymbolTable);
        pub fn llvm_ValueSymbolTable_empty(inst: *const super::llvm_ValueSymbolTable) -> super::libc::c_int;
        pub fn llvm_ValueSymbolTable_lookup(inst: *const super::llvm_ValueSymbolTable, Name: super::llvm_StringRef) -> *mut super::llvm_Value;
        pub fn llvm_ValueSymbolTable_new() -> *mut super::llvm_ValueSymbolTable;
        pub fn llvm_ValueSymbolTable_size(inst: *const super::llvm_ValueSymbolTable) -> super::libc::c_uint;
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
        pub fn llvm_iplist_llvm_Argument_clear(inst: *mut super::llvm_iplist_llvm_Argument);
        pub fn llvm_iplist_llvm_Argument_delete(inst: *mut super::llvm_iplist_llvm_Argument);
        pub fn llvm_iplist_llvm_Argument_first(inst: *const super::llvm_iplist_llvm_Argument) -> *const super::llvm_Argument;
        pub fn llvm_iplist_llvm_Argument_firstMut(inst: *mut super::llvm_iplist_llvm_Argument) -> *mut super::llvm_Argument;
        pub fn llvm_iplist_llvm_Argument_last(inst: *const super::llvm_iplist_llvm_Argument) -> *const super::llvm_Argument;
        pub fn llvm_iplist_llvm_Argument_lastMut(inst: *mut super::llvm_iplist_llvm_Argument) -> *mut super::llvm_Argument;
        pub fn llvm_iplist_llvm_Argument_max_size(inst: *const super::llvm_iplist_llvm_Argument) -> super::libc::size_t;
        pub fn llvm_iplist_llvm_Argument_new() -> *mut super::llvm_iplist_llvm_Argument;
        pub fn llvm_iplist_llvm_Argument_size(inst: *const super::llvm_iplist_llvm_Argument) -> super::libc::size_t;
    }
}

pub mod llvm {
    pub mod AddrSpaceCastInst {
    }

    pub mod AllocaInst {
    }

    pub mod Argument {
        pub fn getArgNo(inst: *const super::super::llvm_Argument) -> usize {
            unsafe {
                super::super::raw::llvm_Argument_getArgNo(inst) as usize
            }
        }

        pub fn getParent(inst: *const super::super::llvm_Argument) -> *const super::super::llvm_Function {
            unsafe {
                super::super::raw::llvm_Argument_getParent(inst)
            }
        }

        pub fn getParentMut(inst: *mut super::super::llvm_Argument) -> *mut super::super::llvm_Function {
            unsafe {
                super::super::raw::llvm_Argument_getParentMut(inst)
            }
        }

        pub fn new(Ty: *mut super::super::llvm_Type, Name: Option<&str>, F: Option<*mut super::super::llvm_Function>) -> *mut super::super::llvm_Argument {
            let opt_hack_467406 = Name.map(|value| super::super::std_string_const {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let F = F.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_Argument_new(Ty, Name, F)
            }
        }

        pub fn next(inst: *const super::super::llvm_Argument) -> *const super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Argument_next(inst)
            }
        }

        pub fn nextMut(inst: *mut super::super::llvm_Argument) -> *mut super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Argument_nextMut(inst)
            }
        }

        pub fn prev(inst: *const super::super::llvm_Argument) -> *const super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Argument_prev(inst)
            }
        }

        pub fn prevMut(inst: *mut super::super::llvm_Argument) -> *mut super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Argument_prevMut(inst)
            }
        }
    }

    pub mod ArrayType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_ArrayType_classof(ty) != 0
            }
        }

        pub fn get(ElementType: *mut super::super::llvm_Type, NumElements: u64) -> *mut super::super::llvm_ArrayType {
            let NumElements = NumElements as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_ArrayType_get(ElementType, NumElements)
            }
        }

        pub fn getNumElements(inst: *const super::super::llvm_ArrayType) -> u64 {
            unsafe {
                super::super::raw::llvm_ArrayType_getNumElements(inst) as u64
            }
        }

        pub fn isValidElementType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_ArrayType_isValidElementType(ty) != 0
            }
        }
    }

    pub mod AtomicCmpXchgInst {
    }

    pub mod AtomicRMWInst {
    }

    pub mod BasicBlock {
        pub fn Create(Context: *mut super::super::llvm_LLVMContext, Name: Option<&str>, Parent: Option<*mut super::super::llvm_Function>, InsertBefore: Option<*mut super::super::llvm_BasicBlock>) -> *mut super::super::llvm_BasicBlock {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let Parent = Parent.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let InsertBefore = InsertBefore.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_BasicBlock_Create(Context, Name, Parent, InsertBefore)
            }
        }

        pub fn classof(Val: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_BasicBlock_classof(Val) != 0
            }
        }

        pub fn delete(inst: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_delete(inst)
            }
        }

        pub fn dropAllReferences(inst: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_dropAllReferences(inst)
            }
        }

        pub fn eraseFromParent(inst: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_eraseFromParent(inst)
            }
        }

        pub fn getDataLayout(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_DataLayout {
            unsafe {
                super::super::raw::llvm_BasicBlock_getDataLayout(inst)
            }
        }

        pub fn getFirstNonPHI(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHI(inst)
            }
        }

        pub fn getFirstNonPHIMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHIMut(inst)
            }
        }

        pub fn getFirstNonPHIOrDbg(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHIOrDbg(inst)
            }
        }

        pub fn getFirstNonPHIOrDbgMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHIOrDbgMut(inst)
            }
        }

        pub fn getFirstNonPHIOrDbgOrLifetime(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(inst)
            }
        }

        pub fn getFirstNonPHIOrDbgOrLifetimeMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(inst)
            }
        }

        pub fn getLandingPadInst(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_LandingPadInst {
            unsafe {
                super::super::raw::llvm_BasicBlock_getLandingPadInst(inst)
            }
        }

        pub fn getLandingPadInstMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_LandingPadInst {
            unsafe {
                super::super::raw::llvm_BasicBlock_getLandingPadInstMut(inst)
            }
        }

        pub fn getParent(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_Function {
            unsafe {
                super::super::raw::llvm_BasicBlock_getParent(inst)
            }
        }

        pub fn getParentMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_Function {
            unsafe {
                super::super::raw::llvm_BasicBlock_getParentMut(inst)
            }
        }

        pub fn getSinglePredecessor(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_BasicBlock_getSinglePredecessor(inst)
            }
        }

        pub fn getSinglePredecessorMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_BasicBlock_getSinglePredecessorMut(inst)
            }
        }

        pub fn getTerminator(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_TerminatorInst {
            unsafe {
                super::super::raw::llvm_BasicBlock_getTerminator(inst)
            }
        }

        pub fn getTerminatorMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_TerminatorInst {
            unsafe {
                super::super::raw::llvm_BasicBlock_getTerminatorMut(inst)
            }
        }

        pub fn getUniquePredecessor(inst: *const super::super::llvm_BasicBlock) -> *const super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_BasicBlock_getUniquePredecessor(inst)
            }
        }

        pub fn getUniquePredecessorMut(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_BasicBlock_getUniquePredecessorMut(inst)
            }
        }

        pub fn getValueSymbolTable(inst: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_ValueSymbolTable {
            unsafe {
                super::super::raw::llvm_BasicBlock_getValueSymbolTable(inst)
            }
        }

        pub fn hasAddressTaken(inst: *const super::super::llvm_BasicBlock) -> bool {
            unsafe {
                super::super::raw::llvm_BasicBlock_hasAddressTaken(inst) != 0
            }
        }

        pub fn isLandingPad(inst: *const super::super::llvm_BasicBlock) -> bool {
            unsafe {
                super::super::raw::llvm_BasicBlock_isLandingPad(inst) != 0
            }
        }

        pub fn moveAfter(inst: *mut super::super::llvm_BasicBlock, MovePos: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_moveAfter(inst, MovePos)
            }
        }

        pub fn moveBefore(inst: *mut super::super::llvm_BasicBlock, MovePos: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_moveBefore(inst, MovePos)
            }
        }

        pub fn removeFromParent(inst: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_removeFromParent(inst)
            }
        }

        pub fn removePredecessor(inst: *mut super::super::llvm_BasicBlock, Pred: *mut super::super::llvm_BasicBlock, DontDeleteUselessPHIs: Option<bool>) {
            let opt_hack_541398 = DontDeleteUselessPHIs.map(|value| if value { 1 } else { 0 });
            let DontDeleteUselessPHIs = opt_hack_541398.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_BasicBlock_removePredecessor(inst, Pred, DontDeleteUselessPHIs)
            }
        }

        pub fn replaceSuccessorsPhiUsesWith(inst: *mut super::super::llvm_BasicBlock, New: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_BasicBlock_replaceSuccessorsPhiUsesWith(inst, New)
            }
        }
    }

    pub mod BasicBlockPass {
    }

    pub mod BinaryOperator {
    }

    pub mod BitCastInst {
    }

    pub mod BlockAddress {
        pub fn destroyConstant(inst: *mut super::super::llvm_BlockAddress) {
            unsafe {
                super::super::raw::llvm_BlockAddress_destroyConstant(inst)
            }
        }

        pub fn getBasicBlock(inst: *const super::super::llvm_BlockAddress) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_BlockAddress_getBasicBlock(inst)
            }
        }

        pub fn getFunction(inst: *const super::super::llvm_BlockAddress) -> *mut super::super::llvm_Function {
            unsafe {
                super::super::raw::llvm_BlockAddress_getFunction(inst)
            }
        }
    }

    pub mod BranchInst {
    }

    pub mod CallGraphSCCPass {
    }

    pub mod CallInst {
    }

    pub mod CallingConv {
    }

    pub mod CastInst {
    }

    pub mod CmpInst {
    }

    pub mod CompositeType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_CompositeType_classof(ty) != 0
            }
        }

        pub fn getTypeAtIndex(inst: *mut super::super::llvm_CompositeType, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_CompositeType_getTypeAtIndex(inst, idx)
            }
        }

        pub fn indexValid(inst: *const super::super::llvm_CompositeType, idx: usize) -> bool {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_CompositeType_indexValid(inst, idx) != 0
            }
        }
    }

    pub mod Constant {
        pub fn canTrap(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_canTrap(inst) != 0
            }
        }

        pub fn classof(V: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_classof(V) != 0
            }
        }

        pub fn destroyConstant(inst: *mut super::super::llvm_Constant) {
            unsafe {
                super::super::raw::llvm_Constant_destroyConstant(inst)
            }
        }

        pub fn getAggregateElement(inst: *const super::super::llvm_Constant, Elt: usize) -> *mut super::super::llvm_Constant {
            let Elt = Elt as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Constant_getAggregateElement(inst, Elt)
            }
        }

        pub fn getAggregateElementConstant(inst: *const super::super::llvm_Constant, Elt: *mut super::super::llvm_Constant) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_getAggregateElementConstant(inst, Elt)
            }
        }

        pub fn getAllOnesValue(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_getAllOnesValue(Ty)
            }
        }

        pub fn getIntegerValue(Ty: *mut super::super::llvm_Type, Value: (&[u64], usize)) -> *mut super::super::llvm_Constant {
            let Value = super::super::llvm_APInt {
                data: super::super::llvm_ArrayRef_uint64 {
                    data: Value.0.as_ptr(),
                    size: Value.0.len() as super::super::libc::size_t,
                },
                numbits: Value.1 as super::super::libc::c_uint,
            };
            unsafe {
                super::super::raw::llvm_Constant_getIntegerValue(Ty, Value)
            }
        }

        pub fn getNullValue(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_getNullValue(Ty)
            }
        }

        pub fn getSplatValue(inst: *const super::super::llvm_Constant) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_getSplatValue(inst)
            }
        }

        pub fn isAllOnesValue(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isAllOnesValue(inst) != 0
            }
        }

        pub fn isConstantUsed(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isConstantUsed(inst) != 0
            }
        }

        pub fn isDLLImportDependent(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isDLLImportDependent(inst) != 0
            }
        }

        pub fn isMinSignedValue(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isMinSignedValue(inst) != 0
            }
        }

        pub fn isNegativeZeroValue(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isNegativeZeroValue(inst) != 0
            }
        }

        pub fn isNullValue(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isNullValue(inst) != 0
            }
        }

        pub fn isThreadDependent(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isThreadDependent(inst) != 0
            }
        }

        pub fn isZeroValue(inst: *const super::super::llvm_Constant) -> bool {
            unsafe {
                super::super::raw::llvm_Constant_isZeroValue(inst) != 0
            }
        }

        pub fn removeDeadConstantUsers(inst: *const super::super::llvm_Constant) {
            unsafe {
                super::super::raw::llvm_Constant_removeDeadConstantUsers(inst)
            }
        }

        pub fn replaceUsesOfWithOnConstant(inst: *mut super::super::llvm_Constant, arg_1: *mut super::super::llvm_Value, arg_2: *mut super::super::llvm_Value, arg_3: *mut super::super::llvm_Use) {
            unsafe {
                super::super::raw::llvm_Constant_replaceUsesOfWithOnConstant(inst, arg_1, arg_2, arg_3)
            }
        }

        pub fn stripPointerCasts(inst: *const super::super::llvm_Constant) -> *const super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_stripPointerCasts(inst)
            }
        }

        pub fn stripPointerCastsMut(inst: *mut super::super::llvm_Constant) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_Constant_stripPointerCastsMut(inst)
            }
        }
    }

    pub mod ConstantAggregateZero {
    }

    pub mod ConstantArray {
        pub fn classof(V: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantArray_classof(V) != 0
            }
        }

        pub fn get(Ty: *mut super::super::llvm_ArrayType, Values: &[*mut super::super::llvm_Constant]) -> *mut super::super::llvm_Constant {
            let Values = super::super::llvm_ArrayRef_ptr_llvm_Constant {
                data: Values.as_ptr(),
                size: Values.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_ConstantArray_get(Ty, Values)
            }
        }

        pub fn getType(inst: *const super::super::llvm_ConstantArray) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_ConstantArray_getType(inst)
            }
        }
    }

    pub mod ConstantDataArray {
    }

    pub mod ConstantDataSequential {
    }

    pub mod ConstantDataVector {
    }

    pub mod ConstantExpr {
    }

    pub mod ConstantFP {
        pub fn classof(V: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantFP_classof(V) != 0
            }
        }

        pub fn fromStr(Ty: *mut super::super::llvm_Type, Val: &str) -> *mut super::super::llvm_Constant {
            let Val = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Val.as_ptr()) },
                length: Val.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_ConstantFP_fromStr(Ty, Val)
            }
        }

        pub fn get(Ty: *mut super::super::llvm_Type, Val: f64) -> *mut super::super::llvm_Constant {
            let Val = Val as super::super::libc::c_double;
            unsafe {
                super::super::raw::llvm_ConstantFP_get(Ty, Val)
            }
        }

        pub fn getInfinity(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_ConstantFP_getInfinity(Ty)
            }
        }

        pub fn getNegativeZero(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_ConstantFP_getNegativeZero(Ty)
            }
        }

        pub fn getZeroValueForNegation(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_ConstantFP_getZeroValueForNegation(Ty)
            }
        }

        pub fn isExactlyValueFloat(inst: *const super::super::llvm_ConstantFP, Val: f64) -> bool {
            let Val = Val as super::super::libc::c_double;
            unsafe {
                super::super::raw::llvm_ConstantFP_isExactlyValueFloat(inst, Val) != 0
            }
        }

        pub fn isNaN(inst: *const super::super::llvm_ConstantFP) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantFP_isNaN(inst) != 0
            }
        }

        pub fn isNegative(inst: *const super::super::llvm_ConstantFP) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantFP_isNegative(inst) != 0
            }
        }

        pub fn isZero(inst: *const super::super::llvm_ConstantFP) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantFP_isZero(inst) != 0
            }
        }
    }

    pub mod ConstantInt {
        pub fn classof(Val: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantInt_classof(Val) != 0
            }
        }

        pub fn equalsInt(inst: *const super::super::llvm_ConstantInt, Val: u64) -> bool {
            let Val = Val as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_equalsInt(inst, Val) != 0
            }
        }

        pub fn fromAPInt(Context: *mut super::super::llvm_LLVMContext, Val: (&[u64], usize)) -> *mut super::super::llvm_ConstantInt {
            let Val = super::super::llvm_APInt {
                data: super::super::llvm_ArrayRef_uint64 {
                    data: Val.0.as_ptr(),
                    size: Val.0.len() as super::super::libc::size_t,
                },
                numbits: Val.1 as super::super::libc::c_uint,
            };
            unsafe {
                super::super::raw::llvm_ConstantInt_fromAPInt(Context, Val)
            }
        }

        pub fn fromStr(Ty: *mut super::super::llvm_IntegerType, Str: &str, radix: u8) -> *mut super::super::llvm_ConstantInt {
            let Str = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
                length: Str.len() as super::super::libc::size_t,
            };
            let radix = radix as super::super::libc::uint8_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_fromStr(Ty, Str, radix)
            }
        }

        pub fn get(Ty: *mut super::super::llvm_IntegerType, Value: u64) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_get(Ty, Value)
            }
        }

        pub fn getBitWidth(inst: *const super::super::llvm_ConstantInt) -> usize {
            unsafe {
                super::super::raw::llvm_ConstantInt_getBitWidth(inst) as usize
            }
        }

        pub fn getFalse(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_ConstantInt_getFalse(Ty)
            }
        }

        pub fn getFalseWithContext(Context: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_ConstantInt {
            unsafe {
                super::super::raw::llvm_ConstantInt_getFalseWithContext(Context)
            }
        }

        pub fn getSExtValue(inst: *const super::super::llvm_ConstantInt) -> i64 {
            unsafe {
                super::super::raw::llvm_ConstantInt_getSExtValue(inst) as i64
            }
        }

        pub fn getSigned(Ty: *mut super::super::llvm_IntegerType, Value: u64, isSigned: bool) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint64_t;
            let isSigned = if isSigned { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_ConstantInt_getSigned(Ty, Value, isSigned)
            }
        }

        pub fn getTrue(Ty: *mut super::super::llvm_Type) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_ConstantInt_getTrue(Ty)
            }
        }

        pub fn getTrueWithContext(Context: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_ConstantInt {
            unsafe {
                super::super::raw::llvm_ConstantInt_getTrueWithContext(Context)
            }
        }

        pub fn getType(inst: *const super::super::llvm_ConstantInt) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_ConstantInt_getType(inst)
            }
        }

        pub fn getZExtValue(inst: *const super::super::llvm_ConstantInt) -> u64 {
            unsafe {
                super::super::raw::llvm_ConstantInt_getZExtValue(inst) as u64
            }
        }

        pub fn isMaxValue(inst: *const super::super::llvm_ConstantInt, isSigned: bool) -> bool {
            let isSigned = if isSigned { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_ConstantInt_isMaxValue(inst, isSigned) != 0
            }
        }

        pub fn isMinValue(inst: *const super::super::llvm_ConstantInt, isSigned: bool) -> bool {
            let isSigned = if isSigned { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_ConstantInt_isMinValue(inst, isSigned) != 0
            }
        }

        pub fn isMinusOne(inst: *const super::super::llvm_ConstantInt) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantInt_isMinusOne(inst) != 0
            }
        }

        pub fn isNegative(inst: *const super::super::llvm_ConstantInt) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantInt_isNegative(inst) != 0
            }
        }

        pub fn isOne(inst: *const super::super::llvm_ConstantInt) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantInt_isOne(inst) != 0
            }
        }

        pub fn isSignedValueValidForType(Ty: *mut super::super::llvm_Type, Val: i64) -> bool {
            let Val = Val as super::super::libc::int64_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_isSignedValueValidForType(Ty, Val) != 0
            }
        }

        pub fn isValueValidForType(Ty: *mut super::super::llvm_Type, Val: u64) -> bool {
            let Val = Val as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_isValueValidForType(Ty, Val) != 0
            }
        }

        pub fn isZero(inst: *const super::super::llvm_ConstantInt) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantInt_isZero(inst) != 0
            }
        }

        pub fn uge(inst: *const super::super::llvm_ConstantInt, Num: u64) -> bool {
            let Num = Num as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_ConstantInt_uge(inst, Num) != 0
            }
        }
    }

    pub mod ConstantPointerNull {
        pub fn classof(Val: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_ConstantPointerNull_classof(Val) != 0
            }
        }

        pub fn destroyConstant(inst: *mut super::super::llvm_ConstantPointerNull) {
            unsafe {
                super::super::raw::llvm_ConstantPointerNull_destroyConstant(inst)
            }
        }

        pub fn get(Ty: *mut super::super::llvm_PointerType) -> *mut super::super::llvm_ConstantPointerNull {
            unsafe {
                super::super::raw::llvm_ConstantPointerNull_get(Ty)
            }
        }

        pub fn getType(inst: *const super::super::llvm_ConstantPointerNull) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_ConstantPointerNull_getType(inst)
            }
        }
    }

    pub mod ConstantStruct {
    }

    pub mod ConstantVector {
    }

    pub mod DataLayout {
    }

    pub mod DebugLoc {
    }

    pub mod ExtractElementInst {
    }

    pub mod ExtractValueInst {
    }

    pub mod FPExtInst {
    }

    pub mod FPToSIInst {
    }

    pub mod FenceInst {
    }

    pub mod Function {
        pub fn Create(Ty: *mut super::super::llvm_FunctionType, Linkage: super::super::llvm_GlobalValue_LinkageTypes, Name: Option<&str>, Module: Option<*mut super::super::llvm_Module>) -> *mut super::super::llvm_Function {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let Module = Module.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_Function_Create(Ty, Linkage, Name, Module)
            }
        }

        pub fn addFnAttr(inst: *mut super::super::llvm_Function, Kind: &str, Val: Option<&str>) {
            let Kind = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
                length: Kind.len() as super::super::libc::size_t,
            };
            let opt_hack_763514 = Val.map(|value| super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Val = opt_hack_763514.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_Function_addFnAttr(inst, Kind, Val)
            }
        }

        pub fn cannotDuplicate(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_cannotDuplicate(inst) != 0
            }
        }

        pub fn classof(Val: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_Function_classof(Val) != 0
            }
        }

        pub fn clearGC(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_clearGC(inst)
            }
        }

        pub fn copyAttributesFrom(inst: *mut super::super::llvm_Function, Src: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_Function_copyAttributesFrom(inst, Src)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_delete(inst)
            }
        }

        pub fn deleteBody(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_deleteBody(inst)
            }
        }

        pub fn doesNotAccessMemory(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_doesNotAccessMemory(inst) != 0
            }
        }

        pub fn doesNotAccessMemoryParam(inst: *const super::super::llvm_Function, n: usize) -> bool {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_doesNotAccessMemoryParam(inst, n) != 0
            }
        }

        pub fn doesNotAlias(inst: *const super::super::llvm_Function, n: usize) -> bool {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_doesNotAlias(inst, n) != 0
            }
        }

        pub fn doesNotCapture(inst: *const super::super::llvm_Function, n: usize) -> bool {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_doesNotCapture(inst, n) != 0
            }
        }

        pub fn doesNotReturn(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_doesNotReturn(inst) != 0
            }
        }

        pub fn doesNotThrow(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_doesNotThrow(inst) != 0
            }
        }

        pub fn eraseFromParent(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_eraseFromParent(inst)
            }
        }

        pub fn getArgumentList(inst: *const super::super::llvm_Function) -> *const super::super::llvm_iplist_llvm_Argument {
            unsafe {
                super::super::raw::llvm_Function_getArgumentList(inst)
            }
        }

        pub fn getArgumentListMut(inst: *mut super::super::llvm_Function) -> *mut super::super::llvm_iplist_llvm_Argument {
            unsafe {
                super::super::raw::llvm_Function_getArgumentListMut(inst)
            }
        }

        pub fn getCallingConv(inst: *const super::super::llvm_Function) -> super::super::llvm_CallingConv_ID {
            unsafe {
                super::super::raw::llvm_Function_getCallingConv(inst)
            }
        }

        pub fn getContext(inst: *const super::super::llvm_Function) -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_Function_getContext(inst)
            }
        }

        pub fn getDereferenceableBytes(inst: *const super::super::llvm_Function, idx: usize) -> u64 {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_getDereferenceableBytes(inst, idx) as u64
            }
        }

        pub fn getEntryBlock(inst: *const super::super::llvm_Function) -> *const super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_Function_getEntryBlock(inst)
            }
        }

        pub fn getEntryBlockMut(inst: *mut super::super::llvm_Function) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_Function_getEntryBlockMut(inst)
            }
        }

        pub fn getFirstArg(inst: *const super::super::llvm_Function) -> *const super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Function_getFirstArg(inst)
            }
        }

        pub fn getFirstArgMut(inst: *mut super::super::llvm_Function) -> *mut super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_Function_getFirstArgMut(inst)
            }
        }

        pub fn getFunctionType(inst: *const super::super::llvm_Function) -> *mut super::super::llvm_FunctionType {
            unsafe {
                super::super::raw::llvm_Function_getFunctionType(inst)
            }
        }

        pub fn getIntrinsicID(inst: *const super::super::llvm_Function) -> usize {
            unsafe {
                super::super::raw::llvm_Function_getIntrinsicID(inst) as usize
            }
        }

        pub fn getParamAlignment(inst: *const super::super::llvm_Function, idx: usize) -> usize {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_getParamAlignment(inst, idx) as usize
            }
        }

        pub fn getReturnType(inst: *const super::super::llvm_Function) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Function_getReturnType(inst)
            }
        }

        pub fn getValueSymbolTable(inst: *const super::super::llvm_Function) -> *const super::super::llvm_ValueSymbolTable {
            unsafe {
                super::super::raw::llvm_Function_getValueSymbolTable(inst)
            }
        }

        pub fn getValueSymbolTableMut(inst: *mut super::super::llvm_Function) -> *mut super::super::llvm_ValueSymbolTable {
            unsafe {
                super::super::raw::llvm_Function_getValueSymbolTableMut(inst)
            }
        }

        pub fn hasFnAttr(inst: *const super::super::llvm_Function, Kind: &str) -> bool {
            let Kind = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
                length: Kind.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Function_hasFnAttr(inst, Kind) != 0
            }
        }

        pub fn hasGC(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_hasGC(inst) != 0
            }
        }

        pub fn hasStructRetAttr(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_hasStructRetAttr(inst) != 0
            }
        }

        pub fn hasUWTable(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_hasUWTable(inst) != 0
            }
        }

        pub fn isIntrinsic(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_isIntrinsic(inst) != 0
            }
        }

        pub fn isVarArg(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_isVarArg(inst) != 0
            }
        }

        pub fn needsUnwindTableEntry(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_needsUnwindTableEntry(inst) != 0
            }
        }

        pub fn onlyReadsMemory(inst: *const super::super::llvm_Function) -> bool {
            unsafe {
                super::super::raw::llvm_Function_onlyReadsMemory(inst) != 0
            }
        }

        pub fn onlyReadsMemoryParam(inst: *const super::super::llvm_Function, n: usize) -> bool {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_onlyReadsMemoryParam(inst, n) != 0
            }
        }

        pub fn removeFromParent(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_removeFromParent(inst)
            }
        }

        pub fn setCallingConv(inst: *mut super::super::llvm_Function, CC: super::super::llvm_CallingConv_ID) {
            unsafe {
                super::super::raw::llvm_Function_setCallingConv(inst, CC)
            }
        }

        pub fn setCannotDuplicate(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setCannotDuplicate(inst)
            }
        }

        pub fn setDoesNotAccessMemory(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setDoesNotAccessMemory(inst)
            }
        }

        pub fn setDoesNotAccessMemoryParam(inst: *mut super::super::llvm_Function, n: usize) {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_setDoesNotAccessMemoryParam(inst, n)
            }
        }

        pub fn setDoesNotAlias(inst: *mut super::super::llvm_Function, n: usize) {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_setDoesNotAlias(inst, n)
            }
        }

        pub fn setDoesNotCapture(inst: *mut super::super::llvm_Function, n: usize) {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_setDoesNotCapture(inst, n)
            }
        }

        pub fn setDoesNotReturn(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setDoesNotReturn(inst)
            }
        }

        pub fn setDoesNotThrow(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setDoesNotThrow(inst)
            }
        }

        pub fn setHasUWTable(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setHasUWTable(inst)
            }
        }

        pub fn setOnlyReadsMemory(inst: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_Function_setOnlyReadsMemory(inst)
            }
        }

        pub fn setOnlyReadsMemoryParam(inst: *mut super::super::llvm_Function, n: usize) {
            let n = n as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Function_setOnlyReadsMemoryParam(inst, n)
            }
        }
    }

    pub mod FunctionPass {
    }

    pub mod FunctionPassManager {
        pub fn add(inst: *mut super::super::llvm_FunctionPassManager, Pass: *mut super::super::llvm_FunctionPass) {
            unsafe {
                super::super::raw::llvm_FunctionPassManager_add(inst, Pass)
            }
        }

        pub fn doFinalization(inst: *mut super::super::llvm_FunctionPassManager) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionPassManager_doFinalization(inst) != 0
            }
        }

        pub fn doInitialization(inst: *mut super::super::llvm_FunctionPassManager) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionPassManager_doInitialization(inst) != 0
            }
        }

        pub fn new(Module: *mut super::super::llvm_Module) -> *mut super::super::llvm_FunctionPassManager {
            unsafe {
                super::super::raw::llvm_FunctionPassManager_new(Module)
            }
        }

        pub fn run(inst: *mut super::super::llvm_FunctionPassManager, Function: *mut super::super::llvm_Function) {
            unsafe {
                super::super::raw::llvm_FunctionPassManager_run(inst, Function)
            }
        }
    }

    pub mod FunctionType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionType_classof(ty) != 0
            }
        }

        pub fn get(Result: *mut super::super::llvm_Type, Params: &[*mut super::super::llvm_Type], isVarArg: bool) -> *mut super::super::llvm_FunctionType {
            let Params = super::super::llvm_ArrayRef_ptr_llvm_Type {
                data: Params.as_ptr(),
                size: Params.len() as super::super::libc::size_t,
            };
            let isVarArg = if isVarArg { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_FunctionType_get(Result, Params, isVarArg)
            }
        }

        pub fn getNumParams(inst: *const super::super::llvm_FunctionType) -> usize {
            unsafe {
                super::super::raw::llvm_FunctionType_getNumParams(inst) as usize
            }
        }

        pub fn getParamType(inst: *const super::super::llvm_FunctionType, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_FunctionType_getParamType(inst, idx)
            }
        }

        pub fn getReturnType(inst: *const super::super::llvm_FunctionType) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_FunctionType_getReturnType(inst)
            }
        }

        pub fn isValidArgumentType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionType_isValidArgumentType(ty) != 0
            }
        }

        pub fn isValidReturnType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionType_isValidReturnType(ty) != 0
            }
        }

        pub fn isVarArg(inst: *const super::super::llvm_FunctionType) -> bool {
            unsafe {
                super::super::raw::llvm_FunctionType_isVarArg(inst) != 0
            }
        }
    }

    pub mod GetElementPtrInst {
    }

    pub mod GlobalAlias {
    }

    pub mod GlobalObject {
        pub fn setSection(inst: *mut super::super::llvm_GlobalObject, S: &str) {
            let S = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(S.as_ptr()) },
                length: S.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_GlobalObject_setSection(inst, S)
            }
        }
    }

    pub mod GlobalValue {
        pub fn copyAttributesFrom(inst: *mut super::super::llvm_GlobalValue, Src: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalValue_copyAttributesFrom(inst, Src)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalValue_delete(inst)
            }
        }

        pub fn destroyConstant(inst: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalValue_destroyConstant(inst)
            }
        }

        pub fn eraseFromParent(inst: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalValue_eraseFromParent(inst)
            }
        }

        pub fn getAlignment(inst: *const super::super::llvm_GlobalValue) -> usize {
            unsafe {
                super::super::raw::llvm_GlobalValue_getAlignment(inst) as usize
            }
        }

        pub fn getDataLayout(inst: *const super::super::llvm_GlobalValue) -> *const super::super::llvm_DataLayout {
            unsafe {
                super::super::raw::llvm_GlobalValue_getDataLayout(inst)
            }
        }

        pub fn getParent(inst: *const super::super::llvm_GlobalValue) -> *const super::super::llvm_Module {
            unsafe {
                super::super::raw::llvm_GlobalValue_getParent(inst)
            }
        }

        pub fn getParentMut(inst: *mut super::super::llvm_GlobalValue) -> *mut super::super::llvm_Module {
            unsafe {
                super::super::raw::llvm_GlobalValue_getParentMut(inst)
            }
        }

        pub fn getType(inst: *const super::super::llvm_GlobalValue) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_GlobalValue_getType(inst)
            }
        }

        pub fn hasAppendingLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasAppendingLinkage(inst) != 0
            }
        }

        pub fn hasAvailableExternallyLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasAvailableExternallyLinkage(inst) != 0
            }
        }

        pub fn hasCommonLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasCommonLinkage(inst) != 0
            }
        }

        pub fn hasDLLExportStorageClass(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasDLLExportStorageClass(inst) != 0
            }
        }

        pub fn hasDLLImportStorageClass(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasDLLImportStorageClass(inst) != 0
            }
        }

        pub fn hasDefaultVisibility(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasDefaultVisibility(inst) != 0
            }
        }

        pub fn hasExternalLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasExternalLinkage(inst) != 0
            }
        }

        pub fn hasExternalWeakLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasExternalWeakLinkage(inst) != 0
            }
        }

        pub fn hasHiddenVisibility(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasHiddenVisibility(inst) != 0
            }
        }

        pub fn hasInternalLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasInternalLinkage(inst) != 0
            }
        }

        pub fn hasLinkOnceLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasLinkOnceLinkage(inst) != 0
            }
        }

        pub fn hasLocalLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasLocalLinkage(inst) != 0
            }
        }

        pub fn hasPrivateLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasPrivateLinkage(inst) != 0
            }
        }

        pub fn hasProtectedVisibility(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasProtectedVisibility(inst) != 0
            }
        }

        pub fn hasSection(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasSection(inst) != 0
            }
        }

        pub fn hasUnnamedAddr(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasUnnamedAddr(inst) != 0
            }
        }

        pub fn hasWeakAnyLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasWeakAnyLinkage(inst) != 0
            }
        }

        pub fn hasWeakLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasWeakLinkage(inst) != 0
            }
        }

        pub fn hasWeakODRLinkage(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_hasWeakODRLinkage(inst) != 0
            }
        }

        pub fn isDeclaration(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_isDeclaration(inst) != 0
            }
        }

        pub fn isDiscardableIfUnused(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_isDiscardableIfUnused(inst) != 0
            }
        }

        pub fn isThreadLocal(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_isThreadLocal(inst) != 0
            }
        }

        pub fn isWeakForLinker(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_isWeakForLinker(inst) != 0
            }
        }

        pub fn mayBeOverridden(inst: *const super::super::llvm_GlobalValue) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalValue_mayBeOverridden(inst) != 0
            }
        }

        pub fn removeFromParent(inst: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalValue_removeFromParent(inst)
            }
        }

        pub fn setThreadLocal(inst: *mut super::super::llvm_GlobalValue, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalValue_setThreadLocal(inst, Val)
            }
        }

        pub fn setUnnamedAddr(inst: *mut super::super::llvm_GlobalValue, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalValue_setUnnamedAddr(inst, Val)
            }
        }
    }

    pub mod GlobalVariable {
        pub fn copyAttributesFrom(inst: *mut super::super::llvm_GlobalVariable, Src: *mut super::super::llvm_GlobalValue) {
            unsafe {
                super::super::raw::llvm_GlobalVariable_copyAttributesFrom(inst, Src)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_GlobalVariable) {
            unsafe {
                super::super::raw::llvm_GlobalVariable_delete(inst)
            }
        }

        pub fn eraseFromParent(inst: *mut super::super::llvm_GlobalVariable) {
            unsafe {
                super::super::raw::llvm_GlobalVariable_eraseFromParent(inst)
            }
        }

        pub fn getInitializer(inst: *const super::super::llvm_GlobalVariable) -> *const super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_GlobalVariable_getInitializer(inst)
            }
        }

        pub fn getInitializerMut(inst: *mut super::super::llvm_GlobalVariable) -> *mut super::super::llvm_Constant {
            unsafe {
                super::super::raw::llvm_GlobalVariable_getInitializerMut(inst)
            }
        }

        pub fn hasDefinitiveInitializer(inst: *const super::super::llvm_GlobalVariable) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalVariable_hasDefinitiveInitializer(inst) != 0
            }
        }

        pub fn hasInitializer(inst: *const super::super::llvm_GlobalVariable) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalVariable_hasInitializer(inst) != 0
            }
        }

        pub fn hasUniqueInitializer(inst: *const super::super::llvm_GlobalVariable) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalVariable_hasUniqueInitializer(inst) != 0
            }
        }

        pub fn isConstant(inst: *const super::super::llvm_GlobalVariable) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalVariable_isConstant(inst) != 0
            }
        }

        pub fn isExternallyInitialized(inst: *const super::super::llvm_GlobalVariable) -> bool {
            unsafe {
                super::super::raw::llvm_GlobalVariable_isExternallyInitialized(inst) != 0
            }
        }

        pub fn new(Ty: *mut super::super::llvm_Type, isConstant: bool, Linkage: super::super::llvm_GlobalValue_LinkageTypes) -> *mut super::super::llvm_GlobalVariable {
            let isConstant = if isConstant { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalVariable_new(Ty, isConstant, Linkage)
            }
        }

        pub fn newWithModule(Module: *mut super::super::llvm_Module, Ty: *mut super::super::llvm_Type, isConstant: bool, Linkage: super::super::llvm_GlobalValue_LinkageTypes, Initializer: *mut super::super::llvm_Constant) -> *mut super::super::llvm_GlobalVariable {
            let isConstant = if isConstant { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalVariable_newWithModule(Module, Ty, isConstant, Linkage, Initializer)
            }
        }

        pub fn removeFromParent(inst: *mut super::super::llvm_GlobalVariable) {
            unsafe {
                super::super::raw::llvm_GlobalVariable_removeFromParent(inst)
            }
        }

        pub fn setConstant(inst: *mut super::super::llvm_GlobalVariable, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalVariable_setConstant(inst, Val)
            }
        }

        pub fn setExternallyInitialized(inst: *mut super::super::llvm_GlobalVariable, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_GlobalVariable_setExternallyInitialized(inst, Val)
            }
        }

        pub fn setInitializer(inst: *mut super::super::llvm_GlobalVariable, InitVal: *mut super::super::llvm_Constant) {
            unsafe {
                super::super::raw::llvm_GlobalVariable_setInitializer(inst, InitVal)
            }
        }
    }

    pub mod IRBuilder {
        pub fn CreateAShr(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAShr(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateAShrByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAShrByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateAdd(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAdd(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateAddrSpaceCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAddrSpaceCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateAlignedLoad(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Align: usize, Name: Option<&str>) -> *mut super::super::llvm_LoadInst {
            let Align = Align as super::super::libc::c_uint;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAlignedLoad(inst, Ptr, Align, Name)
            }
        }

        pub fn CreateAlignedLoadVolatile(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Align: usize, isVolatile: bool, Name: Option<&str>) -> *mut super::super::llvm_LoadInst {
            let Align = Align as super::super::libc::c_uint;
            let isVolatile = if isVolatile { 1 } else { 0 };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAlignedLoadVolatile(inst, Ptr, Align, isVolatile, Name)
            }
        }

        pub fn CreateAlignedStore(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Ptr: *mut super::super::llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut super::super::llvm_StoreInst {
            let Align = Align as super::super::libc::c_uint;
            let opt_hack_872884 = isVolatile.map(|value| if value { 1 } else { 0 });
            let isVolatile = opt_hack_872884.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAlignedStore(inst, Value, Ptr, Align, isVolatile)
            }
        }

        pub fn CreateAlloca(inst: *mut super::super::llvm_IRBuilder, Ty: *mut super::super::llvm_Type, ArraySize: Option<*mut super::super::llvm_Value>, Name: Option<&str>) -> *mut super::super::llvm_AllocaInst {
            let ArraySize = ArraySize.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAlloca(inst, Ty, ArraySize, Name)
            }
        }

        pub fn CreateAnd(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAnd(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateAndByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateAndByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateBinOp(inst: *mut super::super::llvm_IRBuilder, Opcode: super::super::llvm_Instruction_BinaryOps, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateBinOp(inst, Opcode, LHS, RHS, Name)
            }
        }

        pub fn CreateBitCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateBitCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateBr(inst: *mut super::super::llvm_IRBuilder, Dest: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_BranchInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateBr(inst, Dest)
            }
        }

        pub fn CreateCall(inst: *mut super::super::llvm_IRBuilder, Callee: *mut super::super::llvm_Value, Args: &[*mut super::super::llvm_Value], Name: Option<&str>) -> *mut super::super::llvm_CallInst {
            let Args = super::super::llvm_ArrayRef_ptr_llvm_Value {
                data: Args.as_ptr(),
                size: Args.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateCall(inst, Callee, Args, Name)
            }
        }

        pub fn CreateCast(inst: *mut super::super::llvm_IRBuilder, Opcode: super::super::llvm_Instruction_CastOps, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateCast(inst, Opcode, Value, DestTy, Name)
            }
        }

        pub fn CreateCondBr(inst: *mut super::super::llvm_IRBuilder, Cond: *mut super::super::llvm_Value, TrueBlock: *mut super::super::llvm_BasicBlock, FalseBlock: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_BranchInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateCondBr(inst, Cond, TrueBlock, FalseBlock)
            }
        }

        pub fn CreateExactSDiv(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateExactSDiv(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateExactUDiv(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateExactUDiv(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateExtractElement(inst: *mut super::super::llvm_IRBuilder, Vec: *mut super::super::llvm_Value, Idx: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateExtractElement(inst, Vec, Idx, Name)
            }
        }

        pub fn CreateExtractInteger(inst: *mut super::super::llvm_IRBuilder, DL: *const super::super::llvm_DataLayout, From: *mut super::super::llvm_Value, ExtractedTy: *mut super::super::llvm_IntegerType, Offset: u64, Name: &str) -> *mut super::super::llvm_Value {
            let Offset = Offset as super::super::libc::uint64_t;
            let Name = super::super::std_string {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateExtractInteger(inst, DL, From, ExtractedTy, Offset, Name)
            }
        }

        pub fn CreateExtractValue(inst: *mut super::super::llvm_IRBuilder, Agg: *mut super::super::llvm_Value, Indexes: &[super::super::libc::c_uint], Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Indexes = super::super::llvm_ArrayRef_uint {
                data: Indexes.as_ptr(),
                size: Indexes.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateExtractValue(inst, Agg, Indexes, Name)
            }
        }

        pub fn CreateFAdd(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFAdd(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmp(inst: *mut super::super::llvm_IRBuilder, Pred: super::super::llvm_CmpInst_Predicate, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmp(inst, Pred, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpOEQ(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpOEQ(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpOGE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpOGE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpOGT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpOGT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpOLE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpOLE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpOLT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpOLT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpONE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpONE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpORD(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpORD(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpUEQ(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpUEQ(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpUGE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpUGE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpUGT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpUGT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpULE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpULE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpULT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpULT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpUNE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpUNE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFCmpUNO(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFCmpUNO(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFDiv(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFDiv(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFMul(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFMul(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFNeg(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFNeg(inst, Value, Name)
            }
        }

        pub fn CreateFPCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFPCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateFPExt(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFPExt(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateFPToSI(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFPToSI(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateFPToUI(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFPToUI(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateFPTrunc(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFPTrunc(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateFRem(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFRem(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFSub(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFSub(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateFence(inst: *mut super::super::llvm_IRBuilder, Ordering: super::super::llvm_AtomicOrdering, SynchScope: Option<super::super::llvm_SynchronizationScope>, Name: Option<&str>) -> *mut super::super::llvm_FenceInst {
            let SynchScope = SynchScope.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateFence(inst, Ordering, SynchScope, Name)
            }
        }

        pub fn CreateGEP(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Indexes: &[*mut super::super::llvm_Value], Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Indexes = super::super::llvm_ArrayRef_ptr_llvm_Value {
                data: Indexes.as_ptr(),
                size: Indexes.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateGEP(inst, Ptr, Indexes, Name)
            }
        }

        pub fn CreateGlobalStringPtr(inst: *mut super::super::llvm_IRBuilder, Str: &str, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Str = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
                length: Str.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateGlobalStringPtr(inst, Str, Name)
            }
        }

        pub fn CreateICmp(inst: *mut super::super::llvm_IRBuilder, Pred: super::super::llvm_CmpInst_Predicate, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmp(inst, Pred, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpEQ(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpEQ(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpNE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpNE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpSGE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpSGE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpSGT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpSGT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpSLE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpSLE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpSLT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpSLT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpUGE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpUGE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpUGT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpUGT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpULE(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpULE(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateICmpULT(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateICmpULT(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateInBoundsGEP(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Indexes: &[*mut super::super::llvm_Value], Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Indexes = super::super::llvm_ArrayRef_ptr_llvm_Value {
                data: Indexes.as_ptr(),
                size: Indexes.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateInBoundsGEP(inst, Ptr, Indexes, Name)
            }
        }

        pub fn CreateIndirectBr(inst: *mut super::super::llvm_IRBuilder, Addr: *mut super::super::llvm_Value, NumCases: Option<usize>) -> *mut super::super::llvm_IndirectBrInst {
            let opt_hack_775166 = NumCases.map(|value| value as super::super::libc::c_uint);
            let NumCases = opt_hack_775166.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateIndirectBr(inst, Addr, NumCases)
            }
        }

        pub fn CreateInsertElement(inst: *mut super::super::llvm_IRBuilder, Vec: *mut super::super::llvm_Value, NewElt: *mut super::super::llvm_Value, Idx: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateInsertElement(inst, Vec, NewElt, Idx, Name)
            }
        }

        pub fn CreateInsertValue(inst: *mut super::super::llvm_IRBuilder, Agg: *mut super::super::llvm_Value, Value: *mut super::super::llvm_Value, Indexes: &[super::super::libc::c_uint], Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Indexes = super::super::llvm_ArrayRef_uint {
                data: Indexes.as_ptr(),
                size: Indexes.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateInsertValue(inst, Agg, Value, Indexes, Name)
            }
        }

        pub fn CreateIntCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, isSigned: bool, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let isSigned = if isSigned { 1 } else { 0 };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateIntCast(inst, Value, DestTy, isSigned, Name)
            }
        }

        pub fn CreateIntToPtr(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateIntToPtr(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateInvoke(inst: *mut super::super::llvm_IRBuilder, Callee: *mut super::super::llvm_Value, NormalDest: *mut super::super::llvm_BasicBlock, UnwindDest: *mut super::super::llvm_BasicBlock, Args: &[*mut super::super::llvm_Value], Name: Option<&str>) -> *mut super::super::llvm_InvokeInst {
            let Args = super::super::llvm_ArrayRef_ptr_llvm_Value {
                data: Args.as_ptr(),
                size: Args.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string_const {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateInvoke(inst, Callee, NormalDest, UnwindDest, Args, Name)
            }
        }

        pub fn CreateIsNotNull(inst: *mut super::super::llvm_IRBuilder, Arg: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateIsNotNull(inst, Arg, Name)
            }
        }

        pub fn CreateIsNull(inst: *mut super::super::llvm_IRBuilder, Arg: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateIsNull(inst, Arg, Name)
            }
        }

        pub fn CreateLShr(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateLShr(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateLShrByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateLShrByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateLandingPad(inst: *mut super::super::llvm_IRBuilder, Ty: *mut super::super::llvm_Type, PersFn: *mut super::super::llvm_Value, NumClauses: usize, Name: Option<&str>) -> *mut super::super::llvm_LandingPadInst {
            let NumClauses = NumClauses as super::super::libc::c_uint;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateLandingPad(inst, Ty, PersFn, NumClauses, Name)
            }
        }

        pub fn CreateLoad(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_LoadInst {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateLoad(inst, Ptr, Name)
            }
        }

        pub fn CreateLoadVolatile(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, isVolatile: bool, Name: Option<&str>) -> *mut super::super::llvm_LoadInst {
            let isVolatile = if isVolatile { 1 } else { 0 };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateLoadVolatile(inst, Ptr, isVolatile, Name)
            }
        }

        pub fn CreateMul(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateMul(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNSWAdd(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNSWAdd(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNSWMul(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNSWMul(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNSWNeg(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNSWNeg(inst, Value, Name)
            }
        }

        pub fn CreateNSWSub(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNSWSub(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNUWAdd(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNUWAdd(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNUWMul(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNUWMul(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNUWNeg(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNUWNeg(inst, Value, Name)
            }
        }

        pub fn CreateNUWSub(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNUWSub(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateNeg(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNeg(inst, Value, Name)
            }
        }

        pub fn CreateNot(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateNot(inst, Value, Name)
            }
        }

        pub fn CreateOr(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateOr(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateOrByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateOrByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreatePHI(inst: *mut super::super::llvm_IRBuilder, Ty: *mut super::super::llvm_Type, NumReservedValues: usize, Name: Option<&str>) -> *mut super::super::llvm_PHINode {
            let NumReservedValues = NumReservedValues as super::super::libc::c_uint;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreatePHI(inst, Ty, NumReservedValues, Name)
            }
        }

        pub fn CreatePointerBitCastOrAddrSpaceCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreatePointerCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreatePointerCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreatePtrDiff(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreatePtrDiff(inst, LHS, RHS, Name)
            }
        }

        pub fn CreatePtrToInt(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreatePtrToInt(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateResume(inst: *mut super::super::llvm_IRBuilder, Exn: *mut super::super::llvm_Value) -> *mut super::super::llvm_ResumeInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateResume(inst, Exn)
            }
        }

        pub fn CreateRet(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value) -> *mut super::super::llvm_ReturnInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateRet(inst, Value)
            }
        }

        pub fn CreateRetVoid(inst: *mut super::super::llvm_IRBuilder) -> *mut super::super::llvm_ReturnInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateRetVoid(inst)
            }
        }

        pub fn CreateSDiv(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSDiv(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateSExt(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSExt(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateSExtOrBitCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSExtOrBitCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateSExtOrTrunc(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSExtOrTrunc(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateSIToFP(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSIToFP(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateSRem(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSRem(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateSelect(inst: *mut super::super::llvm_IRBuilder, C: *mut super::super::llvm_Value, TrueValue: *mut super::super::llvm_Value, FalseValue: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSelect(inst, C, TrueValue, FalseValue, Name)
            }
        }

        pub fn CreateShl(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateShl(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateShlByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateShlByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateShuffleVector(inst: *mut super::super::llvm_IRBuilder, V1: *mut super::super::llvm_Value, P2: *mut super::super::llvm_Value, Mask: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateShuffleVector(inst, V1, P2, Mask, Name)
            }
        }

        pub fn CreateStore(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Ptr: *mut super::super::llvm_Value, isVolatile: Option<bool>) -> *mut super::super::llvm_StoreInst {
            let opt_hack_872884 = isVolatile.map(|value| if value { 1 } else { 0 });
            let isVolatile = opt_hack_872884.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateStore(inst, Value, Ptr, isVolatile)
            }
        }

        pub fn CreateStructGEP(inst: *mut super::super::llvm_IRBuilder, Ptr: *mut super::super::llvm_Value, Index: usize, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Index = Index as super::super::libc::c_uint;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateStructGEP(inst, Ptr, Index, Name)
            }
        }

        pub fn CreateSub(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSub(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateSwitch(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, Dest: *mut super::super::llvm_BasicBlock, NumCases: Option<usize>) -> *mut super::super::llvm_SwitchInst {
            let opt_hack_775166 = NumCases.map(|value| value as super::super::libc::c_uint);
            let NumCases = opt_hack_775166.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateSwitch(inst, Value, Dest, NumCases)
            }
        }

        pub fn CreateTrunc(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateTrunc(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateTruncOrBitCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateTruncOrBitCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateUDiv(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateUDiv(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateUIToFP(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateUIToFP(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateURem(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateURem(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateUnreachable(inst: *mut super::super::llvm_IRBuilder) -> *mut super::super::llvm_UnreachableInst {
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateUnreachable(inst)
            }
        }

        pub fn CreateVAArg(inst: *mut super::super::llvm_IRBuilder, List: *mut super::super::llvm_Value, Ty: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_VAArgInst {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateVAArg(inst, List, Ty, Name)
            }
        }

        pub fn CreateVectorSplat(inst: *mut super::super::llvm_IRBuilder, NumElements: usize, Value: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let NumElements = NumElements as super::super::libc::c_uint;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateVectorSplat(inst, NumElements, Value, Name)
            }
        }

        pub fn CreateXor(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: *mut super::super::llvm_Value, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateXor(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateXorByValue(inst: *mut super::super::llvm_IRBuilder, LHS: *mut super::super::llvm_Value, RHS: u64, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let RHS = RHS as super::super::libc::uint64_t;
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateXorByValue(inst, LHS, RHS, Name)
            }
        }

        pub fn CreateZExt(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateZExt(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateZExtOrBitCast(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateZExtOrBitCast(inst, Value, DestTy, Name)
            }
        }

        pub fn CreateZExtOrTrunc(inst: *mut super::super::llvm_IRBuilder, Value: *mut super::super::llvm_Value, DestTy: *mut super::super::llvm_Type, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilder_CreateZExtOrTrunc(inst, Value, DestTy, Name)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_IRBuilder) {
            unsafe {
                super::super::raw::llvm_IRBuilder_delete(inst)
            }
        }

        pub fn isNamePreserving(inst: *const super::super::llvm_IRBuilder) -> bool {
            unsafe {
                super::super::raw::llvm_IRBuilder_isNamePreserving(inst) != 0
            }
        }

        pub fn new(Context: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IRBuilder {
            unsafe {
                super::super::raw::llvm_IRBuilder_new(Context)
            }
        }

        pub fn new_in_block(BB: *mut super::super::llvm_BasicBlock) -> *mut super::super::llvm_IRBuilder {
            unsafe {
                super::super::raw::llvm_IRBuilder_new_in_block(BB)
            }
        }
    }

    pub mod IRBuilderBase {
        pub fn ClearInsertionPoint(inst: *mut super::super::llvm_IRBuilderBase) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_ClearInsertionPoint(inst)
            }
        }

        pub fn CreateGlobalString(inst: *mut super::super::llvm_IRBuilderBase, Str: &str, Name: Option<&str>) -> *mut super::super::llvm_Value {
            let Str = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Str.as_ptr()) },
                length: Str.len() as super::super::libc::size_t,
            };
            let opt_hack_467406 = Name.map(|value| super::super::std_string {
                data: unsafe { ::std::mem::transmute(value.as_ptr()) },
                length: value.len() as super::super::libc::size_t,
            });
            let Name = opt_hack_467406.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateGlobalString(inst, Str, Name)
            }
        }

        pub fn CreateLifetimeEnd(inst: *mut super::super::llvm_IRBuilderBase, Ptr: *mut super::super::llvm_Value, Size: Option<*mut super::super::llvm_ConstantInt>) -> *mut super::super::llvm_CallInst {
            let Size = Size.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateLifetimeEnd(inst, Ptr, Size)
            }
        }

        pub fn CreateLifetimeStart(inst: *mut super::super::llvm_IRBuilderBase, Ptr: *mut super::super::llvm_Value, Size: Option<*mut super::super::llvm_ConstantInt>) -> *mut super::super::llvm_CallInst {
            let Size = Size.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateLifetimeStart(inst, Ptr, Size)
            }
        }

        pub fn CreateMemCpy(inst: *mut super::super::llvm_IRBuilderBase, Dst: *mut super::super::llvm_Value, Src: *mut super::super::llvm_Value, Size: *mut super::super::llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut super::super::llvm_CallInst {
            let Align = Align as super::super::libc::c_uint;
            let opt_hack_872884 = isVolatile.map(|value| if value { 1 } else { 0 });
            let isVolatile = opt_hack_872884.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateMemCpy(inst, Dst, Src, Size, Align, isVolatile)
            }
        }

        pub fn CreateMemMove(inst: *mut super::super::llvm_IRBuilderBase, Dst: *mut super::super::llvm_Value, Src: *mut super::super::llvm_Value, Size: *mut super::super::llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut super::super::llvm_CallInst {
            let Align = Align as super::super::libc::c_uint;
            let opt_hack_872884 = isVolatile.map(|value| if value { 1 } else { 0 });
            let isVolatile = opt_hack_872884.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateMemMove(inst, Dst, Src, Size, Align, isVolatile)
            }
        }

        pub fn CreateMemSet(inst: *mut super::super::llvm_IRBuilderBase, Ptr: *mut super::super::llvm_Value, Value: *mut super::super::llvm_Value, Size: *mut super::super::llvm_Value, Align: usize, isVolatile: Option<bool>) -> *mut super::super::llvm_CallInst {
            let Align = Align as super::super::libc::c_uint;
            let opt_hack_872884 = isVolatile.map(|value| if value { 1 } else { 0 });
            let isVolatile = opt_hack_872884.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_CreateMemSet(inst, Ptr, Value, Size, Align, isVolatile)
            }
        }

        pub fn GetInsertBlock(inst: *const super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_GetInsertBlock(inst)
            }
        }

        pub fn SetCurrentDebugLocation(inst: *mut super::super::llvm_IRBuilderBase, Loc: *const super::super::llvm_DebugLoc) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_SetCurrentDebugLocation(inst, Loc)
            }
        }

        pub fn SetDefaultFPMathTag(inst: *mut super::super::llvm_IRBuilderBase, FPMathTag: *mut super::super::llvm_MDNode) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_SetDefaultFPMathTag(inst, FPMathTag)
            }
        }

        pub fn SetInsertPoint(inst: *mut super::super::llvm_IRBuilderBase, BB: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_SetInsertPoint(inst, BB)
            }
        }

        pub fn SetInsertPointAtInst(inst: *mut super::super::llvm_IRBuilderBase, Inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_SetInsertPointAtInst(inst, Inst)
            }
        }

        pub fn SetInstDebugLocation(inst: *const super::super::llvm_IRBuilderBase, Inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_SetInstDebugLocation(inst, Inst)
            }
        }

        pub fn getContext(inst: *const super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getContext(inst)
            }
        }

        pub fn getCurrentFunctionReturnType(inst: *const super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getCurrentFunctionReturnType(inst)
            }
        }

        pub fn getDefaultFPMathTag(inst: *const super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_MDNode {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getDefaultFPMathTag(inst)
            }
        }

        pub fn getDoubleTy(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getDoubleTy(inst)
            }
        }

        pub fn getFalse(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_ConstantInt {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getFalse(inst)
            }
        }

        pub fn getFloatTy(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getFloatTy(inst)
            }
        }

        pub fn getHalfTy(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getHalfTy(inst)
            }
        }

        pub fn getInt(inst: *mut super::super::llvm_IRBuilderBase, Value: (&[u64], usize)) -> *mut super::super::llvm_ConstantInt {
            let Value = super::super::llvm_APInt {
                data: super::super::llvm_ArrayRef_uint64 {
                    data: Value.0.as_ptr(),
                    size: Value.0.len() as super::super::libc::size_t,
                },
                numbits: Value.1 as super::super::libc::c_uint,
            };
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt(inst, Value)
            }
        }

        pub fn getInt1(inst: *mut super::super::llvm_IRBuilderBase, Value: bool) -> *mut super::super::llvm_ConstantInt {
            let Value = if Value { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt1(inst, Value)
            }
        }

        pub fn getInt16(inst: *mut super::super::llvm_IRBuilderBase, Value: u16) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint16_t;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt16(inst, Value)
            }
        }

        pub fn getInt16Ty(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt16Ty(inst)
            }
        }

        pub fn getInt1Ty(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt1Ty(inst)
            }
        }

        pub fn getInt32(inst: *mut super::super::llvm_IRBuilderBase, Value: u32) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint32_t;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt32(inst, Value)
            }
        }

        pub fn getInt32Ty(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt32Ty(inst)
            }
        }

        pub fn getInt64(inst: *mut super::super::llvm_IRBuilderBase, Value: u64) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt64(inst, Value)
            }
        }

        pub fn getInt64Ty(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt64Ty(inst)
            }
        }

        pub fn getInt8(inst: *mut super::super::llvm_IRBuilderBase, Value: u8) -> *mut super::super::llvm_ConstantInt {
            let Value = Value as super::super::libc::uint8_t;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt8(inst, Value)
            }
        }

        pub fn getInt8PtrTy(inst: *mut super::super::llvm_IRBuilderBase, AddrSpace: Option<usize>) -> *mut super::super::llvm_PointerType {
            let opt_hack_647634 = AddrSpace.map(|value| value as super::super::libc::c_uint);
            let AddrSpace = opt_hack_647634.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt8PtrTy(inst, AddrSpace)
            }
        }

        pub fn getInt8Ty(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getInt8Ty(inst)
            }
        }

        pub fn getIntN(inst: *mut super::super::llvm_IRBuilderBase, NumBits: usize, Value: u64) -> *mut super::super::llvm_ConstantInt {
            let NumBits = NumBits as super::super::libc::c_uint;
            let Value = Value as super::super::libc::uint64_t;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getIntN(inst, NumBits, Value)
            }
        }

        pub fn getIntNTy(inst: *mut super::super::llvm_IRBuilderBase, NumBits: usize) -> *mut super::super::llvm_IntegerType {
            let NumBits = NumBits as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getIntNTy(inst, NumBits)
            }
        }

        pub fn getIntPtrTy(inst: *mut super::super::llvm_IRBuilderBase, DL: *const super::super::llvm_DataLayout, AddrSpace: Option<usize>) -> *mut super::super::llvm_IntegerType {
            let opt_hack_647634 = AddrSpace.map(|value| value as super::super::libc::c_uint);
            let AddrSpace = opt_hack_647634.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getIntPtrTy(inst, DL, AddrSpace)
            }
        }

        pub fn getTrue(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_ConstantInt {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getTrue(inst)
            }
        }

        pub fn getVoidTy(inst: *mut super::super::llvm_IRBuilderBase) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_getVoidTy(inst)
            }
        }

        pub fn new(Context: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IRBuilderBase {
            unsafe {
                super::super::raw::llvm_IRBuilderBase_new(Context)
            }
        }
    }

    pub mod IndirectBrInst {
    }

    pub mod InlineAsm {
    }

    pub mod InsertElementInst {
    }

    pub mod InsertValueInst {
    }

    pub mod Instruction {
        pub fn clone(inst: *const super::super::llvm_Instruction) -> *mut super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_Instruction_clone(inst)
            }
        }

        pub fn copyFastMathFlags(inst: *mut super::super::llvm_Instruction, Inst: *const super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_copyFastMathFlags(inst, Inst)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_delete(inst)
            }
        }

        pub fn dropUnknownMetadata(inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_dropUnknownMetadata(inst)
            }
        }

        pub fn dropUnknownMetadataFromIDS(inst: *mut super::super::llvm_Instruction, KnownIDs: &[super::super::libc::c_uint]) {
            let KnownIDs = super::super::llvm_ArrayRef_uint {
                data: KnownIDs.as_ptr(),
                size: KnownIDs.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Instruction_dropUnknownMetadataFromIDS(inst, KnownIDs)
            }
        }

        pub fn eraseFromParent(inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_eraseFromParent(inst)
            }
        }

        pub fn getDataLayout(inst: *const super::super::llvm_Instruction) -> *const super::super::llvm_DataLayout {
            unsafe {
                super::super::raw::llvm_Instruction_getDataLayout(inst)
            }
        }

        pub fn getDebugLoc(inst: *const super::super::llvm_Instruction) -> *const super::super::llvm_DebugLoc {
            unsafe {
                super::super::raw::llvm_Instruction_getDebugLoc(inst)
            }
        }

        pub fn getMetadata(inst: *const super::super::llvm_Instruction, KindID: usize) -> *mut super::super::llvm_MDNode {
            let KindID = KindID as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Instruction_getMetadata(inst, KindID)
            }
        }

        pub fn getMetadataStr(inst: *const super::super::llvm_Instruction, Kind: &str) -> *mut super::super::llvm_MDNode {
            let Kind = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
                length: Kind.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Instruction_getMetadataStr(inst, Kind)
            }
        }

        pub fn getOpcode(inst: *const super::super::llvm_Instruction) -> usize {
            unsafe {
                super::super::raw::llvm_Instruction_getOpcode(inst) as usize
            }
        }

        pub fn getParent(inst: *const super::super::llvm_Instruction) -> *const super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_Instruction_getParent(inst)
            }
        }

        pub fn getParentMut(inst: *mut super::super::llvm_Instruction) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_Instruction_getParentMut(inst)
            }
        }

        pub fn hasAllowReciprocal(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasAllowReciprocal(inst) != 0
            }
        }

        pub fn hasMetadata(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasMetadata(inst) != 0
            }
        }

        pub fn hasMetadataOtherThanDebugLoc(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasMetadataOtherThanDebugLoc(inst) != 0
            }
        }

        pub fn hasNoInfs(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasNoInfs(inst) != 0
            }
        }

        pub fn hasNoNaNs(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasNoNaNs(inst) != 0
            }
        }

        pub fn hasNoSignedZeros(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasNoSignedZeros(inst) != 0
            }
        }

        pub fn hasUnsafeAlgebra(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_hasUnsafeAlgebra(inst) != 0
            }
        }

        pub fn insertAfter(inst: *mut super::super::llvm_Instruction, InsertPos: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_insertAfter(inst, InsertPos)
            }
        }

        pub fn insertBefore(inst: *mut super::super::llvm_Instruction, InsertPos: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_insertBefore(inst, InsertPos)
            }
        }

        pub fn isArithmeticShift(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isArithmeticShift(inst) != 0
            }
        }

        pub fn isAssociative(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isAssociative(inst) != 0
            }
        }

        pub fn isBinaryOp(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isBinaryOp(inst) != 0
            }
        }

        pub fn isCast(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isCast(inst) != 0
            }
        }

        pub fn isCommutative(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isCommutative(inst) != 0
            }
        }

        pub fn isIdempotent(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isIdempotent(inst) != 0
            }
        }

        pub fn isIdenticalTo(inst: *const super::super::llvm_Instruction, Inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isIdenticalTo(inst, Inst) != 0
            }
        }

        pub fn isIdenticalToWhenDefined(inst: *const super::super::llvm_Instruction, Inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isIdenticalToWhenDefined(inst, Inst) != 0
            }
        }

        pub fn isLogicalShift(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isLogicalShift(inst) != 0
            }
        }

        pub fn isNilpotent(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isNilpotent(inst) != 0
            }
        }

        pub fn isSameOperationAs(inst: *const super::super::llvm_Instruction, Inst: *const super::super::llvm_Instruction, flags: usize) -> bool {
            let flags = flags as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Instruction_isSameOperationAs(inst, Inst, flags) != 0
            }
        }

        pub fn isShift(inst: *mut super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isShift(inst) != 0
            }
        }

        pub fn isTerminator(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isTerminator(inst) != 0
            }
        }

        pub fn isUsedOutsideOfBlock(inst: *const super::super::llvm_Instruction, BB: *const super::super::llvm_BasicBlock) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_isUsedOutsideOfBlock(inst, BB) != 0
            }
        }

        pub fn mayHaveSideEffects(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayHaveSideEffects(inst) != 0
            }
        }

        pub fn mayReadFromMemory(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayReadFromMemory(inst) != 0
            }
        }

        pub fn mayReadOrWriteMemory(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayReadOrWriteMemory(inst) != 0
            }
        }

        pub fn mayReturn(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayReturn(inst) != 0
            }
        }

        pub fn mayThrow(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayThrow(inst) != 0
            }
        }

        pub fn mayWriteToMemory(inst: *const super::super::llvm_Instruction) -> bool {
            unsafe {
                super::super::raw::llvm_Instruction_mayWriteToMemory(inst) != 0
            }
        }

        pub fn moveBefore(inst: *mut super::super::llvm_Instruction, MovePos: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_moveBefore(inst, MovePos)
            }
        }

        pub fn removeFromParent(inst: *mut super::super::llvm_Instruction) {
            unsafe {
                super::super::raw::llvm_Instruction_removeFromParent(inst)
            }
        }

        pub fn setDebugLoc(inst: *mut super::super::llvm_Instruction, Loc: *const super::super::llvm_DebugLoc) {
            unsafe {
                super::super::raw::llvm_Instruction_setDebugLoc(inst, Loc)
            }
        }

        pub fn setHasAllowReciprocal(inst: *mut super::super::llvm_Instruction, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_Instruction_setHasAllowReciprocal(inst, Val)
            }
        }

        pub fn setHasNoInfs(inst: *mut super::super::llvm_Instruction, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_Instruction_setHasNoInfs(inst, Val)
            }
        }

        pub fn setHasNoNaNs(inst: *mut super::super::llvm_Instruction, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_Instruction_setHasNoNaNs(inst, Val)
            }
        }

        pub fn setHasNoSignedZeros(inst: *mut super::super::llvm_Instruction, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_Instruction_setHasNoSignedZeros(inst, Val)
            }
        }

        pub fn setHasUnsafeAlgebra(inst: *mut super::super::llvm_Instruction, Val: bool) {
            let Val = if Val { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_Instruction_setHasUnsafeAlgebra(inst, Val)
            }
        }

        pub fn setMetadata(inst: *mut super::super::llvm_Instruction, KindID: usize, Node: *mut super::super::llvm_MDNode) {
            let KindID = KindID as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Instruction_setMetadata(inst, KindID, Node)
            }
        }

        pub fn setMetadataStr(inst: *mut super::super::llvm_Instruction, Kind: &str, Node: *mut super::super::llvm_MDNode) {
            let Kind = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Kind.as_ptr()) },
                length: Kind.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Instruction_setMetadataStr(inst, Kind, Node)
            }
        }

        pub fn user_back(inst: *const super::super::llvm_Instruction) -> *const super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_Instruction_user_back(inst)
            }
        }

        pub fn user_back_mut(inst: *mut super::super::llvm_Instruction) -> *mut super::super::llvm_Instruction {
            unsafe {
                super::super::raw::llvm_Instruction_user_back_mut(inst)
            }
        }
    }

    pub mod IntegerType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_IntegerType_classof(ty) != 0
            }
        }

        pub fn get(ctx: *mut super::super::llvm_LLVMContext, NumBits: usize) -> *mut super::super::llvm_IntegerType {
            let NumBits = NumBits as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_IntegerType_get(ctx, NumBits)
            }
        }

        pub fn getBitMask(inst: *const super::super::llvm_IntegerType) -> u64 {
            unsafe {
                super::super::raw::llvm_IntegerType_getBitMask(inst) as u64
            }
        }

        pub fn getBitWidth(inst: *const super::super::llvm_IntegerType) -> usize {
            unsafe {
                super::super::raw::llvm_IntegerType_getBitWidth(inst) as usize
            }
        }

        pub fn getSignBit(inst: *const super::super::llvm_IntegerType) -> u64 {
            unsafe {
                super::super::raw::llvm_IntegerType_getSignBit(inst) as u64
            }
        }

        pub fn isPowerOf2ByteWidth(inst: *const super::super::llvm_IntegerType) -> bool {
            unsafe {
                super::super::raw::llvm_IntegerType_isPowerOf2ByteWidth(inst) != 0
            }
        }
    }

    pub mod InvokeInst {
    }

    pub mod LLVMContext {
        pub fn delete() -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_LLVMContext_delete()
            }
        }

        pub fn new() -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_LLVMContext_new()
            }
        }
    }

    pub mod LandingPadInst {
    }

    pub mod LoadInst {
    }

    pub mod LoopPass {
    }

    pub mod MDNode {
    }

    pub mod MDString {
    }

    pub mod Module {
        pub fn appendModuleInlineAsm(inst: *mut super::super::llvm_Module, Asm: &str) {
            let Asm = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Asm.as_ptr()) },
                length: Asm.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_appendModuleInlineAsm(inst, Asm)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_Module) {
            unsafe {
                super::super::raw::llvm_Module_delete(inst)
            }
        }

        pub fn dump(inst: *const super::super::llvm_Module) {
            unsafe {
                super::super::raw::llvm_Module_dump(inst)
            }
        }

        pub fn getContext(inst: *const super::super::llvm_Module) -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_Module_getContext(inst)
            }
        }

        pub fn getDataLayout(inst: *const super::super::llvm_Module) -> *const super::super::llvm_DataLayout {
            unsafe {
                super::super::raw::llvm_Module_getDataLayout(inst)
            }
        }

        pub fn getDataLayoutStr(inst: *const super::super::llvm_Module) -> super::super::std_string_const {
            unsafe {
                super::super::raw::llvm_Module_getDataLayoutStr(inst)
            }
        }

        pub fn getFunction(inst: *const super::super::llvm_Module, Name: &str) -> *mut super::super::llvm_Function {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_getFunction(inst, Name)
            }
        }

        pub fn getMDKindID(inst: *const super::super::llvm_Module, Name: &str) -> usize {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_getMDKindID(inst, Name) as usize
            }
        }

        pub fn getModuleIdentifier(inst: *const super::super::llvm_Module) -> super::super::std_string_const {
            unsafe {
                super::super::raw::llvm_Module_getModuleIdentifier(inst)
            }
        }

        pub fn getModuleInlineAsm(inst: *const super::super::llvm_Module) -> super::super::std_string_const {
            unsafe {
                super::super::raw::llvm_Module_getModuleInlineAsm(inst)
            }
        }

        pub fn getNamedValue(inst: *const super::super::llvm_Module, Name: &str) -> *mut super::super::llvm_GlobalValue {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_getNamedValue(inst, Name)
            }
        }

        pub fn getOrInsertFunction(inst: *mut super::super::llvm_Module, Name: &str, ty: *mut super::super::llvm_FunctionType) -> *mut super::super::llvm_Constant {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_getOrInsertFunction(inst, Name, ty)
            }
        }

        pub fn getTargetTriple(inst: *const super::super::llvm_Module) -> super::super::std_string_const {
            unsafe {
                super::super::raw::llvm_Module_getTargetTriple(inst)
            }
        }

        pub fn getTypeByName(inst: *const super::super::llvm_Module, Name: &str) -> *mut super::super::llvm_StructType {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_getTypeByName(inst, Name)
            }
        }

        pub fn new(ModuleID: &str, Context: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Module {
            let ModuleID = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(ModuleID.as_ptr()) },
                length: ModuleID.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_new(ModuleID, Context)
            }
        }

        pub fn setDataLayout(inst: *mut super::super::llvm_Module, Other: *const super::super::llvm_DataLayout) {
            unsafe {
                super::super::raw::llvm_Module_setDataLayout(inst, Other)
            }
        }

        pub fn setDataLayoutStr(inst: *mut super::super::llvm_Module, Desc: &str) {
            let Desc = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Desc.as_ptr()) },
                length: Desc.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_setDataLayoutStr(inst, Desc)
            }
        }

        pub fn setModuleIdentifier(inst: *mut super::super::llvm_Module, ID: &str) {
            let ID = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(ID.as_ptr()) },
                length: ID.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_setModuleIdentifier(inst, ID)
            }
        }

        pub fn setModuleInlineAsm(inst: *mut super::super::llvm_Module, Asm: &str) {
            let Asm = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Asm.as_ptr()) },
                length: Asm.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_setModuleInlineAsm(inst, Asm)
            }
        }

        pub fn setTargetTriple(inst: *mut super::super::llvm_Module, Triple: &str) {
            let Triple = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Triple.as_ptr()) },
                length: Triple.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Module_setTargetTriple(inst, Triple)
            }
        }
    }

    pub mod ModulePass {
    }

    pub mod Operator {
        pub fn getOpcode(inst: *const super::super::llvm_Operator) -> usize {
            unsafe {
                super::super::raw::llvm_Operator_getOpcode(inst) as usize
            }
        }
    }

    pub mod PHINode {
        pub fn addIncoming(inst: *mut super::super::llvm_PHINode, V: *mut super::super::llvm_Value, BB: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_PHINode_addIncoming(inst, V, BB)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_PHINode) {
            unsafe {
                super::super::raw::llvm_PHINode_delete(inst)
            }
        }

        pub fn getIncomingBlock(inst: *const super::super::llvm_PHINode, i: usize) -> *mut super::super::llvm_BasicBlock {
            let i = i as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_PHINode_getIncomingBlock(inst, i)
            }
        }

        pub fn getIncomingValue(inst: *const super::super::llvm_PHINode, i: usize) -> *mut super::super::llvm_Value {
            let i = i as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_PHINode_getIncomingValue(inst, i)
            }
        }

        pub fn getNumIncomingValues(inst: *const super::super::llvm_PHINode) -> usize {
            unsafe {
                super::super::raw::llvm_PHINode_getNumIncomingValues(inst) as usize
            }
        }

        pub fn setIncomingBlock(inst: *mut super::super::llvm_PHINode, i: usize, BB: *mut super::super::llvm_BasicBlock) {
            let i = i as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_PHINode_setIncomingBlock(inst, i, BB)
            }
        }

        pub fn setIncomingValue(inst: *mut super::super::llvm_PHINode, i: usize, V: *mut super::super::llvm_Value) {
            let i = i as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_PHINode_setIncomingValue(inst, i, V)
            }
        }
    }

    pub mod Pass {
        pub fn delete(inst: *mut super::super::llvm_Pass) {
            unsafe {
                super::super::raw::llvm_Pass_delete(inst)
            }
        }

        pub fn doFinalization(inst: *mut super::super::llvm_Pass, Module: *mut super::super::llvm_Module) -> bool {
            unsafe {
                super::super::raw::llvm_Pass_doFinalization(inst, Module) != 0
            }
        }

        pub fn doInitialization(inst: *mut super::super::llvm_Pass, Module: *mut super::super::llvm_Module) -> bool {
            unsafe {
                super::super::raw::llvm_Pass_doInitialization(inst, Module) != 0
            }
        }

        pub fn dump(inst: *const super::super::llvm_Pass) {
            unsafe {
                super::super::raw::llvm_Pass_dump(inst)
            }
        }

        pub fn getPassKind(inst: *const super::super::llvm_Pass) -> super::super::llvm_PassKind {
            unsafe {
                super::super::raw::llvm_Pass_getPassKind(inst)
            }
        }
    }

    pub mod PassManager {
        pub fn add(inst: *mut super::super::llvm_PassManager, Pass: *mut super::super::llvm_Pass) {
            unsafe {
                super::super::raw::llvm_PassManager_add(inst, Pass)
            }
        }

        pub fn new() -> *mut super::super::llvm_PassManager {
            unsafe {
                super::super::raw::llvm_PassManager_new()
            }
        }

        pub fn run(inst: *mut super::super::llvm_PassManager, Module: *mut super::super::llvm_Module) {
            unsafe {
                super::super::raw::llvm_PassManager_run(inst, Module)
            }
        }
    }

    pub mod PointerType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_PointerType_classof(ty) != 0
            }
        }

        pub fn get(ElementType: *mut super::super::llvm_Type, AddressSpace: usize) -> *mut super::super::llvm_PointerType {
            let AddressSpace = AddressSpace as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_PointerType_get(ElementType, AddressSpace)
            }
        }

        pub fn getAddressSpace(inst: *const super::super::llvm_PointerType) -> usize {
            unsafe {
                super::super::raw::llvm_PointerType_getAddressSpace(inst) as usize
            }
        }

        pub fn getUnqual(ElementType: *mut super::super::llvm_Type) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_PointerType_getUnqual(ElementType)
            }
        }

        pub fn isValidElementType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_PointerType_isValidElementType(ty) != 0
            }
        }
    }

    pub mod RegionPass {
    }

    pub mod ResumeInst {
    }

    pub mod ReturnInst {
    }

    pub mod SelectInst {
    }

    pub mod SequentialType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_SequentialType_classof(ty) != 0
            }
        }

        pub fn getElementType(inst: *const super::super::llvm_SequentialType) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_SequentialType_getElementType(inst)
            }
        }
    }

    pub mod ShuffleVectorInst {
    }

    pub mod StoreInst {
    }

    pub mod StructType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_classof(ty) != 0
            }
        }

        pub fn create(ctx: *mut super::super::llvm_LLVMContext, Elements: &[*mut super::super::llvm_Type], Name: &str) -> *mut super::super::llvm_StructType {
            let Elements = super::super::llvm_ArrayRef_ptr_llvm_Type {
                data: Elements.as_ptr(),
                size: Elements.len() as super::super::libc::size_t,
            };
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_StructType_create(ctx, Elements, Name)
            }
        }

        pub fn createPacked(ctx: *mut super::super::llvm_LLVMContext, Elements: &[*mut super::super::llvm_Type], Name: &str, isPacked: bool) -> *mut super::super::llvm_StructType {
            let Elements = super::super::llvm_ArrayRef_ptr_llvm_Type {
                data: Elements.as_ptr(),
                size: Elements.len() as super::super::libc::size_t,
            };
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            let isPacked = if isPacked { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_StructType_createPacked(ctx, Elements, Name, isPacked)
            }
        }

        pub fn getElementType(inst: *const super::super::llvm_StructType, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_StructType_getElementType(inst, idx)
            }
        }

        pub fn getName(inst: *const super::super::llvm_StructType) -> super::super::llvm_StringRef {
            unsafe {
                super::super::raw::llvm_StructType_getName(inst)
            }
        }

        pub fn getNumElements(inst: *const super::super::llvm_StructType) -> usize {
            unsafe {
                super::super::raw::llvm_StructType_getNumElements(inst) as usize
            }
        }

        pub fn hasName(inst: *const super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_hasName(inst) != 0
            }
        }

        pub fn isLayoutIdentical(inst: *const super::super::llvm_StructType, Other: *mut super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isLayoutIdentical(inst, Other) != 0
            }
        }

        pub fn isLiteral(inst: *const super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isLiteral(inst) != 0
            }
        }

        pub fn isOpaque(inst: *const super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isOpaque(inst) != 0
            }
        }

        pub fn isPacked(inst: *const super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isPacked(inst) != 0
            }
        }

        pub fn isSized(inst: *const super::super::llvm_StructType) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isSized(inst) != 0
            }
        }

        pub fn isValidElementType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_StructType_isValidElementType(ty) != 0
            }
        }

        pub fn setBody(inst: *mut super::super::llvm_StructType, Elements: &[*mut super::super::llvm_Type]) {
            let Elements = super::super::llvm_ArrayRef_ptr_llvm_Type {
                data: Elements.as_ptr(),
                size: Elements.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_StructType_setBody(inst, Elements)
            }
        }

        pub fn setBodyPacked(inst: *mut super::super::llvm_StructType, Elements: &[*mut super::super::llvm_Type], isPacked: bool) {
            let Elements = super::super::llvm_ArrayRef_ptr_llvm_Type {
                data: Elements.as_ptr(),
                size: Elements.len() as super::super::libc::size_t,
            };
            let isPacked = if isPacked { 1 } else { 0 };
            unsafe {
                super::super::raw::llvm_StructType_setBodyPacked(inst, Elements, isPacked)
            }
        }

        pub fn setName(inst: *mut super::super::llvm_StructType, Name: &str) {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_StructType_setName(inst, Name)
            }
        }
    }

    pub mod SwitchInst {
        pub fn addCase(inst: *mut super::super::llvm_SwitchInst, OnVal: *mut super::super::llvm_ConstantInt, Dest: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_SwitchInst_addCase(inst, OnVal, Dest)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_SwitchInst) {
            unsafe {
                super::super::raw::llvm_SwitchInst_delete(inst)
            }
        }

        pub fn getCondition(inst: *const super::super::llvm_SwitchInst) -> *mut super::super::llvm_Value {
            unsafe {
                super::super::raw::llvm_SwitchInst_getCondition(inst)
            }
        }

        pub fn getDefaultDest(inst: *const super::super::llvm_SwitchInst) -> *mut super::super::llvm_BasicBlock {
            unsafe {
                super::super::raw::llvm_SwitchInst_getDefaultDest(inst)
            }
        }

        pub fn getNumCases(inst: *const super::super::llvm_SwitchInst) -> usize {
            unsafe {
                super::super::raw::llvm_SwitchInst_getNumCases(inst) as usize
            }
        }

        pub fn setCondition(inst: *mut super::super::llvm_SwitchInst, V: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_SwitchInst_setCondition(inst, V)
            }
        }

        pub fn setDefaultDest(inst: *mut super::super::llvm_SwitchInst, DefaultCase: *mut super::super::llvm_BasicBlock) {
            unsafe {
                super::super::raw::llvm_SwitchInst_setDefaultDest(inst, DefaultCase)
            }
        }
    }

    pub mod TerminatorInst {
    }

    pub mod Type {
        pub fn dump(inst: *const super::super::llvm_Type) {
            unsafe {
                super::super::raw::llvm_Type_dump(inst)
            }
        }

        pub fn getContainedType(inst: *const super::super::llvm_Type, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getContainedType(inst, idx)
            }
        }

        pub fn getContext(inst: *const super::super::llvm_Type) -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_Type_getContext(inst)
            }
        }

        pub fn getDoublePtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getDoublePtrTy(ctx)
            }
        }

        pub fn getDoubleTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getDoubleTy(ctx)
            }
        }

        pub fn getFP128PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getFP128PtrTy(ctx)
            }
        }

        pub fn getFP128Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getFP128Ty(ctx)
            }
        }

        pub fn getFloatPtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getFloatPtrTy(ctx)
            }
        }

        pub fn getFloatTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getFloatTy(ctx)
            }
        }

        pub fn getFunctionNumParams(inst: *const super::super::llvm_Type) -> usize {
            unsafe {
                super::super::raw::llvm_Type_getFunctionNumParams(inst) as usize
            }
        }

        pub fn getFunctionParamType(inst: *const super::super::llvm_Type, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getFunctionParamType(inst, idx)
            }
        }

        pub fn getHalfPtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getHalfPtrTy(ctx)
            }
        }

        pub fn getHalfTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getHalfTy(ctx)
            }
        }

        pub fn getInt16PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getInt16PtrTy(ctx)
            }
        }

        pub fn getInt16Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_Type_getInt16Ty(ctx)
            }
        }

        pub fn getInt1PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getInt1PtrTy(ctx)
            }
        }

        pub fn getInt1Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_Type_getInt1Ty(ctx)
            }
        }

        pub fn getInt32PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getInt32PtrTy(ctx)
            }
        }

        pub fn getInt32Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_Type_getInt32Ty(ctx)
            }
        }

        pub fn getInt64PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getInt64PtrTy(ctx)
            }
        }

        pub fn getInt64Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_Type_getInt64Ty(ctx)
            }
        }

        pub fn getInt8PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getInt8PtrTy(ctx)
            }
        }

        pub fn getInt8Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_IntegerType {
            unsafe {
                super::super::raw::llvm_Type_getInt8Ty(ctx)
            }
        }

        pub fn getIntNPtrTy(ctx: *mut super::super::llvm_LLVMContext, size: usize) -> *mut super::super::llvm_PointerType {
            let size = size as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getIntNPtrTy(ctx, size)
            }
        }

        pub fn getIntNTy(ctx: *mut super::super::llvm_LLVMContext, size: usize) -> *mut super::super::llvm_IntegerType {
            let size = size as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getIntNTy(ctx, size)
            }
        }

        pub fn getLabelTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getLabelTy(ctx)
            }
        }

        pub fn getMetadataTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getMetadataTy(ctx)
            }
        }

        pub fn getNumContainedTypes(inst: *const super::super::llvm_Type) -> usize {
            unsafe {
                super::super::raw::llvm_Type_getNumContainedTypes(inst) as usize
            }
        }

        pub fn getPPC_FP128PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getPPC_FP128PtrTy(ctx)
            }
        }

        pub fn getPPC_FP128Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getPPC_FP128Ty(ctx)
            }
        }

        pub fn getPointerAddressSpace(inst: *const super::super::llvm_Type) -> usize {
            unsafe {
                super::super::raw::llvm_Type_getPointerAddressSpace(inst) as usize
            }
        }

        pub fn getPointerElementType(inst: *const super::super::llvm_Type) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getPointerElementType(inst)
            }
        }

        pub fn getPointerTo(inst: *mut super::super::llvm_Type, idx: usize) -> *mut super::super::llvm_PointerType {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getPointerTo(inst, idx)
            }
        }

        pub fn getSequentialElementType(inst: *const super::super::llvm_Type) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getSequentialElementType(inst)
            }
        }

        pub fn getStructElementType(inst: *const super::super::llvm_Type, idx: usize) -> *mut super::super::llvm_Type {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Type_getStructElementType(inst, idx)
            }
        }

        pub fn getStructName(inst: *const super::super::llvm_Type) -> super::super::llvm_StringRef {
            unsafe {
                super::super::raw::llvm_Type_getStructName(inst)
            }
        }

        pub fn getStructNumElements(inst: *const super::super::llvm_Type) -> usize {
            unsafe {
                super::super::raw::llvm_Type_getStructNumElements(inst) as usize
            }
        }

        pub fn getTypeID(inst: *const super::super::llvm_Type) -> super::super::llvm_Type_TypeID {
            unsafe {
                super::super::raw::llvm_Type_getTypeID(inst)
            }
        }

        pub fn getVoidTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getVoidTy(ctx)
            }
        }

        pub fn getX86_FP80PtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getX86_FP80PtrTy(ctx)
            }
        }

        pub fn getX86_FP80Ty(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getX86_FP80Ty(ctx)
            }
        }

        pub fn getX86_MMXPtrTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_PointerType {
            unsafe {
                super::super::raw::llvm_Type_getX86_MMXPtrTy(ctx)
            }
        }

        pub fn getX86_MMXTy(ctx: *mut super::super::llvm_LLVMContext) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Type_getX86_MMXTy(ctx)
            }
        }

        pub fn isAggregateType(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isAggregateType(inst) != 0
            }
        }

        pub fn isArrayTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isArrayTy(inst) != 0
            }
        }

        pub fn isDoubleTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isDoubleTy(inst) != 0
            }
        }

        pub fn isEmptyTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isEmptyTy(inst) != 0
            }
        }

        pub fn isFP128Ty(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFP128Ty(inst) != 0
            }
        }

        pub fn isFPOrFPVectorTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFPOrFPVectorTy(inst) != 0
            }
        }

        pub fn isFirstClassType(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFirstClassType(inst) != 0
            }
        }

        pub fn isFloatTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFloatTy(inst) != 0
            }
        }

        pub fn isFloatingPointTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFloatingPointTy(inst) != 0
            }
        }

        pub fn isFunctionTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFunctionTy(inst) != 0
            }
        }

        pub fn isFunctionVarArg(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isFunctionVarArg(inst) != 0
            }
        }

        pub fn isHalfTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isHalfTy(inst) != 0
            }
        }

        pub fn isIntOrIntVectorTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isIntOrIntVectorTy(inst) != 0
            }
        }

        pub fn isIntegerTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isIntegerTy(inst) != 0
            }
        }

        pub fn isLabelTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isLabelTy(inst) != 0
            }
        }

        pub fn isMetadataTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isMetadataTy(inst) != 0
            }
        }

        pub fn isPPC_FP128Ty(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isPPC_FP128Ty(inst) != 0
            }
        }

        pub fn isPointerTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isPointerTy(inst) != 0
            }
        }

        pub fn isPtrOrPtrVectorTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isPtrOrPtrVectorTy(inst) != 0
            }
        }

        pub fn isSingleValueType(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isSingleValueType(inst) != 0
            }
        }

        pub fn isSized(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isSized(inst) != 0
            }
        }

        pub fn isStructTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isStructTy(inst) != 0
            }
        }

        pub fn isVectorTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isVectorTy(inst) != 0
            }
        }

        pub fn isVoidTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isVoidTy(inst) != 0
            }
        }

        pub fn isX86_FP80Ty(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isX86_FP80Ty(inst) != 0
            }
        }

        pub fn isX86_MMXTy(inst: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_Type_isX86_MMXTy(inst) != 0
            }
        }
    }

    pub mod UnaryInstruction {
    }

    pub mod UndefValue {
    }

    pub mod UnreachableInst {
    }

    pub mod Use {
        pub fn get(inst: *const super::super::llvm_Use) -> *mut super::super::llvm_Value {
            unsafe {
                super::super::raw::llvm_Use_get(inst)
            }
        }

        pub fn getNext(inst: *const super::super::llvm_Use) -> *mut super::super::llvm_Use {
            unsafe {
                super::super::raw::llvm_Use_getNext(inst)
            }
        }

        pub fn getOperandNo(inst: *const super::super::llvm_Use) -> usize {
            unsafe {
                super::super::raw::llvm_Use_getOperandNo(inst) as usize
            }
        }

        pub fn getUser(inst: *const super::super::llvm_Use) -> *mut super::super::llvm_User {
            unsafe {
                super::super::raw::llvm_Use_getUser(inst)
            }
        }

        pub fn initTags(Start: *mut super::super::llvm_Use, Stop: *mut super::super::llvm_Use) -> *mut super::super::llvm_Use {
            unsafe {
                super::super::raw::llvm_Use_initTags(Start, Stop)
            }
        }

        pub fn set(inst: *mut super::super::llvm_Use, Val: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_Use_set(inst, Val)
            }
        }

        pub fn swap(inst: *mut super::super::llvm_Use, RHS: *mut super::super::llvm_Use) {
            unsafe {
                super::super::raw::llvm_Use_swap(inst, RHS)
            }
        }
    }

    pub mod User {
        pub fn classof(V: *mut super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_User_classof(V) != 0
            }
        }

        pub fn delete(inst: *mut super::super::llvm_User) {
            unsafe {
                super::super::raw::llvm_User_delete(inst)
            }
        }

        pub fn dropAllReferences(inst: *mut super::super::llvm_User) {
            unsafe {
                super::super::raw::llvm_User_dropAllReferences(inst)
            }
        }

        pub fn getNumOperands(inst: *const super::super::llvm_User) -> usize {
            unsafe {
                super::super::raw::llvm_User_getNumOperands(inst) as usize
            }
        }

        pub fn getOperand(inst: *const super::super::llvm_User, idx: usize) -> *mut super::super::llvm_Value {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_User_getOperand(inst, idx)
            }
        }

        pub fn replaceUsesOfWith(inst: *mut super::super::llvm_User, From: *mut super::super::llvm_Value, To: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_User_replaceUsesOfWith(inst, From, To)
            }
        }

        pub fn setOperand(inst: *mut super::super::llvm_User, idx: usize, Val: *mut super::super::llvm_Value) {
            let idx = idx as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_User_setOperand(inst, idx, Val)
            }
        }
    }

    pub mod VAArgInst {
    }

    pub mod Value {
        pub fn delete(inst: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_Value_delete(inst)
            }
        }

        pub fn dump(inst: *const super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_Value_dump(inst)
            }
        }

        pub fn getContext(inst: *const super::super::llvm_Value) -> *mut super::super::llvm_LLVMContext {
            unsafe {
                super::super::raw::llvm_Value_getContext(inst)
            }
        }

        pub fn getName(inst: *const super::super::llvm_Value) -> super::super::llvm_StringRef {
            unsafe {
                super::super::raw::llvm_Value_getName(inst)
            }
        }

        pub fn getNumUses(inst: *const super::super::llvm_Value) -> usize {
            unsafe {
                super::super::raw::llvm_Value_getNumUses(inst) as usize
            }
        }

        pub fn getType(inst: *const super::super::llvm_Value) -> *mut super::super::llvm_Type {
            unsafe {
                super::super::raw::llvm_Value_getType(inst)
            }
        }

        pub fn getValueID(inst: *const super::super::llvm_Value) -> usize {
            unsafe {
                super::super::raw::llvm_Value_getValueID(inst) as usize
            }
        }

        pub fn hasNUses(inst: *const super::super::llvm_Value, N: usize) -> bool {
            let N = N as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Value_hasNUses(inst, N) != 0
            }
        }

        pub fn hasNUsesOrMore(inst: *const super::super::llvm_Value, N: usize) -> bool {
            let N = N as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_Value_hasNUsesOrMore(inst, N) != 0
            }
        }

        pub fn hasName(inst: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_Value_hasName(inst) != 0
            }
        }

        pub fn hasOneUse(inst: *const super::super::llvm_Value) -> bool {
            unsafe {
                super::super::raw::llvm_Value_hasOneUse(inst) != 0
            }
        }

        pub fn isUsedInBasicBlock(inst: *const super::super::llvm_Value, BB: *const super::super::llvm_BasicBlock) -> bool {
            unsafe {
                super::super::raw::llvm_Value_isUsedInBasicBlock(inst, BB) != 0
            }
        }

        pub fn mutateType(inst: *mut super::super::llvm_Value, ty: *mut super::super::llvm_Type) {
            unsafe {
                super::super::raw::llvm_Value_mutateType(inst, ty)
            }
        }

        pub fn replaceAllUsesWith(inst: *mut super::super::llvm_Value, Value: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_Value_replaceAllUsesWith(inst, Value)
            }
        }

        pub fn setName(inst: *mut super::super::llvm_Value, Name: &str) {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_Value_setName(inst, Name)
            }
        }

        pub fn takeName(inst: *mut super::super::llvm_Value, Value: *mut super::super::llvm_Value) {
            unsafe {
                super::super::raw::llvm_Value_takeName(inst, Value)
            }
        }
    }

    pub mod ValueSymbolTable {
        pub fn delete(inst: *mut super::super::llvm_ValueSymbolTable) {
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_delete(inst)
            }
        }

        pub fn dump(inst: *const super::super::llvm_ValueSymbolTable) {
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_dump(inst)
            }
        }

        pub fn empty(inst: *const super::super::llvm_ValueSymbolTable) -> bool {
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_empty(inst) != 0
            }
        }

        pub fn lookup(inst: *const super::super::llvm_ValueSymbolTable, Name: &str) -> *mut super::super::llvm_Value {
            let Name = super::super::llvm_StringRef {
                data: unsafe { ::std::mem::transmute(Name.as_ptr()) },
                length: Name.len() as super::super::libc::size_t,
            };
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_lookup(inst, Name)
            }
        }

        pub fn new() -> *mut super::super::llvm_ValueSymbolTable {
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_new()
            }
        }

        pub fn size(inst: *const super::super::llvm_ValueSymbolTable) -> usize {
            unsafe {
                super::super::raw::llvm_ValueSymbolTable_size(inst) as usize
            }
        }
    }

    pub mod VectorType {
        pub fn classof(ty: *const super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_VectorType_classof(ty) != 0
            }
        }

        pub fn get(ty: *mut super::super::llvm_Type, NumElements: usize) -> *mut super::super::llvm_VectorType {
            let NumElements = NumElements as super::super::libc::c_uint;
            unsafe {
                super::super::raw::llvm_VectorType_get(ty, NumElements)
            }
        }

        pub fn getBitWidth(inst: *const super::super::llvm_VectorType) -> usize {
            unsafe {
                super::super::raw::llvm_VectorType_getBitWidth(inst) as usize
            }
        }

        pub fn getDoubleElementsVectorType(ty: *mut super::super::llvm_VectorType) -> *mut super::super::llvm_VectorType {
            unsafe {
                super::super::raw::llvm_VectorType_getDoubleElementsVectorType(ty)
            }
        }

        pub fn getExtendedElementVectorType(ty: *mut super::super::llvm_VectorType) -> *mut super::super::llvm_VectorType {
            unsafe {
                super::super::raw::llvm_VectorType_getExtendedElementVectorType(ty)
            }
        }

        pub fn getHalfElementsVectorType(ty: *mut super::super::llvm_VectorType) -> *mut super::super::llvm_VectorType {
            unsafe {
                super::super::raw::llvm_VectorType_getHalfElementsVectorType(ty)
            }
        }

        pub fn getInteger(ty: *mut super::super::llvm_VectorType) -> *mut super::super::llvm_VectorType {
            unsafe {
                super::super::raw::llvm_VectorType_getInteger(ty)
            }
        }

        pub fn getNumElements(inst: *const super::super::llvm_VectorType) -> usize {
            unsafe {
                super::super::raw::llvm_VectorType_getNumElements(inst) as usize
            }
        }

        pub fn getTruncatedElementVectorType(ty: *mut super::super::llvm_VectorType) -> *mut super::super::llvm_VectorType {
            unsafe {
                super::super::raw::llvm_VectorType_getTruncatedElementVectorType(ty)
            }
        }

        pub fn isValidElementType(ty: *mut super::super::llvm_Type) -> bool {
            unsafe {
                super::super::raw::llvm_VectorType_isValidElementType(ty) != 0
            }
        }
    }

    pub mod iplist_llvm_Argument {
        pub fn clear(inst: *mut super::super::llvm_iplist_llvm_Argument) {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_clear(inst)
            }
        }

        pub fn delete(inst: *mut super::super::llvm_iplist_llvm_Argument) {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_delete(inst)
            }
        }

        pub fn first(inst: *const super::super::llvm_iplist_llvm_Argument) -> *const super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_first(inst)
            }
        }

        pub fn firstMut(inst: *mut super::super::llvm_iplist_llvm_Argument) -> *mut super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_firstMut(inst)
            }
        }

        pub fn last(inst: *const super::super::llvm_iplist_llvm_Argument) -> *const super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_last(inst)
            }
        }

        pub fn lastMut(inst: *mut super::super::llvm_iplist_llvm_Argument) -> *mut super::super::llvm_Argument {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_lastMut(inst)
            }
        }

        pub fn max_size(inst: *const super::super::llvm_iplist_llvm_Argument) -> usize {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_max_size(inst) as usize
            }
        }

        pub fn new() -> *mut super::super::llvm_iplist_llvm_Argument {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_new()
            }
        }

        pub fn size(inst: *const super::super::llvm_iplist_llvm_Argument) -> usize {
            unsafe {
                super::super::raw::llvm_iplist_llvm_Argument_size(inst) as usize
            }
        }
    }

    pub fn createAddDiscriminatorsPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createAddDiscriminatorsPass()
        }
    }

    pub fn createAddressSanitizerFunctionPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createAddressSanitizerFunctionPass()
        }
    }

    pub fn createAddressSanitizerModulePass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createAddressSanitizerModulePass()
        }
    }

    pub fn createAggressiveDCEPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createAggressiveDCEPass()
        }
    }

    pub fn createAlwaysInlinerPass(InsertLifetime: Option<bool>) -> *mut super::llvm_Pass {
        let opt_hack_367948 = InsertLifetime.map(|value| if value { 1 } else { 0 });
        let InsertLifetime = opt_hack_367948.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createAlwaysInlinerPass(InsertLifetime)
        }
    }

    pub fn createArgumentPromotionPass(maxElements: Option<usize>) -> *mut super::llvm_Pass {
        let opt_hack_398720 = maxElements.map(|value| value as super::libc::c_uint);
        let maxElements = opt_hack_398720.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createArgumentPromotionPass(maxElements)
        }
    }

    pub fn createBarrierNoopPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createBarrierNoopPass()
        }
    }

    pub fn createBlockExtractorPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createBlockExtractorPass()
        }
    }

    pub fn createBoundsCheckingPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createBoundsCheckingPass()
        }
    }

    pub fn createBreakCriticalEdgesPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createBreakCriticalEdgesPass()
        }
    }

    pub fn createCFGSimplificationPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createCFGSimplificationPass()
        }
    }

    pub fn createConstantHoistingPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createConstantHoistingPass()
        }
    }

    pub fn createConstantMergePass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createConstantMergePass()
        }
    }

    pub fn createConstantPropagationPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createConstantPropagationPass()
        }
    }

    pub fn createCorrelatedValuePropagationPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createCorrelatedValuePropagationPass()
        }
    }

    pub fn createDataFlowSanitizerPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createDataFlowSanitizerPass()
        }
    }

    pub fn createDeadArgEliminationPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createDeadArgEliminationPass()
        }
    }

    pub fn createDeadArgHackingPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createDeadArgHackingPass()
        }
    }

    pub fn createDeadCodeEliminationPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createDeadCodeEliminationPass()
        }
    }

    pub fn createDeadInstEliminationPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createDeadInstEliminationPass()
        }
    }

    pub fn createDeadStoreEliminationPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createDeadStoreEliminationPass()
        }
    }

    pub fn createDebugIRPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createDebugIRPass()
        }
    }

    pub fn createDemoteRegisterToMemoryPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createDemoteRegisterToMemoryPass()
        }
    }

    pub fn createEarlyCSEPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createEarlyCSEPass()
        }
    }

    pub fn createFlattenCFGPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createFlattenCFGPass()
        }
    }

    pub fn createFunctionAttrsPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createFunctionAttrsPass()
        }
    }

    pub fn createFunctionInliningPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createFunctionInliningPass()
        }
    }

    pub fn createFunctionInliningPassWithOptLevel(OptLevel: usize, SizeOptLevel: usize) -> *mut super::llvm_Pass {
        let OptLevel = OptLevel as super::libc::c_uint;
        let SizeOptLevel = SizeOptLevel as super::libc::c_uint;
        unsafe {
            super::raw::llvm_createFunctionInliningPassWithOptLevel(OptLevel, SizeOptLevel)
        }
    }

    pub fn createGCOVProfilerPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createGCOVProfilerPass()
        }
    }

    pub fn createGVNPass(NoLoads: Option<bool>) -> *mut super::llvm_FunctionPass {
        let opt_hack_834650 = NoLoads.map(|value| if value { 1 } else { 0 });
        let NoLoads = opt_hack_834650.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createGVNPass(NoLoads)
        }
    }

    pub fn createGlobalDCEPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createGlobalDCEPass()
        }
    }

    pub fn createGlobalMergePass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createGlobalMergePass()
        }
    }

    pub fn createGlobalOptimizerPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createGlobalOptimizerPass()
        }
    }

    pub fn createIPConstantPropagationPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createIPConstantPropagationPass()
        }
    }

    pub fn createIPSCCPPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createIPSCCPPass()
        }
    }

    pub fn createIndVarSimplifyPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createIndVarSimplifyPass()
        }
    }

    pub fn createInstructionCombiningPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createInstructionCombiningPass()
        }
    }

    pub fn createInstructionNamerPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createInstructionNamerPass()
        }
    }

    pub fn createInstructionSimplifierPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createInstructionSimplifierPass()
        }
    }

    pub fn createInternalizePass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createInternalizePass()
        }
    }

    pub fn createJumpThreadingPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createJumpThreadingPass()
        }
    }

    pub fn createLCSSAPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLCSSAPass()
        }
    }

    pub fn createLICMPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLICMPass()
        }
    }

    pub fn createLoadCombinePass() -> *mut super::llvm_BasicBlockPass {
        unsafe {
            super::raw::llvm_createLoadCombinePass()
        }
    }

    pub fn createLoopDeletionPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopDeletionPass()
        }
    }

    pub fn createLoopExtractorPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopExtractorPass()
        }
    }

    pub fn createLoopIdiomPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopIdiomPass()
        }
    }

    pub fn createLoopInstSimplifyPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopInstSimplifyPass()
        }
    }

    pub fn createLoopRerollPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopRerollPass()
        }
    }

    pub fn createLoopRotatePass(MaxHeaderSize: Option<isize>) -> *mut super::llvm_Pass {
        let opt_hack_534157 = MaxHeaderSize.map(|value| value as super::libc::c_int);
        let MaxHeaderSize = opt_hack_534157.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createLoopRotatePass(MaxHeaderSize)
        }
    }

    pub fn createLoopSimplifyPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopSimplifyPass()
        }
    }

    pub fn createLoopStrengthReducePass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopStrengthReducePass()
        }
    }

    pub fn createLoopUnrollPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLoopUnrollPass()
        }
    }

    pub fn createLoopUnswitchPass(OptimizeForSize: Option<bool>) -> *mut super::llvm_Pass {
        let opt_hack_790824 = OptimizeForSize.map(|value| if value { 1 } else { 0 });
        let OptimizeForSize = opt_hack_790824.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createLoopUnswitchPass(OptimizeForSize)
        }
    }

    pub fn createLowerAtomicPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createLowerAtomicPass()
        }
    }

    pub fn createLowerExpectIntrinsicPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createLowerExpectIntrinsicPass()
        }
    }

    pub fn createLowerInvokePass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createLowerInvokePass()
        }
    }

    pub fn createLowerSwitchPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createLowerSwitchPass()
        }
    }

    pub fn createMemCpyOptPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createMemCpyOptPass()
        }
    }

    pub fn createMemorySanitizerPass(TrackOrigins: Option<isize>) -> *mut super::llvm_FunctionPass {
        let opt_hack_649743 = TrackOrigins.map(|value| value as super::libc::c_int);
        let TrackOrigins = opt_hack_649743.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createMemorySanitizerPass(TrackOrigins)
        }
    }

    pub fn createMergeFunctionsPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createMergeFunctionsPass()
        }
    }

    pub fn createMergedLoadStoreMotionPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createMergedLoadStoreMotionPass()
        }
    }

    pub fn createMetaRenamerPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createMetaRenamerPass()
        }
    }

    pub fn createObjCARCAPElimPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createObjCARCAPElimPass()
        }
    }

    pub fn createObjCARCContractPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createObjCARCContractPass()
        }
    }

    pub fn createObjCARCExpandPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createObjCARCExpandPass()
        }
    }

    pub fn createObjCARCOptPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createObjCARCOptPass()
        }
    }

    pub fn createPartialInliningPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createPartialInliningPass()
        }
    }

    pub fn createPartiallyInlineLibCallsPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createPartiallyInlineLibCallsPass()
        }
    }

    pub fn createPromoteMemoryToRegisterPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createPromoteMemoryToRegisterPass()
        }
    }

    pub fn createPruneEHPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createPruneEHPass()
        }
    }

    pub fn createReassociatePass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createReassociatePass()
        }
    }

    pub fn createSCCPPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createSCCPPass()
        }
    }

    pub fn createSROAPass(RequiresDomTree: Option<bool>) -> *mut super::llvm_FunctionPass {
        let opt_hack_610466 = RequiresDomTree.map(|value| if value { 1 } else { 0 });
        let RequiresDomTree = opt_hack_610466.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createSROAPass(RequiresDomTree)
        }
    }

    pub fn createSampleProfileLoaderPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createSampleProfileLoaderPass()
        }
    }

    pub fn createScalarReplAggregatesPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createScalarReplAggregatesPass()
        }
    }

    pub fn createScalarizerPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createScalarizerPass()
        }
    }

    pub fn createSeparateConstOffsetFromGEPPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createSeparateConstOffsetFromGEPPass()
        }
    }

    pub fn createSimpleLoopUnrollPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createSimpleLoopUnrollPass()
        }
    }

    pub fn createSingleLoopExtractorPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createSingleLoopExtractorPass()
        }
    }

    pub fn createSinkingPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createSinkingPass()
        }
    }

    pub fn createStripDeadDebugInfoPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createStripDeadDebugInfoPass()
        }
    }

    pub fn createStripDeadPrototypesPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createStripDeadPrototypesPass()
        }
    }

    pub fn createStripDebugDeclarePass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createStripDebugDeclarePass()
        }
    }

    pub fn createStripNonDebugSymbolsPass() -> *mut super::llvm_ModulePass {
        unsafe {
            super::raw::llvm_createStripNonDebugSymbolsPass()
        }
    }

    pub fn createStripSymbolsPass(OnlyDebugInfo: Option<bool>) -> *mut super::llvm_ModulePass {
        let opt_hack_192717 = OnlyDebugInfo.map(|value| if value { 1 } else { 0 });
        let OnlyDebugInfo = opt_hack_192717.as_ref().map(|value| value as *const _).unwrap_or(::std::ptr::null());
        unsafe {
            super::raw::llvm_createStripSymbolsPass(OnlyDebugInfo)
        }
    }

    pub fn createStructurizeCFGPass() -> *mut super::llvm_Pass {
        unsafe {
            super::raw::llvm_createStructurizeCFGPass()
        }
    }

    pub fn createTailCallEliminationPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createTailCallEliminationPass()
        }
    }

    pub fn createThreadSanitizerPass() -> *mut super::llvm_FunctionPass {
        unsafe {
            super::raw::llvm_createThreadSanitizerPass()
        }
    }

    pub fn getGlobalContext() -> *mut super::llvm_LLVMContext {
        unsafe {
            super::raw::llvm_getGlobalContext()
        }
    }

    pub fn verifyFunction(Function: *const super::llvm_Function) -> bool {
        unsafe {
            super::raw::llvm_verifyFunction(Function) != 0
        }
    }

    pub fn verifyModule(Module: *const super::llvm_Module) -> bool {
        unsafe {
            super::raw::llvm_verifyModule(Module) != 0
        }
    }
}
