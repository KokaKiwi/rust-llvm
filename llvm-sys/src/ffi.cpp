#include <string>
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

struct llvm_ArrayRef_llvm_Type_ptr {
    ::llvm::Type* const* data;
    size_t length;
};
struct llvm_StringRef {
    char const* data;
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
