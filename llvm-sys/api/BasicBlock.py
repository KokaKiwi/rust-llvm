from bindgen.ast import *
from .defs import *
from .ADT.StringRef import StringRef


@BasicBlock.body
class BasicBlock:
    delete = Destructor()

    Create = StaticMethod(sptr(BasicBlock), (ref(LLVMContext), 'Context'), (OptionString(
    ), 'Name'), (OptionPointer(Function), 'Parent'), (OptionPointer(BasicBlock), 'InsertBefore'))

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'Val'))

    getDataLayout = Method(ptr(DataLayout, const=True), const=True)

    getParent = Method(ptr(Function, const=True), const=True)
    getParentMut = Method(ptr(Function)).with_real_name('getParent')

    getTerminator = Method(ptr(TerminatorInst, const=True), const=True)
    getTerminatorMut = Method(
        ptr(TerminatorInst)).with_real_name('getTerminator')

    getFirstNonPHI = Method(ptr(Instruction, const=True), const=True)
    getFirstNonPHIMut = Method(
        ptr(Instruction)).with_real_name('getFirstNonPHI')

    getFirstNonPHIOrDbg = Method(ptr(Instruction, const=True), const=True)
    getFirstNonPHIOrDbgMut = Method(
        ptr(Instruction)).with_real_name('getFirstNonPHIOrDbg')

    getFirstNonPHIOrDbgOrLifetime = Method(
        ptr(Instruction, const=True), const=True)
    getFirstNonPHIOrDbgOrLifetimeMut = Method(
        ptr(Instruction)).with_real_name('getFirstNonPHIOrDbgOrLifetime')

    removeFromParent = Method()
    eraseFromParent = Method()

    moveBefore = Method(Void, (ptr(BasicBlock), 'MovePos'))
    moveAfter = Method(Void, (ptr(BasicBlock), 'MovePos'))

    getSinglePredecessor = Method(ptr(BasicBlock, const=True), const=True)
    getSinglePredecessorMut = Method(
        ptr(BasicBlock)).with_real_name('getSinglePredecessor')

    getUniquePredecessor = Method(ptr(BasicBlock, const=True), const=True)
    getUniquePredecessorMut = Method(
        ptr(BasicBlock)).with_real_name('getUniquePredecessor')

    getValueSymbolTable = Method(ptr(ValueSymbolTable))

    dropAllReferences = Method()
    removePredecessor = Method(
        Void, (ptr(BasicBlock), 'Pred'), (Option(Bool, 'false'), 'DontDeleteUselessPHIs'))

    # splitBasicBlock

    hasAddressTaken = Method(Bool, const=True)

    replaceSuccessorsPhiUsesWith = Method(Void, (ptr(BasicBlock), 'New'))

    isLandingPad = Method(Bool, const=True)
    getLandingPadInst = Method(ptr(LandingPadInst, const=True), const=True)
    getLandingPadInstMut = Method(
        ptr(LandingPadInst)).with_real_name('getLandingPadInst')
