from .prelude import *

AnalysisID = ptr(Void, const=True)


@Pass.body
class Pass:
    delete = Destructor()

    getPassKind = Method(llvm['PassKind'], const=True)
    # getPassName
    # getPassID = Method(AnalysisID, const=True)

    doInitialization = Method(Bool, (ref(Module), 'Module'))
    doFinalization = Method(Bool, (ref(Module), 'Module'))

    dump = Method(const=True)


def CreatePassFunction(ty, *args, **kwargs):
    func = ast.Function(ptr(ty, owned=True), *args, **kwargs)

    return func


@llvm.body
class llvm_body:
    createGCOVProfilerPass = CreatePassFunction(ModulePass)
    createAddressSanitizerFunctionPass = CreatePassFunction(FunctionPass)
    createAddressSanitizerModulePass = CreatePassFunction(ModulePass)
    createMemorySanitizerPass = CreatePassFunction(
        FunctionPass, (Option(Int, '0'), 'TrackOrigins'))

    createThreadSanitizerPass = CreatePassFunction(FunctionPass)
    createDataFlowSanitizerPass = CreatePassFunction(ModulePass)
    createBoundsCheckingPass = CreatePassFunction(FunctionPass)
    # Removed in LLVM 3.6
    # createDebugIRPass = CreatePassFunction(ModulePass)
    createStripSymbolsPass = CreatePassFunction(
        ModulePass, (Option(Bool, 'false'), 'OnlyDebugInfo'))
    createStripNonDebugSymbolsPass = CreatePassFunction(ModulePass)
    createStripDebugDeclarePass = CreatePassFunction(ModulePass)
    createStripDeadDebugInfoPass = CreatePassFunction(ModulePass)
    createConstantMergePass = CreatePassFunction(ModulePass)
    createGlobalOptimizerPass = CreatePassFunction(ModulePass)
    createGlobalDCEPass = CreatePassFunction(ModulePass)
    # createGVExtractionPass = CreatePassFunction(ModulePass)
    createFunctionInliningPass = CreatePassFunction(Pass)
    createFunctionInliningPassWithOptLevel = CreatePassFunction(
        Pass, (UnsignedInt, 'OptLevel'), (UnsignedInt, 'SizeOptLevel')).with_real_name('createFunctionInliningPass')
    createAlwaysInlinerPass = CreatePassFunction(
        Pass, (Option(Bool, 'false'), 'InsertLifetime'))
    createPruneEHPass = CreatePassFunction(Pass)
    createInternalizePass = CreatePassFunction(ModulePass)
    createDeadArgEliminationPass = CreatePassFunction(ModulePass)
    createDeadArgHackingPass = CreatePassFunction(ModulePass)
    createArgumentPromotionPass = CreatePassFunction(
        Pass, (Option(UnsignedInt, '3'), 'maxElements'))
    createIPConstantPropagationPass = CreatePassFunction(ModulePass)
    createIPSCCPPass = CreatePassFunction(ModulePass)
    createLoopExtractorPass = CreatePassFunction(Pass)
    createSingleLoopExtractorPass = CreatePassFunction(Pass)
    createBlockExtractorPass = CreatePassFunction(ModulePass)
    createStripDeadPrototypesPass = CreatePassFunction(ModulePass)
    createFunctionAttrsPass = CreatePassFunction(Pass)
    createMergeFunctionsPass = CreatePassFunction(ModulePass)
    createPartialInliningPass = CreatePassFunction(ModulePass)
    createMetaRenamerPass = CreatePassFunction(ModulePass)
    createBarrierNoopPass = CreatePassFunction(ModulePass)
    createObjCARCAPElimPass = CreatePassFunction(Pass)
    createObjCARCExpandPass = CreatePassFunction(Pass)
    createObjCARCContractPass = CreatePassFunction(Pass)
    createObjCARCOptPass = CreatePassFunction(Pass)
    createConstantPropagationPass = CreatePassFunction(FunctionPass)
    createSCCPPass = CreatePassFunction(FunctionPass)
    createDeadInstEliminationPass = CreatePassFunction(Pass)
    createDeadCodeEliminationPass = CreatePassFunction(FunctionPass)
    createDeadStoreEliminationPass = CreatePassFunction(FunctionPass)
    createAggressiveDCEPass = CreatePassFunction(FunctionPass)
    createSROAPass = CreatePassFunction(
        FunctionPass, (Option(Bool, 'true'), 'RequiresDomTree'))
    # FunctionPass        *createScalarReplAggregatesPass (signed
    # Threshold=-1, bool UseDomTree=true, signed StructMemberThreshold=-1,
    # signed ArrayElementThreshold=-1, signed ScalarLoadThreshold=-1)
    createScalarReplAggregatesPass = CreatePassFunction(FunctionPass)
    createIndVarSimplifyPass = CreatePassFunction(Pass)
    createInstructionCombiningPass = CreatePassFunction(FunctionPass)
    createLICMPass = CreatePassFunction(Pass)
    createLoopStrengthReducePass = CreatePassFunction(Pass)
    createGlobalMergePass = CreatePassFunction(Pass)
    createLoopUnswitchPass = CreatePassFunction(
        Pass, (Option(Bool, 'false'), 'OptimizeForSize'))
    createLoopInstSimplifyPass = CreatePassFunction(Pass)
    # Pass                *createLoopUnrollPass (int Threshold=-1, int
    # Count=-1, int AllowPartial=-1, int Runtime=-1)
    createLoopUnrollPass = CreatePassFunction(Pass)
    createSimpleLoopUnrollPass = CreatePassFunction(Pass)
    createLoopRerollPass = CreatePassFunction(Pass)
    createLoopRotatePass = CreatePassFunction(
        Pass, (Option(Int, '-1'), 'MaxHeaderSize'))
    createLoopIdiomPass = CreatePassFunction(Pass)
    createPromoteMemoryToRegisterPass = CreatePassFunction(FunctionPass)
    createDemoteRegisterToMemoryPass = CreatePassFunction(FunctionPass)
    createReassociatePass = CreatePassFunction(FunctionPass)
    createJumpThreadingPass = CreatePassFunction(FunctionPass)
    createCFGSimplificationPass = CreatePassFunction(FunctionPass)
    createFlattenCFGPass = CreatePassFunction(FunctionPass)
    createStructurizeCFGPass = CreatePassFunction(Pass)
    createBreakCriticalEdgesPass = CreatePassFunction(FunctionPass)
    createLoopSimplifyPass = CreatePassFunction(Pass)
    createTailCallEliminationPass = CreatePassFunction(FunctionPass)
    createLowerSwitchPass = CreatePassFunction(FunctionPass)
    createLowerInvokePass = CreatePassFunction(FunctionPass)
    createLCSSAPass = CreatePassFunction(Pass)
    createEarlyCSEPass = CreatePassFunction(FunctionPass)
    createMergedLoadStoreMotionPass = CreatePassFunction(FunctionPass)
    createGVNPass = CreatePassFunction(
        FunctionPass, (Option(Bool, 'false'), 'NoLoads'))
    createMemCpyOptPass = CreatePassFunction(FunctionPass)
    createLoopDeletionPass = CreatePassFunction(Pass)
    createConstantHoistingPass = CreatePassFunction(FunctionPass)
    createInstructionNamerPass = CreatePassFunction(FunctionPass)
    createSinkingPass = CreatePassFunction(FunctionPass)
    createLowerAtomicPass = CreatePassFunction(Pass)
    createCorrelatedValuePropagationPass = CreatePassFunction(Pass)
    createInstructionSimplifierPass = CreatePassFunction(FunctionPass)
    createLowerExpectIntrinsicPass = CreatePassFunction(FunctionPass)
    createPartiallyInlineLibCallsPass = CreatePassFunction(FunctionPass)
    createSampleProfileLoaderPass = CreatePassFunction(FunctionPass)
    createScalarizerPass = CreatePassFunction(FunctionPass)
    createAddDiscriminatorsPass = CreatePassFunction(FunctionPass)
    createSeparateConstOffsetFromGEPPass = CreatePassFunction(FunctionPass)
    createLoadCombinePass = CreatePassFunction(BasicBlockPass)
