#include <string>
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
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

struct llvm_ArrayRef__libc_uint64_t {
    uint64_t const* data;
    size_t length;
};
struct std_string {
    char* data;
    size_t length;
};
struct llvm_APInt {
    unsigned int num_bits;
    llvm_ArrayRef__libc_uint64_t value;
};
struct llvm_ArrayRef_llvm_Constant_ptr {
    ::llvm::Constant* const* data;
    size_t length;
};
struct llvm_StringRef {
    char const* data;
    size_t length;
};
struct llvm_ArrayRef_llvm_Type_ptr {
    ::llvm::Type* const* data;
    size_t length;
};
struct llvm_ArrayRef_llvm_Value_ptr {
    ::llvm::Value* const* data;
    size_t length;
};
struct llvm_ArrayRef__libc_c_uint {
    unsigned int const* data;
    size_t length;
};
struct std_string_const {
    char const* data;
    size_t length;
};

// ::llvm::ArrayType::classof
extern "C"
int llvm_ArrayType_classof(::llvm::Type const* ty)
{
    return (::llvm::ArrayType::classof(ty) == true ? 1 : 0);
}

// ::llvm::ArrayType::get
extern "C"
::llvm::ArrayType* llvm_ArrayType_get(::llvm::Type* ElementType, uint64_t NumElements)
{
    return ::llvm::ArrayType::get(ElementType, NumElements);
}

// ::llvm::ArrayType::getNumElements
extern "C"
uint64_t llvm_ArrayType_getNumElements(::llvm::ArrayType const* inst)
{
    return inst->getNumElements();
}

// ::llvm::ArrayType::isValidElementType
extern "C"
int llvm_ArrayType_isValidElementType(::llvm::Type* ty)
{
    return (::llvm::ArrayType::isValidElementType(ty) == true ? 1 : 0);
}

// ::llvm::BasicBlock::Create
extern "C"
::llvm::BasicBlock* llvm_BasicBlock_Create(::llvm::LLVMContext* Context, std_string _Name, ::llvm::Function* Parent, ::llvm::BasicBlock* InsertBefore)
{
    auto Name = std::string(_Name.data, _Name.length);
    return ::llvm::BasicBlock::Create(*Context, Name, Parent, InsertBefore);
}

// ::llvm::BasicBlock::classof
extern "C"
int llvm_BasicBlock_classof(::llvm::Value const* Val)
{
    return (::llvm::BasicBlock::classof(Val) == true ? 1 : 0);
}

// ::llvm::BasicBlock::delete
extern "C"
void llvm_BasicBlock_delete(::llvm::BasicBlock* inst)
{
    delete inst;
}

// ::llvm::BasicBlock::dropAllReferences
extern "C"
void llvm_BasicBlock_dropAllReferences(::llvm::BasicBlock* inst)
{
    inst->dropAllReferences();
}

// ::llvm::BasicBlock::eraseFromParent
extern "C"
void llvm_BasicBlock_eraseFromParent(::llvm::BasicBlock* inst)
{
    inst->eraseFromParent();
}

// ::llvm::BasicBlock::getDataLayout
extern "C"
::llvm::DataLayout const* llvm_BasicBlock_getDataLayout(::llvm::BasicBlock const* inst)
{
    return inst->getDataLayout();
}

// ::llvm::BasicBlock::getFirstNonPHI
extern "C"
::llvm::Instruction const* llvm_BasicBlock_getFirstNonPHI(::llvm::BasicBlock const* inst)
{
    return inst->getFirstNonPHI();
}

// ::llvm::BasicBlock::getFirstNonPHIMut
extern "C"
::llvm::Instruction* llvm_BasicBlock_getFirstNonPHIMut(::llvm::BasicBlock* inst)
{
    return inst->getFirstNonPHI();
}

// ::llvm::BasicBlock::getFirstNonPHIOrDbg
extern "C"
::llvm::Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbg(::llvm::BasicBlock const* inst)
{
    return inst->getFirstNonPHIOrDbg();
}

// ::llvm::BasicBlock::getFirstNonPHIOrDbgMut
extern "C"
::llvm::Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgMut(::llvm::BasicBlock* inst)
{
    return inst->getFirstNonPHIOrDbg();
}

// ::llvm::BasicBlock::getFirstNonPHIOrDbgOrLifetime
extern "C"
::llvm::Instruction const* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetime(::llvm::BasicBlock const* inst)
{
    return inst->getFirstNonPHIOrDbgOrLifetime();
}

// ::llvm::BasicBlock::getFirstNonPHIOrDbgOrLifetimeMut
extern "C"
::llvm::Instruction* llvm_BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(::llvm::BasicBlock* inst)
{
    return inst->getFirstNonPHIOrDbgOrLifetime();
}

// ::llvm::BasicBlock::getLandingPadInst
extern "C"
::llvm::LandingPadInst const* llvm_BasicBlock_getLandingPadInst(::llvm::BasicBlock const* inst)
{
    return inst->getLandingPadInst();
}

// ::llvm::BasicBlock::getLandingPadInstMut
extern "C"
::llvm::LandingPadInst* llvm_BasicBlock_getLandingPadInstMut(::llvm::BasicBlock* inst)
{
    return inst->getLandingPadInst();
}

// ::llvm::BasicBlock::getParent
extern "C"
::llvm::Function const* llvm_BasicBlock_getParent(::llvm::BasicBlock const* inst)
{
    return inst->getParent();
}

// ::llvm::BasicBlock::getParentMut
extern "C"
::llvm::Function* llvm_BasicBlock_getParentMut(::llvm::BasicBlock* inst)
{
    return inst->getParent();
}

// ::llvm::BasicBlock::getSinglePredecessor
extern "C"
::llvm::BasicBlock const* llvm_BasicBlock_getSinglePredecessor(::llvm::BasicBlock const* inst)
{
    return inst->getSinglePredecessor();
}

// ::llvm::BasicBlock::getSinglePredecessorMut
extern "C"
::llvm::BasicBlock* llvm_BasicBlock_getSinglePredecessorMut(::llvm::BasicBlock* inst)
{
    return inst->getSinglePredecessor();
}

// ::llvm::BasicBlock::getTerminator
extern "C"
::llvm::TerminatorInst const* llvm_BasicBlock_getTerminator(::llvm::BasicBlock const* inst)
{
    return inst->getTerminator();
}

// ::llvm::BasicBlock::getTerminatorMut
extern "C"
::llvm::TerminatorInst* llvm_BasicBlock_getTerminatorMut(::llvm::BasicBlock* inst)
{
    return inst->getTerminator();
}

// ::llvm::BasicBlock::getUniquePredecessor
extern "C"
::llvm::BasicBlock const* llvm_BasicBlock_getUniquePredecessor(::llvm::BasicBlock const* inst)
{
    return inst->getUniquePredecessor();
}

// ::llvm::BasicBlock::getUniquePredecessorMut
extern "C"
::llvm::BasicBlock* llvm_BasicBlock_getUniquePredecessorMut(::llvm::BasicBlock* inst)
{
    return inst->getUniquePredecessor();
}

// ::llvm::BasicBlock::getValueSymbolTable
extern "C"
::llvm::ValueSymbolTable* llvm_BasicBlock_getValueSymbolTable(::llvm::BasicBlock* inst)
{
    return inst->getValueSymbolTable();
}

// ::llvm::BasicBlock::hasAddressTaken
extern "C"
int llvm_BasicBlock_hasAddressTaken(::llvm::BasicBlock const* inst)
{
    return (inst->hasAddressTaken() == true ? 1 : 0);
}

// ::llvm::BasicBlock::isLandingPad
extern "C"
int llvm_BasicBlock_isLandingPad(::llvm::BasicBlock const* inst)
{
    return (inst->isLandingPad() == true ? 1 : 0);
}

// ::llvm::BasicBlock::moveAfter
extern "C"
void llvm_BasicBlock_moveAfter(::llvm::BasicBlock* inst, ::llvm::BasicBlock* MovePos)
{
    inst->moveAfter(MovePos);
}

// ::llvm::BasicBlock::moveBefore
extern "C"
void llvm_BasicBlock_moveBefore(::llvm::BasicBlock* inst, ::llvm::BasicBlock* MovePos)
{
    inst->moveBefore(MovePos);
}

// ::llvm::BasicBlock::removeFromParent
extern "C"
void llvm_BasicBlock_removeFromParent(::llvm::BasicBlock* inst)
{
    inst->removeFromParent();
}

// ::llvm::BasicBlock::removePredecessor
extern "C"
void llvm_BasicBlock_removePredecessor(::llvm::BasicBlock* inst, ::llvm::BasicBlock* Pred, int DontDeleteUselessPHIs)
{
    inst->removePredecessor(Pred, (DontDeleteUselessPHIs == 1 ? true : false));
}

// ::llvm::BasicBlock::replaceSuccessorsPhiUsesWith
extern "C"
void llvm_BasicBlock_replaceSuccessorsPhiUsesWith(::llvm::BasicBlock* inst, ::llvm::BasicBlock* New)
{
    inst->replaceSuccessorsPhiUsesWith(New);
}

// ::llvm::BlockAddress::destroyConstant
extern "C"
void llvm_BlockAddress_destroyConstant(::llvm::BlockAddress* inst)
{
    inst->destroyConstant();
}

// ::llvm::BlockAddress::getBasicBlock
extern "C"
::llvm::BasicBlock* llvm_BlockAddress_getBasicBlock(::llvm::BlockAddress const* inst)
{
    return inst->getBasicBlock();
}

// ::llvm::BlockAddress::getFunction
extern "C"
::llvm::Function* llvm_BlockAddress_getFunction(::llvm::BlockAddress const* inst)
{
    return inst->getFunction();
}

// ::llvm::CompositeType::classof
extern "C"
int llvm_CompositeType_classof(::llvm::Type const* ty)
{
    return (::llvm::CompositeType::classof(ty) == true ? 1 : 0);
}

// ::llvm::CompositeType::getTypeAtIndex
extern "C"
::llvm::Type* llvm_CompositeType_getTypeAtIndex(::llvm::CompositeType* inst, unsigned int idx)
{
    return inst->getTypeAtIndex(idx);
}

// ::llvm::CompositeType::indexValid
extern "C"
int llvm_CompositeType_indexValid(::llvm::CompositeType const* inst, unsigned int idx)
{
    return (inst->indexValid(idx) == true ? 1 : 0);
}

// ::llvm::Constant::canTrap
extern "C"
int llvm_Constant_canTrap(::llvm::Constant const* inst)
{
    return (inst->canTrap() == true ? 1 : 0);
}

// ::llvm::Constant::classof
extern "C"
int llvm_Constant_classof(::llvm::Value const* V)
{
    return (::llvm::Constant::classof(V) == true ? 1 : 0);
}

// ::llvm::Constant::destroyConstant
extern "C"
void llvm_Constant_destroyConstant(::llvm::Constant* inst)
{
    inst->destroyConstant();
}

// ::llvm::Constant::getAggregateElement
extern "C"
::llvm::Constant* llvm_Constant_getAggregateElement(::llvm::Constant const* inst, unsigned int Elt)
{
    return inst->getAggregateElement(Elt);
}

// ::llvm::Constant::getAggregateElementConstant
extern "C"
::llvm::Constant* llvm_Constant_getAggregateElementConstant(::llvm::Constant const* inst, ::llvm::Constant* Elt)
{
    return inst->getAggregateElement(Elt);
}

// ::llvm::Constant::getAllOnesValue
extern "C"
::llvm::Constant* llvm_Constant_getAllOnesValue(::llvm::Type* Ty)
{
    return ::llvm::Constant::getAllOnesValue(Ty);
}

// ::llvm::Constant::getIntegerValue
extern "C"
::llvm::Constant* llvm_Constant_getIntegerValue(::llvm::Type* Ty, llvm_APInt _Value)
{
    auto Value = ::llvm::APInt(_Value.num_bits, ::llvm::ArrayRef<uint64_t>(_Value.value.data, _Value.value.length));
    return ::llvm::Constant::getIntegerValue(Ty, Value);
}

// ::llvm::Constant::getNullValue
extern "C"
::llvm::Constant* llvm_Constant_getNullValue(::llvm::Type* Ty)
{
    return ::llvm::Constant::getNullValue(Ty);
}

// ::llvm::Constant::getSplatValue
extern "C"
::llvm::Constant* llvm_Constant_getSplatValue(::llvm::Constant const* inst)
{
    return inst->getSplatValue();
}

// ::llvm::Constant::isAllOnesValue
extern "C"
int llvm_Constant_isAllOnesValue(::llvm::Constant const* inst)
{
    return (inst->isAllOnesValue() == true ? 1 : 0);
}

// ::llvm::Constant::isConstantUsed
extern "C"
int llvm_Constant_isConstantUsed(::llvm::Constant const* inst)
{
    return (inst->isConstantUsed() == true ? 1 : 0);
}

// ::llvm::Constant::isDLLImportDependent
extern "C"
int llvm_Constant_isDLLImportDependent(::llvm::Constant const* inst)
{
    return (inst->isDLLImportDependent() == true ? 1 : 0);
}

// ::llvm::Constant::isMinSignedValue
extern "C"
int llvm_Constant_isMinSignedValue(::llvm::Constant const* inst)
{
    return (inst->isMinSignedValue() == true ? 1 : 0);
}

// ::llvm::Constant::isNegativeZeroValue
extern "C"
int llvm_Constant_isNegativeZeroValue(::llvm::Constant const* inst)
{
    return (inst->isNegativeZeroValue() == true ? 1 : 0);
}

// ::llvm::Constant::isNullValue
extern "C"
int llvm_Constant_isNullValue(::llvm::Constant const* inst)
{
    return (inst->isNullValue() == true ? 1 : 0);
}

// ::llvm::Constant::isThreadDependent
extern "C"
int llvm_Constant_isThreadDependent(::llvm::Constant const* inst)
{
    return (inst->isThreadDependent() == true ? 1 : 0);
}

// ::llvm::Constant::isZeroValue
extern "C"
int llvm_Constant_isZeroValue(::llvm::Constant const* inst)
{
    return (inst->isZeroValue() == true ? 1 : 0);
}

// ::llvm::Constant::removeDeadConstantUsers
extern "C"
void llvm_Constant_removeDeadConstantUsers(::llvm::Constant const* inst)
{
    inst->removeDeadConstantUsers();
}

// ::llvm::Constant::stripPointerCasts
extern "C"
::llvm::Constant const* llvm_Constant_stripPointerCasts(::llvm::Constant const* inst)
{
    return inst->stripPointerCasts();
}

// ::llvm::Constant::stripPointerCastsMut
extern "C"
::llvm::Constant* llvm_Constant_stripPointerCastsMut(::llvm::Constant* inst)
{
    return inst->stripPointerCasts();
}

// ::llvm::ConstantArray::classof
extern "C"
int llvm_ConstantArray_classof(::llvm::Value const* V)
{
    return (::llvm::ConstantArray::classof(V) == true ? 1 : 0);
}

// ::llvm::ConstantArray::get
extern "C"
::llvm::Constant* llvm_ConstantArray_get(::llvm::ArrayType* Ty, llvm_ArrayRef_llvm_Constant_ptr _Values)
{
    auto Values = ::llvm::ArrayRef<::llvm::Constant*>(_Values.data, _Values.length);
    return ::llvm::ConstantArray::get(Ty, Values);
}

// ::llvm::ConstantArray::getType
extern "C"
::llvm::Type* llvm_ConstantArray_getType(::llvm::ConstantArray const* inst)
{
    return inst->getType();
}

// ::llvm::ConstantFP::classof
extern "C"
int llvm_ConstantFP_classof(::llvm::Value const* V)
{
    return (::llvm::ConstantFP::classof(V) == true ? 1 : 0);
}

// ::llvm::ConstantFP::fromStr
extern "C"
::llvm::Constant* llvm_ConstantFP_fromStr(::llvm::Type* Ty, llvm_StringRef _Val)
{
    auto Val = ::llvm::StringRef(_Val.data, _Val.length);
    return ::llvm::ConstantFP::get(Ty, Val);
}

// ::llvm::ConstantFP::get
extern "C"
::llvm::Constant* llvm_ConstantFP_get(::llvm::Type* Ty, double Val)
{
    return ::llvm::ConstantFP::get(Ty, Val);
}

// ::llvm::ConstantFP::getInfinity
extern "C"
::llvm::Constant* llvm_ConstantFP_getInfinity(::llvm::Type* Ty)
{
    return ::llvm::ConstantFP::getInfinity(Ty);
}

// ::llvm::ConstantFP::getNegativeZero
extern "C"
::llvm::Constant* llvm_ConstantFP_getNegativeZero(::llvm::Type* Ty)
{
    return ::llvm::ConstantFP::getNegativeZero(Ty);
}

// ::llvm::ConstantFP::getZeroValueForNegation
extern "C"
::llvm::Constant* llvm_ConstantFP_getZeroValueForNegation(::llvm::Type* Ty)
{
    return ::llvm::ConstantFP::getZeroValueForNegation(Ty);
}

// ::llvm::ConstantFP::isExactlyValueFloat
extern "C"
int llvm_ConstantFP_isExactlyValueFloat(::llvm::ConstantFP const* inst, double Val)
{
    return (inst->isExactlyValue(Val) == true ? 1 : 0);
}

// ::llvm::ConstantFP::isNaN
extern "C"
int llvm_ConstantFP_isNaN(::llvm::ConstantFP const* inst)
{
    return (inst->isNaN() == true ? 1 : 0);
}

// ::llvm::ConstantFP::isNegative
extern "C"
int llvm_ConstantFP_isNegative(::llvm::ConstantFP const* inst)
{
    return (inst->isNegative() == true ? 1 : 0);
}

// ::llvm::ConstantFP::isZero
extern "C"
int llvm_ConstantFP_isZero(::llvm::ConstantFP const* inst)
{
    return (inst->isZero() == true ? 1 : 0);
}

// ::llvm::ConstantInt::classof
extern "C"
int llvm_ConstantInt_classof(::llvm::Value const* Val)
{
    return (::llvm::ConstantInt::classof(Val) == true ? 1 : 0);
}

// ::llvm::ConstantInt::equalsInt
extern "C"
int llvm_ConstantInt_equalsInt(::llvm::ConstantInt const* inst, uint64_t Val)
{
    return (inst->equalsInt(Val) == true ? 1 : 0);
}

// ::llvm::ConstantInt::fromAPInt
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_fromAPInt(::llvm::LLVMContext* Context, llvm_APInt _Val)
{
    auto Val = ::llvm::APInt(_Val.num_bits, ::llvm::ArrayRef<uint64_t>(_Val.value.data, _Val.value.length));
    return ::llvm::ConstantInt::get(*Context, Val);
}

// ::llvm::ConstantInt::fromStr
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_fromStr(::llvm::IntegerType* Ty, llvm_StringRef _Str, uint8_t radix)
{
    auto Str = ::llvm::StringRef(_Str.data, _Str.length);
    return ::llvm::ConstantInt::get(Ty, Str, radix);
}

// ::llvm::ConstantInt::get
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_get(::llvm::IntegerType* Ty, uint64_t Value)
{
    return ::llvm::ConstantInt::get(Ty, Value);
}

// ::llvm::ConstantInt::getBitWidth
extern "C"
unsigned int llvm_ConstantInt_getBitWidth(::llvm::ConstantInt const* inst)
{
    return inst->getBitWidth();
}

// ::llvm::ConstantInt::getFalse
extern "C"
::llvm::Constant* llvm_ConstantInt_getFalse(::llvm::Type* Ty)
{
    return ::llvm::ConstantInt::getFalse(Ty);
}

// ::llvm::ConstantInt::getFalseWithContext
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_getFalseWithContext(::llvm::LLVMContext* Context)
{
    return ::llvm::ConstantInt::getFalse(*Context);
}

// ::llvm::ConstantInt::getSExtValue
extern "C"
int64_t llvm_ConstantInt_getSExtValue(::llvm::ConstantInt const* inst)
{
    return inst->getSExtValue();
}

// ::llvm::ConstantInt::getSigned
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_getSigned(::llvm::IntegerType* Ty, uint64_t Value, int isSigned)
{
    return ::llvm::ConstantInt::get(Ty, Value, (isSigned == 1 ? true : false));
}

// ::llvm::ConstantInt::getTrue
extern "C"
::llvm::Constant* llvm_ConstantInt_getTrue(::llvm::Type* Ty)
{
    return ::llvm::ConstantInt::getTrue(Ty);
}

// ::llvm::ConstantInt::getTrueWithContext
extern "C"
::llvm::ConstantInt* llvm_ConstantInt_getTrueWithContext(::llvm::LLVMContext* Context)
{
    return ::llvm::ConstantInt::getTrue(*Context);
}

// ::llvm::ConstantInt::getType
extern "C"
::llvm::IntegerType* llvm_ConstantInt_getType(::llvm::ConstantInt const* inst)
{
    return inst->getType();
}

// ::llvm::ConstantInt::getZExtValue
extern "C"
uint64_t llvm_ConstantInt_getZExtValue(::llvm::ConstantInt const* inst)
{
    return inst->getZExtValue();
}

// ::llvm::ConstantInt::isMaxValue
extern "C"
int llvm_ConstantInt_isMaxValue(::llvm::ConstantInt const* inst, int isSigned)
{
    return (inst->isMaxValue((isSigned == 1 ? true : false)) == true ? 1 : 0);
}

// ::llvm::ConstantInt::isMinValue
extern "C"
int llvm_ConstantInt_isMinValue(::llvm::ConstantInt const* inst, int isSigned)
{
    return (inst->isMinValue((isSigned == 1 ? true : false)) == true ? 1 : 0);
}

// ::llvm::ConstantInt::isMinusOne
extern "C"
int llvm_ConstantInt_isMinusOne(::llvm::ConstantInt const* inst)
{
    return (inst->isMinusOne() == true ? 1 : 0);
}

// ::llvm::ConstantInt::isNegative
extern "C"
int llvm_ConstantInt_isNegative(::llvm::ConstantInt const* inst)
{
    return (inst->isNegative() == true ? 1 : 0);
}

// ::llvm::ConstantInt::isOne
extern "C"
int llvm_ConstantInt_isOne(::llvm::ConstantInt const* inst)
{
    return (inst->isOne() == true ? 1 : 0);
}

// ::llvm::ConstantInt::isSignedValueValidForType
extern "C"
int llvm_ConstantInt_isSignedValueValidForType(::llvm::Type* Ty, int64_t Val)
{
    return (::llvm::ConstantInt::isValueValidForType(Ty, Val) == true ? 1 : 0);
}

// ::llvm::ConstantInt::isValueValidForType
extern "C"
int llvm_ConstantInt_isValueValidForType(::llvm::Type* Ty, uint64_t Val)
{
    return (::llvm::ConstantInt::isValueValidForType(Ty, Val) == true ? 1 : 0);
}

// ::llvm::ConstantInt::isZero
extern "C"
int llvm_ConstantInt_isZero(::llvm::ConstantInt const* inst)
{
    return (inst->isZero() == true ? 1 : 0);
}

// ::llvm::ConstantInt::uge
extern "C"
int llvm_ConstantInt_uge(::llvm::ConstantInt const* inst, uint64_t Num)
{
    return (inst->uge(Num) == true ? 1 : 0);
}

// ::llvm::ConstantPointerNull::classof
extern "C"
int llvm_ConstantPointerNull_classof(::llvm::Value const* Val)
{
    return (::llvm::ConstantPointerNull::classof(Val) == true ? 1 : 0);
}

// ::llvm::ConstantPointerNull::destroyConstant
extern "C"
void llvm_ConstantPointerNull_destroyConstant(::llvm::ConstantPointerNull* inst)
{
    inst->destroyConstant();
}

// ::llvm::ConstantPointerNull::get
extern "C"
::llvm::ConstantPointerNull* llvm_ConstantPointerNull_get(::llvm::PointerType* Ty)
{
    return ::llvm::ConstantPointerNull::get(Ty);
}

// ::llvm::ConstantPointerNull::getType
extern "C"
::llvm::PointerType* llvm_ConstantPointerNull_getType(::llvm::ConstantPointerNull const* inst)
{
    return inst->getType();
}

// ::llvm::DebugLoc::dump
extern "C"
void llvm_DebugLoc_dump(::llvm::DebugLoc const* inst, ::llvm::LLVMContext const* Ctx)
{
    inst->dump(*Ctx);
}

// ::llvm::DebugLoc::getAsMDNode
extern "C"
::llvm::MDNode* llvm_DebugLoc_getAsMDNode(::llvm::DebugLoc const* inst, ::llvm::LLVMContext const* Ctx)
{
    return inst->getAsMDNode(*Ctx);
}

// ::llvm::DebugLoc::getCol
extern "C"
unsigned int llvm_DebugLoc_getCol(::llvm::DebugLoc const* inst)
{
    return inst->getCol();
}

// ::llvm::DebugLoc::getInlinedAt
extern "C"
::llvm::MDNode* llvm_DebugLoc_getInlinedAt(::llvm::DebugLoc const* inst, ::llvm::LLVMContext const* Ctx)
{
    return inst->getInlinedAt(*Ctx);
}

// ::llvm::DebugLoc::getLine
extern "C"
unsigned int llvm_DebugLoc_getLine(::llvm::DebugLoc const* inst)
{
    return inst->getLine();
}

// ::llvm::DebugLoc::getScope
extern "C"
::llvm::MDNode* llvm_DebugLoc_getScope(::llvm::DebugLoc const* inst, ::llvm::LLVMContext const* Ctx)
{
    return inst->getScope(*Ctx);
}

// ::llvm::DebugLoc::getScopeNode
extern "C"
::llvm::MDNode* llvm_DebugLoc_getScopeNode(::llvm::DebugLoc const* inst, ::llvm::LLVMContext const* Ctx)
{
    return inst->getScopeNode(*Ctx);
}

// ::llvm::DebugLoc::isUnknown
extern "C"
int llvm_DebugLoc_isUnknown(::llvm::DebugLoc const* inst)
{
    return (inst->isUnknown() == true ? 1 : 0);
}

// ::llvm::DebugLoc::new
extern "C"
::llvm::DebugLoc* llvm_DebugLoc_new()
{
    return new(std::nothrow) ::llvm::DebugLoc();
}

// ::llvm::Function::Create
extern "C"
::llvm::Function* llvm_Function_Create(::llvm::FunctionType* Ty, ::llvm::GlobalValue::LinkageTypes Linkage, std_string _Name, ::llvm::Module* Module)
{
    auto Name = std::string(_Name.data, _Name.length);
    return ::llvm::Function::Create(Ty, Linkage, Name, Module);
}

// ::llvm::Function::addFnAttr
extern "C"
void llvm_Function_addFnAttr(::llvm::Function* inst, llvm_StringRef _Kind)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    inst->addFnAttr(Kind);
}

// ::llvm::Function::addFnAttrWithValue
extern "C"
void llvm_Function_addFnAttrWithValue(::llvm::Function* inst, llvm_StringRef _Kind, llvm_StringRef _Val)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    auto Val = ::llvm::StringRef(_Val.data, _Val.length);
    inst->addFnAttr(Kind, Val);
}

// ::llvm::Function::cannotDuplicate
extern "C"
int llvm_Function_cannotDuplicate(::llvm::Function const* inst)
{
    return (inst->cannotDuplicate() == true ? 1 : 0);
}

// ::llvm::Function::classof
extern "C"
int llvm_Function_classof(::llvm::Value const* Val)
{
    return (::llvm::Function::classof(Val) == true ? 1 : 0);
}

// ::llvm::Function::clearGC
extern "C"
void llvm_Function_clearGC(::llvm::Function* inst)
{
    inst->clearGC();
}

// ::llvm::Function::copyAttributesFrom
extern "C"
void llvm_Function_copyAttributesFrom(::llvm::Function* inst, ::llvm::GlobalValue* Src)
{
    inst->copyAttributesFrom(Src);
}

// ::llvm::Function::delete
extern "C"
void llvm_Function_delete(::llvm::Function* inst)
{
    delete inst;
}

// ::llvm::Function::deleteBody
extern "C"
void llvm_Function_deleteBody(::llvm::Function* inst)
{
    inst->deleteBody();
}

// ::llvm::Function::doesNotAccessMemory
extern "C"
int llvm_Function_doesNotAccessMemory(::llvm::Function const* inst)
{
    return (inst->doesNotAccessMemory() == true ? 1 : 0);
}

// ::llvm::Function::doesNotAccessMemoryParam
extern "C"
int llvm_Function_doesNotAccessMemoryParam(::llvm::Function const* inst, unsigned int n)
{
    return (inst->doesNotAccessMemory(n) == true ? 1 : 0);
}

// ::llvm::Function::doesNotAlias
extern "C"
int llvm_Function_doesNotAlias(::llvm::Function const* inst, unsigned int n)
{
    return (inst->doesNotAlias(n) == true ? 1 : 0);
}

// ::llvm::Function::doesNotCapture
extern "C"
int llvm_Function_doesNotCapture(::llvm::Function const* inst, unsigned int n)
{
    return (inst->doesNotCapture(n) == true ? 1 : 0);
}

// ::llvm::Function::doesNotReturn
extern "C"
int llvm_Function_doesNotReturn(::llvm::Function const* inst)
{
    return (inst->doesNotReturn() == true ? 1 : 0);
}

// ::llvm::Function::doesNotThrow
extern "C"
int llvm_Function_doesNotThrow(::llvm::Function const* inst)
{
    return (inst->doesNotThrow() == true ? 1 : 0);
}

// ::llvm::Function::eraseFromParent
extern "C"
void llvm_Function_eraseFromParent(::llvm::Function* inst)
{
    inst->eraseFromParent();
}

// ::llvm::Function::getCallingConv
extern "C"
::llvm::CallingConv::ID llvm_Function_getCallingConv(::llvm::Function const* inst)
{
    return inst->getCallingConv();
}

// ::llvm::Function::getContext
extern "C"
::llvm::LLVMContext* llvm_Function_getContext(::llvm::Function const* inst)
{
    return &(inst->getContext());
}

// ::llvm::Function::getDereferenceableBytes
extern "C"
uint64_t llvm_Function_getDereferenceableBytes(::llvm::Function const* inst, unsigned int idx)
{
    return inst->getDereferenceableBytes(idx);
}

// ::llvm::Function::getFunctionType
extern "C"
::llvm::FunctionType* llvm_Function_getFunctionType(::llvm::Function const* inst)
{
    return inst->getFunctionType();
}

// ::llvm::Function::getIntrinsicID
extern "C"
unsigned int llvm_Function_getIntrinsicID(::llvm::Function const* inst)
{
    return inst->getIntrinsicID();
}

// ::llvm::Function::getParamAlignment
extern "C"
unsigned int llvm_Function_getParamAlignment(::llvm::Function const* inst, unsigned int idx)
{
    return inst->getParamAlignment(idx);
}

// ::llvm::Function::getReturnType
extern "C"
::llvm::Type* llvm_Function_getReturnType(::llvm::Function const* inst)
{
    return inst->getReturnType();
}

// ::llvm::Function::hasFnAttr
extern "C"
int llvm_Function_hasFnAttr(::llvm::Function const* inst, llvm_StringRef _Kind)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    return (inst->hasFnAttribute(Kind) == true ? 1 : 0);
}

// ::llvm::Function::hasGC
extern "C"
int llvm_Function_hasGC(::llvm::Function const* inst)
{
    return (inst->hasGC() == true ? 1 : 0);
}

// ::llvm::Function::hasStructRetAttr
extern "C"
int llvm_Function_hasStructRetAttr(::llvm::Function const* inst)
{
    return (inst->hasStructRetAttr() == true ? 1 : 0);
}

// ::llvm::Function::hasUWTable
extern "C"
int llvm_Function_hasUWTable(::llvm::Function const* inst)
{
    return (inst->hasUWTable() == true ? 1 : 0);
}

// ::llvm::Function::isIntrinsic
extern "C"
int llvm_Function_isIntrinsic(::llvm::Function const* inst)
{
    return (inst->isIntrinsic() == true ? 1 : 0);
}

// ::llvm::Function::isVarArg
extern "C"
int llvm_Function_isVarArg(::llvm::Function const* inst)
{
    return (inst->isVarArg() == true ? 1 : 0);
}

// ::llvm::Function::needsUnwindTableEntry
extern "C"
int llvm_Function_needsUnwindTableEntry(::llvm::Function const* inst)
{
    return (inst->needsUnwindTableEntry() == true ? 1 : 0);
}

// ::llvm::Function::onlyReadsMemory
extern "C"
int llvm_Function_onlyReadsMemory(::llvm::Function const* inst)
{
    return (inst->onlyReadsMemory() == true ? 1 : 0);
}

// ::llvm::Function::onlyReadsMemoryParam
extern "C"
int llvm_Function_onlyReadsMemoryParam(::llvm::Function const* inst, unsigned int n)
{
    return (inst->onlyReadsMemory(n) == true ? 1 : 0);
}

// ::llvm::Function::removeFromParent
extern "C"
void llvm_Function_removeFromParent(::llvm::Function* inst)
{
    inst->removeFromParent();
}

// ::llvm::Function::setCallingConv
extern "C"
void llvm_Function_setCallingConv(::llvm::Function* inst, ::llvm::CallingConv::ID CC)
{
    inst->setCallingConv(CC);
}

// ::llvm::Function::setCannotDuplicate
extern "C"
void llvm_Function_setCannotDuplicate(::llvm::Function* inst)
{
    inst->setCannotDuplicate();
}

// ::llvm::Function::setDoesNotAccessMemory
extern "C"
void llvm_Function_setDoesNotAccessMemory(::llvm::Function* inst)
{
    inst->setDoesNotAccessMemory();
}

// ::llvm::Function::setDoesNotAccessMemoryParam
extern "C"
void llvm_Function_setDoesNotAccessMemoryParam(::llvm::Function* inst, unsigned int n)
{
    inst->setDoesNotAccessMemory(n);
}

// ::llvm::Function::setDoesNotAlias
extern "C"
void llvm_Function_setDoesNotAlias(::llvm::Function* inst, unsigned int n)
{
    inst->setDoesNotAlias(n);
}

// ::llvm::Function::setDoesNotCapture
extern "C"
void llvm_Function_setDoesNotCapture(::llvm::Function* inst, unsigned int n)
{
    inst->setDoesNotCapture(n);
}

// ::llvm::Function::setDoesNotReturn
extern "C"
void llvm_Function_setDoesNotReturn(::llvm::Function* inst)
{
    inst->setDoesNotReturn();
}

// ::llvm::Function::setDoesNotThrow
extern "C"
void llvm_Function_setDoesNotThrow(::llvm::Function* inst)
{
    inst->setDoesNotThrow();
}

// ::llvm::Function::setHasUWTable
extern "C"
void llvm_Function_setHasUWTable(::llvm::Function* inst)
{
    inst->setHasUWTable();
}

// ::llvm::Function::setOnlyReadsMemory
extern "C"
void llvm_Function_setOnlyReadsMemory(::llvm::Function* inst)
{
    inst->setOnlyReadsMemory();
}

// ::llvm::Function::setOnlyReadsMemoryParam
extern "C"
void llvm_Function_setOnlyReadsMemoryParam(::llvm::Function* inst, unsigned int n)
{
    inst->setOnlyReadsMemory(n);
}

// ::llvm::FunctionPassManager::add
extern "C"
void llvm_FunctionPassManager_add(::llvm::FunctionPassManager* inst, ::llvm::FunctionPass* Pass)
{
    inst->add(Pass);
}

// ::llvm::FunctionPassManager::doFinalization
extern "C"
int llvm_FunctionPassManager_doFinalization(::llvm::FunctionPassManager* inst)
{
    return (inst->doFinalization() == true ? 1 : 0);
}

// ::llvm::FunctionPassManager::doInitialization
extern "C"
int llvm_FunctionPassManager_doInitialization(::llvm::FunctionPassManager* inst)
{
    return (inst->doInitialization() == true ? 1 : 0);
}

// ::llvm::FunctionPassManager::new
extern "C"
::llvm::FunctionPassManager* llvm_FunctionPassManager_new(::llvm::Module* Module)
{
    return new(std::nothrow) ::llvm::FunctionPassManager(Module);
}

// ::llvm::FunctionPassManager::run
extern "C"
void llvm_FunctionPassManager_run(::llvm::FunctionPassManager* inst, ::llvm::Function* Function)
{
    inst->run(*Function);
}

// ::llvm::FunctionType::classof
extern "C"
int llvm_FunctionType_classof(::llvm::Type const* ty)
{
    return (::llvm::FunctionType::classof(ty) == true ? 1 : 0);
}

// ::llvm::FunctionType::get
extern "C"
::llvm::FunctionType* llvm_FunctionType_get(::llvm::Type* Result, llvm_ArrayRef_llvm_Type_ptr _Params, int isVarArg)
{
    auto Params = ::llvm::ArrayRef<::llvm::Type*>(_Params.data, _Params.length);
    return ::llvm::FunctionType::get(Result, Params, (isVarArg == 1 ? true : false));
}

// ::llvm::FunctionType::getNumParams
extern "C"
unsigned int llvm_FunctionType_getNumParams(::llvm::FunctionType const* inst)
{
    return inst->getNumParams();
}

// ::llvm::FunctionType::getParamType
extern "C"
::llvm::Type* llvm_FunctionType_getParamType(::llvm::FunctionType const* inst, unsigned int idx)
{
    return inst->getParamType(idx);
}

// ::llvm::FunctionType::getReturnType
extern "C"
::llvm::Type* llvm_FunctionType_getReturnType(::llvm::FunctionType const* inst)
{
    return inst->getReturnType();
}

// ::llvm::FunctionType::isValidArgumentType
extern "C"
int llvm_FunctionType_isValidArgumentType(::llvm::Type* ty)
{
    return (::llvm::FunctionType::isValidArgumentType(ty) == true ? 1 : 0);
}

// ::llvm::FunctionType::isValidReturnType
extern "C"
int llvm_FunctionType_isValidReturnType(::llvm::Type* ty)
{
    return (::llvm::FunctionType::isValidReturnType(ty) == true ? 1 : 0);
}

// ::llvm::FunctionType::isVarArg
extern "C"
int llvm_FunctionType_isVarArg(::llvm::FunctionType const* inst)
{
    return (inst->isVarArg() == true ? 1 : 0);
}

// ::llvm::GlobalObject::setSection
extern "C"
void llvm_GlobalObject_setSection(::llvm::GlobalObject* inst, llvm_StringRef _S)
{
    auto S = ::llvm::StringRef(_S.data, _S.length);
    inst->setSection(S);
}

// ::llvm::GlobalValue::copyAttributesFrom
extern "C"
void llvm_GlobalValue_copyAttributesFrom(::llvm::GlobalValue* inst, ::llvm::GlobalValue* Src)
{
    inst->copyAttributesFrom(Src);
}

// ::llvm::GlobalValue::delete
extern "C"
void llvm_GlobalValue_delete(::llvm::GlobalValue* inst)
{
    delete inst;
}

// ::llvm::GlobalValue::destroyConstant
extern "C"
void llvm_GlobalValue_destroyConstant(::llvm::GlobalValue* inst)
{
    inst->destroyConstant();
}

// ::llvm::GlobalValue::eraseFromParent
extern "C"
void llvm_GlobalValue_eraseFromParent(::llvm::GlobalValue* inst)
{
    inst->eraseFromParent();
}

// ::llvm::GlobalValue::getAlignment
extern "C"
unsigned int llvm_GlobalValue_getAlignment(::llvm::GlobalValue const* inst)
{
    return inst->getAlignment();
}

// ::llvm::GlobalValue::getDataLayout
extern "C"
::llvm::DataLayout const* llvm_GlobalValue_getDataLayout(::llvm::GlobalValue const* inst)
{
    return inst->getDataLayout();
}

// ::llvm::GlobalValue::getParent
extern "C"
::llvm::Module const* llvm_GlobalValue_getParent(::llvm::GlobalValue const* inst)
{
    return inst->getParent();
}

// ::llvm::GlobalValue::getParentMut
extern "C"
::llvm::Module* llvm_GlobalValue_getParentMut(::llvm::GlobalValue* inst)
{
    return inst->getParent();
}

// ::llvm::GlobalValue::getType
extern "C"
::llvm::PointerType* llvm_GlobalValue_getType(::llvm::GlobalValue const* inst)
{
    return inst->getType();
}

// ::llvm::GlobalValue::hasAppendingLinkage
extern "C"
int llvm_GlobalValue_hasAppendingLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasAppendingLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasAvailableExternallyLinkage
extern "C"
int llvm_GlobalValue_hasAvailableExternallyLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasAvailableExternallyLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasCommonLinkage
extern "C"
int llvm_GlobalValue_hasCommonLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasCommonLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasDLLExportStorageClass
extern "C"
int llvm_GlobalValue_hasDLLExportStorageClass(::llvm::GlobalValue const* inst)
{
    return (inst->hasDLLExportStorageClass() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasDLLImportStorageClass
extern "C"
int llvm_GlobalValue_hasDLLImportStorageClass(::llvm::GlobalValue const* inst)
{
    return (inst->hasDLLImportStorageClass() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasDefaultVisibility
extern "C"
int llvm_GlobalValue_hasDefaultVisibility(::llvm::GlobalValue const* inst)
{
    return (inst->hasDefaultVisibility() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasExternalLinkage
extern "C"
int llvm_GlobalValue_hasExternalLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasExternalLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasExternalWeakLinkage
extern "C"
int llvm_GlobalValue_hasExternalWeakLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasExternalWeakLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasHiddenVisibility
extern "C"
int llvm_GlobalValue_hasHiddenVisibility(::llvm::GlobalValue const* inst)
{
    return (inst->hasHiddenVisibility() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasInternalLinkage
extern "C"
int llvm_GlobalValue_hasInternalLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasInternalLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasLinkOnceLinkage
extern "C"
int llvm_GlobalValue_hasLinkOnceLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasLinkOnceLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasLocalLinkage
extern "C"
int llvm_GlobalValue_hasLocalLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasLocalLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasPrivateLinkage
extern "C"
int llvm_GlobalValue_hasPrivateLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasPrivateLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasProtectedVisibility
extern "C"
int llvm_GlobalValue_hasProtectedVisibility(::llvm::GlobalValue const* inst)
{
    return (inst->hasProtectedVisibility() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasSection
extern "C"
int llvm_GlobalValue_hasSection(::llvm::GlobalValue const* inst)
{
    return (inst->hasSection() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasUnnamedAddr
extern "C"
int llvm_GlobalValue_hasUnnamedAddr(::llvm::GlobalValue const* inst)
{
    return (inst->hasUnnamedAddr() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasWeakAnyLinkage
extern "C"
int llvm_GlobalValue_hasWeakAnyLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasWeakAnyLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasWeakLinkage
extern "C"
int llvm_GlobalValue_hasWeakLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasWeakLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::hasWeakODRLinkage
extern "C"
int llvm_GlobalValue_hasWeakODRLinkage(::llvm::GlobalValue const* inst)
{
    return (inst->hasWeakODRLinkage() == true ? 1 : 0);
}

// ::llvm::GlobalValue::isDeclaration
extern "C"
int llvm_GlobalValue_isDeclaration(::llvm::GlobalValue const* inst)
{
    return (inst->isDeclaration() == true ? 1 : 0);
}

// ::llvm::GlobalValue::isDiscardableIfUnused
extern "C"
int llvm_GlobalValue_isDiscardableIfUnused(::llvm::GlobalValue const* inst)
{
    return (inst->isDiscardableIfUnused() == true ? 1 : 0);
}

// ::llvm::GlobalValue::isThreadLocal
extern "C"
int llvm_GlobalValue_isThreadLocal(::llvm::GlobalValue const* inst)
{
    return (inst->isThreadLocal() == true ? 1 : 0);
}

// ::llvm::GlobalValue::isWeakForLinker
extern "C"
int llvm_GlobalValue_isWeakForLinker(::llvm::GlobalValue const* inst)
{
    return (inst->isWeakForLinker() == true ? 1 : 0);
}

// ::llvm::GlobalValue::mayBeOverridden
extern "C"
int llvm_GlobalValue_mayBeOverridden(::llvm::GlobalValue const* inst)
{
    return (inst->mayBeOverridden() == true ? 1 : 0);
}

// ::llvm::GlobalValue::removeFromParent
extern "C"
void llvm_GlobalValue_removeFromParent(::llvm::GlobalValue* inst)
{
    inst->removeFromParent();
}

// ::llvm::GlobalValue::setThreadLocal
extern "C"
void llvm_GlobalValue_setThreadLocal(::llvm::GlobalValue* inst, int Val)
{
    inst->setThreadLocal((Val == 1 ? true : false));
}

// ::llvm::GlobalValue::setUnnamedAddr
extern "C"
void llvm_GlobalValue_setUnnamedAddr(::llvm::GlobalValue* inst, int Val)
{
    inst->setUnnamedAddr((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::copyAttributesFrom
extern "C"
void llvm_GlobalVariable_copyAttributesFrom(::llvm::GlobalVariable* inst, ::llvm::GlobalValue* Src)
{
    inst->copyAttributesFrom(Src);
}

// ::llvm::GlobalVariable::delete
extern "C"
void llvm_GlobalVariable_delete(::llvm::GlobalVariable* inst)
{
    delete inst;
}

// ::llvm::GlobalVariable::eraseFromParent
extern "C"
void llvm_GlobalVariable_eraseFromParent(::llvm::GlobalVariable* inst)
{
    inst->eraseFromParent();
}

// ::llvm::GlobalVariable::getInitializer
extern "C"
::llvm::Constant const* llvm_GlobalVariable_getInitializer(::llvm::GlobalVariable const* inst)
{
    return inst->getInitializer();
}

// ::llvm::GlobalVariable::getInitializerMut
extern "C"
::llvm::Constant* llvm_GlobalVariable_getInitializerMut(::llvm::GlobalVariable* inst)
{
    return inst->getInitializer();
}

// ::llvm::GlobalVariable::hasDefinitiveInitializer
extern "C"
int llvm_GlobalVariable_hasDefinitiveInitializer(::llvm::GlobalVariable const* inst)
{
    return (inst->hasDefinitiveInitializer() == true ? 1 : 0);
}

// ::llvm::GlobalVariable::hasInitializer
extern "C"
int llvm_GlobalVariable_hasInitializer(::llvm::GlobalVariable const* inst)
{
    return (inst->hasInitializer() == true ? 1 : 0);
}

// ::llvm::GlobalVariable::hasUniqueInitializer
extern "C"
int llvm_GlobalVariable_hasUniqueInitializer(::llvm::GlobalVariable const* inst)
{
    return (inst->hasUniqueInitializer() == true ? 1 : 0);
}

// ::llvm::GlobalVariable::isConstant
extern "C"
int llvm_GlobalVariable_isConstant(::llvm::GlobalVariable const* inst)
{
    return (inst->isConstant() == true ? 1 : 0);
}

// ::llvm::GlobalVariable::isExternallyInitialized
extern "C"
int llvm_GlobalVariable_isExternallyInitialized(::llvm::GlobalVariable const* inst)
{
    return (inst->isExternallyInitialized() == true ? 1 : 0);
}

// ::llvm::GlobalVariable::new
extern "C"
::llvm::GlobalVariable* llvm_GlobalVariable_new(::llvm::Type* Ty, int isConstant, ::llvm::GlobalValue::LinkageTypes Linkage)
{
    return new ::llvm::GlobalVariable(Ty, (isConstant == 1 ? true : false), Linkage);
}

// ::llvm::GlobalVariable::newWithModule
extern "C"
::llvm::GlobalVariable* llvm_GlobalVariable_newWithModule(::llvm::Module* Module, ::llvm::Type* Ty, int isConstant, ::llvm::GlobalValue::LinkageTypes Linkage, ::llvm::Constant* Initializer)
{
    return new ::llvm::GlobalVariable(*Module, Ty, (isConstant == 1 ? true : false), Linkage, Initializer);
}

// ::llvm::GlobalVariable::removeFromParent
extern "C"
void llvm_GlobalVariable_removeFromParent(::llvm::GlobalVariable* inst)
{
    inst->removeFromParent();
}

// ::llvm::GlobalVariable::setConstant
extern "C"
void llvm_GlobalVariable_setConstant(::llvm::GlobalVariable* inst, int Val)
{
    inst->setConstant((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::setExternallyInitialized
extern "C"
void llvm_GlobalVariable_setExternallyInitialized(::llvm::GlobalVariable* inst, int Val)
{
    inst->setExternallyInitialized((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::setInitializer
extern "C"
void llvm_GlobalVariable_setInitializer(::llvm::GlobalVariable* inst, ::llvm::Constant* InitVal)
{
    inst->setInitializer(InitVal);
}

// ::llvm::IRBuilder::CreateAShr
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAShr(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAShr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateAShrByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAShrByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAShr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateAdd
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAdd(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAdd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateAddrSpaceCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAddrSpaceCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAddrSpaceCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateAggregateRet
extern "C"
::llvm::ReturnInst* llvm_IRBuilder_CreateAggregateRet(::llvm::IRBuilder<>* inst, ::llvm::Value* const* retVals, unsigned int N)
{
    return inst->CreateAggregateRet(retVals, N);
}

// ::llvm::IRBuilder::CreateAlignedLoad
extern "C"
::llvm::LoadInst* llvm_IRBuilder_CreateAlignedLoad(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, unsigned int Align, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAlignedLoad(Ptr, Align, Name);
}

// ::llvm::IRBuilder::CreateAlignedLoadVolatile
extern "C"
::llvm::LoadInst* llvm_IRBuilder_CreateAlignedLoadVolatile(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, unsigned int Align, int isVolatile, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAlignedLoad(Ptr, Align, (isVolatile == 1 ? true : false), Name);
}

// ::llvm::IRBuilder::CreateAlignedStore
extern "C"
::llvm::StoreInst* llvm_IRBuilder_CreateAlignedStore(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Value* Ptr, unsigned int Align, int isVolatile)
{
    return inst->CreateAlignedStore(Value, Ptr, Align, (isVolatile == 1 ? true : false));
}

// ::llvm::IRBuilder::CreateAlloca
extern "C"
::llvm::AllocaInst* llvm_IRBuilder_CreateAlloca(::llvm::IRBuilder<>* inst, ::llvm::Type* Ty, ::llvm::Value* ArraySize, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAlloca(Ty, ArraySize, Name);
}

// ::llvm::IRBuilder::CreateAnd
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAnd(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAnd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateAndByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateAndByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateAnd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateBinOp
extern "C"
::llvm::Value* llvm_IRBuilder_CreateBinOp(::llvm::IRBuilder<>* inst, ::llvm::Instruction::BinaryOps Opcode, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateBinOp(Opcode, LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateBitCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateBitCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateBitCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateBr
extern "C"
::llvm::BranchInst* llvm_IRBuilder_CreateBr(::llvm::IRBuilder<>* inst, ::llvm::BasicBlock* Dest)
{
    return inst->CreateBr(Dest);
}

// ::llvm::IRBuilder::CreateCall
extern "C"
::llvm::CallInst* llvm_IRBuilder_CreateCall(::llvm::IRBuilder<>* inst, ::llvm::Value* Callee, llvm_ArrayRef_llvm_Value_ptr _Args, std_string _Name)
{
    auto Args = ::llvm::ArrayRef<::llvm::Value*>(_Args.data, _Args.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateCall(Callee, Args, Name);
}

// ::llvm::IRBuilder::CreateCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateCast(::llvm::IRBuilder<>* inst, ::llvm::Instruction::CastOps Opcode, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateCast(Opcode, Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateCondBr
extern "C"
::llvm::BranchInst* llvm_IRBuilder_CreateCondBr(::llvm::IRBuilder<>* inst, ::llvm::Value* Cond, ::llvm::BasicBlock* TrueBlock, ::llvm::BasicBlock* FalseBlock)
{
    return inst->CreateCondBr(Cond, TrueBlock, FalseBlock);
}

// ::llvm::IRBuilder::CreateExactSDiv
extern "C"
::llvm::Value* llvm_IRBuilder_CreateExactSDiv(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateExactSDiv(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateExactUDiv
extern "C"
::llvm::Value* llvm_IRBuilder_CreateExactUDiv(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateExactUDiv(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateExtractElement
extern "C"
::llvm::Value* llvm_IRBuilder_CreateExtractElement(::llvm::IRBuilder<>* inst, ::llvm::Value* Vec, ::llvm::Value* Idx, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateExtractElement(Vec, Idx, Name);
}

// ::llvm::IRBuilder::CreateExtractInteger
extern "C"
::llvm::Value* llvm_IRBuilder_CreateExtractInteger(::llvm::IRBuilder<>* inst, ::llvm::DataLayout const* DL, ::llvm::Value* From, ::llvm::IntegerType* ExtractedTy, uint64_t Offset, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateExtractInteger(*DL, From, ExtractedTy, Offset, Name);
}

// ::llvm::IRBuilder::CreateExtractValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateExtractValue(::llvm::IRBuilder<>* inst, ::llvm::Value* Agg, llvm_ArrayRef__libc_c_uint _Indexes, std_string _Name)
{
    auto Indexes = ::llvm::ArrayRef<unsigned int>(_Indexes.data, _Indexes.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateExtractValue(Agg, Indexes, Name);
}

// ::llvm::IRBuilder::CreateFAdd
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFAdd(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFAdd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmp
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmp(::llvm::IRBuilder<>* inst, ::llvm::CmpInst::Predicate Pred, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmp(Pred, LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpOEQ
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpOEQ(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpOEQ(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpOGE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpOGE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpOGE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpOGT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpOGT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpOGT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpOLE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpOLE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpOLE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpOLT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpOLT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpOLT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpONE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpONE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpONE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpORD
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpORD(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpORD(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpUEQ
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpUEQ(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpUEQ(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpUGE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpUGE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpUGE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpUGT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpUGT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpUGT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpULE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpULE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpULE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpULT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpULT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpULT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpUNE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpUNE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpUNE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFCmpUNO
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFCmpUNO(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFCmpUNO(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFDiv
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFDiv(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFDiv(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFMul
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFMul(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFMul(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFNeg
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFNeg(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFNeg(Value, Name);
}

// ::llvm::IRBuilder::CreateFPCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFPCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFPCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateFPExt
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFPExt(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFPExt(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateFPToSI
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFPToSI(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFPToSI(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateFPToUI
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFPToUI(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFPToUI(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateFPTrunc
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFPTrunc(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFPTrunc(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateFRem
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFRem(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFRem(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFSub
extern "C"
::llvm::Value* llvm_IRBuilder_CreateFSub(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFSub(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateFence
extern "C"
::llvm::FenceInst* llvm_IRBuilder_CreateFence(::llvm::IRBuilder<>* inst, ::llvm::AtomicOrdering Ordering, ::llvm::SynchronizationScope SynchScope, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateFence(Ordering, SynchScope, Name);
}

// ::llvm::IRBuilder::CreateGEP
extern "C"
::llvm::Value* llvm_IRBuilder_CreateGEP(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, llvm_ArrayRef_llvm_Value_ptr _Indexes, std_string _Name)
{
    auto Indexes = ::llvm::ArrayRef<::llvm::Value*>(_Indexes.data, _Indexes.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateGEP(Ptr, Indexes, Name);
}

// ::llvm::IRBuilder::CreateGlobalStringPtr
extern "C"
::llvm::Value* llvm_IRBuilder_CreateGlobalStringPtr(::llvm::IRBuilder<>* inst, llvm_StringRef _Str, std_string _Name)
{
    auto Str = ::llvm::StringRef(_Str.data, _Str.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateGlobalStringPtr(Str, Name);
}

// ::llvm::IRBuilder::CreateICmp
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmp(::llvm::IRBuilder<>* inst, ::llvm::CmpInst::Predicate Pred, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmp(Pred, LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpEQ
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpEQ(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpEQ(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpNE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpNE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpNE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpSGE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpSGE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpSGE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpSGT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpSGT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpSGT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpSLE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpSLE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpSLE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpSLT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpSLT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpSLT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpUGE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpUGE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpUGE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpUGT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpUGT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpUGT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpULE
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpULE(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpULE(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateICmpULT
extern "C"
::llvm::Value* llvm_IRBuilder_CreateICmpULT(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateICmpULT(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateInBoundsGEP
extern "C"
::llvm::Value* llvm_IRBuilder_CreateInBoundsGEP(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, llvm_ArrayRef_llvm_Value_ptr _Indexes, std_string _Name)
{
    auto Indexes = ::llvm::ArrayRef<::llvm::Value*>(_Indexes.data, _Indexes.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateInBoundsGEP(Ptr, Indexes, Name);
}

// ::llvm::IRBuilder::CreateIndirectBr
extern "C"
::llvm::IndirectBrInst* llvm_IRBuilder_CreateIndirectBr(::llvm::IRBuilder<>* inst, ::llvm::Value* Addr, unsigned int NumCases)
{
    return inst->CreateIndirectBr(Addr, NumCases);
}

// ::llvm::IRBuilder::CreateInsertElement
extern "C"
::llvm::Value* llvm_IRBuilder_CreateInsertElement(::llvm::IRBuilder<>* inst, ::llvm::Value* Vec, ::llvm::Value* NewElt, ::llvm::Value* Idx, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateInsertElement(Vec, NewElt, Idx, Name);
}

// ::llvm::IRBuilder::CreateInsertValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateInsertValue(::llvm::IRBuilder<>* inst, ::llvm::Value* Agg, ::llvm::Value* Value, llvm_ArrayRef__libc_c_uint _Indexes, std_string _Name)
{
    auto Indexes = ::llvm::ArrayRef<unsigned int>(_Indexes.data, _Indexes.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateInsertValue(Agg, Value, Indexes, Name);
}

// ::llvm::IRBuilder::CreateIntCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateIntCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, int isSigned, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateIntCast(Value, DestTy, (isSigned == 1 ? true : false), Name);
}

// ::llvm::IRBuilder::CreateIntToPtr
extern "C"
::llvm::Value* llvm_IRBuilder_CreateIntToPtr(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateIntToPtr(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateInvoke
extern "C"
::llvm::InvokeInst* llvm_IRBuilder_CreateInvoke(::llvm::IRBuilder<>* inst, ::llvm::Value* Callee, ::llvm::BasicBlock* NormalDest, ::llvm::BasicBlock* UnwindDest, llvm_ArrayRef_llvm_Value_ptr _Args, std_string_const _Name)
{
    auto Args = ::llvm::ArrayRef<::llvm::Value*>(_Args.data, _Args.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateInvoke(Callee, NormalDest, UnwindDest, Args, Name);
}

// ::llvm::IRBuilder::CreateIsNotNull
extern "C"
::llvm::Value* llvm_IRBuilder_CreateIsNotNull(::llvm::IRBuilder<>* inst, ::llvm::Value* Arg, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateIsNotNull(Arg, Name);
}

// ::llvm::IRBuilder::CreateIsNull
extern "C"
::llvm::Value* llvm_IRBuilder_CreateIsNull(::llvm::IRBuilder<>* inst, ::llvm::Value* Arg, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateIsNull(Arg, Name);
}

// ::llvm::IRBuilder::CreateLShr
extern "C"
::llvm::Value* llvm_IRBuilder_CreateLShr(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateLShr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateLShrByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateLShrByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateLShr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateLandingPad
extern "C"
::llvm::LandingPadInst* llvm_IRBuilder_CreateLandingPad(::llvm::IRBuilder<>* inst, ::llvm::Type* Ty, ::llvm::Value* PersFn, unsigned int NumClauses, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateLandingPad(Ty, PersFn, NumClauses, Name);
}

// ::llvm::IRBuilder::CreateLoad
extern "C"
::llvm::LoadInst* llvm_IRBuilder_CreateLoad(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateLoad(Ptr, Name);
}

// ::llvm::IRBuilder::CreateLoadVolatile
extern "C"
::llvm::LoadInst* llvm_IRBuilder_CreateLoadVolatile(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, int isVolatile, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateLoad(Ptr, (isVolatile == 1 ? true : false), Name);
}

// ::llvm::IRBuilder::CreateMul
extern "C"
::llvm::Value* llvm_IRBuilder_CreateMul(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateMul(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNSWAdd
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNSWAdd(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNSWAdd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNSWMul
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNSWMul(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNSWMul(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNSWNeg
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNSWNeg(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNSWNeg(Value, Name);
}

// ::llvm::IRBuilder::CreateNSWSub
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNSWSub(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNSWSub(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNUWAdd
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNUWAdd(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNUWAdd(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNUWMul
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNUWMul(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNUWMul(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNUWNeg
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNUWNeg(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNUWNeg(Value, Name);
}

// ::llvm::IRBuilder::CreateNUWSub
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNUWSub(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNUWSub(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateNeg
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNeg(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNeg(Value, Name);
}

// ::llvm::IRBuilder::CreateNot
extern "C"
::llvm::Value* llvm_IRBuilder_CreateNot(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateNot(Value, Name);
}

// ::llvm::IRBuilder::CreateOr
extern "C"
::llvm::Value* llvm_IRBuilder_CreateOr(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateOr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateOrByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateOrByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateOr(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreatePHI
extern "C"
::llvm::PHINode* llvm_IRBuilder_CreatePHI(::llvm::IRBuilder<>* inst, ::llvm::Type* Ty, unsigned int NumReservedValues, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreatePHI(Ty, NumReservedValues, Name);
}

// ::llvm::IRBuilder::CreatePointerBitCastOrAddrSpaceCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreatePointerBitCastOrAddrSpaceCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreatePointerBitCastOrAddrSpaceCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreatePointerCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreatePointerCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreatePointerCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreatePtrDiff
extern "C"
::llvm::Value* llvm_IRBuilder_CreatePtrDiff(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreatePtrDiff(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreatePtrToInt
extern "C"
::llvm::Value* llvm_IRBuilder_CreatePtrToInt(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreatePtrToInt(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateResume
extern "C"
::llvm::ResumeInst* llvm_IRBuilder_CreateResume(::llvm::IRBuilder<>* inst, ::llvm::Value* Exn)
{
    return inst->CreateResume(Exn);
}

// ::llvm::IRBuilder::CreateRet
extern "C"
::llvm::ReturnInst* llvm_IRBuilder_CreateRet(::llvm::IRBuilder<>* inst, ::llvm::Value* Value)
{
    return inst->CreateRet(Value);
}

// ::llvm::IRBuilder::CreateRetVoid
extern "C"
::llvm::ReturnInst* llvm_IRBuilder_CreateRetVoid(::llvm::IRBuilder<>* inst)
{
    return inst->CreateRetVoid();
}

// ::llvm::IRBuilder::CreateSDiv
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSDiv(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSDiv(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateSExt
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSExt(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSExt(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateSExtOrBitCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSExtOrBitCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSExtOrBitCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateSExtOrTrunc
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSExtOrTrunc(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSExtOrTrunc(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateSIToFP
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSIToFP(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSIToFP(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateSRem
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSRem(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSRem(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateSelect
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSelect(::llvm::IRBuilder<>* inst, ::llvm::Value* C, ::llvm::Value* TrueValue, ::llvm::Value* FalseValue, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSelect(C, TrueValue, FalseValue, Name);
}

// ::llvm::IRBuilder::CreateShl
extern "C"
::llvm::Value* llvm_IRBuilder_CreateShl(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateShl(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateShlByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateShlByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateShl(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateShuffleVector
extern "C"
::llvm::Value* llvm_IRBuilder_CreateShuffleVector(::llvm::IRBuilder<>* inst, ::llvm::Value* V1, ::llvm::Value* P2, ::llvm::Value* Mask, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateShuffleVector(V1, P2, Mask, Name);
}

// ::llvm::IRBuilder::CreateStore
extern "C"
::llvm::StoreInst* llvm_IRBuilder_CreateStore(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Value* Ptr, int isVolatile)
{
    return inst->CreateStore(Value, Ptr, (isVolatile == 1 ? true : false));
}

// ::llvm::IRBuilder::CreateStructGEP
extern "C"
::llvm::Value* llvm_IRBuilder_CreateStructGEP(::llvm::IRBuilder<>* inst, ::llvm::Value* Ptr, unsigned int Index, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateStructGEP(Ptr, Index, Name);
}

// ::llvm::IRBuilder::CreateSub
extern "C"
::llvm::Value* llvm_IRBuilder_CreateSub(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateSub(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateSwitch
extern "C"
::llvm::SwitchInst* llvm_IRBuilder_CreateSwitch(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::BasicBlock* Dest, unsigned int NumCases)
{
    return inst->CreateSwitch(Value, Dest, NumCases);
}

// ::llvm::IRBuilder::CreateTrunc
extern "C"
::llvm::Value* llvm_IRBuilder_CreateTrunc(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateTrunc(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateTruncOrBitCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateTruncOrBitCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateTruncOrBitCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateUDiv
extern "C"
::llvm::Value* llvm_IRBuilder_CreateUDiv(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateUDiv(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateUIToFP
extern "C"
::llvm::Value* llvm_IRBuilder_CreateUIToFP(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateUIToFP(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateURem
extern "C"
::llvm::Value* llvm_IRBuilder_CreateURem(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateURem(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateUnreachable
extern "C"
::llvm::UnreachableInst* llvm_IRBuilder_CreateUnreachable(::llvm::IRBuilder<>* inst)
{
    return inst->CreateUnreachable();
}

// ::llvm::IRBuilder::CreateVAArg
extern "C"
::llvm::VAArgInst* llvm_IRBuilder_CreateVAArg(::llvm::IRBuilder<>* inst, ::llvm::Value* List, ::llvm::Type* Ty, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateVAArg(List, Ty, Name);
}

// ::llvm::IRBuilder::CreateVectorSplat
extern "C"
::llvm::Value* llvm_IRBuilder_CreateVectorSplat(::llvm::IRBuilder<>* inst, unsigned int NumElements, ::llvm::Value* Value, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateVectorSplat(NumElements, Value, Name);
}

// ::llvm::IRBuilder::CreateXor
extern "C"
::llvm::Value* llvm_IRBuilder_CreateXor(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, ::llvm::Value* RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateXor(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateXorByValue
extern "C"
::llvm::Value* llvm_IRBuilder_CreateXorByValue(::llvm::IRBuilder<>* inst, ::llvm::Value* LHS, uint64_t RHS, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateXor(LHS, RHS, Name);
}

// ::llvm::IRBuilder::CreateZExt
extern "C"
::llvm::Value* llvm_IRBuilder_CreateZExt(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateZExt(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateZExtOrBitCast
extern "C"
::llvm::Value* llvm_IRBuilder_CreateZExtOrBitCast(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateZExtOrBitCast(Value, DestTy, Name);
}

// ::llvm::IRBuilder::CreateZExtOrTrunc
extern "C"
::llvm::Value* llvm_IRBuilder_CreateZExtOrTrunc(::llvm::IRBuilder<>* inst, ::llvm::Value* Value, ::llvm::Type* DestTy, std_string _Name)
{
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateZExtOrTrunc(Value, DestTy, Name);
}

// ::llvm::IRBuilder::delete
extern "C"
void llvm_IRBuilder_delete(::llvm::IRBuilder<>* inst)
{
    delete inst;
}

// ::llvm::IRBuilder::isNamePreserving
extern "C"
int llvm_IRBuilder_isNamePreserving(::llvm::IRBuilder<> const* inst)
{
    return (inst->isNamePreserving() == true ? 1 : 0);
}

// ::llvm::IRBuilder::new
extern "C"
::llvm::IRBuilder<>* llvm_IRBuilder_new(::llvm::LLVMContext* Context)
{
    return new(std::nothrow) ::llvm::IRBuilder<>(*Context);
}

// ::llvm::IRBuilder::new_in_block
extern "C"
::llvm::IRBuilder<>* llvm_IRBuilder_new_in_block(::llvm::BasicBlock* BB)
{
    return new(std::nothrow) ::llvm::IRBuilder<>(BB);
}

// ::llvm::IRBuilderBase::ClearInsertionPoint
extern "C"
void llvm_IRBuilderBase_ClearInsertionPoint(::llvm::IRBuilderBase* inst)
{
    inst->ClearInsertionPoint();
}

// ::llvm::IRBuilderBase::CreateGlobalString
extern "C"
::llvm::Value* llvm_IRBuilderBase_CreateGlobalString(::llvm::IRBuilderBase* inst, llvm_StringRef _Str, std_string _Name)
{
    auto Str = ::llvm::StringRef(_Str.data, _Str.length);
    auto Name = std::string(_Name.data, _Name.length);
    return inst->CreateGlobalString(Str, Name);
}

// ::llvm::IRBuilderBase::CreateLifetimeEnd
extern "C"
::llvm::CallInst* llvm_IRBuilderBase_CreateLifetimeEnd(::llvm::IRBuilderBase* inst, ::llvm::Value* Ptr, ::llvm::ConstantInt* Size)
{
    return inst->CreateLifetimeEnd(Ptr, Size);
}

// ::llvm::IRBuilderBase::CreateLifetimeStart
extern "C"
::llvm::CallInst* llvm_IRBuilderBase_CreateLifetimeStart(::llvm::IRBuilderBase* inst, ::llvm::Value* Ptr, ::llvm::ConstantInt* Size)
{
    return inst->CreateLifetimeStart(Ptr, Size);
}

// ::llvm::IRBuilderBase::CreateMemCpy
extern "C"
::llvm::CallInst* llvm_IRBuilderBase_CreateMemCpy(::llvm::IRBuilderBase* inst, ::llvm::Value* Dst, ::llvm::Value* Src, ::llvm::Value* Size, unsigned int Align, int isVolatile)
{
    return inst->CreateMemCpy(Dst, Src, Size, Align, (isVolatile == 1 ? true : false));
}

// ::llvm::IRBuilderBase::CreateMemMove
extern "C"
::llvm::CallInst* llvm_IRBuilderBase_CreateMemMove(::llvm::IRBuilderBase* inst, ::llvm::Value* Dst, ::llvm::Value* Src, ::llvm::Value* Size, unsigned int Align, int isVolatile)
{
    return inst->CreateMemMove(Dst, Src, Size, Align, (isVolatile == 1 ? true : false));
}

// ::llvm::IRBuilderBase::CreateMemSet
extern "C"
::llvm::CallInst* llvm_IRBuilderBase_CreateMemSet(::llvm::IRBuilderBase* inst, ::llvm::Value* Ptr, ::llvm::Value* Value, ::llvm::Value* Size, unsigned int Align, int isVolatile)
{
    return inst->CreateMemSet(Ptr, Value, Size, Align, (isVolatile == 1 ? true : false));
}

// ::llvm::IRBuilderBase::GetInsertBlock
extern "C"
::llvm::BasicBlock* llvm_IRBuilderBase_GetInsertBlock(::llvm::IRBuilderBase const* inst)
{
    return inst->GetInsertBlock();
}

// ::llvm::IRBuilderBase::SetCurrentDebugLocation
extern "C"
void llvm_IRBuilderBase_SetCurrentDebugLocation(::llvm::IRBuilderBase* inst, ::llvm::DebugLoc const* Loc)
{
    inst->SetCurrentDebugLocation(*Loc);
}

// ::llvm::IRBuilderBase::SetDefaultFPMathTag
extern "C"
void llvm_IRBuilderBase_SetDefaultFPMathTag(::llvm::IRBuilderBase* inst, ::llvm::MDNode* FPMathTag)
{
    inst->SetDefaultFPMathTag(FPMathTag);
}

// ::llvm::IRBuilderBase::SetInsertPoint
extern "C"
void llvm_IRBuilderBase_SetInsertPoint(::llvm::IRBuilderBase* inst, ::llvm::BasicBlock* BB)
{
    inst->SetInsertPoint(BB);
}

// ::llvm::IRBuilderBase::SetInsertPointAtInst
extern "C"
void llvm_IRBuilderBase_SetInsertPointAtInst(::llvm::IRBuilderBase* inst, ::llvm::Instruction* Inst)
{
    inst->SetInsertPoint(Inst);
}

// ::llvm::IRBuilderBase::SetInstDebugLocation
extern "C"
void llvm_IRBuilderBase_SetInstDebugLocation(::llvm::IRBuilderBase const* inst, ::llvm::Instruction* Inst)
{
    inst->SetInstDebugLocation(Inst);
}

// ::llvm::IRBuilderBase::getContext
extern "C"
::llvm::LLVMContext* llvm_IRBuilderBase_getContext(::llvm::IRBuilderBase const* inst)
{
    return &(inst->getContext());
}

// ::llvm::IRBuilderBase::getCurrentFunctionReturnType
extern "C"
::llvm::Type* llvm_IRBuilderBase_getCurrentFunctionReturnType(::llvm::IRBuilderBase const* inst)
{
    return inst->getCurrentFunctionReturnType();
}

// ::llvm::IRBuilderBase::getDefaultFPMathTag
extern "C"
::llvm::MDNode* llvm_IRBuilderBase_getDefaultFPMathTag(::llvm::IRBuilderBase const* inst)
{
    return inst->getDefaultFPMathTag();
}

// ::llvm::IRBuilderBase::getDoubleTy
extern "C"
::llvm::Type* llvm_IRBuilderBase_getDoubleTy(::llvm::IRBuilderBase* inst)
{
    return inst->getDoubleTy();
}

// ::llvm::IRBuilderBase::getFalse
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getFalse(::llvm::IRBuilderBase* inst)
{
    return inst->getFalse();
}

// ::llvm::IRBuilderBase::getFloatTy
extern "C"
::llvm::Type* llvm_IRBuilderBase_getFloatTy(::llvm::IRBuilderBase* inst)
{
    return inst->getFloatTy();
}

// ::llvm::IRBuilderBase::getHalfTy
extern "C"
::llvm::Type* llvm_IRBuilderBase_getHalfTy(::llvm::IRBuilderBase* inst)
{
    return inst->getHalfTy();
}

// ::llvm::IRBuilderBase::getInt
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt(::llvm::IRBuilderBase* inst, llvm_APInt _Value)
{
    auto Value = ::llvm::APInt(_Value.num_bits, ::llvm::ArrayRef<uint64_t>(_Value.value.data, _Value.value.length));
    return inst->getInt(Value);
}

// ::llvm::IRBuilderBase::getInt1
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt1(::llvm::IRBuilderBase* inst, int Value)
{
    return inst->getInt1((Value == 1 ? true : false));
}

// ::llvm::IRBuilderBase::getInt16
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt16(::llvm::IRBuilderBase* inst, uint16_t Value)
{
    return inst->getInt16(Value);
}

// ::llvm::IRBuilderBase::getInt16Ty
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getInt16Ty(::llvm::IRBuilderBase* inst)
{
    return inst->getInt16Ty();
}

// ::llvm::IRBuilderBase::getInt1Ty
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getInt1Ty(::llvm::IRBuilderBase* inst)
{
    return inst->getInt1Ty();
}

// ::llvm::IRBuilderBase::getInt32
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt32(::llvm::IRBuilderBase* inst, uint32_t Value)
{
    return inst->getInt32(Value);
}

// ::llvm::IRBuilderBase::getInt32Ty
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getInt32Ty(::llvm::IRBuilderBase* inst)
{
    return inst->getInt32Ty();
}

// ::llvm::IRBuilderBase::getInt64
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt64(::llvm::IRBuilderBase* inst, uint64_t Value)
{
    return inst->getInt64(Value);
}

// ::llvm::IRBuilderBase::getInt64Ty
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getInt64Ty(::llvm::IRBuilderBase* inst)
{
    return inst->getInt64Ty();
}

// ::llvm::IRBuilderBase::getInt8
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getInt8(::llvm::IRBuilderBase* inst, uint8_t Value)
{
    return inst->getInt8(Value);
}

// ::llvm::IRBuilderBase::getInt8PtrTy
extern "C"
::llvm::PointerType* llvm_IRBuilderBase_getInt8PtrTy(::llvm::IRBuilderBase* inst, unsigned int AddrSpace)
{
    return inst->getInt8PtrTy(AddrSpace);
}

// ::llvm::IRBuilderBase::getInt8Ty
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getInt8Ty(::llvm::IRBuilderBase* inst)
{
    return inst->getInt8Ty();
}

// ::llvm::IRBuilderBase::getIntN
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getIntN(::llvm::IRBuilderBase* inst, unsigned int NumBits, uint64_t Value)
{
    return inst->getIntN(NumBits, Value);
}

// ::llvm::IRBuilderBase::getIntNTy
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getIntNTy(::llvm::IRBuilderBase* inst, unsigned int NumBits)
{
    return inst->getIntNTy(NumBits);
}

// ::llvm::IRBuilderBase::getIntPtrTy
extern "C"
::llvm::IntegerType* llvm_IRBuilderBase_getIntPtrTy(::llvm::IRBuilderBase* inst, ::llvm::DataLayout const* DL, unsigned int AddrSpace)
{
    return inst->getIntPtrTy(DL, AddrSpace);
}

// ::llvm::IRBuilderBase::getTrue
extern "C"
::llvm::ConstantInt* llvm_IRBuilderBase_getTrue(::llvm::IRBuilderBase* inst)
{
    return inst->getTrue();
}

// ::llvm::IRBuilderBase::getVoidTy
extern "C"
::llvm::Type* llvm_IRBuilderBase_getVoidTy(::llvm::IRBuilderBase* inst)
{
    return inst->getVoidTy();
}

// ::llvm::IRBuilderBase::new
extern "C"
::llvm::IRBuilderBase* llvm_IRBuilderBase_new(::llvm::LLVMContext* Context)
{
    return new(std::nothrow) ::llvm::IRBuilderBase(*Context);
}

// ::llvm::Instruction::clone
extern "C"
::llvm::Instruction* llvm_Instruction_clone(::llvm::Instruction const* inst)
{
    return inst->clone();
}

// ::llvm::Instruction::copyFastMathFlags
extern "C"
void llvm_Instruction_copyFastMathFlags(::llvm::Instruction* inst, ::llvm::Instruction const* Inst)
{
    inst->copyFastMathFlags(Inst);
}

// ::llvm::Instruction::delete
extern "C"
void llvm_Instruction_delete(::llvm::Instruction* inst)
{
    delete inst;
}

// ::llvm::Instruction::dropUnknownMetadata
extern "C"
void llvm_Instruction_dropUnknownMetadata(::llvm::Instruction* inst)
{
    inst->dropUnknownMetadata();
}

// ::llvm::Instruction::dropUnknownMetadataFromIDS
extern "C"
void llvm_Instruction_dropUnknownMetadataFromIDS(::llvm::Instruction* inst, llvm_ArrayRef__libc_c_uint _KnownIDs)
{
    auto KnownIDs = ::llvm::ArrayRef<unsigned int>(_KnownIDs.data, _KnownIDs.length);
    inst->dropUnknownMetadata(KnownIDs);
}

// ::llvm::Instruction::eraseFromParent
extern "C"
void llvm_Instruction_eraseFromParent(::llvm::Instruction* inst)
{
    inst->eraseFromParent();
}

// ::llvm::Instruction::getDataLayout
extern "C"
::llvm::DataLayout const* llvm_Instruction_getDataLayout(::llvm::Instruction const* inst)
{
    return inst->getDataLayout();
}

// ::llvm::Instruction::getDebugLoc
extern "C"
::llvm::DebugLoc const* llvm_Instruction_getDebugLoc(::llvm::Instruction const* inst)
{
    return &(inst->getDebugLoc());
}

// ::llvm::Instruction::getMetadata
extern "C"
::llvm::MDNode* llvm_Instruction_getMetadata(::llvm::Instruction const* inst, unsigned int KindID)
{
    return inst->getMetadata(KindID);
}

// ::llvm::Instruction::getMetadataStr
extern "C"
::llvm::MDNode* llvm_Instruction_getMetadataStr(::llvm::Instruction const* inst, llvm_StringRef _Kind)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    return inst->getMetadata(Kind);
}

// ::llvm::Instruction::getOpcode
extern "C"
unsigned int llvm_Instruction_getOpcode(::llvm::Instruction const* inst)
{
    return inst->getOpcode();
}

// ::llvm::Instruction::getParent
extern "C"
::llvm::BasicBlock const* llvm_Instruction_getParent(::llvm::Instruction const* inst)
{
    return inst->getParent();
}

// ::llvm::Instruction::getParentMut
extern "C"
::llvm::BasicBlock* llvm_Instruction_getParentMut(::llvm::Instruction* inst)
{
    return inst->getParent();
}

// ::llvm::Instruction::hasAllowReciprocal
extern "C"
int llvm_Instruction_hasAllowReciprocal(::llvm::Instruction const* inst)
{
    return (inst->hasAllowReciprocal() == true ? 1 : 0);
}

// ::llvm::Instruction::hasMetadata
extern "C"
int llvm_Instruction_hasMetadata(::llvm::Instruction const* inst)
{
    return (inst->hasMetadata() == true ? 1 : 0);
}

// ::llvm::Instruction::hasMetadataOtherThanDebugLoc
extern "C"
int llvm_Instruction_hasMetadataOtherThanDebugLoc(::llvm::Instruction const* inst)
{
    return (inst->hasMetadataOtherThanDebugLoc() == true ? 1 : 0);
}

// ::llvm::Instruction::hasNoInfs
extern "C"
int llvm_Instruction_hasNoInfs(::llvm::Instruction const* inst)
{
    return (inst->hasNoInfs() == true ? 1 : 0);
}

// ::llvm::Instruction::hasNoNaNs
extern "C"
int llvm_Instruction_hasNoNaNs(::llvm::Instruction const* inst)
{
    return (inst->hasNoNaNs() == true ? 1 : 0);
}

// ::llvm::Instruction::hasNoSignedZeros
extern "C"
int llvm_Instruction_hasNoSignedZeros(::llvm::Instruction const* inst)
{
    return (inst->hasNoSignedZeros() == true ? 1 : 0);
}

// ::llvm::Instruction::hasUnsafeAlgebra
extern "C"
int llvm_Instruction_hasUnsafeAlgebra(::llvm::Instruction const* inst)
{
    return (inst->hasUnsafeAlgebra() == true ? 1 : 0);
}

// ::llvm::Instruction::insertAfter
extern "C"
void llvm_Instruction_insertAfter(::llvm::Instruction* inst, ::llvm::Instruction* InsertPos)
{
    inst->insertAfter(InsertPos);
}

// ::llvm::Instruction::insertBefore
extern "C"
void llvm_Instruction_insertBefore(::llvm::Instruction* inst, ::llvm::Instruction* InsertPos)
{
    inst->insertBefore(InsertPos);
}

// ::llvm::Instruction::isArithmeticShift
extern "C"
int llvm_Instruction_isArithmeticShift(::llvm::Instruction const* inst)
{
    return (inst->isArithmeticShift() == true ? 1 : 0);
}

// ::llvm::Instruction::isAssociative
extern "C"
int llvm_Instruction_isAssociative(::llvm::Instruction const* inst)
{
    return (inst->isAssociative() == true ? 1 : 0);
}

// ::llvm::Instruction::isBinaryOp
extern "C"
int llvm_Instruction_isBinaryOp(::llvm::Instruction const* inst)
{
    return (inst->isBinaryOp() == true ? 1 : 0);
}

// ::llvm::Instruction::isCast
extern "C"
int llvm_Instruction_isCast(::llvm::Instruction const* inst)
{
    return (inst->isCast() == true ? 1 : 0);
}

// ::llvm::Instruction::isCommutative
extern "C"
int llvm_Instruction_isCommutative(::llvm::Instruction const* inst)
{
    return (inst->isCommutative() == true ? 1 : 0);
}

// ::llvm::Instruction::isIdempotent
extern "C"
int llvm_Instruction_isIdempotent(::llvm::Instruction const* inst)
{
    return (inst->isIdempotent() == true ? 1 : 0);
}

// ::llvm::Instruction::isIdenticalTo
extern "C"
int llvm_Instruction_isIdenticalTo(::llvm::Instruction const* inst, ::llvm::Instruction const* Inst)
{
    return (inst->isIdenticalTo(Inst) == true ? 1 : 0);
}

// ::llvm::Instruction::isIdenticalToWhenDefined
extern "C"
int llvm_Instruction_isIdenticalToWhenDefined(::llvm::Instruction const* inst, ::llvm::Instruction const* Inst)
{
    return (inst->isIdenticalToWhenDefined(Inst) == true ? 1 : 0);
}

// ::llvm::Instruction::isLogicalShift
extern "C"
int llvm_Instruction_isLogicalShift(::llvm::Instruction const* inst)
{
    return (inst->isLogicalShift() == true ? 1 : 0);
}

// ::llvm::Instruction::isNilpotent
extern "C"
int llvm_Instruction_isNilpotent(::llvm::Instruction const* inst)
{
    return (inst->isNilpotent() == true ? 1 : 0);
}

// ::llvm::Instruction::isSameOperationAs
extern "C"
int llvm_Instruction_isSameOperationAs(::llvm::Instruction const* inst, ::llvm::Instruction const* Inst, unsigned int flags)
{
    return (inst->isSameOperationAs(Inst, flags) == true ? 1 : 0);
}

// ::llvm::Instruction::isShift
extern "C"
int llvm_Instruction_isShift(::llvm::Instruction* inst)
{
    return (inst->isShift() == true ? 1 : 0);
}

// ::llvm::Instruction::isTerminator
extern "C"
int llvm_Instruction_isTerminator(::llvm::Instruction const* inst)
{
    return (inst->isTerminator() == true ? 1 : 0);
}

// ::llvm::Instruction::isUsedOutsideOfBlock
extern "C"
int llvm_Instruction_isUsedOutsideOfBlock(::llvm::Instruction const* inst, ::llvm::BasicBlock const* BB)
{
    return (inst->isUsedOutsideOfBlock(BB) == true ? 1 : 0);
}

// ::llvm::Instruction::mayHaveSideEffects
extern "C"
int llvm_Instruction_mayHaveSideEffects(::llvm::Instruction const* inst)
{
    return (inst->mayHaveSideEffects() == true ? 1 : 0);
}

// ::llvm::Instruction::mayReadFromMemory
extern "C"
int llvm_Instruction_mayReadFromMemory(::llvm::Instruction const* inst)
{
    return (inst->mayReadFromMemory() == true ? 1 : 0);
}

// ::llvm::Instruction::mayReadOrWriteMemory
extern "C"
int llvm_Instruction_mayReadOrWriteMemory(::llvm::Instruction const* inst)
{
    return (inst->mayReadOrWriteMemory() == true ? 1 : 0);
}

// ::llvm::Instruction::mayReturn
extern "C"
int llvm_Instruction_mayReturn(::llvm::Instruction const* inst)
{
    return (inst->mayReturn() == true ? 1 : 0);
}

// ::llvm::Instruction::mayThrow
extern "C"
int llvm_Instruction_mayThrow(::llvm::Instruction const* inst)
{
    return (inst->mayThrow() == true ? 1 : 0);
}

// ::llvm::Instruction::mayWriteToMemory
extern "C"
int llvm_Instruction_mayWriteToMemory(::llvm::Instruction const* inst)
{
    return (inst->mayWriteToMemory() == true ? 1 : 0);
}

// ::llvm::Instruction::moveBefore
extern "C"
void llvm_Instruction_moveBefore(::llvm::Instruction* inst, ::llvm::Instruction* MovePos)
{
    inst->moveBefore(MovePos);
}

// ::llvm::Instruction::removeFromParent
extern "C"
void llvm_Instruction_removeFromParent(::llvm::Instruction* inst)
{
    inst->removeFromParent();
}

// ::llvm::Instruction::setDebugLoc
extern "C"
void llvm_Instruction_setDebugLoc(::llvm::Instruction* inst, ::llvm::DebugLoc const* Loc)
{
    inst->setDebugLoc(*Loc);
}

// ::llvm::Instruction::setHasAllowReciprocal
extern "C"
void llvm_Instruction_setHasAllowReciprocal(::llvm::Instruction* inst, int Val)
{
    inst->setHasAllowReciprocal((Val == 1 ? true : false));
}

// ::llvm::Instruction::setHasNoInfs
extern "C"
void llvm_Instruction_setHasNoInfs(::llvm::Instruction* inst, int Val)
{
    inst->setHasNoInfs((Val == 1 ? true : false));
}

// ::llvm::Instruction::setHasNoNaNs
extern "C"
void llvm_Instruction_setHasNoNaNs(::llvm::Instruction* inst, int Val)
{
    inst->setHasNoNaNs((Val == 1 ? true : false));
}

// ::llvm::Instruction::setHasNoSignedZeros
extern "C"
void llvm_Instruction_setHasNoSignedZeros(::llvm::Instruction* inst, int Val)
{
    inst->setHasNoSignedZeros((Val == 1 ? true : false));
}

// ::llvm::Instruction::setHasUnsafeAlgebra
extern "C"
void llvm_Instruction_setHasUnsafeAlgebra(::llvm::Instruction* inst, int Val)
{
    inst->setHasUnsafeAlgebra((Val == 1 ? true : false));
}

// ::llvm::Instruction::setMetadata
extern "C"
void llvm_Instruction_setMetadata(::llvm::Instruction* inst, unsigned int KindID, ::llvm::MDNode* Node)
{
    inst->setMetadata(KindID, Node);
}

// ::llvm::Instruction::setMetadataStr
extern "C"
void llvm_Instruction_setMetadataStr(::llvm::Instruction* inst, llvm_StringRef _Kind, ::llvm::MDNode* Node)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    inst->setMetadata(Kind, Node);
}

// ::llvm::Instruction::user_back
extern "C"
::llvm::Instruction const* llvm_Instruction_user_back(::llvm::Instruction const* inst)
{
    return inst->user_back();
}

// ::llvm::Instruction::user_back_mut
extern "C"
::llvm::Instruction* llvm_Instruction_user_back_mut(::llvm::Instruction* inst)
{
    return inst->user_back();
}

// ::llvm::IntegerType::classof
extern "C"
int llvm_IntegerType_classof(::llvm::Type const* ty)
{
    return (::llvm::IntegerType::classof(ty) == true ? 1 : 0);
}

// ::llvm::IntegerType::get
extern "C"
::llvm::IntegerType* llvm_IntegerType_get(::llvm::LLVMContext* ctx, unsigned int NumBits)
{
    return ::llvm::IntegerType::get(*ctx, NumBits);
}

// ::llvm::IntegerType::getBitMask
extern "C"
uint64_t llvm_IntegerType_getBitMask(::llvm::IntegerType const* inst)
{
    return inst->getBitMask();
}

// ::llvm::IntegerType::getBitWidth
extern "C"
unsigned int llvm_IntegerType_getBitWidth(::llvm::IntegerType const* inst)
{
    return inst->getBitWidth();
}

// ::llvm::IntegerType::getSignBit
extern "C"
uint64_t llvm_IntegerType_getSignBit(::llvm::IntegerType const* inst)
{
    return inst->getSignBit();
}

// ::llvm::IntegerType::isPowerOf2ByteWidth
extern "C"
int llvm_IntegerType_isPowerOf2ByteWidth(::llvm::IntegerType const* inst)
{
    return (inst->isPowerOf2ByteWidth() == true ? 1 : 0);
}

// ::llvm::LLVMContext::delete
extern "C"
::llvm::LLVMContext* llvm_LLVMContext_delete()
{
    return new(std::nothrow) ::llvm::LLVMContext();
}

// ::llvm::LLVMContext::new
extern "C"
::llvm::LLVMContext* llvm_LLVMContext_new()
{
    return new(std::nothrow) ::llvm::LLVMContext();
}

// ::llvm::Module::appendModuleInlineAsm
extern "C"
void llvm_Module_appendModuleInlineAsm(::llvm::Module* inst, llvm_StringRef _Asm)
{
    auto Asm = ::llvm::StringRef(_Asm.data, _Asm.length);
    inst->appendModuleInlineAsm(Asm);
}

// ::llvm::Module::delete
extern "C"
void llvm_Module_delete(::llvm::Module* inst)
{
    delete inst;
}

// ::llvm::Module::dump
extern "C"
void llvm_Module_dump(::llvm::Module const* inst)
{
    inst->dump();
}

// ::llvm::Module::getContext
extern "C"
::llvm::LLVMContext* llvm_Module_getContext(::llvm::Module const* inst)
{
    return &(inst->getContext());
}

// ::llvm::Module::getDataLayout
extern "C"
::llvm::DataLayout const* llvm_Module_getDataLayout(::llvm::Module const* inst)
{
    return inst->getDataLayout();
}

// ::llvm::Module::getDataLayoutStr
extern "C"
std_string_const llvm_Module_getDataLayoutStr(::llvm::Module const* inst)
{
    auto ret = inst->getDataLayoutStr();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Module::getFunction
extern "C"
::llvm::Function* llvm_Module_getFunction(::llvm::Module const* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->getFunction(Name);
}

// ::llvm::Module::getMDKindID
extern "C"
unsigned int llvm_Module_getMDKindID(::llvm::Module const* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->getMDKindID(Name);
}

// ::llvm::Module::getModuleIdentifier
extern "C"
std_string_const llvm_Module_getModuleIdentifier(::llvm::Module const* inst)
{
    auto ret = inst->getModuleIdentifier();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Module::getModuleInlineAsm
extern "C"
std_string_const llvm_Module_getModuleInlineAsm(::llvm::Module const* inst)
{
    auto ret = inst->getModuleInlineAsm();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Module::getNamedValue
extern "C"
::llvm::GlobalValue* llvm_Module_getNamedValue(::llvm::Module const* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->getNamedValue(Name);
}

// ::llvm::Module::getOrInsertFunction
extern "C"
::llvm::Constant* llvm_Module_getOrInsertFunction(::llvm::Module* inst, llvm_StringRef _Name, ::llvm::FunctionType* ty)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->getOrInsertFunction(Name, ty);
}

// ::llvm::Module::getTargetTriple
extern "C"
std_string_const llvm_Module_getTargetTriple(::llvm::Module const* inst)
{
    auto ret = inst->getTargetTriple();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Module::getTypeByName
extern "C"
::llvm::StructType* llvm_Module_getTypeByName(::llvm::Module const* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->getTypeByName(Name);
}

// ::llvm::Module::new
extern "C"
::llvm::Module* llvm_Module_new(llvm_StringRef _ModuleID, ::llvm::LLVMContext* Context)
{
    auto ModuleID = ::llvm::StringRef(_ModuleID.data, _ModuleID.length);
    return new(std::nothrow) ::llvm::Module(ModuleID, *Context);
}

// ::llvm::Module::setDataLayout
extern "C"
void llvm_Module_setDataLayout(::llvm::Module* inst, ::llvm::DataLayout const* Other)
{
    inst->setDataLayout(Other);
}

// ::llvm::Module::setDataLayoutStr
extern "C"
void llvm_Module_setDataLayoutStr(::llvm::Module* inst, llvm_StringRef _Desc)
{
    auto Desc = ::llvm::StringRef(_Desc.data, _Desc.length);
    inst->setDataLayout(Desc);
}

// ::llvm::Module::setModuleIdentifier
extern "C"
void llvm_Module_setModuleIdentifier(::llvm::Module* inst, llvm_StringRef _ID)
{
    auto ID = ::llvm::StringRef(_ID.data, _ID.length);
    inst->setModuleIdentifier(ID);
}

// ::llvm::Module::setModuleInlineAsm
extern "C"
void llvm_Module_setModuleInlineAsm(::llvm::Module* inst, llvm_StringRef _Asm)
{
    auto Asm = ::llvm::StringRef(_Asm.data, _Asm.length);
    inst->setModuleInlineAsm(Asm);
}

// ::llvm::Module::setTargetTriple
extern "C"
void llvm_Module_setTargetTriple(::llvm::Module* inst, llvm_StringRef _Triple)
{
    auto Triple = ::llvm::StringRef(_Triple.data, _Triple.length);
    inst->setTargetTriple(Triple);
}

// ::llvm::Operator::getOpcode
extern "C"
unsigned int llvm_Operator_getOpcode(::llvm::Operator const* inst)
{
    return inst->getOpcode();
}

// ::llvm::Pass::delete
extern "C"
void llvm_Pass_delete(::llvm::Pass* inst)
{
    delete inst;
}

// ::llvm::Pass::doFinalization
extern "C"
int llvm_Pass_doFinalization(::llvm::Pass* inst, ::llvm::Module* Module)
{
    return (inst->doFinalization(*Module) == true ? 1 : 0);
}

// ::llvm::Pass::doInitialization
extern "C"
int llvm_Pass_doInitialization(::llvm::Pass* inst, ::llvm::Module* Module)
{
    return (inst->doInitialization(*Module) == true ? 1 : 0);
}

// ::llvm::Pass::dump
extern "C"
void llvm_Pass_dump(::llvm::Pass const* inst)
{
    inst->dump();
}

// ::llvm::Pass::getPassKind
extern "C"
::llvm::PassKind llvm_Pass_getPassKind(::llvm::Pass const* inst)
{
    return inst->getPassKind();
}

// ::llvm::PassManager::add
extern "C"
void llvm_PassManager_add(::llvm::PassManager* inst, ::llvm::Pass* Pass)
{
    inst->add(Pass);
}

// ::llvm::PassManager::new
extern "C"
::llvm::PassManager* llvm_PassManager_new()
{
    return new(std::nothrow) ::llvm::PassManager();
}

// ::llvm::PassManager::run
extern "C"
void llvm_PassManager_run(::llvm::PassManager* inst, ::llvm::Module* Module)
{
    inst->run(*Module);
}

// ::llvm::PointerType::classof
extern "C"
int llvm_PointerType_classof(::llvm::Type const* ty)
{
    return (::llvm::PointerType::classof(ty) == true ? 1 : 0);
}

// ::llvm::PointerType::get
extern "C"
::llvm::PointerType* llvm_PointerType_get(::llvm::Type* ElementType, unsigned int AddressSpace)
{
    return ::llvm::PointerType::get(ElementType, AddressSpace);
}

// ::llvm::PointerType::getAddressSpace
extern "C"
unsigned int llvm_PointerType_getAddressSpace(::llvm::PointerType const* inst)
{
    return inst->getAddressSpace();
}

// ::llvm::PointerType::getUnqual
extern "C"
::llvm::PointerType* llvm_PointerType_getUnqual(::llvm::Type* ElementType)
{
    return ::llvm::PointerType::getUnqual(ElementType);
}

// ::llvm::PointerType::isValidElementType
extern "C"
int llvm_PointerType_isValidElementType(::llvm::Type* ty)
{
    return (::llvm::PointerType::isValidElementType(ty) == true ? 1 : 0);
}

// ::llvm::SequentialType::classof
extern "C"
int llvm_SequentialType_classof(::llvm::Type const* ty)
{
    return (::llvm::SequentialType::classof(ty) == true ? 1 : 0);
}

// ::llvm::SequentialType::getElementType
extern "C"
::llvm::Type* llvm_SequentialType_getElementType(::llvm::SequentialType const* inst)
{
    return inst->getElementType();
}

// ::llvm::StructType::classof
extern "C"
int llvm_StructType_classof(::llvm::Type const* ty)
{
    return (::llvm::StructType::classof(ty) == true ? 1 : 0);
}

// ::llvm::StructType::create
extern "C"
::llvm::StructType* llvm_StructType_create(::llvm::LLVMContext* ctx, llvm_ArrayRef_llvm_Type_ptr _Elements, llvm_StringRef _Name)
{
    auto Elements = ::llvm::ArrayRef<::llvm::Type*>(_Elements.data, _Elements.length);
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return ::llvm::StructType::create(*ctx, Elements, Name);
}

// ::llvm::StructType::createPacked
extern "C"
::llvm::StructType* llvm_StructType_createPacked(::llvm::LLVMContext* ctx, llvm_ArrayRef_llvm_Type_ptr _Elements, llvm_StringRef _Name, int isPacked)
{
    auto Elements = ::llvm::ArrayRef<::llvm::Type*>(_Elements.data, _Elements.length);
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return ::llvm::StructType::create(*ctx, Elements, Name, (isPacked == 1 ? true : false));
}

// ::llvm::StructType::getElementType
extern "C"
::llvm::Type* llvm_StructType_getElementType(::llvm::StructType const* inst, unsigned int idx)
{
    return inst->getElementType(idx);
}

// ::llvm::StructType::getName
extern "C"
llvm_StringRef llvm_StructType_getName(::llvm::StructType const* inst)
{
    auto ret = inst->getName();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::StructType::getNumElements
extern "C"
unsigned int llvm_StructType_getNumElements(::llvm::StructType const* inst)
{
    return inst->getNumElements();
}

// ::llvm::StructType::hasName
extern "C"
int llvm_StructType_hasName(::llvm::StructType const* inst)
{
    return (inst->hasName() == true ? 1 : 0);
}

// ::llvm::StructType::isLayoutIdentical
extern "C"
int llvm_StructType_isLayoutIdentical(::llvm::StructType const* inst, ::llvm::StructType* Other)
{
    return (inst->isLayoutIdentical(Other) == true ? 1 : 0);
}

// ::llvm::StructType::isLiteral
extern "C"
int llvm_StructType_isLiteral(::llvm::StructType const* inst)
{
    return (inst->isLiteral() == true ? 1 : 0);
}

// ::llvm::StructType::isOpaque
extern "C"
int llvm_StructType_isOpaque(::llvm::StructType const* inst)
{
    return (inst->isOpaque() == true ? 1 : 0);
}

// ::llvm::StructType::isPacked
extern "C"
int llvm_StructType_isPacked(::llvm::StructType const* inst)
{
    return (inst->isPacked() == true ? 1 : 0);
}

// ::llvm::StructType::isSized
extern "C"
int llvm_StructType_isSized(::llvm::StructType const* inst)
{
    return (inst->isSized() == true ? 1 : 0);
}

// ::llvm::StructType::isValidElementType
extern "C"
int llvm_StructType_isValidElementType(::llvm::Type* ty)
{
    return (::llvm::StructType::isValidElementType(ty) == true ? 1 : 0);
}

// ::llvm::StructType::setBody
extern "C"
void llvm_StructType_setBody(::llvm::StructType* inst, llvm_ArrayRef_llvm_Type_ptr _Elements)
{
    auto Elements = ::llvm::ArrayRef<::llvm::Type*>(_Elements.data, _Elements.length);
    inst->setBody(Elements);
}

// ::llvm::StructType::setBodyPacked
extern "C"
void llvm_StructType_setBodyPacked(::llvm::StructType* inst, llvm_ArrayRef_llvm_Type_ptr _Elements, int isPacked)
{
    auto Elements = ::llvm::ArrayRef<::llvm::Type*>(_Elements.data, _Elements.length);
    inst->setBody(Elements, (isPacked == 1 ? true : false));
}

// ::llvm::StructType::setName
extern "C"
void llvm_StructType_setName(::llvm::StructType* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    inst->setName(Name);
}

// ::llvm::Type::dump
extern "C"
void llvm_Type_dump(::llvm::Type const* inst)
{
    inst->dump();
}

// ::llvm::Type::getContainedType
extern "C"
::llvm::Type* llvm_Type_getContainedType(::llvm::Type const* inst, unsigned int idx)
{
    return inst->getContainedType(idx);
}

// ::llvm::Type::getContext
extern "C"
::llvm::LLVMContext* llvm_Type_getContext(::llvm::Type const* inst)
{
    return &(inst->getContext());
}

// ::llvm::Type::getDoublePtrTy
extern "C"
::llvm::PointerType* llvm_Type_getDoublePtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getDoublePtrTy(*ctx);
}

// ::llvm::Type::getDoubleTy
extern "C"
::llvm::Type* llvm_Type_getDoubleTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getDoubleTy(*ctx);
}

// ::llvm::Type::getFP128PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getFP128PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getFP128PtrTy(*ctx);
}

// ::llvm::Type::getFP128Ty
extern "C"
::llvm::Type* llvm_Type_getFP128Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getFP128Ty(*ctx);
}

// ::llvm::Type::getFloatPtrTy
extern "C"
::llvm::PointerType* llvm_Type_getFloatPtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getFloatPtrTy(*ctx);
}

// ::llvm::Type::getFloatTy
extern "C"
::llvm::Type* llvm_Type_getFloatTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getFloatTy(*ctx);
}

// ::llvm::Type::getFunctionNumParams
extern "C"
unsigned int llvm_Type_getFunctionNumParams(::llvm::Type const* inst)
{
    return inst->getFunctionNumParams();
}

// ::llvm::Type::getFunctionParamType
extern "C"
::llvm::Type* llvm_Type_getFunctionParamType(::llvm::Type const* inst, unsigned int idx)
{
    return inst->getFunctionParamType(idx);
}

// ::llvm::Type::getHalfPtrTy
extern "C"
::llvm::PointerType* llvm_Type_getHalfPtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getHalfPtrTy(*ctx);
}

// ::llvm::Type::getHalfTy
extern "C"
::llvm::Type* llvm_Type_getHalfTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getHalfTy(*ctx);
}

// ::llvm::Type::getInt16PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getInt16PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt16PtrTy(*ctx);
}

// ::llvm::Type::getInt16Ty
extern "C"
::llvm::IntegerType* llvm_Type_getInt16Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt16Ty(*ctx);
}

// ::llvm::Type::getInt1PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getInt1PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt1PtrTy(*ctx);
}

// ::llvm::Type::getInt1Ty
extern "C"
::llvm::IntegerType* llvm_Type_getInt1Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt1Ty(*ctx);
}

// ::llvm::Type::getInt32PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getInt32PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt32PtrTy(*ctx);
}

// ::llvm::Type::getInt32Ty
extern "C"
::llvm::IntegerType* llvm_Type_getInt32Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt32Ty(*ctx);
}

// ::llvm::Type::getInt64PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getInt64PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt64PtrTy(*ctx);
}

// ::llvm::Type::getInt64Ty
extern "C"
::llvm::IntegerType* llvm_Type_getInt64Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt64Ty(*ctx);
}

// ::llvm::Type::getInt8PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getInt8PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt8PtrTy(*ctx);
}

// ::llvm::Type::getInt8Ty
extern "C"
::llvm::IntegerType* llvm_Type_getInt8Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getInt8Ty(*ctx);
}

// ::llvm::Type::getIntNPtrTy
extern "C"
::llvm::PointerType* llvm_Type_getIntNPtrTy(::llvm::LLVMContext* ctx, unsigned int size)
{
    return ::llvm::Type::getIntNPtrTy(*ctx, size);
}

// ::llvm::Type::getIntNTy
extern "C"
::llvm::IntegerType* llvm_Type_getIntNTy(::llvm::LLVMContext* ctx, unsigned int size)
{
    return ::llvm::Type::getIntNTy(*ctx, size);
}

// ::llvm::Type::getLabelTy
extern "C"
::llvm::Type* llvm_Type_getLabelTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getLabelTy(*ctx);
}

// ::llvm::Type::getMetadataTy
extern "C"
::llvm::Type* llvm_Type_getMetadataTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getMetadataTy(*ctx);
}

// ::llvm::Type::getNumContainedTypes
extern "C"
unsigned int llvm_Type_getNumContainedTypes(::llvm::Type const* inst)
{
    return inst->getNumContainedTypes();
}

// ::llvm::Type::getPPC_FP128PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getPPC_FP128PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getPPC_FP128PtrTy(*ctx);
}

// ::llvm::Type::getPPC_FP128Ty
extern "C"
::llvm::Type* llvm_Type_getPPC_FP128Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getPPC_FP128Ty(*ctx);
}

// ::llvm::Type::getPointerAddressSpace
extern "C"
unsigned int llvm_Type_getPointerAddressSpace(::llvm::Type const* inst)
{
    return inst->getPointerAddressSpace();
}

// ::llvm::Type::getPointerElementType
extern "C"
::llvm::Type* llvm_Type_getPointerElementType(::llvm::Type const* inst)
{
    return inst->getPointerElementType();
}

// ::llvm::Type::getPointerTo
extern "C"
::llvm::PointerType* llvm_Type_getPointerTo(::llvm::Type* inst, unsigned int idx)
{
    return inst->getPointerTo(idx);
}

// ::llvm::Type::getSequentialElementType
extern "C"
::llvm::Type* llvm_Type_getSequentialElementType(::llvm::Type const* inst)
{
    return inst->getSequentialElementType();
}

// ::llvm::Type::getStructElementType
extern "C"
::llvm::Type* llvm_Type_getStructElementType(::llvm::Type const* inst, unsigned int idx)
{
    return inst->getStructElementType(idx);
}

// ::llvm::Type::getStructName
extern "C"
llvm_StringRef llvm_Type_getStructName(::llvm::Type const* inst)
{
    auto ret = inst->getStructName();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Type::getStructNumElements
extern "C"
unsigned int llvm_Type_getStructNumElements(::llvm::Type const* inst)
{
    return inst->getStructNumElements();
}

// ::llvm::Type::getTypeID
extern "C"
::llvm::Type::TypeID llvm_Type_getTypeID(::llvm::Type const* inst)
{
    return inst->getTypeID();
}

// ::llvm::Type::getVoidTy
extern "C"
::llvm::Type* llvm_Type_getVoidTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getVoidTy(*ctx);
}

// ::llvm::Type::getX86_FP80PtrTy
extern "C"
::llvm::PointerType* llvm_Type_getX86_FP80PtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getX86_FP80PtrTy(*ctx);
}

// ::llvm::Type::getX86_FP80Ty
extern "C"
::llvm::Type* llvm_Type_getX86_FP80Ty(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getX86_FP80Ty(*ctx);
}

// ::llvm::Type::getX86_MMXPtrTy
extern "C"
::llvm::PointerType* llvm_Type_getX86_MMXPtrTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getX86_MMXPtrTy(*ctx);
}

// ::llvm::Type::getX86_MMXTy
extern "C"
::llvm::Type* llvm_Type_getX86_MMXTy(::llvm::LLVMContext* ctx)
{
    return ::llvm::Type::getX86_MMXTy(*ctx);
}

// ::llvm::Type::isAggregateType
extern "C"
int llvm_Type_isAggregateType(::llvm::Type const* inst)
{
    return (inst->isAggregateType() == true ? 1 : 0);
}

// ::llvm::Type::isArrayTy
extern "C"
int llvm_Type_isArrayTy(::llvm::Type const* inst)
{
    return (inst->isArrayTy() == true ? 1 : 0);
}

// ::llvm::Type::isDoubleTy
extern "C"
int llvm_Type_isDoubleTy(::llvm::Type const* inst)
{
    return (inst->isDoubleTy() == true ? 1 : 0);
}

// ::llvm::Type::isEmptyTy
extern "C"
int llvm_Type_isEmptyTy(::llvm::Type const* inst)
{
    return (inst->isEmptyTy() == true ? 1 : 0);
}

// ::llvm::Type::isFP128Ty
extern "C"
int llvm_Type_isFP128Ty(::llvm::Type const* inst)
{
    return (inst->isFP128Ty() == true ? 1 : 0);
}

// ::llvm::Type::isFPOrFPVectorTy
extern "C"
int llvm_Type_isFPOrFPVectorTy(::llvm::Type const* inst)
{
    return (inst->isFPOrFPVectorTy() == true ? 1 : 0);
}

// ::llvm::Type::isFirstClassType
extern "C"
int llvm_Type_isFirstClassType(::llvm::Type const* inst)
{
    return (inst->isFirstClassType() == true ? 1 : 0);
}

// ::llvm::Type::isFloatTy
extern "C"
int llvm_Type_isFloatTy(::llvm::Type const* inst)
{
    return (inst->isFloatTy() == true ? 1 : 0);
}

// ::llvm::Type::isFloatingPointTy
extern "C"
int llvm_Type_isFloatingPointTy(::llvm::Type const* inst)
{
    return (inst->isFloatingPointTy() == true ? 1 : 0);
}

// ::llvm::Type::isFunctionTy
extern "C"
int llvm_Type_isFunctionTy(::llvm::Type const* inst)
{
    return (inst->isFunctionTy() == true ? 1 : 0);
}

// ::llvm::Type::isFunctionVarArg
extern "C"
int llvm_Type_isFunctionVarArg(::llvm::Type const* inst)
{
    return (inst->isFunctionVarArg() == true ? 1 : 0);
}

// ::llvm::Type::isHalfTy
extern "C"
int llvm_Type_isHalfTy(::llvm::Type const* inst)
{
    return (inst->isHalfTy() == true ? 1 : 0);
}

// ::llvm::Type::isIntOrIntVectorTy
extern "C"
int llvm_Type_isIntOrIntVectorTy(::llvm::Type const* inst)
{
    return (inst->isIntOrIntVectorTy() == true ? 1 : 0);
}

// ::llvm::Type::isIntegerTy
extern "C"
int llvm_Type_isIntegerTy(::llvm::Type const* inst)
{
    return (inst->isIntegerTy() == true ? 1 : 0);
}

// ::llvm::Type::isLabelTy
extern "C"
int llvm_Type_isLabelTy(::llvm::Type const* inst)
{
    return (inst->isLabelTy() == true ? 1 : 0);
}

// ::llvm::Type::isMetadataTy
extern "C"
int llvm_Type_isMetadataTy(::llvm::Type const* inst)
{
    return (inst->isMetadataTy() == true ? 1 : 0);
}

// ::llvm::Type::isPPC_FP128Ty
extern "C"
int llvm_Type_isPPC_FP128Ty(::llvm::Type const* inst)
{
    return (inst->isPPC_FP128Ty() == true ? 1 : 0);
}

// ::llvm::Type::isPointerTy
extern "C"
int llvm_Type_isPointerTy(::llvm::Type const* inst)
{
    return (inst->isPointerTy() == true ? 1 : 0);
}

// ::llvm::Type::isPtrOrPtrVectorTy
extern "C"
int llvm_Type_isPtrOrPtrVectorTy(::llvm::Type const* inst)
{
    return (inst->isPtrOrPtrVectorTy() == true ? 1 : 0);
}

// ::llvm::Type::isSingleValueType
extern "C"
int llvm_Type_isSingleValueType(::llvm::Type const* inst)
{
    return (inst->isSingleValueType() == true ? 1 : 0);
}

// ::llvm::Type::isSized
extern "C"
int llvm_Type_isSized(::llvm::Type const* inst)
{
    return (inst->isSized() == true ? 1 : 0);
}

// ::llvm::Type::isStructTy
extern "C"
int llvm_Type_isStructTy(::llvm::Type const* inst)
{
    return (inst->isStructTy() == true ? 1 : 0);
}

// ::llvm::Type::isVectorTy
extern "C"
int llvm_Type_isVectorTy(::llvm::Type const* inst)
{
    return (inst->isVectorTy() == true ? 1 : 0);
}

// ::llvm::Type::isVoidTy
extern "C"
int llvm_Type_isVoidTy(::llvm::Type const* inst)
{
    return (inst->isVoidTy() == true ? 1 : 0);
}

// ::llvm::Type::isX86_FP80Ty
extern "C"
int llvm_Type_isX86_FP80Ty(::llvm::Type const* inst)
{
    return (inst->isX86_FP80Ty() == true ? 1 : 0);
}

// ::llvm::Type::isX86_MMXTy
extern "C"
int llvm_Type_isX86_MMXTy(::llvm::Type const* inst)
{
    return (inst->isX86_MMXTy() == true ? 1 : 0);
}

// ::llvm::Use::get
extern "C"
::llvm::Value* llvm_Use_get(::llvm::Use const* inst)
{
    return inst->get();
}

// ::llvm::Use::getNext
extern "C"
::llvm::Use* llvm_Use_getNext(::llvm::Use const* inst)
{
    return inst->getNext();
}

// ::llvm::Use::getOperandNo
extern "C"
unsigned int llvm_Use_getOperandNo(::llvm::Use const* inst)
{
    return inst->getOperandNo();
}

// ::llvm::Use::getUser
extern "C"
::llvm::User* llvm_Use_getUser(::llvm::Use const* inst)
{
    return inst->getUser();
}

// ::llvm::Use::initTags
extern "C"
::llvm::Use* llvm_Use_initTags(::llvm::Use* Start, ::llvm::Use* Stop)
{
    return ::llvm::Use::initTags(Start, Stop);
}

// ::llvm::Use::set
extern "C"
void llvm_Use_set(::llvm::Use* inst, ::llvm::Value* Val)
{
    inst->set(Val);
}

// ::llvm::Use::swap
extern "C"
void llvm_Use_swap(::llvm::Use* inst, ::llvm::Use* RHS)
{
    inst->swap(*RHS);
}

// ::llvm::User::classof
extern "C"
int llvm_User_classof(::llvm::Value* V)
{
    return (::llvm::User::classof(V) == true ? 1 : 0);
}

// ::llvm::User::delete
extern "C"
void llvm_User_delete(::llvm::User* inst)
{
    delete inst;
}

// ::llvm::User::dropAllReferences
extern "C"
void llvm_User_dropAllReferences(::llvm::User* inst)
{
    inst->dropAllReferences();
}

// ::llvm::User::getNumOperands
extern "C"
unsigned int llvm_User_getNumOperands(::llvm::User const* inst)
{
    return inst->getNumOperands();
}

// ::llvm::User::getOperand
extern "C"
::llvm::Value* llvm_User_getOperand(::llvm::User const* inst, unsigned int idx)
{
    return inst->getOperand(idx);
}

// ::llvm::User::replaceUsesOfWith
extern "C"
void llvm_User_replaceUsesOfWith(::llvm::User* inst, ::llvm::Value* From, ::llvm::Value* To)
{
    inst->replaceUsesOfWith(From, To);
}

// ::llvm::User::setOperand
extern "C"
void llvm_User_setOperand(::llvm::User* inst, unsigned int idx, ::llvm::Value* Val)
{
    inst->setOperand(idx, Val);
}

// ::llvm::Value::delete
extern "C"
void llvm_Value_delete(::llvm::Value* inst)
{
    delete inst;
}

// ::llvm::Value::dump
extern "C"
void llvm_Value_dump(::llvm::Value const* inst)
{
    inst->dump();
}

// ::llvm::Value::getContext
extern "C"
::llvm::LLVMContext* llvm_Value_getContext(::llvm::Value const* inst)
{
    return &(inst->getContext());
}

// ::llvm::Value::getName
extern "C"
llvm_StringRef llvm_Value_getName(::llvm::Value const* inst)
{
    auto ret = inst->getName();
    return {
        .data = ret.data(),
        .length = ret.size(),
    };
}

// ::llvm::Value::getNumUses
extern "C"
unsigned int llvm_Value_getNumUses(::llvm::Value const* inst)
{
    return inst->getNumUses();
}

// ::llvm::Value::getType
extern "C"
::llvm::Type* llvm_Value_getType(::llvm::Value const* inst)
{
    return inst->getType();
}

// ::llvm::Value::getValueID
extern "C"
unsigned int llvm_Value_getValueID(::llvm::Value const* inst)
{
    return inst->getValueID();
}

// ::llvm::Value::hasNUses
extern "C"
int llvm_Value_hasNUses(::llvm::Value const* inst, unsigned int N)
{
    return (inst->hasNUses(N) == true ? 1 : 0);
}

// ::llvm::Value::hasNUsesOrMore
extern "C"
int llvm_Value_hasNUsesOrMore(::llvm::Value const* inst, unsigned int N)
{
    return (inst->hasNUsesOrMore(N) == true ? 1 : 0);
}

// ::llvm::Value::hasName
extern "C"
int llvm_Value_hasName(::llvm::Value const* inst)
{
    return (inst->hasName() == true ? 1 : 0);
}

// ::llvm::Value::hasOneUse
extern "C"
int llvm_Value_hasOneUse(::llvm::Value const* inst)
{
    return (inst->hasOneUse() == true ? 1 : 0);
}

// ::llvm::Value::isUsedInBasicBlock
extern "C"
int llvm_Value_isUsedInBasicBlock(::llvm::Value const* inst, ::llvm::BasicBlock const* BB)
{
    return (inst->isUsedInBasicBlock(BB) == true ? 1 : 0);
}

// ::llvm::Value::mutateType
extern "C"
void llvm_Value_mutateType(::llvm::Value* inst, ::llvm::Type* ty)
{
    inst->mutateType(ty);
}

// ::llvm::Value::replaceAllUsesWith
extern "C"
void llvm_Value_replaceAllUsesWith(::llvm::Value* inst, ::llvm::Value* Value)
{
    inst->replaceAllUsesWith(Value);
}

// ::llvm::Value::setName
extern "C"
void llvm_Value_setName(::llvm::Value* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    inst->setName(Name);
}

// ::llvm::Value::takeName
extern "C"
void llvm_Value_takeName(::llvm::Value* inst, ::llvm::Value* Value)
{
    inst->takeName(Value);
}

// ::llvm::ValueSymbolTable::delete
extern "C"
::llvm::ValueSymbolTable* llvm_ValueSymbolTable_delete()
{
    return new(std::nothrow) ::llvm::ValueSymbolTable();
}

// ::llvm::ValueSymbolTable::dump
extern "C"
void llvm_ValueSymbolTable_dump(::llvm::ValueSymbolTable const* inst)
{
    inst->dump();
}

// ::llvm::ValueSymbolTable::empty
extern "C"
int llvm_ValueSymbolTable_empty(::llvm::ValueSymbolTable const* inst)
{
    return (inst->empty() == true ? 1 : 0);
}

// ::llvm::ValueSymbolTable::lookup
extern "C"
::llvm::Value* llvm_ValueSymbolTable_lookup(::llvm::ValueSymbolTable const* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->lookup(Name);
}

// ::llvm::ValueSymbolTable::new
extern "C"
::llvm::ValueSymbolTable* llvm_ValueSymbolTable_new()
{
    return new(std::nothrow) ::llvm::ValueSymbolTable();
}

// ::llvm::ValueSymbolTable::size
extern "C"
unsigned int llvm_ValueSymbolTable_size(::llvm::ValueSymbolTable const* inst)
{
    return inst->size();
}

// ::llvm::VectorType::classof
extern "C"
int llvm_VectorType_classof(::llvm::Type const* ty)
{
    return (::llvm::VectorType::classof(ty) == true ? 1 : 0);
}

// ::llvm::VectorType::get
extern "C"
::llvm::VectorType* llvm_VectorType_get(::llvm::Type* ty, unsigned int NumElements)
{
    return ::llvm::VectorType::get(ty, NumElements);
}

// ::llvm::VectorType::getBitWidth
extern "C"
unsigned int llvm_VectorType_getBitWidth(::llvm::VectorType const* inst)
{
    return inst->getBitWidth();
}

// ::llvm::VectorType::getDoubleElementsVectorType
extern "C"
::llvm::VectorType* llvm_VectorType_getDoubleElementsVectorType(::llvm::VectorType* ty)
{
    return ::llvm::VectorType::getDoubleElementsVectorType(ty);
}

// ::llvm::VectorType::getExtendedElementVectorType
extern "C"
::llvm::VectorType* llvm_VectorType_getExtendedElementVectorType(::llvm::VectorType* ty)
{
    return ::llvm::VectorType::getExtendedElementVectorType(ty);
}

// ::llvm::VectorType::getHalfElementsVectorType
extern "C"
::llvm::VectorType* llvm_VectorType_getHalfElementsVectorType(::llvm::VectorType* ty)
{
    return ::llvm::VectorType::getHalfElementsVectorType(ty);
}

// ::llvm::VectorType::getInteger
extern "C"
::llvm::VectorType* llvm_VectorType_getInteger(::llvm::VectorType* ty)
{
    return ::llvm::VectorType::getInteger(ty);
}

// ::llvm::VectorType::getNumElements
extern "C"
unsigned int llvm_VectorType_getNumElements(::llvm::VectorType const* inst)
{
    return inst->getNumElements();
}

// ::llvm::VectorType::getTruncatedElementVectorType
extern "C"
::llvm::VectorType* llvm_VectorType_getTruncatedElementVectorType(::llvm::VectorType* ty)
{
    return ::llvm::VectorType::getTruncatedElementVectorType(ty);
}

// ::llvm::VectorType::isValidElementType
extern "C"
int llvm_VectorType_isValidElementType(::llvm::Type* ty)
{
    return (::llvm::VectorType::isValidElementType(ty) == true ? 1 : 0);
}

// ::llvm::createAddDiscriminatorsPass
extern "C"
::llvm::FunctionPass* llvm_createAddDiscriminatorsPass()
{
    return ::llvm::createAddDiscriminatorsPass();
}

// ::llvm::createAddressSanitizerFunctionPass
extern "C"
::llvm::FunctionPass* llvm_createAddressSanitizerFunctionPass()
{
    return ::llvm::createAddressSanitizerFunctionPass();
}

// ::llvm::createAddressSanitizerModulePass
extern "C"
::llvm::ModulePass* llvm_createAddressSanitizerModulePass()
{
    return ::llvm::createAddressSanitizerModulePass();
}

// ::llvm::createAggressiveDCEPass
extern "C"
::llvm::FunctionPass* llvm_createAggressiveDCEPass()
{
    return ::llvm::createAggressiveDCEPass();
}

// ::llvm::createAlwaysInlinerPass
extern "C"
::llvm::Pass* llvm_createAlwaysInlinerPass(int InsertLifetime)
{
    return ::llvm::createAlwaysInlinerPass((InsertLifetime == 1 ? true : false));
}

// ::llvm::createArgumentPromotionPass
extern "C"
::llvm::Pass* llvm_createArgumentPromotionPass(unsigned int maxElements)
{
    return ::llvm::createArgumentPromotionPass(maxElements);
}

// ::llvm::createBarrierNoopPass
extern "C"
::llvm::ModulePass* llvm_createBarrierNoopPass()
{
    return ::llvm::createBarrierNoopPass();
}

// ::llvm::createBlockExtractorPass
extern "C"
::llvm::ModulePass* llvm_createBlockExtractorPass()
{
    return ::llvm::createBlockExtractorPass();
}

// ::llvm::createBoundsCheckingPass
extern "C"
::llvm::FunctionPass* llvm_createBoundsCheckingPass()
{
    return ::llvm::createBoundsCheckingPass();
}

// ::llvm::createBreakCriticalEdgesPass
extern "C"
::llvm::FunctionPass* llvm_createBreakCriticalEdgesPass()
{
    return ::llvm::createBreakCriticalEdgesPass();
}

// ::llvm::createCFGSimplificationPass
extern "C"
::llvm::FunctionPass* llvm_createCFGSimplificationPass()
{
    return ::llvm::createCFGSimplificationPass();
}

// ::llvm::createConstantHoistingPass
extern "C"
::llvm::FunctionPass* llvm_createConstantHoistingPass()
{
    return ::llvm::createConstantHoistingPass();
}

// ::llvm::createConstantMergePass
extern "C"
::llvm::ModulePass* llvm_createConstantMergePass()
{
    return ::llvm::createConstantMergePass();
}

// ::llvm::createConstantPropagationPass
extern "C"
::llvm::FunctionPass* llvm_createConstantPropagationPass()
{
    return ::llvm::createConstantPropagationPass();
}

// ::llvm::createCorrelatedValuePropagationPass
extern "C"
::llvm::Pass* llvm_createCorrelatedValuePropagationPass()
{
    return ::llvm::createCorrelatedValuePropagationPass();
}

// ::llvm::createDataFlowSanitizerPass
extern "C"
::llvm::ModulePass* llvm_createDataFlowSanitizerPass()
{
    return ::llvm::createDataFlowSanitizerPass();
}

// ::llvm::createDeadArgEliminationPass
extern "C"
::llvm::ModulePass* llvm_createDeadArgEliminationPass()
{
    return ::llvm::createDeadArgEliminationPass();
}

// ::llvm::createDeadArgHackingPass
extern "C"
::llvm::ModulePass* llvm_createDeadArgHackingPass()
{
    return ::llvm::createDeadArgHackingPass();
}

// ::llvm::createDeadCodeEliminationPass
extern "C"
::llvm::FunctionPass* llvm_createDeadCodeEliminationPass()
{
    return ::llvm::createDeadCodeEliminationPass();
}

// ::llvm::createDeadInstEliminationPass
extern "C"
::llvm::Pass* llvm_createDeadInstEliminationPass()
{
    return ::llvm::createDeadInstEliminationPass();
}

// ::llvm::createDeadStoreEliminationPass
extern "C"
::llvm::FunctionPass* llvm_createDeadStoreEliminationPass()
{
    return ::llvm::createDeadStoreEliminationPass();
}

// ::llvm::createDebugIRPass
extern "C"
::llvm::ModulePass* llvm_createDebugIRPass()
{
    return ::llvm::createDebugIRPass();
}

// ::llvm::createDemoteRegisterToMemoryPass
extern "C"
::llvm::FunctionPass* llvm_createDemoteRegisterToMemoryPass()
{
    return ::llvm::createDemoteRegisterToMemoryPass();
}

// ::llvm::createEarlyCSEPass
extern "C"
::llvm::FunctionPass* llvm_createEarlyCSEPass()
{
    return ::llvm::createEarlyCSEPass();
}

// ::llvm::createFlattenCFGPass
extern "C"
::llvm::FunctionPass* llvm_createFlattenCFGPass()
{
    return ::llvm::createFlattenCFGPass();
}

// ::llvm::createFunctionAttrsPass
extern "C"
::llvm::Pass* llvm_createFunctionAttrsPass()
{
    return ::llvm::createFunctionAttrsPass();
}

// ::llvm::createFunctionInliningPass
extern "C"
::llvm::Pass* llvm_createFunctionInliningPass()
{
    return ::llvm::createFunctionInliningPass();
}

// ::llvm::createGCOVProfilerPass
extern "C"
::llvm::ModulePass* llvm_createGCOVProfilerPass()
{
    return ::llvm::createGCOVProfilerPass();
}

// ::llvm::createGVNPass
extern "C"
::llvm::FunctionPass* llvm_createGVNPass(int NoLoads)
{
    return ::llvm::createGVNPass((NoLoads == 1 ? true : false));
}

// ::llvm::createGlobalDCEPass
extern "C"
::llvm::ModulePass* llvm_createGlobalDCEPass()
{
    return ::llvm::createGlobalDCEPass();
}

// ::llvm::createGlobalMergePass
extern "C"
::llvm::Pass* llvm_createGlobalMergePass()
{
    return ::llvm::createGlobalMergePass();
}

// ::llvm::createGlobalOptimizerPass
extern "C"
::llvm::ModulePass* llvm_createGlobalOptimizerPass()
{
    return ::llvm::createGlobalOptimizerPass();
}

// ::llvm::createIPConstantPropagationPass
extern "C"
::llvm::ModulePass* llvm_createIPConstantPropagationPass()
{
    return ::llvm::createIPConstantPropagationPass();
}

// ::llvm::createIPSCCPPass
extern "C"
::llvm::ModulePass* llvm_createIPSCCPPass()
{
    return ::llvm::createIPSCCPPass();
}

// ::llvm::createIndVarSimplifyPass
extern "C"
::llvm::Pass* llvm_createIndVarSimplifyPass()
{
    return ::llvm::createIndVarSimplifyPass();
}

// ::llvm::createInstructionCombiningPass
extern "C"
::llvm::FunctionPass* llvm_createInstructionCombiningPass()
{
    return ::llvm::createInstructionCombiningPass();
}

// ::llvm::createInstructionNamerPass
extern "C"
::llvm::FunctionPass* llvm_createInstructionNamerPass()
{
    return ::llvm::createInstructionNamerPass();
}

// ::llvm::createInstructionSimplifierPass
extern "C"
::llvm::FunctionPass* llvm_createInstructionSimplifierPass()
{
    return ::llvm::createInstructionSimplifierPass();
}

// ::llvm::createInternalizePass
extern "C"
::llvm::ModulePass* llvm_createInternalizePass()
{
    return ::llvm::createInternalizePass();
}

// ::llvm::createJumpThreadingPass
extern "C"
::llvm::FunctionPass* llvm_createJumpThreadingPass()
{
    return ::llvm::createJumpThreadingPass();
}

// ::llvm::createLCSSAPass
extern "C"
::llvm::Pass* llvm_createLCSSAPass()
{
    return ::llvm::createLCSSAPass();
}

// ::llvm::createLICMPass
extern "C"
::llvm::Pass* llvm_createLICMPass()
{
    return ::llvm::createLICMPass();
}

// ::llvm::createLoadCombinePass
extern "C"
::llvm::BasicBlockPass* llvm_createLoadCombinePass()
{
    return ::llvm::createLoadCombinePass();
}

// ::llvm::createLoopDeletionPass
extern "C"
::llvm::Pass* llvm_createLoopDeletionPass()
{
    return ::llvm::createLoopDeletionPass();
}

// ::llvm::createLoopExtractorPass
extern "C"
::llvm::Pass* llvm_createLoopExtractorPass()
{
    return ::llvm::createLoopExtractorPass();
}

// ::llvm::createLoopIdiomPass
extern "C"
::llvm::Pass* llvm_createLoopIdiomPass()
{
    return ::llvm::createLoopIdiomPass();
}

// ::llvm::createLoopInstSimplifyPass
extern "C"
::llvm::Pass* llvm_createLoopInstSimplifyPass()
{
    return ::llvm::createLoopInstSimplifyPass();
}

// ::llvm::createLoopRerollPass
extern "C"
::llvm::Pass* llvm_createLoopRerollPass()
{
    return ::llvm::createLoopRerollPass();
}

// ::llvm::createLoopRotatePass
extern "C"
::llvm::Pass* llvm_createLoopRotatePass(int MaxHeaderSize)
{
    return ::llvm::createLoopRotatePass(MaxHeaderSize);
}

// ::llvm::createLoopSimplifyPass
extern "C"
::llvm::Pass* llvm_createLoopSimplifyPass()
{
    return ::llvm::createLoopSimplifyPass();
}

// ::llvm::createLoopStrengthReducePass
extern "C"
::llvm::Pass* llvm_createLoopStrengthReducePass()
{
    return ::llvm::createLoopStrengthReducePass();
}

// ::llvm::createLoopUnrollPass
extern "C"
::llvm::Pass* llvm_createLoopUnrollPass()
{
    return ::llvm::createLoopUnrollPass();
}

// ::llvm::createLoopUnswitchPass
extern "C"
::llvm::Pass* llvm_createLoopUnswitchPass(int OptimizeForSize)
{
    return ::llvm::createLoopUnswitchPass((OptimizeForSize == 1 ? true : false));
}

// ::llvm::createLowerAtomicPass
extern "C"
::llvm::Pass* llvm_createLowerAtomicPass()
{
    return ::llvm::createLowerAtomicPass();
}

// ::llvm::createLowerExpectIntrinsicPass
extern "C"
::llvm::FunctionPass* llvm_createLowerExpectIntrinsicPass()
{
    return ::llvm::createLowerExpectIntrinsicPass();
}

// ::llvm::createLowerInvokePass
extern "C"
::llvm::FunctionPass* llvm_createLowerInvokePass()
{
    return ::llvm::createLowerInvokePass();
}

// ::llvm::createLowerSwitchPass
extern "C"
::llvm::FunctionPass* llvm_createLowerSwitchPass()
{
    return ::llvm::createLowerSwitchPass();
}

// ::llvm::createMemCpyOptPass
extern "C"
::llvm::FunctionPass* llvm_createMemCpyOptPass()
{
    return ::llvm::createMemCpyOptPass();
}

// ::llvm::createMemorySanitizerPass
extern "C"
::llvm::FunctionPass* llvm_createMemorySanitizerPass(int TrackOrigins)
{
    return ::llvm::createMemorySanitizerPass(TrackOrigins);
}

// ::llvm::createMergeFunctionsPass
extern "C"
::llvm::ModulePass* llvm_createMergeFunctionsPass()
{
    return ::llvm::createMergeFunctionsPass();
}

// ::llvm::createMergedLoadStoreMotionPass
extern "C"
::llvm::FunctionPass* llvm_createMergedLoadStoreMotionPass()
{
    return ::llvm::createMergedLoadStoreMotionPass();
}

// ::llvm::createMetaRenamerPass
extern "C"
::llvm::ModulePass* llvm_createMetaRenamerPass()
{
    return ::llvm::createMetaRenamerPass();
}

// ::llvm::createObjCARCAPElimPass
extern "C"
::llvm::Pass* llvm_createObjCARCAPElimPass()
{
    return ::llvm::createObjCARCAPElimPass();
}

// ::llvm::createObjCARCContractPass
extern "C"
::llvm::Pass* llvm_createObjCARCContractPass()
{
    return ::llvm::createObjCARCContractPass();
}

// ::llvm::createObjCARCExpandPass
extern "C"
::llvm::Pass* llvm_createObjCARCExpandPass()
{
    return ::llvm::createObjCARCExpandPass();
}

// ::llvm::createObjCARCOptPass
extern "C"
::llvm::Pass* llvm_createObjCARCOptPass()
{
    return ::llvm::createObjCARCOptPass();
}

// ::llvm::createPartialInliningPass
extern "C"
::llvm::ModulePass* llvm_createPartialInliningPass()
{
    return ::llvm::createPartialInliningPass();
}

// ::llvm::createPartiallyInlineLibCallsPass
extern "C"
::llvm::FunctionPass* llvm_createPartiallyInlineLibCallsPass()
{
    return ::llvm::createPartiallyInlineLibCallsPass();
}

// ::llvm::createPromoteMemoryToRegisterPass
extern "C"
::llvm::FunctionPass* llvm_createPromoteMemoryToRegisterPass()
{
    return ::llvm::createPromoteMemoryToRegisterPass();
}

// ::llvm::createPruneEHPass
extern "C"
::llvm::Pass* llvm_createPruneEHPass()
{
    return ::llvm::createPruneEHPass();
}

// ::llvm::createReassociatePass
extern "C"
::llvm::FunctionPass* llvm_createReassociatePass()
{
    return ::llvm::createReassociatePass();
}

// ::llvm::createSCCPPass
extern "C"
::llvm::FunctionPass* llvm_createSCCPPass()
{
    return ::llvm::createSCCPPass();
}

// ::llvm::createSROAPass
extern "C"
::llvm::FunctionPass* llvm_createSROAPass(int RequiresDomTree)
{
    return ::llvm::createSROAPass((RequiresDomTree == 1 ? true : false));
}

// ::llvm::createSampleProfileLoaderPass
extern "C"
::llvm::FunctionPass* llvm_createSampleProfileLoaderPass()
{
    return ::llvm::createSampleProfileLoaderPass();
}

// ::llvm::createScalarReplAggregatesPass
extern "C"
::llvm::FunctionPass* llvm_createScalarReplAggregatesPass()
{
    return ::llvm::createScalarReplAggregatesPass();
}

// ::llvm::createScalarizerPass
extern "C"
::llvm::FunctionPass* llvm_createScalarizerPass()
{
    return ::llvm::createScalarizerPass();
}

// ::llvm::createSeparateConstOffsetFromGEPPass
extern "C"
::llvm::FunctionPass* llvm_createSeparateConstOffsetFromGEPPass()
{
    return ::llvm::createSeparateConstOffsetFromGEPPass();
}

// ::llvm::createSimpleLoopUnrollPass
extern "C"
::llvm::Pass* llvm_createSimpleLoopUnrollPass()
{
    return ::llvm::createSimpleLoopUnrollPass();
}

// ::llvm::createSingleLoopExtractorPass
extern "C"
::llvm::Pass* llvm_createSingleLoopExtractorPass()
{
    return ::llvm::createSingleLoopExtractorPass();
}

// ::llvm::createSinkingPass
extern "C"
::llvm::FunctionPass* llvm_createSinkingPass()
{
    return ::llvm::createSinkingPass();
}

// ::llvm::createStripDeadDebugInfoPass
extern "C"
::llvm::ModulePass* llvm_createStripDeadDebugInfoPass()
{
    return ::llvm::createStripDeadDebugInfoPass();
}

// ::llvm::createStripDeadPrototypesPass
extern "C"
::llvm::ModulePass* llvm_createStripDeadPrototypesPass()
{
    return ::llvm::createStripDeadPrototypesPass();
}

// ::llvm::createStripDebugDeclarePass
extern "C"
::llvm::ModulePass* llvm_createStripDebugDeclarePass()
{
    return ::llvm::createStripDebugDeclarePass();
}

// ::llvm::createStripNonDebugSymbolsPass
extern "C"
::llvm::ModulePass* llvm_createStripNonDebugSymbolsPass()
{
    return ::llvm::createStripNonDebugSymbolsPass();
}

// ::llvm::createStripSymbolsPass
extern "C"
::llvm::ModulePass* llvm_createStripSymbolsPass(int OnlyDebugInfo)
{
    return ::llvm::createStripSymbolsPass((OnlyDebugInfo == 1 ? true : false));
}

// ::llvm::createStructurizeCFGPass
extern "C"
::llvm::Pass* llvm_createStructurizeCFGPass()
{
    return ::llvm::createStructurizeCFGPass();
}

// ::llvm::createTailCallEliminationPass
extern "C"
::llvm::FunctionPass* llvm_createTailCallEliminationPass()
{
    return ::llvm::createTailCallEliminationPass();
}

// ::llvm::createThreadSanitizerPass
extern "C"
::llvm::FunctionPass* llvm_createThreadSanitizerPass()
{
    return ::llvm::createThreadSanitizerPass();
}

// ::llvm::getGlobalContext
extern "C"
::llvm::LLVMContext* llvm_getGlobalContext()
{
    return &(::llvm::getGlobalContext());
}

// ::llvm::verifyFunction
extern "C"
int llvm_verifyFunction(::llvm::Function const* Function)
{
    return (::llvm::verifyFunction(*Function) == true ? 1 : 0);
}

// ::llvm::verifyModule
extern "C"
int llvm_verifyModule(::llvm::Module const* Module)
{
    return (::llvm::verifyModule(*Module) == true ? 1 : 0);
}
