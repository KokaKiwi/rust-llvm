pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstExt: ::llvm::value::user::inst::CastInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut AddrSpaceCastInstInner;
}

pub struct AddrSpaceCastInst {
    inner: ::core::nonzero::NonZero<*mut AddrSpaceCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AddrSpaceCastInstExt for AddrSpaceCastInst {
    fn inner(&self) -> *mut AddrSpaceCastInstInner {
        *self.inner
    }
}
impl AddrSpaceCastInst {
    pub unsafe fn from_inner(inner: *mut AddrSpaceCastInstInner, owned: bool) -> AddrSpaceCastInst {
        AddrSpaceCastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AddrSpaceCastInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut AllocaInstInner;
}

pub struct AllocaInst {
    inner: ::core::nonzero::NonZero<*mut AllocaInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AllocaInstExt for AllocaInst {
    fn inner(&self) -> *mut AllocaInstInner {
        *self.inner
    }
}
impl AllocaInst {
    pub unsafe fn from_inner(inner: *mut AllocaInstInner, owned: bool) -> AllocaInst {
        AllocaInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AllocaInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut AtomicCmpXchgInstInner;
}

pub struct AtomicCmpXchgInst {
    inner: ::core::nonzero::NonZero<*mut AtomicCmpXchgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicCmpXchgInstExt for AtomicCmpXchgInst {
    fn inner(&self) -> *mut AtomicCmpXchgInstInner {
        *self.inner
    }
}
impl AtomicCmpXchgInst {
    pub unsafe fn from_inner(inner: *mut AtomicCmpXchgInstInner, owned: bool) -> AtomicCmpXchgInst {
        AtomicCmpXchgInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AtomicCmpXchgInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut AtomicRMWInstInner;
}

pub struct AtomicRMWInst {
    inner: ::core::nonzero::NonZero<*mut AtomicRMWInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicRMWInstExt for AtomicRMWInst {
    fn inner(&self) -> *mut AtomicRMWInstInner {
        *self.inner
    }
}
impl AtomicRMWInst {
    pub unsafe fn from_inner(inner: *mut AtomicRMWInstInner, owned: bool) -> AtomicRMWInst {
        AtomicRMWInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AtomicRMWInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut BinaryOperatorInner;
}

pub struct BinaryOperator {
    inner: ::core::nonzero::NonZero<*mut BinaryOperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BinaryOperatorExt for BinaryOperator {
    fn inner(&self) -> *mut BinaryOperatorInner {
        *self.inner
    }
}
impl BinaryOperator {
    pub unsafe fn from_inner(inner: *mut BinaryOperatorInner, owned: bool) -> BinaryOperator {
        BinaryOperator {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BinaryOperator {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstExt: ::llvm::value::user::inst::CastInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut BitCastInstInner;
}

pub struct BitCastInst {
    inner: ::core::nonzero::NonZero<*mut BitCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BitCastInstExt for BitCastInst {
    fn inner(&self) -> *mut BitCastInstInner {
        *self.inner
    }
}
impl BitCastInst {
    pub unsafe fn from_inner(inner: *mut BitCastInstInner, owned: bool) -> BitCastInst {
        BitCastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BitCastInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut BranchInstInner;
}

pub struct BranchInst {
    inner: ::core::nonzero::NonZero<*mut BranchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BranchInstExt for BranchInst {
    fn inner(&self) -> *mut BranchInstInner {
        *self.inner
    }
}
impl BranchInst {
    pub unsafe fn from_inner(inner: *mut BranchInstInner, owned: bool) -> BranchInst {
        BranchInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BranchInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut CallInstInner;
}

pub struct CallInst {
    inner: ::core::nonzero::NonZero<*mut CallInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CallInstExt for CallInst {
    fn inner(&self) -> *mut CallInstInner {
        *self.inner
    }
}
impl CallInst {
    pub unsafe fn from_inner(inner: *mut CallInstInner, owned: bool) -> CallInst {
        CallInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CallInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut CastInstInner;
}

pub struct CastInst {
    inner: ::core::nonzero::NonZero<*mut CastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CastInstExt for CastInst {
    fn inner(&self) -> *mut CastInstInner {
        *self.inner
    }
}
impl CastInst {
    pub unsafe fn from_inner(inner: *mut CastInstInner, owned: bool) -> CastInst {
        CastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CastInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type CmpInstInner = ::ffi::llvm_CmpInst;

pub trait CmpInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut CmpInstInner;
}

pub struct CmpInst {
    inner: ::core::nonzero::NonZero<*mut CmpInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CmpInstExt for CmpInst {
    fn inner(&self) -> *mut CmpInstInner {
        *self.inner
    }
}
impl CmpInst {
    pub unsafe fn from_inner(inner: *mut CmpInstInner, owned: bool) -> CmpInst {
        CmpInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CmpInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ExtractElementInstInner;
}

pub struct ExtractElementInst {
    inner: ::core::nonzero::NonZero<*mut ExtractElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractElementInstExt for ExtractElementInst {
    fn inner(&self) -> *mut ExtractElementInstInner {
        *self.inner
    }
}
impl ExtractElementInst {
    pub unsafe fn from_inner(inner: *mut ExtractElementInstInner, owned: bool) -> ExtractElementInst {
        ExtractElementInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ExtractElementInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ExtractValueInstInner;
}

pub struct ExtractValueInst {
    inner: ::core::nonzero::NonZero<*mut ExtractValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractValueInstExt for ExtractValueInst {
    fn inner(&self) -> *mut ExtractValueInstInner {
        *self.inner
    }
}
impl ExtractValueInst {
    pub unsafe fn from_inner(inner: *mut ExtractValueInstInner, owned: bool) -> ExtractValueInst {
        ExtractValueInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ExtractValueInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstExt: ::llvm::value::user::inst::CastInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut FPExtInstInner;
}

pub struct FPExtInst {
    inner: ::core::nonzero::NonZero<*mut FPExtInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPExtInstExt for FPExtInst {
    fn inner(&self) -> *mut FPExtInstInner {
        *self.inner
    }
}
impl FPExtInst {
    pub unsafe fn from_inner(inner: *mut FPExtInstInner, owned: bool) -> FPExtInst {
        FPExtInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FPExtInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstExt: ::llvm::value::user::inst::CastInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut FPToSIInstInner;
}

pub struct FPToSIInst {
    inner: ::core::nonzero::NonZero<*mut FPToSIInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPToSIInstExt for FPToSIInst {
    fn inner(&self) -> *mut FPToSIInstInner {
        *self.inner
    }
}
impl FPToSIInst {
    pub unsafe fn from_inner(inner: *mut FPToSIInstInner, owned: bool) -> FPToSIInst {
        FPToSIInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FPToSIInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut FenceInstInner;
}

pub struct FenceInst {
    inner: ::core::nonzero::NonZero<*mut FenceInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FenceInstExt for FenceInst {
    fn inner(&self) -> *mut FenceInstInner {
        *self.inner
    }
}
impl FenceInst {
    pub unsafe fn from_inner(inner: *mut FenceInstInner, owned: bool) -> FenceInst {
        FenceInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FenceInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut GetElementPtrInstInner;
}

pub struct GetElementPtrInst {
    inner: ::core::nonzero::NonZero<*mut GetElementPtrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GetElementPtrInstExt for GetElementPtrInst {
    fn inner(&self) -> *mut GetElementPtrInstInner {
        *self.inner
    }
}
impl GetElementPtrInst {
    pub unsafe fn from_inner(inner: *mut GetElementPtrInstInner, owned: bool) -> GetElementPtrInst {
        GetElementPtrInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for GetElementPtrInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut IndirectBrInstInner;
}

pub struct IndirectBrInst {
    inner: ::core::nonzero::NonZero<*mut IndirectBrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IndirectBrInstExt for IndirectBrInst {
    fn inner(&self) -> *mut IndirectBrInstInner {
        *self.inner
    }
}
impl IndirectBrInst {
    pub unsafe fn from_inner(inner: *mut IndirectBrInstInner, owned: bool) -> IndirectBrInst {
        IndirectBrInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for IndirectBrInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut InsertElementInstInner;
}

pub struct InsertElementInst {
    inner: ::core::nonzero::NonZero<*mut InsertElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertElementInstExt for InsertElementInst {
    fn inner(&self) -> *mut InsertElementInstInner {
        *self.inner
    }
}
impl InsertElementInst {
    pub unsafe fn from_inner(inner: *mut InsertElementInstInner, owned: bool) -> InsertElementInst {
        InsertElementInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InsertElementInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut InsertValueInstInner;
}

pub struct InsertValueInst {
    inner: ::core::nonzero::NonZero<*mut InsertValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertValueInstExt for InsertValueInst {
    fn inner(&self) -> *mut InsertValueInstInner {
        *self.inner
    }
}
impl InsertValueInst {
    pub unsafe fn from_inner(inner: *mut InsertValueInstInner, owned: bool) -> InsertValueInst {
        InsertValueInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InsertValueInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type InstructionInner = ::ffi::llvm_Instruction;

pub trait InstructionExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut InstructionInner;

    fn clone(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_clone(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn copy_fast_math_flags(&mut self, inst: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_copyFastMathFlags(::llvm::value::user::inst::InstructionExt::inner(self), ::llvm::value::user::inst::InstructionExt::inner(inst));
        }
    }

    fn drop_unknown_metadata(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_dropUnknownMetadata(::llvm::value::user::inst::InstructionExt::inner(self));
        }
    }

    fn drop_unknown_metadata_from_ids(&mut self, known_i_ds: &[u32]) {
        unsafe {
            let c_known_i_ds = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: known_i_ds.as_ptr(),
                length: known_i_ds.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_dropUnknownMetadataFromIDS(::llvm::value::user::inst::InstructionExt::inner(self), c_known_i_ds);
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_eraseFromParent(::llvm::value::user::inst::InstructionExt::inner(self));
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDataLayout(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_debug_loc(&self) -> ::llvm::DebugLoc {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDebugLoc(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ::llvm::DebugLoc::from_inner(ret as *mut ::ffi::llvm_DebugLoc)
        }
    }

    fn get_metadata(&self, kind_id: u32) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getMetadata(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, kind_id as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_metadata_str(&self, kind: &str) -> Option<::llvm::value::MDNode> {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Instruction_getMetadataStr(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, c_kind);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getOpcode(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction) as u32;
            ret
        }
    }

    fn get_parent(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParent(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParentMut(::llvm::value::user::inst::InstructionExt::inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn has_allow_reciprocal(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasAllowReciprocal(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadata(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata_other_than_debug_loc(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadataOtherThanDebugLoc(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_infs(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoInfs(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_na_ns(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoNaNs(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_signed_zeros(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoSignedZeros(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_unsafe_algebra(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasUnsafeAlgebra(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn insert_after(&mut self, insert_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_insertAfter(::llvm::value::user::inst::InstructionExt::inner(self), ::llvm::value::user::inst::InstructionExt::inner(insert_pos));
        }
    }

    fn insert_before(&mut self, insert_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_insertBefore(::llvm::value::user::inst::InstructionExt::inner(self), ::llvm::value::user::inst::InstructionExt::inner(insert_pos));
        }
    }

    fn is_arithmetic_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isArithmeticShift(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_associative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isAssociative(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_binary_op(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isBinaryOp(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_cast(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCast(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_commutative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCommutative(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_idempotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdempotent(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_identical_to(&self, inst: &::llvm::value::user::inst::InstructionExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalTo(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionExt::inner(inst));
            ret
        }
    }

    fn is_identical_to_when_defined(&self, inst: &::llvm::value::user::inst::InstructionExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalToWhenDefined(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionExt::inner(inst));
            ret
        }
    }

    fn is_logical_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isLogicalShift(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_nilpotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isNilpotent(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_same_operation_as(&self, inst: &::llvm::value::user::inst::InstructionExt, flags: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isSameOperationAs(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionExt::inner(inst), flags as ::libc::c_uint);
            ret
        }
    }

    fn is_shift(&mut self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isShift(::llvm::value::user::inst::InstructionExt::inner(self));
            ret
        }
    }

    fn is_terminator(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isTerminator(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_used_outside_of_block(&self, bb: &::llvm::value::BasicBlockExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isUsedOutsideOfBlock(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::BasicBlockExt::inner(bb));
            ret
        }
    }

    fn may_have_side_effects(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayHaveSideEffects(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_from_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadFromMemory(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_or_write_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadOrWriteMemory(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_return(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReturn(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_throw(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayThrow(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_write_to_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayWriteToMemory(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn move_before(&mut self, move_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_moveBefore(::llvm::value::user::inst::InstructionExt::inner(self), ::llvm::value::user::inst::InstructionExt::inner(move_pos));
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_removeFromParent(::llvm::value::user::inst::InstructionExt::inner(self));
        }
    }

    fn set_debug_loc(&mut self, loc: &::llvm::DebugLocExt) {
        unsafe {
            ::ffi::llvm::Instruction_setDebugLoc(::llvm::value::user::inst::InstructionExt::inner(self), ::llvm::DebugLocExt::inner(loc));
        }
    }

    fn set_has_allow_reciprocal(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasAllowReciprocal(::llvm::value::user::inst::InstructionExt::inner(self), val);
        }
    }

    fn set_has_no_infs(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoInfs(::llvm::value::user::inst::InstructionExt::inner(self), val);
        }
    }

    fn set_has_no_na_ns(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoNaNs(::llvm::value::user::inst::InstructionExt::inner(self), val);
        }
    }

    fn set_has_no_signed_zeros(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoSignedZeros(::llvm::value::user::inst::InstructionExt::inner(self), val);
        }
    }

    fn set_has_unsafe_algebra(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasUnsafeAlgebra(::llvm::value::user::inst::InstructionExt::inner(self), val);
        }
    }

    fn set_metadata(&mut self, kind_id: u32, node: &::llvm::value::MDNodeExt) {
        unsafe {
            ::ffi::llvm::Instruction_setMetadata(::llvm::value::user::inst::InstructionExt::inner(self), kind_id as ::libc::c_uint, ::llvm::value::MDNodeExt::inner(node));
        }
    }

    fn set_metadata_str(&mut self, kind: &str, node: &::llvm::value::MDNodeExt) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_setMetadataStr(::llvm::value::user::inst::InstructionExt::inner(self), c_kind, ::llvm::value::MDNodeExt::inner(node));
        }
    }

    fn user_back(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back(::llvm::value::user::inst::InstructionExt::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn user_back_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back_mut(::llvm::value::user::inst::InstructionExt::inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }
}

pub struct Instruction {
    inner: ::core::nonzero::NonZero<*mut InstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Instruction {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Instruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InstructionExt for Instruction {
    fn inner(&self) -> *mut InstructionInner {
        *self.inner
    }
}
impl Instruction {
    pub unsafe fn from_inner(inner: *mut InstructionInner, owned: bool) -> Instruction {
        Instruction {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Instruction {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut InvokeInstInner;
}

pub struct InvokeInst {
    inner: ::core::nonzero::NonZero<*mut InvokeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InvokeInstExt for InvokeInst {
    fn inner(&self) -> *mut InvokeInstInner {
        *self.inner
    }
}
impl InvokeInst {
    pub unsafe fn from_inner(inner: *mut InvokeInstInner, owned: bool) -> InvokeInst {
        InvokeInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InvokeInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut LandingPadInstInner;
}

pub struct LandingPadInst {
    inner: ::core::nonzero::NonZero<*mut LandingPadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LandingPadInstExt for LandingPadInst {
    fn inner(&self) -> *mut LandingPadInstInner {
        *self.inner
    }
}
impl LandingPadInst {
    pub unsafe fn from_inner(inner: *mut LandingPadInstInner, owned: bool) -> LandingPadInst {
        LandingPadInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for LandingPadInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut LoadInstInner;
}

pub struct LoadInst {
    inner: ::core::nonzero::NonZero<*mut LoadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LoadInstExt for LoadInst {
    fn inner(&self) -> *mut LoadInstInner {
        *self.inner
    }
}
impl LoadInst {
    pub unsafe fn from_inner(inner: *mut LoadInstInner, owned: bool) -> LoadInst {
        LoadInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for LoadInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut PHINodeInner;
}

pub struct PHINode {
    inner: ::core::nonzero::NonZero<*mut PHINodeInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PHINodeExt for PHINode {
    fn inner(&self) -> *mut PHINodeInner {
        *self.inner
    }
}
impl PHINode {
    pub unsafe fn from_inner(inner: *mut PHINodeInner, owned: bool) -> PHINode {
        PHINode {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for PHINode {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ResumeInstInner;
}

pub struct ResumeInst {
    inner: ::core::nonzero::NonZero<*mut ResumeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ResumeInstExt for ResumeInst {
    fn inner(&self) -> *mut ResumeInstInner {
        *self.inner
    }
}
impl ResumeInst {
    pub unsafe fn from_inner(inner: *mut ResumeInstInner, owned: bool) -> ResumeInst {
        ResumeInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ResumeInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ReturnInstInner;
}

pub struct ReturnInst {
    inner: ::core::nonzero::NonZero<*mut ReturnInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ReturnInstExt for ReturnInst {
    fn inner(&self) -> *mut ReturnInstInner {
        *self.inner
    }
}
impl ReturnInst {
    pub unsafe fn from_inner(inner: *mut ReturnInstInner, owned: bool) -> ReturnInst {
        ReturnInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ReturnInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut SelectInstInner;
}

pub struct SelectInst {
    inner: ::core::nonzero::NonZero<*mut SelectInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SelectInstExt for SelectInst {
    fn inner(&self) -> *mut SelectInstInner {
        *self.inner
    }
}
impl SelectInst {
    pub unsafe fn from_inner(inner: *mut SelectInstInner, owned: bool) -> SelectInst {
        SelectInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for SelectInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ShuffleVectorInstInner;
}

pub struct ShuffleVectorInst {
    inner: ::core::nonzero::NonZero<*mut ShuffleVectorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ShuffleVectorInstExt for ShuffleVectorInst {
    fn inner(&self) -> *mut ShuffleVectorInstInner {
        *self.inner
    }
}
impl ShuffleVectorInst {
    pub unsafe fn from_inner(inner: *mut ShuffleVectorInstInner, owned: bool) -> ShuffleVectorInst {
        ShuffleVectorInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ShuffleVectorInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut StoreInstInner;
}

pub struct StoreInst {
    inner: ::core::nonzero::NonZero<*mut StoreInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl StoreInstExt for StoreInst {
    fn inner(&self) -> *mut StoreInstInner {
        *self.inner
    }
}
impl StoreInst {
    pub unsafe fn from_inner(inner: *mut StoreInstInner, owned: bool) -> StoreInst {
        StoreInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for StoreInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut SwitchInstInner;
}

pub struct SwitchInst {
    inner: ::core::nonzero::NonZero<*mut SwitchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SwitchInstExt for SwitchInst {
    fn inner(&self) -> *mut SwitchInstInner {
        *self.inner
    }
}
impl SwitchInst {
    pub unsafe fn from_inner(inner: *mut SwitchInstInner, owned: bool) -> SwitchInst {
        SwitchInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for SwitchInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut TerminatorInstInner;
}

pub struct TerminatorInst {
    inner: ::core::nonzero::NonZero<*mut TerminatorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl TerminatorInstExt for TerminatorInst {
    fn inner(&self) -> *mut TerminatorInstInner {
        *self.inner
    }
}
impl TerminatorInst {
    pub unsafe fn from_inner(inner: *mut TerminatorInstInner, owned: bool) -> TerminatorInst {
        TerminatorInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for TerminatorInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionExt: ::llvm::value::user::inst::InstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut UnaryInstructionInner;
}

pub struct UnaryInstruction {
    inner: ::core::nonzero::NonZero<*mut UnaryInstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnaryInstructionExt for UnaryInstruction {
    fn inner(&self) -> *mut UnaryInstructionInner {
        *self.inner
    }
}
impl UnaryInstruction {
    pub unsafe fn from_inner(inner: *mut UnaryInstructionInner, owned: bool) -> UnaryInstruction {
        UnaryInstruction {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for UnaryInstruction {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstExt: ::llvm::value::user::inst::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut UnreachableInstInner;
}

pub struct UnreachableInst {
    inner: ::core::nonzero::NonZero<*mut UnreachableInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnreachableInstExt for UnreachableInst {
    fn inner(&self) -> *mut UnreachableInstInner {
        *self.inner
    }
}
impl UnreachableInst {
    pub unsafe fn from_inner(inner: *mut UnreachableInstInner, owned: bool) -> UnreachableInst {
        UnreachableInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for UnreachableInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut VAArgInstInner;
}

pub struct VAArgInst {
    inner: ::core::nonzero::NonZero<*mut VAArgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VAArgInstExt for VAArgInst {
    fn inner(&self) -> *mut VAArgInstInner {
        *self.inner
    }
}
impl VAArgInst {
    pub unsafe fn from_inner(inner: *mut VAArgInstInner, owned: bool) -> VAArgInst {
        VAArgInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for VAArgInst {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner(self));
            }
        }
    }
}
