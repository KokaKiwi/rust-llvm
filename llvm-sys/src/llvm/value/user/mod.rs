pub mod constant;
pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstExt: ::llvm::value::user::CastInstExt {

    fn inner(&self) -> *mut AddrSpaceCastInstInner;
}

pub struct AddrSpaceCastInst {
    inner: *mut AddrSpaceCastInstInner,
}
impl ::llvm::value::ValueExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl AddrSpaceCastInstExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut AddrSpaceCastInstInner {
        self.inner
    }
}
impl AddrSpaceCastInst {
    pub unsafe fn from_inner(inner: *mut AddrSpaceCastInstInner) -> AddrSpaceCastInst {
        AddrSpaceCastInst {
            inner: inner,
        }
    }
}
impl Drop for AddrSpaceCastInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstExt: ::llvm::value::user::UnaryInstructionExt {

    fn inner(&self) -> *mut AllocaInstInner;
}

pub struct AllocaInst {
    inner: *mut AllocaInstInner,
}
impl ::llvm::value::ValueExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl AllocaInstExt for AllocaInst {
    fn inner(&self) -> *mut AllocaInstInner {
        self.inner
    }
}
impl AllocaInst {
    pub unsafe fn from_inner(inner: *mut AllocaInstInner) -> AllocaInst {
        AllocaInst {
            inner: inner,
        }
    }
}
impl Drop for AllocaInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut AtomicCmpXchgInstInner;
}

pub struct AtomicCmpXchgInst {
    inner: *mut AtomicCmpXchgInstInner,
}
impl ::llvm::value::ValueExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl AtomicCmpXchgInstExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut AtomicCmpXchgInstInner {
        self.inner
    }
}
impl AtomicCmpXchgInst {
    pub unsafe fn from_inner(inner: *mut AtomicCmpXchgInstInner) -> AtomicCmpXchgInst {
        AtomicCmpXchgInst {
            inner: inner,
        }
    }
}
impl Drop for AtomicCmpXchgInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut AtomicRMWInstInner;
}

pub struct AtomicRMWInst {
    inner: *mut AtomicRMWInstInner,
}
impl ::llvm::value::ValueExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl AtomicRMWInstExt for AtomicRMWInst {
    fn inner(&self) -> *mut AtomicRMWInstInner {
        self.inner
    }
}
impl AtomicRMWInst {
    pub unsafe fn from_inner(inner: *mut AtomicRMWInstInner) -> AtomicRMWInst {
        AtomicRMWInst {
            inner: inner,
        }
    }
}
impl Drop for AtomicRMWInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut BinaryOperatorInner;
}

pub struct BinaryOperator {
    inner: *mut BinaryOperatorInner,
}
impl ::llvm::value::ValueExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BinaryOperatorExt for BinaryOperator {
    fn inner(&self) -> *mut BinaryOperatorInner {
        self.inner
    }
}
impl BinaryOperator {
    pub unsafe fn from_inner(inner: *mut BinaryOperatorInner) -> BinaryOperator {
        BinaryOperator {
            inner: inner,
        }
    }
}
impl Drop for BinaryOperator {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstExt: ::llvm::value::user::CastInstExt {

    fn inner(&self) -> *mut BitCastInstInner;
}

pub struct BitCastInst {
    inner: *mut BitCastInstInner,
}
impl ::llvm::value::ValueExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BitCastInstExt for BitCastInst {
    fn inner(&self) -> *mut BitCastInstInner {
        self.inner
    }
}
impl BitCastInst {
    pub unsafe fn from_inner(inner: *mut BitCastInstInner) -> BitCastInst {
        BitCastInst {
            inner: inner,
        }
    }
}
impl Drop for BitCastInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut BranchInstInner;
}

pub struct BranchInst {
    inner: *mut BranchInstInner,
}
impl ::llvm::value::ValueExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BranchInstExt for BranchInst {
    fn inner(&self) -> *mut BranchInstInner {
        self.inner
    }
}
impl BranchInst {
    pub unsafe fn from_inner(inner: *mut BranchInstInner) -> BranchInst {
        BranchInst {
            inner: inner,
        }
    }
}
impl Drop for BranchInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut CallInstInner;
}

pub struct CallInst {
    inner: *mut CallInstInner,
}
impl ::llvm::value::ValueExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl CallInstExt for CallInst {
    fn inner(&self) -> *mut CallInstInner {
        self.inner
    }
}
impl CallInst {
    pub unsafe fn from_inner(inner: *mut CallInstInner) -> CallInst {
        CallInst {
            inner: inner,
        }
    }
}
impl Drop for CallInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstExt: ::llvm::value::user::UnaryInstructionExt {

    fn inner(&self) -> *mut CastInstInner;
}

pub struct CastInst {
    inner: *mut CastInstInner,
}
impl ::llvm::value::ValueExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl CastInstExt for CastInst {
    fn inner(&self) -> *mut CastInstInner {
        self.inner
    }
}
impl CastInst {
    pub unsafe fn from_inner(inner: *mut CastInstInner) -> CastInst {
        CastInst {
            inner: inner,
        }
    }
}
impl Drop for CastInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type CmpInstInner = ::ffi::llvm_CmpInst;

pub trait CmpInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut CmpInstInner;
}

pub struct CmpInst {
    inner: *mut CmpInstInner,
}
impl ::llvm::value::ValueExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl CmpInstExt for CmpInst {
    fn inner(&self) -> *mut CmpInstInner {
        self.inner
    }
}
impl CmpInst {
    pub unsafe fn from_inner(inner: *mut CmpInstInner) -> CmpInst {
        CmpInst {
            inner: inner,
        }
    }
}
impl Drop for CmpInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut ExtractElementInstInner;
}

pub struct ExtractElementInst {
    inner: *mut ExtractElementInstInner,
}
impl ::llvm::value::ValueExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ExtractElementInstExt for ExtractElementInst {
    fn inner(&self) -> *mut ExtractElementInstInner {
        self.inner
    }
}
impl ExtractElementInst {
    pub unsafe fn from_inner(inner: *mut ExtractElementInstInner) -> ExtractElementInst {
        ExtractElementInst {
            inner: inner,
        }
    }
}
impl Drop for ExtractElementInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstExt: ::llvm::value::user::UnaryInstructionExt {

    fn inner(&self) -> *mut ExtractValueInstInner;
}

pub struct ExtractValueInst {
    inner: *mut ExtractValueInstInner,
}
impl ::llvm::value::ValueExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ExtractValueInstExt for ExtractValueInst {
    fn inner(&self) -> *mut ExtractValueInstInner {
        self.inner
    }
}
impl ExtractValueInst {
    pub unsafe fn from_inner(inner: *mut ExtractValueInstInner) -> ExtractValueInst {
        ExtractValueInst {
            inner: inner,
        }
    }
}
impl Drop for ExtractValueInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstExt: ::llvm::value::user::CastInstExt {

    fn inner(&self) -> *mut FPExtInstInner;
}

pub struct FPExtInst {
    inner: *mut FPExtInstInner,
}
impl ::llvm::value::ValueExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FPExtInstExt for FPExtInst {
    fn inner(&self) -> *mut FPExtInstInner {
        self.inner
    }
}
impl FPExtInst {
    pub unsafe fn from_inner(inner: *mut FPExtInstInner) -> FPExtInst {
        FPExtInst {
            inner: inner,
        }
    }
}
impl Drop for FPExtInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstExt: ::llvm::value::user::CastInstExt {

    fn inner(&self) -> *mut FPToSIInstInner;
}

pub struct FPToSIInst {
    inner: *mut FPToSIInstInner,
}
impl ::llvm::value::ValueExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FPToSIInstExt for FPToSIInst {
    fn inner(&self) -> *mut FPToSIInstInner {
        self.inner
    }
}
impl FPToSIInst {
    pub unsafe fn from_inner(inner: *mut FPToSIInstInner) -> FPToSIInst {
        FPToSIInst {
            inner: inner,
        }
    }
}
impl Drop for FPToSIInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut FenceInstInner;
}

pub struct FenceInst {
    inner: *mut FenceInstInner,
}
impl ::llvm::value::ValueExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FenceInstExt for FenceInst {
    fn inner(&self) -> *mut FenceInstInner {
        self.inner
    }
}
impl FenceInst {
    pub unsafe fn from_inner(inner: *mut FenceInstInner) -> FenceInst {
        FenceInst {
            inner: inner,
        }
    }
}
impl Drop for FenceInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut GetElementPtrInstInner;
}

pub struct GetElementPtrInst {
    inner: *mut GetElementPtrInstInner,
}
impl ::llvm::value::ValueExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GetElementPtrInstExt for GetElementPtrInst {
    fn inner(&self) -> *mut GetElementPtrInstInner {
        self.inner
    }
}
impl GetElementPtrInst {
    pub unsafe fn from_inner(inner: *mut GetElementPtrInstInner) -> GetElementPtrInst {
        GetElementPtrInst {
            inner: inner,
        }
    }
}
impl Drop for GetElementPtrInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut IndirectBrInstInner;
}

pub struct IndirectBrInst {
    inner: *mut IndirectBrInstInner,
}
impl ::llvm::value::ValueExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl IndirectBrInstExt for IndirectBrInst {
    fn inner(&self) -> *mut IndirectBrInstInner {
        self.inner
    }
}
impl IndirectBrInst {
    pub unsafe fn from_inner(inner: *mut IndirectBrInstInner) -> IndirectBrInst {
        IndirectBrInst {
            inner: inner,
        }
    }
}
impl Drop for IndirectBrInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut InsertElementInstInner;
}

pub struct InsertElementInst {
    inner: *mut InsertElementInstInner,
}
impl ::llvm::value::ValueExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InsertElementInstExt for InsertElementInst {
    fn inner(&self) -> *mut InsertElementInstInner {
        self.inner
    }
}
impl InsertElementInst {
    pub unsafe fn from_inner(inner: *mut InsertElementInstInner) -> InsertElementInst {
        InsertElementInst {
            inner: inner,
        }
    }
}
impl Drop for InsertElementInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut InsertValueInstInner;
}

pub struct InsertValueInst {
    inner: *mut InsertValueInstInner,
}
impl ::llvm::value::ValueExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InsertValueInstExt for InsertValueInst {
    fn inner(&self) -> *mut InsertValueInstInner {
        self.inner
    }
}
impl InsertValueInst {
    pub unsafe fn from_inner(inner: *mut InsertValueInstInner) -> InsertValueInst {
        InsertValueInst {
            inner: inner,
        }
    }
}
impl Drop for InsertValueInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type InstructionInner = ::ffi::llvm_Instruction;

pub trait InstructionExt: ::llvm::value::user::UserExt {

    fn inner(&self) -> *mut InstructionInner;
}

pub struct Instruction {
    inner: *mut InstructionInner,
}
impl ::llvm::value::ValueExt for Instruction {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Instruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InstructionExt for Instruction {
    fn inner(&self) -> *mut InstructionInner {
        self.inner
    }
}
impl Instruction {
    pub unsafe fn from_inner(inner: *mut InstructionInner) -> Instruction {
        Instruction {
            inner: inner,
        }
    }
}
impl Drop for Instruction {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut InvokeInstInner;
}

pub struct InvokeInst {
    inner: *mut InvokeInstInner,
}
impl ::llvm::value::ValueExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InvokeInstExt for InvokeInst {
    fn inner(&self) -> *mut InvokeInstInner {
        self.inner
    }
}
impl InvokeInst {
    pub unsafe fn from_inner(inner: *mut InvokeInstInner) -> InvokeInst {
        InvokeInst {
            inner: inner,
        }
    }
}
impl Drop for InvokeInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut LandingPadInstInner;
}

pub struct LandingPadInst {
    inner: *mut LandingPadInstInner,
}
impl ::llvm::value::ValueExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl LandingPadInstExt for LandingPadInst {
    fn inner(&self) -> *mut LandingPadInstInner {
        self.inner
    }
}
impl LandingPadInst {
    pub unsafe fn from_inner(inner: *mut LandingPadInstInner) -> LandingPadInst {
        LandingPadInst {
            inner: inner,
        }
    }
}
impl Drop for LandingPadInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstExt: ::llvm::value::user::UnaryInstructionExt {

    fn inner(&self) -> *mut LoadInstInner;
}

pub struct LoadInst {
    inner: *mut LoadInstInner,
}
impl ::llvm::value::ValueExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl LoadInstExt for LoadInst {
    fn inner(&self) -> *mut LoadInstInner {
        self.inner
    }
}
impl LoadInst {
    pub unsafe fn from_inner(inner: *mut LoadInstInner) -> LoadInst {
        LoadInst {
            inner: inner,
        }
    }
}
impl Drop for LoadInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorExt: ::llvm::value::user::UserExt {

    fn inner(&self) -> *mut OperatorInner;
}

pub struct Operator {
    inner: *mut OperatorInner,
}
impl ::llvm::value::ValueExt for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl OperatorExt for Operator {
    fn inner(&self) -> *mut OperatorInner {
        self.inner
    }
}
impl Operator {
    pub unsafe fn from_inner(inner: *mut OperatorInner) -> Operator {
        Operator {
            inner: inner,
        }
    }
}
impl Drop for Operator {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut PHINodeInner;
}

pub struct PHINode {
    inner: *mut PHINodeInner,
}
impl ::llvm::value::ValueExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl PHINodeExt for PHINode {
    fn inner(&self) -> *mut PHINodeInner {
        self.inner
    }
}
impl PHINode {
    pub unsafe fn from_inner(inner: *mut PHINodeInner) -> PHINode {
        PHINode {
            inner: inner,
        }
    }
}
impl Drop for PHINode {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut ResumeInstInner;
}

pub struct ResumeInst {
    inner: *mut ResumeInstInner,
}
impl ::llvm::value::ValueExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ResumeInstExt for ResumeInst {
    fn inner(&self) -> *mut ResumeInstInner {
        self.inner
    }
}
impl ResumeInst {
    pub unsafe fn from_inner(inner: *mut ResumeInstInner) -> ResumeInst {
        ResumeInst {
            inner: inner,
        }
    }
}
impl Drop for ResumeInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut ReturnInstInner;
}

pub struct ReturnInst {
    inner: *mut ReturnInstInner,
}
impl ::llvm::value::ValueExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ReturnInstExt for ReturnInst {
    fn inner(&self) -> *mut ReturnInstInner {
        self.inner
    }
}
impl ReturnInst {
    pub unsafe fn from_inner(inner: *mut ReturnInstInner) -> ReturnInst {
        ReturnInst {
            inner: inner,
        }
    }
}
impl Drop for ReturnInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut SelectInstInner;
}

pub struct SelectInst {
    inner: *mut SelectInstInner,
}
impl ::llvm::value::ValueExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl SelectInstExt for SelectInst {
    fn inner(&self) -> *mut SelectInstInner {
        self.inner
    }
}
impl SelectInst {
    pub unsafe fn from_inner(inner: *mut SelectInstInner) -> SelectInst {
        SelectInst {
            inner: inner,
        }
    }
}
impl Drop for SelectInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut ShuffleVectorInstInner;
}

pub struct ShuffleVectorInst {
    inner: *mut ShuffleVectorInstInner,
}
impl ::llvm::value::ValueExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ShuffleVectorInstExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ShuffleVectorInstInner {
        self.inner
    }
}
impl ShuffleVectorInst {
    pub unsafe fn from_inner(inner: *mut ShuffleVectorInstInner) -> ShuffleVectorInst {
        ShuffleVectorInst {
            inner: inner,
        }
    }
}
impl Drop for ShuffleVectorInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut StoreInstInner;
}

pub struct StoreInst {
    inner: *mut StoreInstInner,
}
impl ::llvm::value::ValueExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl StoreInstExt for StoreInst {
    fn inner(&self) -> *mut StoreInstInner {
        self.inner
    }
}
impl StoreInst {
    pub unsafe fn from_inner(inner: *mut StoreInstInner) -> StoreInst {
        StoreInst {
            inner: inner,
        }
    }
}
impl Drop for StoreInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut SwitchInstInner;
}

pub struct SwitchInst {
    inner: *mut SwitchInstInner,
}
impl ::llvm::value::ValueExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl SwitchInstExt for SwitchInst {
    fn inner(&self) -> *mut SwitchInstInner {
        self.inner
    }
}
impl SwitchInst {
    pub unsafe fn from_inner(inner: *mut SwitchInstInner) -> SwitchInst {
        SwitchInst {
            inner: inner,
        }
    }
}
impl Drop for SwitchInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut TerminatorInstInner;
}

pub struct TerminatorInst {
    inner: *mut TerminatorInstInner,
}
impl ::llvm::value::ValueExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl TerminatorInstExt for TerminatorInst {
    fn inner(&self) -> *mut TerminatorInstInner {
        self.inner
    }
}
impl TerminatorInst {
    pub unsafe fn from_inner(inner: *mut TerminatorInstInner) -> TerminatorInst {
        TerminatorInst {
            inner: inner,
        }
    }
}
impl Drop for TerminatorInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionExt: ::llvm::value::user::InstructionExt {

    fn inner(&self) -> *mut UnaryInstructionInner;
}

pub struct UnaryInstruction {
    inner: *mut UnaryInstructionInner,
}
impl ::llvm::value::ValueExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl UnaryInstructionExt for UnaryInstruction {
    fn inner(&self) -> *mut UnaryInstructionInner {
        self.inner
    }
}
impl UnaryInstruction {
    pub unsafe fn from_inner(inner: *mut UnaryInstructionInner) -> UnaryInstruction {
        UnaryInstruction {
            inner: inner,
        }
    }
}
impl Drop for UnaryInstruction {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstExt: ::llvm::value::user::TerminatorInstExt {

    fn inner(&self) -> *mut UnreachableInstInner;
}

pub struct UnreachableInst {
    inner: *mut UnreachableInstInner,
}
impl ::llvm::value::ValueExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl UnreachableInstExt for UnreachableInst {
    fn inner(&self) -> *mut UnreachableInstInner {
        self.inner
    }
}
impl UnreachableInst {
    pub unsafe fn from_inner(inner: *mut UnreachableInstInner) -> UnreachableInst {
        UnreachableInst {
            inner: inner,
        }
    }
}
impl Drop for UnreachableInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type UserInner = ::ffi::llvm_User;

pub trait UserExt: ::llvm::value::ValueExt {

    fn inner(&self) -> *mut UserInner;

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(::llvm::value::user::UserExt::inner(self));
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            ::ffi::llvm::User_getNumOperands(::llvm::value::user::UserExt::inner(self) as *const ::ffi::llvm_User) as u32
        }
    }

    fn get_operand(&self, idx: u32) -> ::llvm::value::Value {
        unsafe {
            ::llvm::value::Value::from_inner(::ffi::llvm::User_getOperand(::llvm::value::user::UserExt::inner(self) as *const ::ffi::llvm_User, idx as ::libc::c_uint))
        }
    }

    fn replace_uses_of_with<A1: ::llvm::value::ValueExt, A2: ::llvm::value::ValueExt>(&mut self, from: A1, to: A2) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(::llvm::value::user::UserExt::inner(self), ::llvm::value::ValueExt::inner(&from), ::llvm::value::ValueExt::inner(&to));
        }
    }

    fn set_operand<A2: ::llvm::value::ValueExt>(&mut self, idx: u32, val: A2) {
        unsafe {
            ::ffi::llvm::User_setOperand(::llvm::value::user::UserExt::inner(self), idx as ::libc::c_uint, ::llvm::value::ValueExt::inner(&val));
        }
    }
}

pub struct User {
    inner: *mut UserInner,
}
impl ::llvm::value::ValueExt for User {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl UserExt for User {
    fn inner(&self) -> *mut UserInner {
        self.inner
    }
}
impl User {
    pub unsafe fn from_inner(inner: *mut UserInner) -> User {
        User {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueExt>(v: A1) -> bool {
        unsafe {
            ::ffi::llvm::User_classof(::llvm::value::ValueExt::inner(&v))
        }
    }
}
impl Drop for User {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstExt: ::llvm::value::user::UnaryInstructionExt {

    fn inner(&self) -> *mut VAArgInstInner;
}

pub struct VAArgInst {
    inner: *mut VAArgInstInner,
}
impl ::llvm::value::ValueExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl VAArgInstExt for VAArgInst {
    fn inner(&self) -> *mut VAArgInstInner {
        self.inner
    }
}
impl VAArgInst {
    pub unsafe fn from_inner(inner: *mut VAArgInstInner) -> VAArgInst {
        VAArgInst {
            inner: inner,
        }
    }
}
impl Drop for VAArgInst {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
