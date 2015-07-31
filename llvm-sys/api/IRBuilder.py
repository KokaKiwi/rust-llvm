from rust_bindgen import ast
from rust_bindgen.ast import *
from .ns import llvm, CallingConv
from .defs import *
from .ADT.APInt import APInt
from .ADT.ArrayRef import ArrayRef
from .ADT.StringRef import StringRef
# We assume that pointers returned by IRBuilderBase and subclasses are valid.


@IRBuilderBase.body
class IRBuilderBase:
    new = Constructor((ref(LLVMContext), 'Context'))

    getContext = Method(ref(LLVMContext), const=True)

    ClearInsertionPoint = Method()
    GetInsertBlock = Method(sptr(BasicBlock), const=True)
    SetInsertPoint = Method(Void, (ptr(BasicBlock), 'BB'))
    SetInsertPointAtInst = Method(
        Void, (ptr(Instruction), 'Inst')).with_real_name('SetInsertPoint')

    SetCurrentDebugLocation = Method(Void, (ref(DebugLoc, const=True), 'Loc'))
    SetInstDebugLocation = Method(Void, (ptr(Instruction), 'Inst'), const=True)

    getCurrentFunctionReturnType = Method(ptr(Type), const=True)

    # saveIP
    # saveAndClearIP
    # restoreIP

    getDefaultFPMathTag = Method(ptr(MDNode), const=True)
    SetDefaultFPMathTag = Method(Void, (ptr(MDNode), 'FPMathTag'))

    CreateGlobalString = Method(
        sptr(Value), (StringRef, 'Str'), (OptionString(), 'Name'))

    getInt1 = Method(sptr(ConstantInt), (Bool, 'Value'))
    getTrue = Method(sptr(ConstantInt))
    getFalse = Method(sptr(ConstantInt))

    getInt8 = Method(sptr(ConstantInt), (UnsignedInt8, 'Value'))
    getInt16 = Method(sptr(ConstantInt), (UnsignedInt16, 'Value'))
    getInt32 = Method(sptr(ConstantInt), (UnsignedInt32, 'Value'))
    getInt64 = Method(sptr(ConstantInt), (UnsignedInt64, 'Value'))
    getIntN = Method(
        sptr(ConstantInt), (UnsignedInt, 'NumBits'), (UnsignedInt64, 'Value'))
    getInt = Method(sptr(ConstantInt), (APInt, 'Value'))

    getInt1Ty = Method(sptr(IntegerType))
    getInt8Ty = Method(sptr(IntegerType))
    getInt16Ty = Method(sptr(IntegerType))
    getInt32Ty = Method(sptr(IntegerType))
    getInt64Ty = Method(sptr(IntegerType))
    getIntNTy = Method(sptr(IntegerType), (UnsignedInt, 'NumBits'))

    getHalfTy = Method(sptr(Type))
    getFloatTy = Method(sptr(Type))
    getDoubleTy = Method(sptr(Type))

    getVoidTy = Method(sptr(Type))

    getInt8PtrTy = Method(
        sptr(PointerType), (Option(UnsignedInt, '0'), 'AddrSpace'))
    getIntPtrTy = Method(sptr(IntegerType), (ptr(
        DataLayout, const=True), 'DL'), (Option(UnsignedInt, '0'), 'AddrSpace'))

    CreateMemSet = Method(sptr(CallInst), (ptr(Value), 'Ptr'), (ptr(Value), 'Value'), (ptr(
        Value), 'Size'), (UnsignedInt, 'Align'), (Option(Bool, 'false'), 'isVolatile'))
    CreateMemCpy = Method(sptr(CallInst), (ptr(Value), 'Dst'), (ptr(Value), 'Src'), (ptr(
        Value), 'Size'), (UnsignedInt, 'Align'), (Option(Bool, 'false'), 'isVolatile'))
    CreateMemMove = Method(sptr(CallInst), (ptr(Value), 'Dst'), (ptr(Value), 'Src'), (ptr(
        Value), 'Size'), (UnsignedInt, 'Align'), (Option(Bool, 'false'), 'isVolatile'))

    CreateLifetimeStart = Method(
        sptr(CallInst), (ptr(Value), 'Ptr'), (OptionPointer(ConstantInt), 'Size'))
    CreateLifetimeEnd = Method(
        sptr(CallInst), (ptr(Value), 'Ptr'), (OptionPointer(ConstantInt), 'Size'))


@IRBuilder.body
class IRBuilder:
    new = Constructor((ref(LLVMContext), 'Context'))
    new_in_block = Constructor((ptr(BasicBlock), 'BB'))
    delete = Destructor()

    isNamePreserving = Method(Bool, const=True)

    CreateRetVoid = Method(sptr(ReturnInst))
    CreateRet = Method(sptr(ReturnInst), (ptr(Value), 'Value'))
    # CreateAggregateRet = CreateAggregateRet()

    CreateBr = Method(sptr(BranchInst), (ptr(BasicBlock), 'Dest'))
    CreateCondBr = Method(sptr(BranchInst), (ptr(Value), 'Cond'), (ptr(
        BasicBlock), 'TrueBlock'), (ptr(BasicBlock), 'FalseBlock'))

    CreateSwitch = Method(sptr(SwitchInst), (ptr(Value), 'Value'), (ptr(
        BasicBlock), 'Dest'), (Option(UnsignedInt, '10'), 'NumCases'))

    CreateIndirectBr = Method(sptr(
        IndirectBrInst), (ptr(Value), 'Addr'), (Option(UnsignedInt, '10'), 'NumCases'))

    CreateInvoke = Method(sptr(InvokeInst), (ptr(Value), 'Callee'), (ptr(BasicBlock), 'NormalDest'), (ptr(
        BasicBlock), 'UnwindDest'), (ArrayRef(ptr(Value)), 'Args'), (OptionString(const=True), 'Name'))

    CreateResume = Method(sptr(ResumeInst), (ptr(Value), 'Exn'))
    CreateUnreachable = Method(sptr(UnreachableInst))

    CreateAdd = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNSWAdd = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNUWAdd = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFAdd = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateSub = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNSWSub = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNUWSub = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFSub = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateMul = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNSWMul = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateNUWMul = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFMul = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateUDiv = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateExactUDiv = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateSDiv = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateExactSDiv = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFDiv = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateURem = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateSRem = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFRem = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateShl = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateShlByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateShl')

    CreateLShr = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateLShrByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateLShr')

    CreateAShr = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateAShrByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateAShr')

    CreateAnd = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateAndByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateAnd')

    CreateOr = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateOrByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateOr')

    CreateXor = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateXorByValue = Method(sptr(Value), (ptr(
        Value), 'LHS'), (UnsignedInt64, 'RHS'), (OptionString(), 'Name')).with_real_name('CreateXor')

    CreateBinOp = Method(sptr(Value), (Instruction['BinaryOps'], 'Opcode'), (ptr(
        Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateNeg = Method(
        sptr(Value), (ptr(Value), 'Value'), (OptionString(), 'Name'))
    CreateNSWNeg = Method(
        sptr(Value), (ptr(Value), 'Value'), (OptionString(), 'Name'))
    CreateNUWNeg = Method(
        sptr(Value), (ptr(Value), 'Value'), (OptionString(), 'Name'))
    CreateFNeg = Method(
        sptr(Value), (ptr(Value), 'Value'), (OptionString(), 'Name'))

    CreateNot = Method(
        sptr(Value), (ptr(Value), 'Value'), (OptionString(), 'Name'))

    CreateAlloca = Method(sptr(AllocaInst), (ptr(Type), 'Ty'), (OptionPointer(
        Value), 'ArraySize'), (OptionString(), 'Name'))

    CreateLoad = Method(
        sptr(LoadInst), (ptr(Value), 'Ptr'), (OptionString(), 'Name'))
    CreateLoadVolatile = Method(sptr(LoadInst), (ptr(
        Value), 'Ptr'), (Bool, 'isVolatile'), (OptionString(), 'Name')).with_real_name('CreateLoad')
    CreateStore = Method(sptr(StoreInst), (ptr(Value), 'Value'), (ptr(
        Value), 'Ptr'), (Option(Bool, 'false'), 'isVolatile'))

    CreateAlignedLoad = Method(sptr(
        LoadInst), (ptr(Value), 'Ptr'), (UnsignedInt, 'Align'), (OptionString(), 'Name'))
    CreateAlignedLoadVolatile = Method(sptr(LoadInst), (ptr(Value), 'Ptr'), (UnsignedInt, 'Align'), (
        Bool, 'isVolatile'), (OptionString(), 'Name')).with_real_name('CreateAlignedLoad')
    CreateAlignedStore = Method(sptr(StoreInst), (ptr(Value), 'Value'), (ptr(
        Value), 'Ptr'), (UnsignedInt, 'Align'), (Option(Bool, 'false'), 'isVolatile'))

    CreateFence = Method(sptr(FenceInst), (llvm['AtomicOrdering'], 'Ordering'), (Option(
        llvm['SynchronizationScope'], 'llvm::SynchronizationScope::CrossThread'), 'SynchScope'), (OptionString(), 'Name'))

    # CreateAtomicCmpXchg
    # CreateAtomicRMW

    CreateGEP = Method(sptr(Value), (ptr(Value), 'Ptr'), (ArrayRef(
        ptr(Value)), 'Indexes'), (OptionString(), 'Name'))
    CreateInBoundsGEP = Method(sptr(Value), (ptr(Value), 'Ptr'), (ArrayRef(
        ptr(Value)), 'Indexes'), (OptionString(), 'Name'))

    CreateStructGEP = Method(sptr(
        Value), (ptr(Value), 'Ptr'), (UnsignedInt, 'Index'), (OptionString(), 'Name'))

    CreateGlobalStringPtr = Method(
        sptr(Value), (StringRef, 'Str'), (OptionString(), 'Name'))

    CreateTrunc = Method(sptr(Value), (ptr(Value), 'Value'),
                         (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateZExt = Method(sptr(Value), (ptr(Value), 'Value'),
                        (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateSExt = Method(sptr(Value), (ptr(Value), 'Value'),
                        (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateZExtOrTrunc = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateSExtOrTrunc = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))

    CreateFPToUI = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateFPToSI = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateUIToFP = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateSIToFP = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))

    CreateFPTrunc = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateFPExt = Method(sptr(Value), (ptr(Value), 'Value'),
                         (ptr(Type), 'DestTy'), (OptionString(), 'Name'))

    CreatePtrToInt = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateIntToPtr = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateBitCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateAddrSpaceCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateZExtOrBitCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateSExtOrBitCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateTruncOrBitCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))

    CreateCast = Method(sptr(Value), (Instruction['CastOps'], 'Opcode'), (ptr(
        Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreatePointerCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreatePointerBitCastOrAddrSpaceCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))
    CreateIntCast = Method(sptr(Value), (ptr(Value), 'Value'), (ptr(
        Type), 'DestTy'), (Bool, 'isSigned'), (OptionString(), 'Name'))
    CreateFPCast = Method(sptr(
        Value), (ptr(Value), 'Value'), (ptr(Type), 'DestTy'), (OptionString(), 'Name'))

    CreateICmpEQ = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpNE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpUGT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpUGE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpULT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpULE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpSGT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpSGE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpSLT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateICmpSLE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpOEQ = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpOGT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpOGE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpOLT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpOLE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpONE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpORD = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpUNO = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpUEQ = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpUGT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpUGE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpULT = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpULE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmpUNE = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreateICmp = Method(sptr(Value), (CmpInst['Predicate'], 'Pred'), (ptr(
        Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateFCmp = Method(sptr(Value), (CmpInst['Predicate'], 'Pred'), (ptr(
        Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))

    CreatePHI = Method(sptr(PHINode), (ptr(
        Type), 'Ty'), (UnsignedInt, 'NumReservedValues'), (OptionString(), 'Name'))

    CreateCall = Method(sptr(CallInst), (ptr(Value), 'Callee'), (ArrayRef(
        ptr(Value)), 'Args'), (OptionString(), 'Name'))

    CreateSelect = Method(sptr(Value), (ptr(Value), 'C'), (ptr(
        Value), 'TrueValue'), (ptr(Value), 'FalseValue'), (OptionString(), 'Name'))

    CreateVAArg = Method(sptr(
        VAArgInst), (ptr(Value), 'List'), (ptr(Type), 'Ty'), (OptionString(), 'Name'))

    CreateExtractElement = Method(
        sptr(Value), (ptr(Value), 'Vec'), (ptr(Value), 'Idx'), (OptionString(), 'Name'))
    CreateInsertElement = Method(sptr(Value), (ptr(Value), 'Vec'), (ptr(
        Value), 'NewElt'), (ptr(Value), 'Idx'), (OptionString(), 'Name'))
    CreateShuffleVector = Method(sptr(Value), (ptr(Value), 'V1'), (ptr(
        Value), 'P2'), (ptr(Value), 'Mask'), (OptionString(), 'Name'))

    CreateExtractValue = Method(sptr(Value), (ptr(Value), 'Agg'), (ArrayRef(
        UnsignedInt), 'Indexes'), (OptionString(), 'Name'))
    CreateInsertValue = Method(sptr(Value), (ptr(Value), 'Agg'), (ptr(
        Value), 'Value'), (ArrayRef(UnsignedInt), 'Indexes'), (OptionString(), 'Name'))

    CreateLandingPad = Method(sptr(LandingPadInst), (ptr(Type), 'Ty'), (ptr(
        Value), 'PersFn'), (UnsignedInt, 'NumClauses'), (OptionString(), 'Name'))

    CreateIsNull = Method(
        sptr(Value), (ptr(Value), 'Arg'), (OptionString(), 'Name'))
    CreateIsNotNull = Method(
        sptr(Value), (ptr(Value), 'Arg'), (OptionString(), 'Name'))

    CreatePtrDiff = Method(
        sptr(Value), (ptr(Value), 'LHS'), (ptr(Value), 'RHS'), (OptionString(), 'Name'))
    CreateVectorSplat = Method(sptr(Value), (UnsignedInt, 'NumElements'), (ptr(
        Value), 'Value'), (OptionString(), 'Name'))

    CreateExtractInteger = Method(sptr(Value), (ref(DataLayout, const=True), 'DL'), (ptr(
        Value), 'From'), (ptr(IntegerType), 'ExtractedTy'), (UnsignedInt64, 'Offset'), (String(), 'Name'))
