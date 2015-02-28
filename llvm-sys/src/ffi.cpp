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
#include "ffi.h"

extern "C"
llvm_FunctionPass* llvm_createAddDiscriminatorsPass()
{
    return ::llvm::createAddDiscriminatorsPass();
}

extern "C"
llvm_FunctionPass* llvm_createAddressSanitizerFunctionPass()
{
    return ::llvm::createAddressSanitizerFunctionPass();
}

extern "C"
llvm_ModulePass* llvm_createAddressSanitizerModulePass()
{
    return ::llvm::createAddressSanitizerModulePass();
}

extern "C"
llvm_FunctionPass* llvm_createAggressiveDCEPass()
{
    return ::llvm::createAggressiveDCEPass();
}

extern "C"
llvm_Pass* llvm_createAlwaysInlinerPass(int* _InsertLifetime)
{
    bool InsertLifetime = false;
    if (_InsertLifetime != NULL){
        InsertLifetime = ((*_InsertLifetime) == 1 ? true : false);
    }
    return ::llvm::createAlwaysInlinerPass(InsertLifetime);
}

extern "C"
llvm_Pass* llvm_createArgumentPromotionPass(unsigned int* _maxElements)
{
    unsigned int maxElements = 3;
    if (_maxElements != NULL){
        maxElements = *_maxElements;
    }
    return ::llvm::createArgumentPromotionPass(maxElements);
}

extern "C"
llvm_ModulePass* llvm_createBarrierNoopPass()
{
    return ::llvm::createBarrierNoopPass();
}

extern "C"
llvm_ModulePass* llvm_createBlockExtractorPass()
{
    return ::llvm::createBlockExtractorPass();
}

extern "C"
llvm_FunctionPass* llvm_createBoundsCheckingPass()
{
    return ::llvm::createBoundsCheckingPass();
}

extern "C"
llvm_FunctionPass* llvm_createBreakCriticalEdgesPass()
{
    return ::llvm::createBreakCriticalEdgesPass();
}

extern "C"
llvm_FunctionPass* llvm_createCFGSimplificationPass()
{
    return ::llvm::createCFGSimplificationPass();
}

extern "C"
llvm_FunctionPass* llvm_createConstantHoistingPass()
{
    return ::llvm::createConstantHoistingPass();
}

extern "C"
llvm_ModulePass* llvm_createConstantMergePass()
{
    return ::llvm::createConstantMergePass();
}

extern "C"
llvm_FunctionPass* llvm_createConstantPropagationPass()
{
    return ::llvm::createConstantPropagationPass();
}

extern "C"
llvm_Pass* llvm_createCorrelatedValuePropagationPass()
{
    return ::llvm::createCorrelatedValuePropagationPass();
}

extern "C"
llvm_ModulePass* llvm_createDataFlowSanitizerPass()
{
    return ::llvm::createDataFlowSanitizerPass();
}

extern "C"
llvm_ModulePass* llvm_createDeadArgEliminationPass()
{
    return ::llvm::createDeadArgEliminationPass();
}

extern "C"
llvm_ModulePass* llvm_createDeadArgHackingPass()
{
    return ::llvm::createDeadArgHackingPass();
}

extern "C"
llvm_FunctionPass* llvm_createDeadCodeEliminationPass()
{
    return ::llvm::createDeadCodeEliminationPass();
}

extern "C"
llvm_Pass* llvm_createDeadInstEliminationPass()
{
    return ::llvm::createDeadInstEliminationPass();
}

extern "C"
llvm_FunctionPass* llvm_createDeadStoreEliminationPass()
{
    return ::llvm::createDeadStoreEliminationPass();
}

extern "C"
llvm_ModulePass* llvm_createDebugIRPass()
{
    return ::llvm::createDebugIRPass();
}

extern "C"
llvm_FunctionPass* llvm_createDemoteRegisterToMemoryPass()
{
    return ::llvm::createDemoteRegisterToMemoryPass();
}

extern "C"
llvm_FunctionPass* llvm_createEarlyCSEPass()
{
    return ::llvm::createEarlyCSEPass();
}

extern "C"
llvm_FunctionPass* llvm_createFlattenCFGPass()
{
    return ::llvm::createFlattenCFGPass();
}

extern "C"
llvm_Pass* llvm_createFunctionAttrsPass()
{
    return ::llvm::createFunctionAttrsPass();
}

extern "C"
llvm_Pass* llvm_createFunctionInliningPass()
{
    return ::llvm::createFunctionInliningPass();
}

extern "C"
llvm_Pass* llvm_createFunctionInliningPassWithOptLevel(unsigned int OptLevel, unsigned int SizeOptLevel)
{
    return ::llvm::createFunctionInliningPass(OptLevel, SizeOptLevel);
}

extern "C"
llvm_ModulePass* llvm_createGCOVProfilerPass()
{
    return ::llvm::createGCOVProfilerPass();
}

extern "C"
llvm_FunctionPass* llvm_createGVNPass(int* _NoLoads)
{
    bool NoLoads = false;
    if (_NoLoads != NULL){
        NoLoads = ((*_NoLoads) == 1 ? true : false);
    }
    return ::llvm::createGVNPass(NoLoads);
}

extern "C"
llvm_ModulePass* llvm_createGlobalDCEPass()
{
    return ::llvm::createGlobalDCEPass();
}

extern "C"
llvm_Pass* llvm_createGlobalMergePass()
{
    return ::llvm::createGlobalMergePass();
}

extern "C"
llvm_ModulePass* llvm_createGlobalOptimizerPass()
{
    return ::llvm::createGlobalOptimizerPass();
}

extern "C"
llvm_ModulePass* llvm_createIPConstantPropagationPass()
{
    return ::llvm::createIPConstantPropagationPass();
}

extern "C"
llvm_ModulePass* llvm_createIPSCCPPass()
{
    return ::llvm::createIPSCCPPass();
}

extern "C"
llvm_Pass* llvm_createIndVarSimplifyPass()
{
    return ::llvm::createIndVarSimplifyPass();
}

extern "C"
llvm_FunctionPass* llvm_createInstructionCombiningPass()
{
    return ::llvm::createInstructionCombiningPass();
}

extern "C"
llvm_FunctionPass* llvm_createInstructionNamerPass()
{
    return ::llvm::createInstructionNamerPass();
}

extern "C"
llvm_FunctionPass* llvm_createInstructionSimplifierPass()
{
    return ::llvm::createInstructionSimplifierPass();
}

extern "C"
llvm_ModulePass* llvm_createInternalizePass()
{
    return ::llvm::createInternalizePass();
}

extern "C"
llvm_FunctionPass* llvm_createJumpThreadingPass()
{
    return ::llvm::createJumpThreadingPass();
}

extern "C"
llvm_Pass* llvm_createLCSSAPass()
{
    return ::llvm::createLCSSAPass();
}

extern "C"
llvm_Pass* llvm_createLICMPass()
{
    return ::llvm::createLICMPass();
}

extern "C"
llvm_BasicBlockPass* llvm_createLoadCombinePass()
{
    return ::llvm::createLoadCombinePass();
}

extern "C"
llvm_Pass* llvm_createLoopDeletionPass()
{
    return ::llvm::createLoopDeletionPass();
}

extern "C"
llvm_Pass* llvm_createLoopExtractorPass()
{
    return ::llvm::createLoopExtractorPass();
}

extern "C"
llvm_Pass* llvm_createLoopIdiomPass()
{
    return ::llvm::createLoopIdiomPass();
}

extern "C"
llvm_Pass* llvm_createLoopInstSimplifyPass()
{
    return ::llvm::createLoopInstSimplifyPass();
}

extern "C"
llvm_Pass* llvm_createLoopRerollPass()
{
    return ::llvm::createLoopRerollPass();
}

extern "C"
llvm_Pass* llvm_createLoopRotatePass(int* _MaxHeaderSize)
{
    int MaxHeaderSize = -1;
    if (_MaxHeaderSize != NULL){
        MaxHeaderSize = *_MaxHeaderSize;
    }
    return ::llvm::createLoopRotatePass(MaxHeaderSize);
}

extern "C"
llvm_Pass* llvm_createLoopSimplifyPass()
{
    return ::llvm::createLoopSimplifyPass();
}

extern "C"
llvm_Pass* llvm_createLoopStrengthReducePass()
{
    return ::llvm::createLoopStrengthReducePass();
}

extern "C"
llvm_Pass* llvm_createLoopUnrollPass()
{
    return ::llvm::createLoopUnrollPass();
}

extern "C"
llvm_Pass* llvm_createLoopUnswitchPass(int* _OptimizeForSize)
{
    bool OptimizeForSize = false;
    if (_OptimizeForSize != NULL){
        OptimizeForSize = ((*_OptimizeForSize) == 1 ? true : false);
    }
    return ::llvm::createLoopUnswitchPass(OptimizeForSize);
}

extern "C"
llvm_Pass* llvm_createLowerAtomicPass()
{
    return ::llvm::createLowerAtomicPass();
}

extern "C"
llvm_FunctionPass* llvm_createLowerExpectIntrinsicPass()
{
    return ::llvm::createLowerExpectIntrinsicPass();
}

extern "C"
llvm_FunctionPass* llvm_createLowerInvokePass()
{
    return ::llvm::createLowerInvokePass();
}

extern "C"
llvm_FunctionPass* llvm_createLowerSwitchPass()
{
    return ::llvm::createLowerSwitchPass();
}

extern "C"
llvm_FunctionPass* llvm_createMemCpyOptPass()
{
    return ::llvm::createMemCpyOptPass();
}

extern "C"
llvm_FunctionPass* llvm_createMemorySanitizerPass(int* _TrackOrigins)
{
    int TrackOrigins = 0;
    if (_TrackOrigins != NULL){
        TrackOrigins = *_TrackOrigins;
    }
    return ::llvm::createMemorySanitizerPass(TrackOrigins);
}

extern "C"
llvm_ModulePass* llvm_createMergeFunctionsPass()
{
    return ::llvm::createMergeFunctionsPass();
}

extern "C"
llvm_FunctionPass* llvm_createMergedLoadStoreMotionPass()
{
    return ::llvm::createMergedLoadStoreMotionPass();
}

extern "C"
llvm_ModulePass* llvm_createMetaRenamerPass()
{
    return ::llvm::createMetaRenamerPass();
}

extern "C"
llvm_Pass* llvm_createObjCARCAPElimPass()
{
    return ::llvm::createObjCARCAPElimPass();
}

extern "C"
llvm_Pass* llvm_createObjCARCContractPass()
{
    return ::llvm::createObjCARCContractPass();
}

extern "C"
llvm_Pass* llvm_createObjCARCExpandPass()
{
    return ::llvm::createObjCARCExpandPass();
}

extern "C"
llvm_Pass* llvm_createObjCARCOptPass()
{
    return ::llvm::createObjCARCOptPass();
}

extern "C"
llvm_ModulePass* llvm_createPartialInliningPass()
{
    return ::llvm::createPartialInliningPass();
}

extern "C"
llvm_FunctionPass* llvm_createPartiallyInlineLibCallsPass()
{
    return ::llvm::createPartiallyInlineLibCallsPass();
}

extern "C"
llvm_FunctionPass* llvm_createPromoteMemoryToRegisterPass()
{
    return ::llvm::createPromoteMemoryToRegisterPass();
}

extern "C"
llvm_Pass* llvm_createPruneEHPass()
{
    return ::llvm::createPruneEHPass();
}

extern "C"
llvm_FunctionPass* llvm_createReassociatePass()
{
    return ::llvm::createReassociatePass();
}

extern "C"
llvm_FunctionPass* llvm_createSCCPPass()
{
    return ::llvm::createSCCPPass();
}

extern "C"
llvm_FunctionPass* llvm_createSROAPass(int* _RequiresDomTree)
{
    bool RequiresDomTree = true;
    if (_RequiresDomTree != NULL){
        RequiresDomTree = ((*_RequiresDomTree) == 1 ? true : false);
    }
    return ::llvm::createSROAPass(RequiresDomTree);
}

extern "C"
llvm_FunctionPass* llvm_createSampleProfileLoaderPass()
{
    return ::llvm::createSampleProfileLoaderPass();
}

extern "C"
llvm_FunctionPass* llvm_createScalarReplAggregatesPass()
{
    return ::llvm::createScalarReplAggregatesPass();
}

extern "C"
llvm_FunctionPass* llvm_createScalarizerPass()
{
    return ::llvm::createScalarizerPass();
}

extern "C"
llvm_FunctionPass* llvm_createSeparateConstOffsetFromGEPPass()
{
    return ::llvm::createSeparateConstOffsetFromGEPPass();
}

extern "C"
llvm_Pass* llvm_createSimpleLoopUnrollPass()
{
    return ::llvm::createSimpleLoopUnrollPass();
}

extern "C"
llvm_Pass* llvm_createSingleLoopExtractorPass()
{
    return ::llvm::createSingleLoopExtractorPass();
}

extern "C"
llvm_FunctionPass* llvm_createSinkingPass()
{
    return ::llvm::createSinkingPass();
}

extern "C"
llvm_ModulePass* llvm_createStripDeadDebugInfoPass()
{
    return ::llvm::createStripDeadDebugInfoPass();
}

extern "C"
llvm_ModulePass* llvm_createStripDeadPrototypesPass()
{
    return ::llvm::createStripDeadPrototypesPass();
}

extern "C"
llvm_ModulePass* llvm_createStripDebugDeclarePass()
{
    return ::llvm::createStripDebugDeclarePass();
}

extern "C"
llvm_ModulePass* llvm_createStripNonDebugSymbolsPass()
{
    return ::llvm::createStripNonDebugSymbolsPass();
}

extern "C"
llvm_ModulePass* llvm_createStripSymbolsPass(int* _OnlyDebugInfo)
{
    bool OnlyDebugInfo = false;
    if (_OnlyDebugInfo != NULL){
        OnlyDebugInfo = ((*_OnlyDebugInfo) == 1 ? true : false);
    }
    return ::llvm::createStripSymbolsPass(OnlyDebugInfo);
}

extern "C"
llvm_Pass* llvm_createStructurizeCFGPass()
{
    return ::llvm::createStructurizeCFGPass();
}

extern "C"
llvm_FunctionPass* llvm_createTailCallEliminationPass()
{
    return ::llvm::createTailCallEliminationPass();
}

extern "C"
llvm_FunctionPass* llvm_createThreadSanitizerPass()
{
    return ::llvm::createThreadSanitizerPass();
}

extern "C"
llvm_LLVMContext* llvm_getGlobalContext()
{
    return &::llvm::getGlobalContext();
}

extern "C"
int llvm_verifyFunction(llvm_Function const* Function)
{
    return (::llvm::verifyFunction(*Function) ? 1 : 0);
}

extern "C"
int llvm_verifyModule(llvm_Module const* Module)
{
    return (::llvm::verifyModule(*Module) ? 1 : 0);
}

extern "C"
unsigned int llvm_Argument_getArgNo(llvm_Argument const* self)
{
    return self->getArgNo();
}

extern "C"
llvm_Function const* llvm_Argument_getParent(llvm_Argument const* self)
{
    return self->getParent();
}

extern "C"
llvm_Function* llvm_Argument_getParentMut(llvm_Argument* self)
{
    return self->getParent();
}

extern "C"
llvm_Argument* llvm_Argument_new(llvm_Type* Ty, std_string_const* _Name, llvm_Function** _F)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    ::llvm::Function* F = NULL;
    if (_F != NULL){
        F = *_F;
    }
    return new(std::nothrow) ::llvm::Argument(Ty, Name, F);
}

extern "C"
llvm_Argument const* llvm_Argument_next(llvm_Argument const* self)
{
    llvm::Function::const_arg_iterator it = self;
    return ++it;
}

extern "C"
llvm_Argument* llvm_Argument_nextMut(llvm_Argument* self)
{
    llvm::Function::arg_iterator it = self;
    return ++it;
}

extern "C"
llvm_Argument const* llvm_Argument_prev(llvm_Argument const* self)
{
    llvm::Function::const_arg_iterator it = self;
    return ++it;
}

extern "C"
llvm_Argument* llvm_Argument_prevMut(llvm_Argument* self)
{
    llvm::Function::arg_iterator it = self;
    return ++it;
}

extern "C"
int llvm_ArrayType_classof(llvm_Type const* ty)
{
    return (::llvm::ArrayType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_ArrayType* llvm_ArrayType_get(llvm_Type* ElementType, uint64_t NumElements)
{
    return ::llvm::ArrayType::get(ElementType, NumElements);
}

extern "C"
uint64_t llvm_ArrayType_getNumElements(llvm_ArrayType const* self)
{
    return self->getNumElements();
}

extern "C"
int llvm_ArrayType_isValidElementType(llvm_Type* ty)
{
    return (::llvm::ArrayType::isValidElementType(ty) ? 1 : 0);
}

extern "C"
llvm_BasicBlock* llvm_BasicBlock_Create(llvm_LLVMContext* Context, std_string* _Name, llvm_Function** _Parent, llvm_BasicBlock** _InsertBefore)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    ::llvm::Function* Parent = NULL;
    if (_Parent != NULL){
        Parent = *_Parent;
    }
    ::llvm::BasicBlock* InsertBefore = NULL;
    if (_InsertBefore != NULL){
        InsertBefore = *_InsertBefore;
    }
    return ::llvm::BasicBlock::Create(*Context, Name, Parent, InsertBefore);
}

extern "C"
int llvm_BasicBlock_classof(llvm_Value const* Val)
{
    return (::llvm::BasicBlock::classof(Val) ? 1 : 0);
}

extern "C"
void llvm_BasicBlock_delete(llvm_BasicBlock* self)
{
    delete self;
}

extern "C"
void llvm_BasicBlock_dropAllReferences(llvm_BasicBlock* self)
{
    self->dropAllReferences();
}

extern "C"
void llvm_BasicBlock_eraseFromParent(llvm_BasicBlock* self)
{
    self->eraseFromParent();
}

extern "C"
llvm_DataLayout const* llvm_BasicBlock_getDataLayout(llvm_BasicBlock const* self)
{
    return self->getDataLayout();
}

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHI(llvm_BasicBlock const* self)
{
    return self->getFirstNonPHI();
}

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIMut(llvm_BasicBlock* self)
{
    return self->getFirstNonPHI();
}

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbg(llvm_BasicBlock const* self)
{
    return self->getFirstNonPHIOrDbg();
}

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgMut(llvm_BasicBlock* self)
{
    return self->getFirstNonPHIOrDbg();
}

extern "C"
llvm_Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(llvm_BasicBlock const* self)
{
    return self->getFirstNonPHIOrDbgOrLifetime();
}

extern "C"
llvm_Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(llvm_BasicBlock* self)
{
    return self->getFirstNonPHIOrDbgOrLifetime();
}

extern "C"
llvm_LandingPadInst const* llvm_BasicBlock_getLandingPadInst(llvm_BasicBlock const* self)
{
    return self->getLandingPadInst();
}

extern "C"
llvm_LandingPadInst* llvm_BasicBlock_getLandingPadInstMut(llvm_BasicBlock* self)
{
    return self->getLandingPadInst();
}

extern "C"
llvm_Function const* llvm_BasicBlock_getParent(llvm_BasicBlock const* self)
{
    return self->getParent();
}

extern "C"
llvm_Function* llvm_BasicBlock_getParentMut(llvm_BasicBlock* self)
{
    return self->getParent();
}

extern "C"
llvm_BasicBlock const* llvm_BasicBlock_getSinglePredecessor(llvm_BasicBlock const* self)
{
    return self->getSinglePredecessor();
}

extern "C"
llvm_BasicBlock* llvm_BasicBlock_getSinglePredecessorMut(llvm_BasicBlock* self)
{
    return self->getSinglePredecessor();
}

extern "C"
llvm_TerminatorInst const* llvm_BasicBlock_getTerminator(llvm_BasicBlock const* self)
{
    return self->getTerminator();
}

extern "C"
llvm_TerminatorInst* llvm_BasicBlock_getTerminatorMut(llvm_BasicBlock* self)
{
    return self->getTerminator();
}

extern "C"
llvm_BasicBlock const* llvm_BasicBlock_getUniquePredecessor(llvm_BasicBlock const* self)
{
    return self->getUniquePredecessor();
}

extern "C"
llvm_BasicBlock* llvm_BasicBlock_getUniquePredecessorMut(llvm_BasicBlock* self)
{
    return self->getUniquePredecessor();
}

extern "C"
llvm_ValueSymbolTable* llvm_BasicBlock_getValueSymbolTable(llvm_BasicBlock* self)
{
    return self->getValueSymbolTable();
}

extern "C"
int llvm_BasicBlock_hasAddressTaken(llvm_BasicBlock const* self)
{
    return (self->hasAddressTaken() ? 1 : 0);
}

extern "C"
int llvm_BasicBlock_isLandingPad(llvm_BasicBlock const* self)
{
    return (self->isLandingPad() ? 1 : 0);
}

extern "C"
void llvm_BasicBlock_moveAfter(llvm_BasicBlock* self, llvm_BasicBlock* MovePos)
{
    self->moveAfter(MovePos);
}

extern "C"
void llvm_BasicBlock_moveBefore(llvm_BasicBlock* self, llvm_BasicBlock* MovePos)
{
    self->moveBefore(MovePos);
}

extern "C"
void llvm_BasicBlock_removeFromParent(llvm_BasicBlock* self)
{
    self->removeFromParent();
}

extern "C"
void llvm_BasicBlock_removePredecessor(llvm_BasicBlock* self, llvm_BasicBlock* Pred, int* _DontDeleteUselessPHIs)
{
    bool DontDeleteUselessPHIs = false;
    if (_DontDeleteUselessPHIs != NULL){
        DontDeleteUselessPHIs = ((*_DontDeleteUselessPHIs) == 1 ? true : false);
    }
    self->removePredecessor(Pred, DontDeleteUselessPHIs);
}

extern "C"
void llvm_BasicBlock_replaceSuccessorsPhiUsesWith(llvm_BasicBlock* self, llvm_BasicBlock* New)
{
    self->replaceSuccessorsPhiUsesWith(New);
}

extern "C"
void llvm_BlockAddress_destroyConstant(llvm_BlockAddress* self)
{
    self->destroyConstant();
}

extern "C"
llvm_BasicBlock* llvm_BlockAddress_getBasicBlock(llvm_BlockAddress const* self)
{
    return self->getBasicBlock();
}

extern "C"
llvm_Function* llvm_BlockAddress_getFunction(llvm_BlockAddress const* self)
{
    return self->getFunction();
}

extern "C"
int llvm_CompositeType_classof(llvm_Type const* ty)
{
    return (::llvm::CompositeType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_Type* llvm_CompositeType_getTypeAtIndex(llvm_CompositeType* self, unsigned int idx)
{
    return self->getTypeAtIndex(idx);
}

extern "C"
int llvm_CompositeType_indexValid(llvm_CompositeType const* self, unsigned int idx)
{
    return (self->indexValid(idx) ? 1 : 0);
}

extern "C"
int llvm_ConstantArray_classof(llvm_Value const* V)
{
    return (::llvm::ConstantArray::classof(V) ? 1 : 0);
}

extern "C"
llvm_Constant* llvm_ConstantArray_get(llvm_ArrayType* Ty, llvm_ArrayRef_ptr_llvm_Constant _Values)
{
    llvm::ArrayRef<::llvm::Constant*> Values(_Values.data, _Values.size);
    return ::llvm::ConstantArray::get(Ty, Values);
}

extern "C"
llvm_Type* llvm_ConstantArray_getType(llvm_ConstantArray const* self)
{
    return self->getType();
}

extern "C"
int llvm_ConstantFP_classof(llvm_Value const* V)
{
    return (::llvm::ConstantFP::classof(V) ? 1 : 0);
}

extern "C"
llvm_Constant* llvm_ConstantFP_fromStr(llvm_Type* Ty, llvm_StringRef _Val)
{
    llvm::StringRef Val(_Val.data, _Val.length);
    return ::llvm::ConstantFP::get(Ty, Val);
}

extern "C"
llvm_Constant* llvm_ConstantFP_get(llvm_Type* Ty, double Val)
{
    return ::llvm::ConstantFP::get(Ty, Val);
}

extern "C"
llvm_Constant* llvm_ConstantFP_getInfinity(llvm_Type* Ty)
{
    return ::llvm::ConstantFP::getInfinity(Ty);
}

extern "C"
llvm_Constant* llvm_ConstantFP_getNegativeZero(llvm_Type* Ty)
{
    return ::llvm::ConstantFP::getNegativeZero(Ty);
}

extern "C"
llvm_Constant* llvm_ConstantFP_getZeroValueForNegation(llvm_Type* Ty)
{
    return ::llvm::ConstantFP::getZeroValueForNegation(Ty);
}

extern "C"
int llvm_ConstantFP_isExactlyValueFloat(llvm_ConstantFP const* self, double Val)
{
    return (self->isExactlyValue(Val) ? 1 : 0);
}

extern "C"
int llvm_ConstantFP_isNaN(llvm_ConstantFP const* self)
{
    return (self->isNaN() ? 1 : 0);
}

extern "C"
int llvm_ConstantFP_isNegative(llvm_ConstantFP const* self)
{
    return (self->isNegative() ? 1 : 0);
}

extern "C"
int llvm_ConstantFP_isZero(llvm_ConstantFP const* self)
{
    return (self->isZero() ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_classof(llvm_Value const* Val)
{
    return (::llvm::ConstantInt::classof(Val) ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_equalsInt(llvm_ConstantInt const* self, uint64_t Val)
{
    return (self->equalsInt(Val) ? 1 : 0);
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_fromAPInt(llvm_LLVMContext* Context, llvm_APInt _Val)
{
    llvm::ArrayRef<uint64_t> __Val_data(_Val.data.data, _Val.data.size);
    llvm::APInt Val(_Val.numbits, __Val_data);
    return ::llvm::ConstantInt::get(*Context, Val);
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_fromStr(llvm_IntegerType* Ty, llvm_StringRef _Str, uint8_t radix)
{
    llvm::StringRef Str(_Str.data, _Str.length);
    return ::llvm::ConstantInt::get(Ty, Str, radix);
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_get(llvm_IntegerType* Ty, uint64_t Value)
{
    return ::llvm::ConstantInt::get(Ty, Value);
}

extern "C"
unsigned int llvm_ConstantInt_getBitWidth(llvm_ConstantInt const* self)
{
    return self->getBitWidth();
}

extern "C"
llvm_Constant* llvm_ConstantInt_getFalse(llvm_Type* Ty)
{
    return ::llvm::ConstantInt::getFalse(Ty);
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getFalseWithContext(llvm_LLVMContext* Context)
{
    return ::llvm::ConstantInt::getFalse(*Context);
}

extern "C"
int64_t llvm_ConstantInt_getSExtValue(llvm_ConstantInt const* self)
{
    return self->getSExtValue();
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getSigned(llvm_IntegerType* Ty, uint64_t Value, int isSigned)
{
    return ::llvm::ConstantInt::get(Ty, Value, (isSigned == 1 ? true : false));
}

extern "C"
llvm_Constant* llvm_ConstantInt_getTrue(llvm_Type* Ty)
{
    return ::llvm::ConstantInt::getTrue(Ty);
}

extern "C"
llvm_ConstantInt* llvm_ConstantInt_getTrueWithContext(llvm_LLVMContext* Context)
{
    return ::llvm::ConstantInt::getTrue(*Context);
}

extern "C"
llvm_IntegerType* llvm_ConstantInt_getType(llvm_ConstantInt const* self)
{
    return self->getType();
}

extern "C"
uint64_t llvm_ConstantInt_getZExtValue(llvm_ConstantInt const* self)
{
    return self->getZExtValue();
}

extern "C"
int llvm_ConstantInt_isMaxValue(llvm_ConstantInt const* self, int isSigned)
{
    return (self->isMaxValue((isSigned == 1 ? true : false)) ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isMinValue(llvm_ConstantInt const* self, int isSigned)
{
    return (self->isMinValue((isSigned == 1 ? true : false)) ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isMinusOne(llvm_ConstantInt const* self)
{
    return (self->isMinusOne() ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isNegative(llvm_ConstantInt const* self)
{
    return (self->isNegative() ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isOne(llvm_ConstantInt const* self)
{
    return (self->isOne() ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isSignedValueValidForType(llvm_Type* Ty, int64_t Val)
{
    return (::llvm::ConstantInt::isValueValidForType(Ty, Val) ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isValueValidForType(llvm_Type* Ty, uint64_t Val)
{
    return (::llvm::ConstantInt::isValueValidForType(Ty, Val) ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_isZero(llvm_ConstantInt const* self)
{
    return (self->isZero() ? 1 : 0);
}

extern "C"
int llvm_ConstantInt_uge(llvm_ConstantInt const* self, uint64_t Num)
{
    return (self->uge(Num) ? 1 : 0);
}

extern "C"
int llvm_ConstantPointerNull_classof(llvm_Value const* Val)
{
    return (::llvm::ConstantPointerNull::classof(Val) ? 1 : 0);
}

extern "C"
void llvm_ConstantPointerNull_destroyConstant(llvm_ConstantPointerNull* self)
{
    self->destroyConstant();
}

extern "C"
llvm_ConstantPointerNull* llvm_ConstantPointerNull_get(llvm_PointerType* Ty)
{
    return ::llvm::ConstantPointerNull::get(Ty);
}

extern "C"
llvm_PointerType* llvm_ConstantPointerNull_getType(llvm_ConstantPointerNull const* self)
{
    return self->getType();
}

extern "C"
int llvm_Constant_canTrap(llvm_Constant const* self)
{
    return (self->canTrap() ? 1 : 0);
}

extern "C"
int llvm_Constant_classof(llvm_Value const* V)
{
    return (::llvm::Constant::classof(V) ? 1 : 0);
}

extern "C"
void llvm_Constant_destroyConstant(llvm_Constant* self)
{
    self->destroyConstant();
}

extern "C"
llvm_Constant* llvm_Constant_getAggregateElement(llvm_Constant const* self, unsigned int Elt)
{
    return self->getAggregateElement(Elt);
}

extern "C"
llvm_Constant* llvm_Constant_getAggregateElementConstant(llvm_Constant const* self, llvm_Constant* Elt)
{
    return self->getAggregateElement(Elt);
}

extern "C"
llvm_Constant* llvm_Constant_getAllOnesValue(llvm_Type* Ty)
{
    return ::llvm::Constant::getAllOnesValue(Ty);
}

extern "C"
llvm_Constant* llvm_Constant_getIntegerValue(llvm_Type* Ty, llvm_APInt _Value)
{
    llvm::ArrayRef<uint64_t> __Value_data(_Value.data.data, _Value.data.size);
    llvm::APInt Value(_Value.numbits, __Value_data);
    return ::llvm::Constant::getIntegerValue(Ty, Value);
}

extern "C"
llvm_Constant* llvm_Constant_getNullValue(llvm_Type* Ty)
{
    return ::llvm::Constant::getNullValue(Ty);
}

extern "C"
llvm_Constant* llvm_Constant_getSplatValue(llvm_Constant const* self)
{
    return self->getSplatValue();
}

extern "C"
int llvm_Constant_isAllOnesValue(llvm_Constant const* self)
{
    return (self->isAllOnesValue() ? 1 : 0);
}

extern "C"
int llvm_Constant_isConstantUsed(llvm_Constant const* self)
{
    return (self->isConstantUsed() ? 1 : 0);
}

extern "C"
int llvm_Constant_isDLLImportDependent(llvm_Constant const* self)
{
    return (self->isDLLImportDependent() ? 1 : 0);
}

extern "C"
int llvm_Constant_isMinSignedValue(llvm_Constant const* self)
{
    return (self->isMinSignedValue() ? 1 : 0);
}

extern "C"
int llvm_Constant_isNegativeZeroValue(llvm_Constant const* self)
{
    return (self->isNegativeZeroValue() ? 1 : 0);
}

extern "C"
int llvm_Constant_isNullValue(llvm_Constant const* self)
{
    return (self->isNullValue() ? 1 : 0);
}

extern "C"
int llvm_Constant_isThreadDependent(llvm_Constant const* self)
{
    return (self->isThreadDependent() ? 1 : 0);
}

extern "C"
int llvm_Constant_isZeroValue(llvm_Constant const* self)
{
    return (self->isZeroValue() ? 1 : 0);
}

extern "C"
void llvm_Constant_removeDeadConstantUsers(llvm_Constant const* self)
{
    self->removeDeadConstantUsers();
}

extern "C"
void llvm_Constant_replaceUsesOfWithOnConstant(llvm_Constant* self, llvm_Value* arg_1, llvm_Value* arg_2, llvm_Use* arg_3)
{
    self->replaceUsesOfWithOnConstant(arg_1, arg_2, arg_3);
}

extern "C"
llvm_Constant const* llvm_Constant_stripPointerCasts(llvm_Constant const* self)
{
    return self->stripPointerCasts();
}

extern "C"
llvm_Constant* llvm_Constant_stripPointerCastsMut(llvm_Constant* self)
{
    return self->stripPointerCasts();
}

extern "C"
void llvm_FunctionPassManager_add(llvm_FunctionPassManager* self, llvm_FunctionPass* Pass)
{
    self->add(Pass);
}

extern "C"
int llvm_FunctionPassManager_doFinalization(llvm_FunctionPassManager* self)
{
    return (self->doFinalization() ? 1 : 0);
}

extern "C"
int llvm_FunctionPassManager_doInitialization(llvm_FunctionPassManager* self)
{
    return (self->doInitialization() ? 1 : 0);
}

extern "C"
llvm_FunctionPassManager* llvm_FunctionPassManager_new(llvm_Module* Module)
{
    return new(std::nothrow) ::llvm::FunctionPassManager(Module);
}

extern "C"
void llvm_FunctionPassManager_run(llvm_FunctionPassManager* self, llvm_Function* Function)
{
    self->run(*Function);
}

extern "C"
int llvm_FunctionType_classof(llvm_Type const* ty)
{
    return (::llvm::FunctionType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_FunctionType* llvm_FunctionType_get(llvm_Type* Result, llvm_ArrayRef_ptr_llvm_Type _Params, int isVarArg)
{
    llvm::ArrayRef<::llvm::Type*> Params(_Params.data, _Params.size);
    return ::llvm::FunctionType::get(Result, Params, (isVarArg == 1 ? true : false));
}

extern "C"
unsigned int llvm_FunctionType_getNumParams(llvm_FunctionType const* self)
{
    return self->getNumParams();
}

extern "C"
llvm_Type* llvm_FunctionType_getParamType(llvm_FunctionType const* self, unsigned int idx)
{
    return self->getParamType(idx);
}

extern "C"
llvm_Type* llvm_FunctionType_getReturnType(llvm_FunctionType const* self)
{
    return self->getReturnType();
}

extern "C"
int llvm_FunctionType_isValidArgumentType(llvm_Type* ty)
{
    return (::llvm::FunctionType::isValidArgumentType(ty) ? 1 : 0);
}

extern "C"
int llvm_FunctionType_isValidReturnType(llvm_Type* ty)
{
    return (::llvm::FunctionType::isValidReturnType(ty) ? 1 : 0);
}

extern "C"
int llvm_FunctionType_isVarArg(llvm_FunctionType const* self)
{
    return (self->isVarArg() ? 1 : 0);
}

extern "C"
llvm_Function* llvm_Function_Create(llvm_FunctionType* Ty, llvm_GlobalValue_LinkageTypes Linkage, std_string* _Name, llvm_Module** _Module)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    ::llvm::Module* Module = NULL;
    if (_Module != NULL){
        Module = *_Module;
    }
    return ::llvm::Function::Create(Ty, Linkage, Name, Module);
}

extern "C"
void llvm_Function_addFnAttr(llvm_Function* self, llvm_StringRef _Kind, llvm_StringRef* _Val)
{
    llvm::StringRef Kind(_Kind.data, _Kind.length);
    llvm::StringRef Val = "";
    if (_Val != NULL){
        llvm::StringRef __Val_value((*_Val).data, (*_Val).length);
        Val = __Val_value;
    }
    self->addFnAttr(Kind, Val);
}

extern "C"
int llvm_Function_cannotDuplicate(llvm_Function const* self)
{
    return (self->cannotDuplicate() ? 1 : 0);
}

extern "C"
int llvm_Function_classof(llvm_Value const* Val)
{
    return (::llvm::Function::classof(Val) ? 1 : 0);
}

extern "C"
void llvm_Function_clearGC(llvm_Function* self)
{
    self->clearGC();
}

extern "C"
void llvm_Function_copyAttributesFrom(llvm_Function* self, llvm_GlobalValue* Src)
{
    self->copyAttributesFrom(Src);
}

extern "C"
void llvm_Function_delete(llvm_Function* self)
{
    delete self;
}

extern "C"
void llvm_Function_deleteBody(llvm_Function* self)
{
    self->deleteBody();
}

extern "C"
int llvm_Function_doesNotAccessMemory(llvm_Function const* self)
{
    return (self->doesNotAccessMemory() ? 1 : 0);
}

extern "C"
int llvm_Function_doesNotAccessMemoryParam(llvm_Function const* self, unsigned int n)
{
    return (self->doesNotAccessMemory(n) ? 1 : 0);
}

extern "C"
int llvm_Function_doesNotAlias(llvm_Function const* self, unsigned int n)
{
    return (self->doesNotAlias(n) ? 1 : 0);
}

extern "C"
int llvm_Function_doesNotCapture(llvm_Function const* self, unsigned int n)
{
    return (self->doesNotCapture(n) ? 1 : 0);
}

extern "C"
int llvm_Function_doesNotReturn(llvm_Function const* self)
{
    return (self->doesNotReturn() ? 1 : 0);
}

extern "C"
int llvm_Function_doesNotThrow(llvm_Function const* self)
{
    return (self->doesNotThrow() ? 1 : 0);
}

extern "C"
void llvm_Function_eraseFromParent(llvm_Function* self)
{
    self->eraseFromParent();
}

extern "C"
llvm_iplist_llvm_Argument const* llvm_Function_getArgumentList(llvm_Function const* self)
{
    return &self->getArgumentList();
}

extern "C"
llvm_iplist_llvm_Argument* llvm_Function_getArgumentListMut(llvm_Function* self)
{
    return &self->getArgumentList();
}

extern "C"
llvm_CallingConv_ID llvm_Function_getCallingConv(llvm_Function const* self)
{
    return self->getCallingConv();
}

extern "C"
llvm_LLVMContext* llvm_Function_getContext(llvm_Function const* self)
{
    return &self->getContext();
}

extern "C"
uint64_t llvm_Function_getDereferenceableBytes(llvm_Function const* self, unsigned int idx)
{
    return self->getDereferenceableBytes(idx);
}

extern "C"
llvm_BasicBlock const* llvm_Function_getEntryBlock(llvm_Function const* self)
{
    return &self->getEntryBlock();
}

extern "C"
llvm_BasicBlock* llvm_Function_getEntryBlockMut(llvm_Function* self)
{
    return &self->getEntryBlock();
}

extern "C"
llvm_Argument const* llvm_Function_getFirstArg(llvm_Function const* self)
{
    return self->arg_begin();
}

extern "C"
llvm_Argument* llvm_Function_getFirstArgMut(llvm_Function* self)
{
    return self->arg_begin();
}

extern "C"
llvm_FunctionType* llvm_Function_getFunctionType(llvm_Function const* self)
{
    return self->getFunctionType();
}

extern "C"
unsigned int llvm_Function_getIntrinsicID(llvm_Function const* self)
{
    return self->getIntrinsicID();
}

extern "C"
unsigned int llvm_Function_getParamAlignment(llvm_Function const* self, unsigned int idx)
{
    return self->getParamAlignment(idx);
}

extern "C"
llvm_Type* llvm_Function_getReturnType(llvm_Function const* self)
{
    return self->getReturnType();
}

extern "C"
llvm_ValueSymbolTable const* llvm_Function_getValueSymbolTable(llvm_Function const* self)
{
    return &self->getValueSymbolTable();
}

extern "C"
llvm_ValueSymbolTable* llvm_Function_getValueSymbolTableMut(llvm_Function* self)
{
    return &self->getValueSymbolTable();
}

extern "C"
int llvm_Function_hasFnAttr(llvm_Function const* self, llvm_StringRef _Kind)
{
    llvm::StringRef Kind(_Kind.data, _Kind.length);
    return (self->hasFnAttribute(Kind) ? 1 : 0);
}

extern "C"
int llvm_Function_hasGC(llvm_Function const* self)
{
    return (self->hasGC() ? 1 : 0);
}

extern "C"
int llvm_Function_hasStructRetAttr(llvm_Function const* self)
{
    return (self->hasStructRetAttr() ? 1 : 0);
}

extern "C"
int llvm_Function_hasUWTable(llvm_Function const* self)
{
    return (self->hasUWTable() ? 1 : 0);
}

extern "C"
int llvm_Function_isIntrinsic(llvm_Function const* self)
{
    return (self->isIntrinsic() ? 1 : 0);
}

extern "C"
int llvm_Function_isVarArg(llvm_Function const* self)
{
    return (self->isVarArg() ? 1 : 0);
}

extern "C"
int llvm_Function_needsUnwindTableEntry(llvm_Function const* self)
{
    return (self->needsUnwindTableEntry() ? 1 : 0);
}

extern "C"
int llvm_Function_onlyReadsMemory(llvm_Function const* self)
{
    return (self->onlyReadsMemory() ? 1 : 0);
}

extern "C"
int llvm_Function_onlyReadsMemoryParam(llvm_Function const* self, unsigned int n)
{
    return (self->onlyReadsMemory(n) ? 1 : 0);
}

extern "C"
void llvm_Function_removeFromParent(llvm_Function* self)
{
    self->removeFromParent();
}

extern "C"
void llvm_Function_setCallingConv(llvm_Function* self, llvm_CallingConv_ID CC)
{
    self->setCallingConv(CC);
}

extern "C"
void llvm_Function_setCannotDuplicate(llvm_Function* self)
{
    self->setCannotDuplicate();
}

extern "C"
void llvm_Function_setDoesNotAccessMemory(llvm_Function* self)
{
    self->setDoesNotAccessMemory();
}

extern "C"
void llvm_Function_setDoesNotAccessMemoryParam(llvm_Function* self, unsigned int n)
{
    self->setDoesNotAccessMemory(n);
}

extern "C"
void llvm_Function_setDoesNotAlias(llvm_Function* self, unsigned int n)
{
    self->setDoesNotAlias(n);
}

extern "C"
void llvm_Function_setDoesNotCapture(llvm_Function* self, unsigned int n)
{
    self->setDoesNotCapture(n);
}

extern "C"
void llvm_Function_setDoesNotReturn(llvm_Function* self)
{
    self->setDoesNotReturn();
}

extern "C"
void llvm_Function_setDoesNotThrow(llvm_Function* self)
{
    self->setDoesNotThrow();
}

extern "C"
void llvm_Function_setHasUWTable(llvm_Function* self)
{
    self->setHasUWTable();
}

extern "C"
void llvm_Function_setOnlyReadsMemory(llvm_Function* self)
{
    self->setOnlyReadsMemory();
}

extern "C"
void llvm_Function_setOnlyReadsMemoryParam(llvm_Function* self, unsigned int n)
{
    self->setOnlyReadsMemory(n);
}

extern "C"
void llvm_GlobalObject_setSection(llvm_GlobalObject* self, llvm_StringRef _S)
{
    llvm::StringRef S(_S.data, _S.length);
    self->setSection(S);
}

extern "C"
void llvm_GlobalValue_copyAttributesFrom(llvm_GlobalValue* self, llvm_GlobalValue* Src)
{
    self->copyAttributesFrom(Src);
}

extern "C"
void llvm_GlobalValue_delete(llvm_GlobalValue* self)
{
    delete self;
}

extern "C"
void llvm_GlobalValue_destroyConstant(llvm_GlobalValue* self)
{
    self->destroyConstant();
}

extern "C"
void llvm_GlobalValue_eraseFromParent(llvm_GlobalValue* self)
{
    self->eraseFromParent();
}

extern "C"
unsigned int llvm_GlobalValue_getAlignment(llvm_GlobalValue const* self)
{
    return self->getAlignment();
}

extern "C"
llvm_DataLayout const* llvm_GlobalValue_getDataLayout(llvm_GlobalValue const* self)
{
    return self->getDataLayout();
}

extern "C"
llvm_Module const* llvm_GlobalValue_getParent(llvm_GlobalValue const* self)
{
    return self->getParent();
}

extern "C"
llvm_Module* llvm_GlobalValue_getParentMut(llvm_GlobalValue* self)
{
    return self->getParent();
}

extern "C"
llvm_PointerType* llvm_GlobalValue_getType(llvm_GlobalValue const* self)
{
    return self->getType();
}

extern "C"
int llvm_GlobalValue_hasAppendingLinkage(llvm_GlobalValue const* self)
{
    return (self->hasAppendingLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasAvailableExternallyLinkage(llvm_GlobalValue const* self)
{
    return (self->hasAvailableExternallyLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasCommonLinkage(llvm_GlobalValue const* self)
{
    return (self->hasCommonLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasDLLExportStorageClass(llvm_GlobalValue const* self)
{
    return (self->hasDLLExportStorageClass() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasDLLImportStorageClass(llvm_GlobalValue const* self)
{
    return (self->hasDLLImportStorageClass() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasDefaultVisibility(llvm_GlobalValue const* self)
{
    return (self->hasDefaultVisibility() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasExternalLinkage(llvm_GlobalValue const* self)
{
    return (self->hasExternalLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasExternalWeakLinkage(llvm_GlobalValue const* self)
{
    return (self->hasExternalWeakLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasHiddenVisibility(llvm_GlobalValue const* self)
{
    return (self->hasHiddenVisibility() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasInternalLinkage(llvm_GlobalValue const* self)
{
    return (self->hasInternalLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasLinkOnceLinkage(llvm_GlobalValue const* self)
{
    return (self->hasLinkOnceLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasLocalLinkage(llvm_GlobalValue const* self)
{
    return (self->hasLocalLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasPrivateLinkage(llvm_GlobalValue const* self)
{
    return (self->hasPrivateLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasProtectedVisibility(llvm_GlobalValue const* self)
{
    return (self->hasProtectedVisibility() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasSection(llvm_GlobalValue const* self)
{
    return (self->hasSection() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasUnnamedAddr(llvm_GlobalValue const* self)
{
    return (self->hasUnnamedAddr() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasWeakAnyLinkage(llvm_GlobalValue const* self)
{
    return (self->hasWeakAnyLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasWeakLinkage(llvm_GlobalValue const* self)
{
    return (self->hasWeakLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_hasWeakODRLinkage(llvm_GlobalValue const* self)
{
    return (self->hasWeakODRLinkage() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_isDeclaration(llvm_GlobalValue const* self)
{
    return (self->isDeclaration() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_isDiscardableIfUnused(llvm_GlobalValue const* self)
{
    return (self->isDiscardableIfUnused() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_isThreadLocal(llvm_GlobalValue const* self)
{
    return (self->isThreadLocal() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_isWeakForLinker(llvm_GlobalValue const* self)
{
    return (self->isWeakForLinker() ? 1 : 0);
}

extern "C"
int llvm_GlobalValue_mayBeOverridden(llvm_GlobalValue const* self)
{
    return (self->mayBeOverridden() ? 1 : 0);
}

extern "C"
void llvm_GlobalValue_removeFromParent(llvm_GlobalValue* self)
{
    self->removeFromParent();
}

extern "C"
void llvm_GlobalValue_setThreadLocal(llvm_GlobalValue* self, int Val)
{
    self->setThreadLocal((Val == 1 ? true : false));
}

extern "C"
void llvm_GlobalValue_setUnnamedAddr(llvm_GlobalValue* self, int Val)
{
    self->setUnnamedAddr((Val == 1 ? true : false));
}

extern "C"
void llvm_GlobalVariable_copyAttributesFrom(llvm_GlobalVariable* self, llvm_GlobalValue* Src)
{
    self->copyAttributesFrom(Src);
}

extern "C"
void llvm_GlobalVariable_delete(llvm_GlobalVariable* self)
{
    delete self;
}

extern "C"
void llvm_GlobalVariable_eraseFromParent(llvm_GlobalVariable* self)
{
    self->eraseFromParent();
}

extern "C"
llvm_Constant const* llvm_GlobalVariable_getInitializer(llvm_GlobalVariable const* self)
{
    return self->getInitializer();
}

extern "C"
llvm_Constant* llvm_GlobalVariable_getInitializerMut(llvm_GlobalVariable* self)
{
    return self->getInitializer();
}

extern "C"
int llvm_GlobalVariable_hasDefinitiveInitializer(llvm_GlobalVariable const* self)
{
    return (self->hasDefinitiveInitializer() ? 1 : 0);
}

extern "C"
int llvm_GlobalVariable_hasInitializer(llvm_GlobalVariable const* self)
{
    return (self->hasInitializer() ? 1 : 0);
}

extern "C"
int llvm_GlobalVariable_hasUniqueInitializer(llvm_GlobalVariable const* self)
{
    return (self->hasUniqueInitializer() ? 1 : 0);
}

extern "C"
int llvm_GlobalVariable_isConstant(llvm_GlobalVariable const* self)
{
    return (self->isConstant() ? 1 : 0);
}

extern "C"
int llvm_GlobalVariable_isExternallyInitialized(llvm_GlobalVariable const* self)
{
    return (self->isExternallyInitialized() ? 1 : 0);
}

extern "C"
llvm_GlobalVariable* llvm_GlobalVariable_new(llvm_Type* Ty, int isConstant, llvm_GlobalValue_LinkageTypes Linkage)
{
    return new ::llvm::GlobalVariable(Ty, (isConstant == 1 ? true : false), Linkage);
}

extern "C"
llvm_GlobalVariable* llvm_GlobalVariable_newWithModule(llvm_Module* Module, llvm_Type* Ty, int isConstant, llvm_GlobalValue_LinkageTypes Linkage, llvm_Constant* Initializer)
{
    return new ::llvm::GlobalVariable(*Module, Ty, (isConstant == 1 ? true : false), Linkage, Initializer);
}

extern "C"
void llvm_GlobalVariable_removeFromParent(llvm_GlobalVariable* self)
{
    self->removeFromParent();
}

extern "C"
void llvm_GlobalVariable_setConstant(llvm_GlobalVariable* self, int Val)
{
    self->setConstant((Val == 1 ? true : false));
}

extern "C"
void llvm_GlobalVariable_setExternallyInitialized(llvm_GlobalVariable* self, int Val)
{
    self->setExternallyInitialized((Val == 1 ? true : false));
}

extern "C"
void llvm_GlobalVariable_setInitializer(llvm_GlobalVariable* self, llvm_Constant* InitVal)
{
    self->setInitializer(InitVal);
}

extern "C"
void llvm_IRBuilderBase_ClearInsertionPoint(llvm_IRBuilderBase* self)
{
    self->ClearInsertionPoint();
}

extern "C"
llvm_Value* llvm_IRBuilderBase_CreateGlobalString(llvm_IRBuilderBase* self, llvm_StringRef _Str, std_string* _Name)
{
    llvm::StringRef Str(_Str.data, _Str.length);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateGlobalString(Str, Name);
}

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateLifetimeEnd(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_ConstantInt** _Size)
{
    ::llvm::ConstantInt* Size = NULL;
    if (_Size != NULL){
        Size = *_Size;
    }
    return self->CreateLifetimeEnd(Ptr, Size);
}

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateLifetimeStart(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_ConstantInt** _Size)
{
    ::llvm::ConstantInt* Size = NULL;
    if (_Size != NULL){
        Size = *_Size;
    }
    return self->CreateLifetimeStart(Ptr, Size);
}

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemCpy(llvm_IRBuilderBase* self, llvm_Value* Dst, llvm_Value* Src, llvm_Value* Size, unsigned int Align, int* _isVolatile)
{
    bool isVolatile = false;
    if (_isVolatile != NULL){
        isVolatile = ((*_isVolatile) == 1 ? true : false);
    }
    return self->CreateMemCpy(Dst, Src, Size, Align, isVolatile);
}

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemMove(llvm_IRBuilderBase* self, llvm_Value* Dst, llvm_Value* Src, llvm_Value* Size, unsigned int Align, int* _isVolatile)
{
    bool isVolatile = false;
    if (_isVolatile != NULL){
        isVolatile = ((*_isVolatile) == 1 ? true : false);
    }
    return self->CreateMemMove(Dst, Src, Size, Align, isVolatile);
}

extern "C"
llvm_CallInst* llvm_IRBuilderBase_CreateMemSet(llvm_IRBuilderBase* self, llvm_Value* Ptr, llvm_Value* Value, llvm_Value* Size, unsigned int Align, int* _isVolatile)
{
    bool isVolatile = false;
    if (_isVolatile != NULL){
        isVolatile = ((*_isVolatile) == 1 ? true : false);
    }
    return self->CreateMemSet(Ptr, Value, Size, Align, isVolatile);
}

extern "C"
llvm_BasicBlock* llvm_IRBuilderBase_GetInsertBlock(llvm_IRBuilderBase const* self)
{
    return self->GetInsertBlock();
}

extern "C"
void llvm_IRBuilderBase_SetCurrentDebugLocation(llvm_IRBuilderBase* self, llvm_DebugLoc const* Loc)
{
    self->SetCurrentDebugLocation(*Loc);
}

extern "C"
void llvm_IRBuilderBase_SetDefaultFPMathTag(llvm_IRBuilderBase* self, llvm_MDNode* FPMathTag)
{
    self->SetDefaultFPMathTag(FPMathTag);
}

extern "C"
void llvm_IRBuilderBase_SetInsertPoint(llvm_IRBuilderBase* self, llvm_BasicBlock* BB)
{
    self->SetInsertPoint(BB);
}

extern "C"
void llvm_IRBuilderBase_SetInsertPointAtInst(llvm_IRBuilderBase* self, llvm_Instruction* Inst)
{
    self->SetInsertPoint(Inst);
}

extern "C"
void llvm_IRBuilderBase_SetInstDebugLocation(llvm_IRBuilderBase const* self, llvm_Instruction* Inst)
{
    self->SetInstDebugLocation(Inst);
}

extern "C"
llvm_LLVMContext* llvm_IRBuilderBase_getContext(llvm_IRBuilderBase const* self)
{
    return &self->getContext();
}

extern "C"
llvm_Type* llvm_IRBuilderBase_getCurrentFunctionReturnType(llvm_IRBuilderBase const* self)
{
    return self->getCurrentFunctionReturnType();
}

extern "C"
llvm_MDNode* llvm_IRBuilderBase_getDefaultFPMathTag(llvm_IRBuilderBase const* self)
{
    return self->getDefaultFPMathTag();
}

extern "C"
llvm_Type* llvm_IRBuilderBase_getDoubleTy(llvm_IRBuilderBase* self)
{
    return self->getDoubleTy();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getFalse(llvm_IRBuilderBase* self)
{
    return self->getFalse();
}

extern "C"
llvm_Type* llvm_IRBuilderBase_getFloatTy(llvm_IRBuilderBase* self)
{
    return self->getFloatTy();
}

extern "C"
llvm_Type* llvm_IRBuilderBase_getHalfTy(llvm_IRBuilderBase* self)
{
    return self->getHalfTy();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt(llvm_IRBuilderBase* self, llvm_APInt _Value)
{
    llvm::ArrayRef<uint64_t> __Value_data(_Value.data.data, _Value.data.size);
    llvm::APInt Value(_Value.numbits, __Value_data);
    return self->getInt(Value);
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt1(llvm_IRBuilderBase* self, int Value)
{
    return self->getInt1((Value == 1 ? true : false));
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt16(llvm_IRBuilderBase* self, uint16_t Value)
{
    return self->getInt16(Value);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt16Ty(llvm_IRBuilderBase* self)
{
    return self->getInt16Ty();
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt1Ty(llvm_IRBuilderBase* self)
{
    return self->getInt1Ty();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt32(llvm_IRBuilderBase* self, uint32_t Value)
{
    return self->getInt32(Value);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt32Ty(llvm_IRBuilderBase* self)
{
    return self->getInt32Ty();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt64(llvm_IRBuilderBase* self, uint64_t Value)
{
    return self->getInt64(Value);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt64Ty(llvm_IRBuilderBase* self)
{
    return self->getInt64Ty();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getInt8(llvm_IRBuilderBase* self, uint8_t Value)
{
    return self->getInt8(Value);
}

extern "C"
llvm_PointerType* llvm_IRBuilderBase_getInt8PtrTy(llvm_IRBuilderBase* self, unsigned int* _AddrSpace)
{
    unsigned int AddrSpace = 0;
    if (_AddrSpace != NULL){
        AddrSpace = *_AddrSpace;
    }
    return self->getInt8PtrTy(AddrSpace);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getInt8Ty(llvm_IRBuilderBase* self)
{
    return self->getInt8Ty();
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getIntN(llvm_IRBuilderBase* self, unsigned int NumBits, uint64_t Value)
{
    return self->getIntN(NumBits, Value);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getIntNTy(llvm_IRBuilderBase* self, unsigned int NumBits)
{
    return self->getIntNTy(NumBits);
}

extern "C"
llvm_IntegerType* llvm_IRBuilderBase_getIntPtrTy(llvm_IRBuilderBase* self, llvm_DataLayout const* DL, unsigned int* _AddrSpace)
{
    unsigned int AddrSpace = 0;
    if (_AddrSpace != NULL){
        AddrSpace = *_AddrSpace;
    }
    return self->getIntPtrTy(DL, AddrSpace);
}

extern "C"
llvm_ConstantInt* llvm_IRBuilderBase_getTrue(llvm_IRBuilderBase* self)
{
    return self->getTrue();
}

extern "C"
llvm_Type* llvm_IRBuilderBase_getVoidTy(llvm_IRBuilderBase* self)
{
    return self->getVoidTy();
}

extern "C"
llvm_IRBuilderBase* llvm_IRBuilderBase_new(llvm_LLVMContext* Context)
{
    return new(std::nothrow) ::llvm::IRBuilderBase(*Context);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAShr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAShr(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAShrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAShr(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAdd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAddrSpaceCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAddrSpaceCast(Value, DestTy, Name);
}

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateAlignedLoad(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Align, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAlignedLoad(Ptr, Align, Name);
}

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateAlignedLoadVolatile(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Align, int isVolatile, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAlignedLoad(Ptr, Align, (isVolatile == 1 ? true : false), Name);
}

extern "C"
llvm_StoreInst* llvm_IRBuilder_CreateAlignedStore(llvm_IRBuilder* self, llvm_Value* Value, llvm_Value* Ptr, unsigned int Align, int* _isVolatile)
{
    bool isVolatile = false;
    if (_isVolatile != NULL){
        isVolatile = ((*_isVolatile) == 1 ? true : false);
    }
    return self->CreateAlignedStore(Value, Ptr, Align, isVolatile);
}

extern "C"
llvm_AllocaInst* llvm_IRBuilder_CreateAlloca(llvm_IRBuilder* self, llvm_Type* Ty, llvm_Value** _ArraySize, std_string* _Name)
{
    ::llvm::Value* ArraySize = NULL;
    if (_ArraySize != NULL){
        ArraySize = *_ArraySize;
    }
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAlloca(Ty, ArraySize, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAnd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAnd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateAndByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateAnd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateBinOp(llvm_IRBuilder* self, llvm_Instruction_BinaryOps Opcode, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateBinOp(Opcode, LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateBitCast(Value, DestTy, Name);
}

extern "C"
llvm_BranchInst* llvm_IRBuilder_CreateBr(llvm_IRBuilder* self, llvm_BasicBlock* Dest)
{
    return self->CreateBr(Dest);
}

extern "C"
llvm_CallInst* llvm_IRBuilder_CreateCall(llvm_IRBuilder* self, llvm_Value* Callee, llvm_ArrayRef_ptr_llvm_Value _Args, std_string* _Name)
{
    llvm::ArrayRef<::llvm::Value*> Args(_Args.data, _Args.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateCall(Callee, Args, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateCast(llvm_IRBuilder* self, llvm_Instruction_CastOps Opcode, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateCast(Opcode, Value, DestTy, Name);
}

extern "C"
llvm_BranchInst* llvm_IRBuilder_CreateCondBr(llvm_IRBuilder* self, llvm_Value* Cond, llvm_BasicBlock* TrueBlock, llvm_BasicBlock* FalseBlock)
{
    return self->CreateCondBr(Cond, TrueBlock, FalseBlock);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateExactSDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateExactSDiv(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateExactUDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateExactUDiv(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractElement(llvm_IRBuilder* self, llvm_Value* Vec, llvm_Value* Idx, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateExtractElement(Vec, Idx, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractInteger(llvm_IRBuilder* self, llvm_DataLayout const* DL, llvm_Value* From, llvm_IntegerType* ExtractedTy, uint64_t Offset, std_string _Name)
{
    std::string Name(_Name.data, _Name.length);
    return self->CreateExtractInteger(*DL, From, ExtractedTy, Offset, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateExtractValue(llvm_IRBuilder* self, llvm_Value* Agg, llvm_ArrayRef_uint _Indexes, std_string* _Name)
{
    llvm::ArrayRef<unsigned int> Indexes(_Indexes.data, _Indexes.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateExtractValue(Agg, Indexes, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFAdd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmp(llvm_IRBuilder* self, llvm_CmpInst_Predicate Pred, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmp(Pred, LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpOEQ(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpOGE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpOGT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOLE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpOLE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpOLT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpOLT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpONE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpONE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpORD(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpORD(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpUEQ(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpUGE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpUGT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpULE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpULE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpULT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpULT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUNE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpUNE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFCmpUNO(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFCmpUNO(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFDiv(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFMul(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFNeg(Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFPCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFPExt(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPToSI(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFPToSI(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPToUI(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFPToUI(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFPTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFPTrunc(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFRem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFRem(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateFSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFSub(LHS, RHS, Name);
}

extern "C"
llvm_FenceInst* llvm_IRBuilder_CreateFence(llvm_IRBuilder* self, llvm_AtomicOrdering Ordering, llvm_SynchronizationScope* _SynchScope, std_string* _Name)
{
    ::llvm::SynchronizationScope SynchScope = llvm::SynchronizationScope::CrossThread;
    if (_SynchScope != NULL){
        SynchScope = *_SynchScope;
    }
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateFence(Ordering, SynchScope, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateGEP(llvm_IRBuilder* self, llvm_Value* Ptr, llvm_ArrayRef_ptr_llvm_Value _Indexes, std_string* _Name)
{
    llvm::ArrayRef<::llvm::Value*> Indexes(_Indexes.data, _Indexes.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateGEP(Ptr, Indexes, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateGlobalStringPtr(llvm_IRBuilder* self, llvm_StringRef _Str, std_string* _Name)
{
    llvm::StringRef Str(_Str.data, _Str.length);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateGlobalStringPtr(Str, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmp(llvm_IRBuilder* self, llvm_CmpInst_Predicate Pred, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmp(Pred, LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpEQ(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpEQ(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpNE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpNE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpSGE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpSGT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSLE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpSLE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpSLT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpSLT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpUGE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpUGE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpUGT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpUGT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpULE(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpULE(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateICmpULT(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateICmpULT(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateInBoundsGEP(llvm_IRBuilder* self, llvm_Value* Ptr, llvm_ArrayRef_ptr_llvm_Value _Indexes, std_string* _Name)
{
    llvm::ArrayRef<::llvm::Value*> Indexes(_Indexes.data, _Indexes.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateInBoundsGEP(Ptr, Indexes, Name);
}

extern "C"
llvm_IndirectBrInst* llvm_IRBuilder_CreateIndirectBr(llvm_IRBuilder* self, llvm_Value* Addr, unsigned int* _NumCases)
{
    unsigned int NumCases = 10;
    if (_NumCases != NULL){
        NumCases = *_NumCases;
    }
    return self->CreateIndirectBr(Addr, NumCases);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateInsertElement(llvm_IRBuilder* self, llvm_Value* Vec, llvm_Value* NewElt, llvm_Value* Idx, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateInsertElement(Vec, NewElt, Idx, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateInsertValue(llvm_IRBuilder* self, llvm_Value* Agg, llvm_Value* Value, llvm_ArrayRef_uint _Indexes, std_string* _Name)
{
    llvm::ArrayRef<unsigned int> Indexes(_Indexes.data, _Indexes.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateInsertValue(Agg, Value, Indexes, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateIntCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, int isSigned, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateIntCast(Value, DestTy, (isSigned == 1 ? true : false), Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateIntToPtr(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateIntToPtr(Value, DestTy, Name);
}

extern "C"
llvm_InvokeInst* llvm_IRBuilder_CreateInvoke(llvm_IRBuilder* self, llvm_Value* Callee, llvm_BasicBlock* NormalDest, llvm_BasicBlock* UnwindDest, llvm_ArrayRef_ptr_llvm_Value _Args, std_string_const* _Name)
{
    llvm::ArrayRef<::llvm::Value*> Args(_Args.data, _Args.size);
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateInvoke(Callee, NormalDest, UnwindDest, Args, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateIsNotNull(llvm_IRBuilder* self, llvm_Value* Arg, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateIsNotNull(Arg, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateIsNull(llvm_IRBuilder* self, llvm_Value* Arg, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateIsNull(Arg, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateLShr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateLShr(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateLShrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateLShr(LHS, RHS, Name);
}

extern "C"
llvm_LandingPadInst* llvm_IRBuilder_CreateLandingPad(llvm_IRBuilder* self, llvm_Type* Ty, llvm_Value* PersFn, unsigned int NumClauses, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateLandingPad(Ty, PersFn, NumClauses, Name);
}

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateLoad(llvm_IRBuilder* self, llvm_Value* Ptr, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateLoad(Ptr, Name);
}

extern "C"
llvm_LoadInst* llvm_IRBuilder_CreateLoadVolatile(llvm_IRBuilder* self, llvm_Value* Ptr, int isVolatile, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateLoad(Ptr, (isVolatile == 1 ? true : false), Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateMul(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNSWAdd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNSWMul(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNSWNeg(Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNSWSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNSWSub(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWAdd(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNUWAdd(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWMul(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNUWMul(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNUWNeg(Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNUWSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNUWSub(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNeg(llvm_IRBuilder* self, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNeg(Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateNot(llvm_IRBuilder* self, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateNot(Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateOr(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateOr(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateOrByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateOr(LHS, RHS, Name);
}

extern "C"
llvm_PHINode* llvm_IRBuilder_CreatePHI(llvm_IRBuilder* self, llvm_Type* Ty, unsigned int NumReservedValues, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreatePHI(Ty, NumReservedValues, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreatePointerBitCastOrAddrSpaceCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreatePointerCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreatePointerCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreatePtrDiff(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreatePtrDiff(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreatePtrToInt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreatePtrToInt(Value, DestTy, Name);
}

extern "C"
llvm_ResumeInst* llvm_IRBuilder_CreateResume(llvm_IRBuilder* self, llvm_Value* Exn)
{
    return self->CreateResume(Exn);
}

extern "C"
llvm_ReturnInst* llvm_IRBuilder_CreateRet(llvm_IRBuilder* self, llvm_Value* Value)
{
    return self->CreateRet(Value);
}

extern "C"
llvm_ReturnInst* llvm_IRBuilder_CreateRetVoid(llvm_IRBuilder* self)
{
    return self->CreateRetVoid();
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSDiv(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSExt(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExtOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSExtOrBitCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSExtOrTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSExtOrTrunc(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSIToFP(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSIToFP(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSRem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSRem(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSelect(llvm_IRBuilder* self, llvm_Value* C, llvm_Value* TrueValue, llvm_Value* FalseValue, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSelect(C, TrueValue, FalseValue, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateShl(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateShl(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateShlByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateShl(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateShuffleVector(llvm_IRBuilder* self, llvm_Value* V1, llvm_Value* P2, llvm_Value* Mask, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateShuffleVector(V1, P2, Mask, Name);
}

extern "C"
llvm_StoreInst* llvm_IRBuilder_CreateStore(llvm_IRBuilder* self, llvm_Value* Value, llvm_Value* Ptr, int* _isVolatile)
{
    bool isVolatile = false;
    if (_isVolatile != NULL){
        isVolatile = ((*_isVolatile) == 1 ? true : false);
    }
    return self->CreateStore(Value, Ptr, isVolatile);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateStructGEP(llvm_IRBuilder* self, llvm_Value* Ptr, unsigned int Index, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateStructGEP(Ptr, Index, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateSub(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateSub(LHS, RHS, Name);
}

extern "C"
llvm_SwitchInst* llvm_IRBuilder_CreateSwitch(llvm_IRBuilder* self, llvm_Value* Value, llvm_BasicBlock* Dest, unsigned int* _NumCases)
{
    unsigned int NumCases = 10;
    if (_NumCases != NULL){
        NumCases = *_NumCases;
    }
    return self->CreateSwitch(Value, Dest, NumCases);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateTrunc(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateTruncOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateTruncOrBitCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateUDiv(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateUDiv(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateUIToFP(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateUIToFP(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateURem(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateURem(LHS, RHS, Name);
}

extern "C"
llvm_UnreachableInst* llvm_IRBuilder_CreateUnreachable(llvm_IRBuilder* self)
{
    return self->CreateUnreachable();
}

extern "C"
llvm_VAArgInst* llvm_IRBuilder_CreateVAArg(llvm_IRBuilder* self, llvm_Value* List, llvm_Type* Ty, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateVAArg(List, Ty, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateVectorSplat(llvm_IRBuilder* self, unsigned int NumElements, llvm_Value* Value, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateVectorSplat(NumElements, Value, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateXor(llvm_IRBuilder* self, llvm_Value* LHS, llvm_Value* RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateXor(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateXorByValue(llvm_IRBuilder* self, llvm_Value* LHS, uint64_t RHS, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateXor(LHS, RHS, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExt(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateZExt(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExtOrBitCast(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateZExtOrBitCast(Value, DestTy, Name);
}

extern "C"
llvm_Value* llvm_IRBuilder_CreateZExtOrTrunc(llvm_IRBuilder* self, llvm_Value* Value, llvm_Type* DestTy, std_string* _Name)
{
    std::string Name = "";
    if (_Name != NULL){
        std::string __Name_value((*_Name).data, (*_Name).length);
        Name = __Name_value;
    }
    return self->CreateZExtOrTrunc(Value, DestTy, Name);
}

extern "C"
void llvm_IRBuilder_delete(llvm_IRBuilder* self)
{
    delete self;
}

extern "C"
int llvm_IRBuilder_isNamePreserving(llvm_IRBuilder const* self)
{
    return (self->isNamePreserving() ? 1 : 0);
}

extern "C"
llvm_IRBuilder* llvm_IRBuilder_new(llvm_LLVMContext* Context)
{
    return new(std::nothrow) ::llvm::IRBuilder<>(*Context);
}

extern "C"
llvm_IRBuilder* llvm_IRBuilder_new_in_block(llvm_BasicBlock* BB)
{
    return new(std::nothrow) ::llvm::IRBuilder<>(BB);
}

extern "C"
llvm_Instruction* llvm_Instruction_clone(llvm_Instruction const* self)
{
    return self->clone();
}

extern "C"
void llvm_Instruction_copyFastMathFlags(llvm_Instruction* self, llvm_Instruction const* Inst)
{
    self->copyFastMathFlags(Inst);
}

extern "C"
void llvm_Instruction_delete(llvm_Instruction* self)
{
    delete self;
}

extern "C"
void llvm_Instruction_dropUnknownMetadata(llvm_Instruction* self)
{
    self->dropUnknownMetadata();
}

extern "C"
void llvm_Instruction_dropUnknownMetadataFromIDS(llvm_Instruction* self, llvm_ArrayRef_uint _KnownIDs)
{
    llvm::ArrayRef<unsigned int> KnownIDs(_KnownIDs.data, _KnownIDs.size);
    self->dropUnknownMetadata(KnownIDs);
}

extern "C"
void llvm_Instruction_eraseFromParent(llvm_Instruction* self)
{
    self->eraseFromParent();
}

extern "C"
llvm_DataLayout const* llvm_Instruction_getDataLayout(llvm_Instruction const* self)
{
    return self->getDataLayout();
}

extern "C"
llvm_DebugLoc const* llvm_Instruction_getDebugLoc(llvm_Instruction const* self)
{
    return &self->getDebugLoc();
}

extern "C"
llvm_MDNode* llvm_Instruction_getMetadata(llvm_Instruction const* self, unsigned int KindID)
{
    return self->getMetadata(KindID);
}

extern "C"
llvm_MDNode* llvm_Instruction_getMetadataStr(llvm_Instruction const* self, llvm_StringRef _Kind)
{
    llvm::StringRef Kind(_Kind.data, _Kind.length);
    return self->getMetadata(Kind);
}

extern "C"
unsigned int llvm_Instruction_getOpcode(llvm_Instruction const* self)
{
    return self->getOpcode();
}

extern "C"
llvm_BasicBlock const* llvm_Instruction_getParent(llvm_Instruction const* self)
{
    return self->getParent();
}

extern "C"
llvm_BasicBlock* llvm_Instruction_getParentMut(llvm_Instruction* self)
{
    return self->getParent();
}

extern "C"
int llvm_Instruction_hasAllowReciprocal(llvm_Instruction const* self)
{
    return (self->hasAllowReciprocal() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasMetadata(llvm_Instruction const* self)
{
    return (self->hasMetadata() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasMetadataOtherThanDebugLoc(llvm_Instruction const* self)
{
    return (self->hasMetadataOtherThanDebugLoc() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasNoInfs(llvm_Instruction const* self)
{
    return (self->hasNoInfs() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasNoNaNs(llvm_Instruction const* self)
{
    return (self->hasNoNaNs() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasNoSignedZeros(llvm_Instruction const* self)
{
    return (self->hasNoSignedZeros() ? 1 : 0);
}

extern "C"
int llvm_Instruction_hasUnsafeAlgebra(llvm_Instruction const* self)
{
    return (self->hasUnsafeAlgebra() ? 1 : 0);
}

extern "C"
void llvm_Instruction_insertAfter(llvm_Instruction* self, llvm_Instruction* InsertPos)
{
    self->insertAfter(InsertPos);
}

extern "C"
void llvm_Instruction_insertBefore(llvm_Instruction* self, llvm_Instruction* InsertPos)
{
    self->insertBefore(InsertPos);
}

extern "C"
int llvm_Instruction_isArithmeticShift(llvm_Instruction const* self)
{
    return (self->isArithmeticShift() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isAssociative(llvm_Instruction const* self)
{
    return (self->isAssociative() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isBinaryOp(llvm_Instruction const* self)
{
    return (self->isBinaryOp() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isCast(llvm_Instruction const* self)
{
    return (self->isCast() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isCommutative(llvm_Instruction const* self)
{
    return (self->isCommutative() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isIdempotent(llvm_Instruction const* self)
{
    return (self->isIdempotent() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isIdenticalTo(llvm_Instruction const* self, llvm_Instruction const* Inst)
{
    return (self->isIdenticalTo(Inst) ? 1 : 0);
}

extern "C"
int llvm_Instruction_isIdenticalToWhenDefined(llvm_Instruction const* self, llvm_Instruction const* Inst)
{
    return (self->isIdenticalToWhenDefined(Inst) ? 1 : 0);
}

extern "C"
int llvm_Instruction_isLogicalShift(llvm_Instruction const* self)
{
    return (self->isLogicalShift() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isNilpotent(llvm_Instruction const* self)
{
    return (self->isNilpotent() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isSameOperationAs(llvm_Instruction const* self, llvm_Instruction const* Inst, unsigned int flags)
{
    return (self->isSameOperationAs(Inst, flags) ? 1 : 0);
}

extern "C"
int llvm_Instruction_isShift(llvm_Instruction* self)
{
    return (self->isShift() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isTerminator(llvm_Instruction const* self)
{
    return (self->isTerminator() ? 1 : 0);
}

extern "C"
int llvm_Instruction_isUsedOutsideOfBlock(llvm_Instruction const* self, llvm_BasicBlock const* BB)
{
    return (self->isUsedOutsideOfBlock(BB) ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayHaveSideEffects(llvm_Instruction const* self)
{
    return (self->mayHaveSideEffects() ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayReadFromMemory(llvm_Instruction const* self)
{
    return (self->mayReadFromMemory() ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayReadOrWriteMemory(llvm_Instruction const* self)
{
    return (self->mayReadOrWriteMemory() ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayReturn(llvm_Instruction const* self)
{
    return (self->mayReturn() ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayThrow(llvm_Instruction const* self)
{
    return (self->mayThrow() ? 1 : 0);
}

extern "C"
int llvm_Instruction_mayWriteToMemory(llvm_Instruction const* self)
{
    return (self->mayWriteToMemory() ? 1 : 0);
}

extern "C"
void llvm_Instruction_moveBefore(llvm_Instruction* self, llvm_Instruction* MovePos)
{
    self->moveBefore(MovePos);
}

extern "C"
void llvm_Instruction_removeFromParent(llvm_Instruction* self)
{
    self->removeFromParent();
}

extern "C"
void llvm_Instruction_setDebugLoc(llvm_Instruction* self, llvm_DebugLoc const* Loc)
{
    self->setDebugLoc(*Loc);
}

extern "C"
void llvm_Instruction_setHasAllowReciprocal(llvm_Instruction* self, int Val)
{
    self->setHasAllowReciprocal((Val == 1 ? true : false));
}

extern "C"
void llvm_Instruction_setHasNoInfs(llvm_Instruction* self, int Val)
{
    self->setHasNoInfs((Val == 1 ? true : false));
}

extern "C"
void llvm_Instruction_setHasNoNaNs(llvm_Instruction* self, int Val)
{
    self->setHasNoNaNs((Val == 1 ? true : false));
}

extern "C"
void llvm_Instruction_setHasNoSignedZeros(llvm_Instruction* self, int Val)
{
    self->setHasNoSignedZeros((Val == 1 ? true : false));
}

extern "C"
void llvm_Instruction_setHasUnsafeAlgebra(llvm_Instruction* self, int Val)
{
    self->setHasUnsafeAlgebra((Val == 1 ? true : false));
}

extern "C"
void llvm_Instruction_setMetadata(llvm_Instruction* self, unsigned int KindID, llvm_MDNode* Node)
{
    self->setMetadata(KindID, Node);
}

extern "C"
void llvm_Instruction_setMetadataStr(llvm_Instruction* self, llvm_StringRef _Kind, llvm_MDNode* Node)
{
    llvm::StringRef Kind(_Kind.data, _Kind.length);
    self->setMetadata(Kind, Node);
}

extern "C"
llvm_Instruction const* llvm_Instruction_user_back(llvm_Instruction const* self)
{
    return self->user_back();
}

extern "C"
llvm_Instruction* llvm_Instruction_user_back_mut(llvm_Instruction* self)
{
    return self->user_back();
}

extern "C"
int llvm_IntegerType_classof(llvm_Type const* ty)
{
    return (::llvm::IntegerType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_IntegerType* llvm_IntegerType_get(llvm_LLVMContext* ctx, unsigned int NumBits)
{
    return ::llvm::IntegerType::get(*ctx, NumBits);
}

extern "C"
uint64_t llvm_IntegerType_getBitMask(llvm_IntegerType const* self)
{
    return self->getBitMask();
}

extern "C"
unsigned int llvm_IntegerType_getBitWidth(llvm_IntegerType const* self)
{
    return self->getBitWidth();
}

extern "C"
uint64_t llvm_IntegerType_getSignBit(llvm_IntegerType const* self)
{
    return self->getSignBit();
}

extern "C"
int llvm_IntegerType_isPowerOf2ByteWidth(llvm_IntegerType const* self)
{
    return (self->isPowerOf2ByteWidth() ? 1 : 0);
}

extern "C"
llvm_LLVMContext* llvm_LLVMContext_delete()
{
    return new(std::nothrow) ::llvm::LLVMContext();
}

extern "C"
llvm_LLVMContext* llvm_LLVMContext_new()
{
    return new(std::nothrow) ::llvm::LLVMContext();
}

extern "C"
void llvm_Module_appendModuleInlineAsm(llvm_Module* self, llvm_StringRef _Asm)
{
    llvm::StringRef Asm(_Asm.data, _Asm.length);
    self->appendModuleInlineAsm(Asm);
}

extern "C"
void llvm_Module_delete(llvm_Module* self)
{
    delete self;
}

extern "C"
void llvm_Module_dump(llvm_Module const* self)
{
    self->dump();
}

extern "C"
llvm_LLVMContext* llvm_Module_getContext(llvm_Module const* self)
{
    return &self->getContext();
}

extern "C"
llvm_DataLayout const* llvm_Module_getDataLayout(llvm_Module const* self)
{
    return self->getDataLayout();
}

extern "C"
std_string_const llvm_Module_getDataLayoutStr(llvm_Module const* self)
{
    return {
        .data = self->getDataLayoutStr().data(),
        .length = self->getDataLayoutStr().length(),
    };
}

extern "C"
llvm_Function* llvm_Module_getFunction(llvm_Module const* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->getFunction(Name);
}

extern "C"
unsigned int llvm_Module_getMDKindID(llvm_Module const* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->getMDKindID(Name);
}

extern "C"
std_string_const llvm_Module_getModuleIdentifier(llvm_Module const* self)
{
    return {
        .data = self->getModuleIdentifier().data(),
        .length = self->getModuleIdentifier().length(),
    };
}

extern "C"
std_string_const llvm_Module_getModuleInlineAsm(llvm_Module const* self)
{
    return {
        .data = self->getModuleInlineAsm().data(),
        .length = self->getModuleInlineAsm().length(),
    };
}

extern "C"
llvm_GlobalValue* llvm_Module_getNamedValue(llvm_Module const* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->getNamedValue(Name);
}

extern "C"
llvm_Constant* llvm_Module_getOrInsertFunction(llvm_Module* self, llvm_StringRef _Name, llvm_FunctionType* ty)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->getOrInsertFunction(Name, ty);
}

extern "C"
std_string_const llvm_Module_getTargetTriple(llvm_Module const* self)
{
    return {
        .data = self->getTargetTriple().data(),
        .length = self->getTargetTriple().length(),
    };
}

extern "C"
llvm_StructType* llvm_Module_getTypeByName(llvm_Module const* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->getTypeByName(Name);
}

extern "C"
llvm_Module* llvm_Module_new(llvm_StringRef _ModuleID, llvm_LLVMContext* Context)
{
    llvm::StringRef ModuleID(_ModuleID.data, _ModuleID.length);
    return new(std::nothrow) ::llvm::Module(ModuleID, *Context);
}

extern "C"
void llvm_Module_setDataLayout(llvm_Module* self, llvm_DataLayout const* Other)
{
    self->setDataLayout(Other);
}

extern "C"
void llvm_Module_setDataLayoutStr(llvm_Module* self, llvm_StringRef _Desc)
{
    llvm::StringRef Desc(_Desc.data, _Desc.length);
    self->setDataLayout(Desc);
}

extern "C"
void llvm_Module_setModuleIdentifier(llvm_Module* self, llvm_StringRef _ID)
{
    llvm::StringRef ID(_ID.data, _ID.length);
    self->setModuleIdentifier(ID);
}

extern "C"
void llvm_Module_setModuleInlineAsm(llvm_Module* self, llvm_StringRef _Asm)
{
    llvm::StringRef Asm(_Asm.data, _Asm.length);
    self->setModuleInlineAsm(Asm);
}

extern "C"
void llvm_Module_setTargetTriple(llvm_Module* self, llvm_StringRef _Triple)
{
    llvm::StringRef Triple(_Triple.data, _Triple.length);
    self->setTargetTriple(Triple);
}

extern "C"
unsigned int llvm_Operator_getOpcode(llvm_Operator const* self)
{
    return self->getOpcode();
}

extern "C"
void llvm_PHINode_addIncoming(llvm_PHINode* self, llvm_Value* V, llvm_BasicBlock* BB)
{
    self->addIncoming(V, BB);
}

extern "C"
void llvm_PHINode_delete(llvm_PHINode* self)
{
    delete self;
}

extern "C"
llvm_BasicBlock* llvm_PHINode_getIncomingBlock(llvm_PHINode const* self, unsigned int i)
{
    return self->getIncomingBlock(i);
}

extern "C"
llvm_Value* llvm_PHINode_getIncomingValue(llvm_PHINode const* self, unsigned int i)
{
    return self->getIncomingValue(i);
}

extern "C"
unsigned int llvm_PHINode_getNumIncomingValues(llvm_PHINode const* self)
{
    return self->getNumIncomingValues();
}

extern "C"
void llvm_PHINode_setIncomingBlock(llvm_PHINode* self, unsigned int i, llvm_BasicBlock* BB)
{
    self->setIncomingBlock(i, BB);
}

extern "C"
void llvm_PHINode_setIncomingValue(llvm_PHINode* self, unsigned int i, llvm_Value* V)
{
    self->setIncomingValue(i, V);
}

extern "C"
void llvm_PassManager_add(llvm_PassManager* self, llvm_Pass* Pass)
{
    self->add(Pass);
}

extern "C"
llvm_PassManager* llvm_PassManager_new()
{
    return new(std::nothrow) ::llvm::PassManager();
}

extern "C"
void llvm_PassManager_run(llvm_PassManager* self, llvm_Module* Module)
{
    self->run(*Module);
}

extern "C"
void llvm_Pass_delete(llvm_Pass* self)
{
    delete self;
}

extern "C"
int llvm_Pass_doFinalization(llvm_Pass* self, llvm_Module* Module)
{
    return (self->doFinalization(*Module) ? 1 : 0);
}

extern "C"
int llvm_Pass_doInitialization(llvm_Pass* self, llvm_Module* Module)
{
    return (self->doInitialization(*Module) ? 1 : 0);
}

extern "C"
void llvm_Pass_dump(llvm_Pass const* self)
{
    self->dump();
}

extern "C"
llvm_PassKind llvm_Pass_getPassKind(llvm_Pass const* self)
{
    return self->getPassKind();
}

extern "C"
int llvm_PointerType_classof(llvm_Type const* ty)
{
    return (::llvm::PointerType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_PointerType* llvm_PointerType_get(llvm_Type* ElementType, unsigned int AddressSpace)
{
    return ::llvm::PointerType::get(ElementType, AddressSpace);
}

extern "C"
unsigned int llvm_PointerType_getAddressSpace(llvm_PointerType const* self)
{
    return self->getAddressSpace();
}

extern "C"
llvm_PointerType* llvm_PointerType_getUnqual(llvm_Type* ElementType)
{
    return ::llvm::PointerType::getUnqual(ElementType);
}

extern "C"
int llvm_PointerType_isValidElementType(llvm_Type* ty)
{
    return (::llvm::PointerType::isValidElementType(ty) ? 1 : 0);
}

extern "C"
int llvm_SequentialType_classof(llvm_Type const* ty)
{
    return (::llvm::SequentialType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_Type* llvm_SequentialType_getElementType(llvm_SequentialType const* self)
{
    return self->getElementType();
}

extern "C"
int llvm_StructType_classof(llvm_Type const* ty)
{
    return (::llvm::StructType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_StructType* llvm_StructType_create(llvm_LLVMContext* ctx, llvm_ArrayRef_ptr_llvm_Type _Elements, llvm_StringRef _Name)
{
    llvm::ArrayRef<::llvm::Type*> Elements(_Elements.data, _Elements.size);
    llvm::StringRef Name(_Name.data, _Name.length);
    return ::llvm::StructType::create(*ctx, Elements, Name);
}

extern "C"
llvm_StructType* llvm_StructType_createPacked(llvm_LLVMContext* ctx, llvm_ArrayRef_ptr_llvm_Type _Elements, llvm_StringRef _Name, int isPacked)
{
    llvm::ArrayRef<::llvm::Type*> Elements(_Elements.data, _Elements.size);
    llvm::StringRef Name(_Name.data, _Name.length);
    return ::llvm::StructType::create(*ctx, Elements, Name, (isPacked == 1 ? true : false));
}

extern "C"
llvm_Type* llvm_StructType_getElementType(llvm_StructType const* self, unsigned int idx)
{
    return self->getElementType(idx);
}

extern "C"
llvm_StringRef llvm_StructType_getName(llvm_StructType const* self)
{
    return {
        .data = self->getName().data(),
        .length = self->getName().size(),
    };
}

extern "C"
unsigned int llvm_StructType_getNumElements(llvm_StructType const* self)
{
    return self->getNumElements();
}

extern "C"
int llvm_StructType_hasName(llvm_StructType const* self)
{
    return (self->hasName() ? 1 : 0);
}

extern "C"
int llvm_StructType_isLayoutIdentical(llvm_StructType const* self, llvm_StructType* Other)
{
    return (self->isLayoutIdentical(Other) ? 1 : 0);
}

extern "C"
int llvm_StructType_isLiteral(llvm_StructType const* self)
{
    return (self->isLiteral() ? 1 : 0);
}

extern "C"
int llvm_StructType_isOpaque(llvm_StructType const* self)
{
    return (self->isOpaque() ? 1 : 0);
}

extern "C"
int llvm_StructType_isPacked(llvm_StructType const* self)
{
    return (self->isPacked() ? 1 : 0);
}

extern "C"
int llvm_StructType_isSized(llvm_StructType const* self)
{
    return (self->isSized() ? 1 : 0);
}

extern "C"
int llvm_StructType_isValidElementType(llvm_Type* ty)
{
    return (::llvm::StructType::isValidElementType(ty) ? 1 : 0);
}

extern "C"
void llvm_StructType_setBody(llvm_StructType* self, llvm_ArrayRef_ptr_llvm_Type _Elements)
{
    llvm::ArrayRef<::llvm::Type*> Elements(_Elements.data, _Elements.size);
    self->setBody(Elements);
}

extern "C"
void llvm_StructType_setBodyPacked(llvm_StructType* self, llvm_ArrayRef_ptr_llvm_Type _Elements, int isPacked)
{
    llvm::ArrayRef<::llvm::Type*> Elements(_Elements.data, _Elements.size);
    self->setBody(Elements, (isPacked == 1 ? true : false));
}

extern "C"
void llvm_StructType_setName(llvm_StructType* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    self->setName(Name);
}

extern "C"
void llvm_SwitchInst_addCase(llvm_SwitchInst* self, llvm_ConstantInt* OnVal, llvm_BasicBlock* Dest)
{
    self->addCase(OnVal, Dest);
}

extern "C"
void llvm_SwitchInst_delete(llvm_SwitchInst* self)
{
    delete self;
}

extern "C"
llvm_Value* llvm_SwitchInst_getCondition(llvm_SwitchInst const* self)
{
    return self->getCondition();
}

extern "C"
llvm_BasicBlock* llvm_SwitchInst_getDefaultDest(llvm_SwitchInst const* self)
{
    return self->getDefaultDest();
}

extern "C"
unsigned int llvm_SwitchInst_getNumCases(llvm_SwitchInst const* self)
{
    return self->getNumCases();
}

extern "C"
void llvm_SwitchInst_setCondition(llvm_SwitchInst* self, llvm_Value* V)
{
    self->setCondition(V);
}

extern "C"
void llvm_SwitchInst_setDefaultDest(llvm_SwitchInst* self, llvm_BasicBlock* DefaultCase)
{
    self->setDefaultDest(DefaultCase);
}

extern "C"
void llvm_Type_dump(llvm_Type const* self)
{
    self->dump();
}

extern "C"
llvm_Type* llvm_Type_getContainedType(llvm_Type const* self, unsigned int idx)
{
    return self->getContainedType(idx);
}

extern "C"
llvm_LLVMContext* llvm_Type_getContext(llvm_Type const* self)
{
    return &self->getContext();
}

extern "C"
llvm_PointerType* llvm_Type_getDoublePtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getDoublePtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getDoubleTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getDoubleTy(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getFP128PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getFP128PtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getFP128Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getFP128Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getFloatPtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getFloatPtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getFloatTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getFloatTy(*ctx);
}

extern "C"
unsigned int llvm_Type_getFunctionNumParams(llvm_Type const* self)
{
    return self->getFunctionNumParams();
}

extern "C"
llvm_Type* llvm_Type_getFunctionParamType(llvm_Type const* self, unsigned int idx)
{
    return self->getFunctionParamType(idx);
}

extern "C"
llvm_PointerType* llvm_Type_getHalfPtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getHalfPtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getHalfTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getHalfTy(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getInt16PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt16PtrTy(*ctx);
}

extern "C"
llvm_IntegerType* llvm_Type_getInt16Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt16Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getInt1PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt1PtrTy(*ctx);
}

extern "C"
llvm_IntegerType* llvm_Type_getInt1Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt1Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getInt32PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt32PtrTy(*ctx);
}

extern "C"
llvm_IntegerType* llvm_Type_getInt32Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt32Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getInt64PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt64PtrTy(*ctx);
}

extern "C"
llvm_IntegerType* llvm_Type_getInt64Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt64Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getInt8PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt8PtrTy(*ctx);
}

extern "C"
llvm_IntegerType* llvm_Type_getInt8Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getInt8Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getIntNPtrTy(llvm_LLVMContext* ctx, unsigned int size)
{
    return ::llvm::Type::getIntNPtrTy(*ctx, size);
}

extern "C"
llvm_IntegerType* llvm_Type_getIntNTy(llvm_LLVMContext* ctx, unsigned int size)
{
    return ::llvm::Type::getIntNTy(*ctx, size);
}

extern "C"
llvm_Type* llvm_Type_getLabelTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getLabelTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getMetadataTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getMetadataTy(*ctx);
}

extern "C"
unsigned int llvm_Type_getNumContainedTypes(llvm_Type const* self)
{
    return self->getNumContainedTypes();
}

extern "C"
llvm_PointerType* llvm_Type_getPPC_FP128PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getPPC_FP128PtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getPPC_FP128Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getPPC_FP128Ty(*ctx);
}

extern "C"
unsigned int llvm_Type_getPointerAddressSpace(llvm_Type const* self)
{
    return self->getPointerAddressSpace();
}

extern "C"
llvm_Type* llvm_Type_getPointerElementType(llvm_Type const* self)
{
    return self->getPointerElementType();
}

extern "C"
llvm_PointerType* llvm_Type_getPointerTo(llvm_Type* self, unsigned int idx)
{
    return self->getPointerTo(idx);
}

extern "C"
llvm_Type* llvm_Type_getSequentialElementType(llvm_Type const* self)
{
    return self->getSequentialElementType();
}

extern "C"
llvm_Type* llvm_Type_getStructElementType(llvm_Type const* self, unsigned int idx)
{
    return self->getStructElementType(idx);
}

extern "C"
llvm_StringRef llvm_Type_getStructName(llvm_Type const* self)
{
    return {
        .data = self->getStructName().data(),
        .length = self->getStructName().size(),
    };
}

extern "C"
unsigned int llvm_Type_getStructNumElements(llvm_Type const* self)
{
    return self->getStructNumElements();
}

extern "C"
llvm_Type_TypeID llvm_Type_getTypeID(llvm_Type const* self)
{
    return self->getTypeID();
}

extern "C"
llvm_Type* llvm_Type_getVoidTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getVoidTy(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getX86_FP80PtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getX86_FP80PtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getX86_FP80Ty(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getX86_FP80Ty(*ctx);
}

extern "C"
llvm_PointerType* llvm_Type_getX86_MMXPtrTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getX86_MMXPtrTy(*ctx);
}

extern "C"
llvm_Type* llvm_Type_getX86_MMXTy(llvm_LLVMContext* ctx)
{
    return ::llvm::Type::getX86_MMXTy(*ctx);
}

extern "C"
int llvm_Type_isAggregateType(llvm_Type const* self)
{
    return (self->isAggregateType() ? 1 : 0);
}

extern "C"
int llvm_Type_isArrayTy(llvm_Type const* self)
{
    return (self->isArrayTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isDoubleTy(llvm_Type const* self)
{
    return (self->isDoubleTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isEmptyTy(llvm_Type const* self)
{
    return (self->isEmptyTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isFP128Ty(llvm_Type const* self)
{
    return (self->isFP128Ty() ? 1 : 0);
}

extern "C"
int llvm_Type_isFPOrFPVectorTy(llvm_Type const* self)
{
    return (self->isFPOrFPVectorTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isFirstClassType(llvm_Type const* self)
{
    return (self->isFirstClassType() ? 1 : 0);
}

extern "C"
int llvm_Type_isFloatTy(llvm_Type const* self)
{
    return (self->isFloatTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isFloatingPointTy(llvm_Type const* self)
{
    return (self->isFloatingPointTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isFunctionTy(llvm_Type const* self)
{
    return (self->isFunctionTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isFunctionVarArg(llvm_Type const* self)
{
    return (self->isFunctionVarArg() ? 1 : 0);
}

extern "C"
int llvm_Type_isHalfTy(llvm_Type const* self)
{
    return (self->isHalfTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isIntOrIntVectorTy(llvm_Type const* self)
{
    return (self->isIntOrIntVectorTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isIntegerTy(llvm_Type const* self)
{
    return (self->isIntegerTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isLabelTy(llvm_Type const* self)
{
    return (self->isLabelTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isMetadataTy(llvm_Type const* self)
{
    return (self->isMetadataTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isPPC_FP128Ty(llvm_Type const* self)
{
    return (self->isPPC_FP128Ty() ? 1 : 0);
}

extern "C"
int llvm_Type_isPointerTy(llvm_Type const* self)
{
    return (self->isPointerTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isPtrOrPtrVectorTy(llvm_Type const* self)
{
    return (self->isPtrOrPtrVectorTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isSingleValueType(llvm_Type const* self)
{
    return (self->isSingleValueType() ? 1 : 0);
}

extern "C"
int llvm_Type_isSized(llvm_Type const* self)
{
    return (self->isSized() ? 1 : 0);
}

extern "C"
int llvm_Type_isStructTy(llvm_Type const* self)
{
    return (self->isStructTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isVectorTy(llvm_Type const* self)
{
    return (self->isVectorTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isVoidTy(llvm_Type const* self)
{
    return (self->isVoidTy() ? 1 : 0);
}

extern "C"
int llvm_Type_isX86_FP80Ty(llvm_Type const* self)
{
    return (self->isX86_FP80Ty() ? 1 : 0);
}

extern "C"
int llvm_Type_isX86_MMXTy(llvm_Type const* self)
{
    return (self->isX86_MMXTy() ? 1 : 0);
}

extern "C"
llvm_Value* llvm_Use_get(llvm_Use const* self)
{
    return self->get();
}

extern "C"
llvm_Use* llvm_Use_getNext(llvm_Use const* self)
{
    return self->getNext();
}

extern "C"
unsigned int llvm_Use_getOperandNo(llvm_Use const* self)
{
    return self->getOperandNo();
}

extern "C"
llvm_User* llvm_Use_getUser(llvm_Use const* self)
{
    return self->getUser();
}

extern "C"
llvm_Use* llvm_Use_initTags(llvm_Use* Start, llvm_Use* Stop)
{
    return ::llvm::Use::initTags(Start, Stop);
}

extern "C"
void llvm_Use_set(llvm_Use* self, llvm_Value* Val)
{
    self->set(Val);
}

extern "C"
void llvm_Use_swap(llvm_Use* self, llvm_Use* RHS)
{
    self->swap(*RHS);
}

extern "C"
int llvm_User_classof(llvm_Value* V)
{
    return (::llvm::User::classof(V) ? 1 : 0);
}

extern "C"
void llvm_User_delete(llvm_User* self)
{
    delete self;
}

extern "C"
void llvm_User_dropAllReferences(llvm_User* self)
{
    self->dropAllReferences();
}

extern "C"
unsigned int llvm_User_getNumOperands(llvm_User const* self)
{
    return self->getNumOperands();
}

extern "C"
llvm_Value* llvm_User_getOperand(llvm_User const* self, unsigned int idx)
{
    return self->getOperand(idx);
}

extern "C"
void llvm_User_replaceUsesOfWith(llvm_User* self, llvm_Value* From, llvm_Value* To)
{
    self->replaceUsesOfWith(From, To);
}

extern "C"
void llvm_User_setOperand(llvm_User* self, unsigned int idx, llvm_Value* Val)
{
    self->setOperand(idx, Val);
}

extern "C"
void llvm_ValueSymbolTable_delete(llvm_ValueSymbolTable* self)
{
    delete self;
}

extern "C"
void llvm_ValueSymbolTable_dump(llvm_ValueSymbolTable const* self)
{
    self->dump();
}

extern "C"
int llvm_ValueSymbolTable_empty(llvm_ValueSymbolTable const* self)
{
    return (self->empty() ? 1 : 0);
}

extern "C"
llvm_Value* llvm_ValueSymbolTable_lookup(llvm_ValueSymbolTable const* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    return self->lookup(Name);
}

extern "C"
llvm_ValueSymbolTable* llvm_ValueSymbolTable_new()
{
    return new(std::nothrow) ::llvm::ValueSymbolTable();
}

extern "C"
unsigned int llvm_ValueSymbolTable_size(llvm_ValueSymbolTable const* self)
{
    return self->size();
}

extern "C"
void llvm_Value_delete(llvm_Value* self)
{
    delete self;
}

extern "C"
void llvm_Value_dump(llvm_Value const* self)
{
    self->dump();
}

extern "C"
llvm_LLVMContext* llvm_Value_getContext(llvm_Value const* self)
{
    return &self->getContext();
}

extern "C"
llvm_StringRef llvm_Value_getName(llvm_Value const* self)
{
    return {
        .data = self->getName().data(),
        .length = self->getName().size(),
    };
}

extern "C"
unsigned int llvm_Value_getNumUses(llvm_Value const* self)
{
    return self->getNumUses();
}

extern "C"
llvm_Type* llvm_Value_getType(llvm_Value const* self)
{
    return self->getType();
}

extern "C"
unsigned int llvm_Value_getValueID(llvm_Value const* self)
{
    return self->getValueID();
}

extern "C"
int llvm_Value_hasNUses(llvm_Value const* self, unsigned int N)
{
    return (self->hasNUses(N) ? 1 : 0);
}

extern "C"
int llvm_Value_hasNUsesOrMore(llvm_Value const* self, unsigned int N)
{
    return (self->hasNUsesOrMore(N) ? 1 : 0);
}

extern "C"
int llvm_Value_hasName(llvm_Value const* self)
{
    return (self->hasName() ? 1 : 0);
}

extern "C"
int llvm_Value_hasOneUse(llvm_Value const* self)
{
    return (self->hasOneUse() ? 1 : 0);
}

extern "C"
int llvm_Value_isUsedInBasicBlock(llvm_Value const* self, llvm_BasicBlock const* BB)
{
    return (self->isUsedInBasicBlock(BB) ? 1 : 0);
}

extern "C"
void llvm_Value_mutateType(llvm_Value* self, llvm_Type* ty)
{
    self->mutateType(ty);
}

extern "C"
void llvm_Value_replaceAllUsesWith(llvm_Value* self, llvm_Value* Value)
{
    self->replaceAllUsesWith(Value);
}

extern "C"
void llvm_Value_setName(llvm_Value* self, llvm_StringRef _Name)
{
    llvm::StringRef Name(_Name.data, _Name.length);
    self->setName(Name);
}

extern "C"
void llvm_Value_takeName(llvm_Value* self, llvm_Value* Value)
{
    self->takeName(Value);
}

extern "C"
int llvm_VectorType_classof(llvm_Type const* ty)
{
    return (::llvm::VectorType::classof(ty) ? 1 : 0);
}

extern "C"
llvm_VectorType* llvm_VectorType_get(llvm_Type* ty, unsigned int NumElements)
{
    return ::llvm::VectorType::get(ty, NumElements);
}

extern "C"
unsigned int llvm_VectorType_getBitWidth(llvm_VectorType const* self)
{
    return self->getBitWidth();
}

extern "C"
llvm_VectorType* llvm_VectorType_getDoubleElementsVectorType(llvm_VectorType* ty)
{
    return ::llvm::VectorType::getDoubleElementsVectorType(ty);
}

extern "C"
llvm_VectorType* llvm_VectorType_getExtendedElementVectorType(llvm_VectorType* ty)
{
    return ::llvm::VectorType::getExtendedElementVectorType(ty);
}

extern "C"
llvm_VectorType* llvm_VectorType_getHalfElementsVectorType(llvm_VectorType* ty)
{
    return ::llvm::VectorType::getHalfElementsVectorType(ty);
}

extern "C"
llvm_VectorType* llvm_VectorType_getInteger(llvm_VectorType* ty)
{
    return ::llvm::VectorType::getInteger(ty);
}

extern "C"
unsigned int llvm_VectorType_getNumElements(llvm_VectorType const* self)
{
    return self->getNumElements();
}

extern "C"
llvm_VectorType* llvm_VectorType_getTruncatedElementVectorType(llvm_VectorType* ty)
{
    return ::llvm::VectorType::getTruncatedElementVectorType(ty);
}

extern "C"
int llvm_VectorType_isValidElementType(llvm_Type* ty)
{
    return (::llvm::VectorType::isValidElementType(ty) ? 1 : 0);
}

extern "C"
void llvm_iplist_llvm_Argument_clear(llvm_iplist_llvm_Argument* self)
{
    self->clear();
}

extern "C"
void llvm_iplist_llvm_Argument_delete(llvm_iplist_llvm_Argument* self)
{
    delete self;
}

extern "C"
llvm_Argument const* llvm_iplist_llvm_Argument_first(llvm_iplist_llvm_Argument const* self)
{
    return &self->front();
}

extern "C"
llvm_Argument* llvm_iplist_llvm_Argument_firstMut(llvm_iplist_llvm_Argument* self)
{
    return &self->front();
}

extern "C"
llvm_Argument const* llvm_iplist_llvm_Argument_last(llvm_iplist_llvm_Argument const* self)
{
    return &self->back();
}

extern "C"
llvm_Argument* llvm_iplist_llvm_Argument_lastMut(llvm_iplist_llvm_Argument* self)
{
    return &self->back();
}

extern "C"
size_t llvm_iplist_llvm_Argument_max_size(llvm_iplist_llvm_Argument const* self)
{
    return self->max_size();
}

extern "C"
llvm_iplist_llvm_Argument* llvm_iplist_llvm_Argument_new()
{
    return new(std::nothrow) ::llvm::iplist<::llvm::Argument>();
}

extern "C"
size_t llvm_iplist_llvm_Argument_size(llvm_iplist_llvm_Argument const* self)
{
    return self->size();
}
