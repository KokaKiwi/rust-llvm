#include <string>
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/Instruction.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Type.h"
#include "llvm/IR/Value.h"

struct llvm_ArrayRef__libc_uint64_t {
    uint64_t const* data;
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

// ::llvm::BlockAddress::destroyConstant
extern "C"
void llvm_BlockAddress_destroyConstant(::llvm::BlockAddress* inst)
{
    return inst->destroyConstant();
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
    return inst->destroyConstant();
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
    return inst->removeDeadConstantUsers();
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
    return inst->destroyConstant();
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

// ::llvm::Function::Create
extern "C"
::llvm::Function* llvm_Function_Create(::llvm::FunctionType* Ty, ::llvm::GlobalValue::LinkageTypes Linkage)
{
    return ::llvm::Function::Create(Ty, Linkage);
}

// ::llvm::Function::CreateWithName
extern "C"
::llvm::Function* llvm_Function_CreateWithName(::llvm::FunctionType* Ty, ::llvm::GlobalValue::LinkageTypes Linkage, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return ::llvm::Function::Create(Ty, Linkage, Name);
}

// ::llvm::Function::addFnAttr
extern "C"
void llvm_Function_addFnAttr(::llvm::Function* inst, llvm_StringRef _Kind)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    return inst->addFnAttr(Kind);
}

// ::llvm::Function::addFnAttrWithValue
extern "C"
void llvm_Function_addFnAttrWithValue(::llvm::Function* inst, llvm_StringRef _Kind, llvm_StringRef _Val)
{
    auto Kind = ::llvm::StringRef(_Kind.data, _Kind.length);
    auto Val = ::llvm::StringRef(_Val.data, _Val.length);
    return inst->addFnAttr(Kind, Val);
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
    return inst->clearGC();
}

// ::llvm::Function::copyAttributesFrom
extern "C"
void llvm_Function_copyAttributesFrom(::llvm::Function* inst, ::llvm::GlobalValue* Src)
{
    return inst->copyAttributesFrom(Src);
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
    return inst->deleteBody();
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
    return inst->eraseFromParent();
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
    return inst->removeFromParent();
}

// ::llvm::Function::setCallingConv
extern "C"
void llvm_Function_setCallingConv(::llvm::Function* inst, ::llvm::CallingConv::ID CC)
{
    return inst->setCallingConv(CC);
}

// ::llvm::Function::setCannotDuplicate
extern "C"
void llvm_Function_setCannotDuplicate(::llvm::Function* inst)
{
    return inst->setCannotDuplicate();
}

// ::llvm::Function::setDoesNotAccessMemory
extern "C"
void llvm_Function_setDoesNotAccessMemory(::llvm::Function* inst)
{
    return inst->setDoesNotAccessMemory();
}

// ::llvm::Function::setDoesNotAccessMemoryParam
extern "C"
void llvm_Function_setDoesNotAccessMemoryParam(::llvm::Function* inst, unsigned int n)
{
    return inst->setDoesNotAccessMemory(n);
}

// ::llvm::Function::setDoesNotAlias
extern "C"
void llvm_Function_setDoesNotAlias(::llvm::Function* inst, unsigned int n)
{
    return inst->setDoesNotAlias(n);
}

// ::llvm::Function::setDoesNotCapture
extern "C"
void llvm_Function_setDoesNotCapture(::llvm::Function* inst, unsigned int n)
{
    return inst->setDoesNotCapture(n);
}

// ::llvm::Function::setDoesNotReturn
extern "C"
void llvm_Function_setDoesNotReturn(::llvm::Function* inst)
{
    return inst->setDoesNotReturn();
}

// ::llvm::Function::setDoesNotThrow
extern "C"
void llvm_Function_setDoesNotThrow(::llvm::Function* inst)
{
    return inst->setDoesNotThrow();
}

// ::llvm::Function::setHasUWTable
extern "C"
void llvm_Function_setHasUWTable(::llvm::Function* inst)
{
    return inst->setHasUWTable();
}

// ::llvm::Function::setOnlyReadsMemory
extern "C"
void llvm_Function_setOnlyReadsMemory(::llvm::Function* inst)
{
    return inst->setOnlyReadsMemory();
}

// ::llvm::Function::setOnlyReadsMemoryParam
extern "C"
void llvm_Function_setOnlyReadsMemoryParam(::llvm::Function* inst, unsigned int n)
{
    return inst->setOnlyReadsMemory(n);
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
    return inst->setSection(S);
}

// ::llvm::GlobalValue::copyAttributesFrom
extern "C"
void llvm_GlobalValue_copyAttributesFrom(::llvm::GlobalValue* inst, ::llvm::GlobalValue* Src)
{
    return inst->copyAttributesFrom(Src);
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
    return inst->destroyConstant();
}

// ::llvm::GlobalValue::eraseFromParent
extern "C"
void llvm_GlobalValue_eraseFromParent(::llvm::GlobalValue* inst)
{
    return inst->eraseFromParent();
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
    return inst->removeFromParent();
}

// ::llvm::GlobalValue::setThreadLocal
extern "C"
void llvm_GlobalValue_setThreadLocal(::llvm::GlobalValue* inst, int Val)
{
    return inst->setThreadLocal((Val == 1 ? true : false));
}

// ::llvm::GlobalValue::setUnnamedAddr
extern "C"
void llvm_GlobalValue_setUnnamedAddr(::llvm::GlobalValue* inst, int Val)
{
    return inst->setUnnamedAddr((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::copyAttributesFrom
extern "C"
void llvm_GlobalVariable_copyAttributesFrom(::llvm::GlobalVariable* inst, ::llvm::GlobalValue* Src)
{
    return inst->copyAttributesFrom(Src);
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
    return inst->eraseFromParent();
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
    return inst->removeFromParent();
}

// ::llvm::GlobalVariable::setConstant
extern "C"
void llvm_GlobalVariable_setConstant(::llvm::GlobalVariable* inst, int Val)
{
    return inst->setConstant((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::setExternallyInitialized
extern "C"
void llvm_GlobalVariable_setExternallyInitialized(::llvm::GlobalVariable* inst, int Val)
{
    return inst->setExternallyInitialized((Val == 1 ? true : false));
}

// ::llvm::GlobalVariable::setInitializer
extern "C"
void llvm_GlobalVariable_setInitializer(::llvm::GlobalVariable* inst, ::llvm::Constant* InitVal)
{
    return inst->setInitializer(InitVal);
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

// ::llvm::Module::appendModuleInlineAsm
extern "C"
void llvm_Module_appendModuleInlineAsm(::llvm::Module* inst, llvm_StringRef _Asm)
{
    auto Asm = ::llvm::StringRef(_Asm.data, _Asm.length);
    return inst->appendModuleInlineAsm(Asm);
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
    return inst->dump();
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
    return new ::llvm::Module(ModuleID, *Context);
}

// ::llvm::Module::setDataLayout
extern "C"
void llvm_Module_setDataLayout(::llvm::Module* inst, ::llvm::DataLayout const* Other)
{
    return inst->setDataLayout(Other);
}

// ::llvm::Module::setDataLayoutStr
extern "C"
void llvm_Module_setDataLayoutStr(::llvm::Module* inst, llvm_StringRef _Desc)
{
    auto Desc = ::llvm::StringRef(_Desc.data, _Desc.length);
    return inst->setDataLayout(Desc);
}

// ::llvm::Module::setModuleIdentifier
extern "C"
void llvm_Module_setModuleIdentifier(::llvm::Module* inst, llvm_StringRef _ID)
{
    auto ID = ::llvm::StringRef(_ID.data, _ID.length);
    return inst->setModuleIdentifier(ID);
}

// ::llvm::Module::setModuleInlineAsm
extern "C"
void llvm_Module_setModuleInlineAsm(::llvm::Module* inst, llvm_StringRef _Asm)
{
    auto Asm = ::llvm::StringRef(_Asm.data, _Asm.length);
    return inst->setModuleInlineAsm(Asm);
}

// ::llvm::Module::setTargetTriple
extern "C"
void llvm_Module_setTargetTriple(::llvm::Module* inst, llvm_StringRef _Triple)
{
    auto Triple = ::llvm::StringRef(_Triple.data, _Triple.length);
    return inst->setTargetTriple(Triple);
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
    return inst->setBody(Elements);
}

// ::llvm::StructType::setBodyPacked
extern "C"
void llvm_StructType_setBodyPacked(::llvm::StructType* inst, llvm_ArrayRef_llvm_Type_ptr _Elements, int isPacked)
{
    auto Elements = ::llvm::ArrayRef<::llvm::Type*>(_Elements.data, _Elements.length);
    return inst->setBody(Elements, (isPacked == 1 ? true : false));
}

// ::llvm::StructType::setName
extern "C"
void llvm_StructType_setName(::llvm::StructType* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->setName(Name);
}

// ::llvm::Type::dump
extern "C"
void llvm_Type_dump(::llvm::Type const* inst)
{
    return inst->dump();
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
    return inst->dropAllReferences();
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
    return inst->replaceUsesOfWith(From, To);
}

// ::llvm::User::setOperand
extern "C"
void llvm_User_setOperand(::llvm::User* inst, unsigned int idx, ::llvm::Value* Val)
{
    return inst->setOperand(idx, Val);
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
    return inst->dump();
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
    return inst->mutateType(ty);
}

// ::llvm::Value::replaceAllUsesWith
extern "C"
void llvm_Value_replaceAllUsesWith(::llvm::Value* inst, ::llvm::Value* Value)
{
    return inst->replaceAllUsesWith(Value);
}

// ::llvm::Value::setName
extern "C"
void llvm_Value_setName(::llvm::Value* inst, llvm_StringRef _Name)
{
    auto Name = ::llvm::StringRef(_Name.data, _Name.length);
    return inst->setName(Name);
}

// ::llvm::Value::takeName
extern "C"
void llvm_Value_takeName(::llvm::Value* inst, ::llvm::Value* Value)
{
    return inst->takeName(Value);
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

// ::llvm::getGlobalContext
extern "C"
::llvm::LLVMContext* llvm_getGlobalContext()
{
    return &(::llvm::getGlobalContext());
}
