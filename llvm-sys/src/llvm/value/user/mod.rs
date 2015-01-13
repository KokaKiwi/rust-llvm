pub mod constant;
pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstExt: ::llvm::value::user::CastInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_AddrSpaceCastInst(&self) -> *mut AddrSpaceCastInstInner;

    fn inner(&self) -> *mut AddrSpaceCastInstInner {
        self.inner_llvm_AddrSpaceCastInst()
    }
}

pub struct AddrSpaceCastInst {
    inner: ::core::nonzero::NonZero<*mut AddrSpaceCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AddrSpaceCastInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AddrSpaceCastInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AddrSpaceCastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for AddrSpaceCastInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for AddrSpaceCastInst {
    fn inner_llvm_CastInst(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AddrSpaceCastInstExt for AddrSpaceCastInst {
    fn inner_llvm_AddrSpaceCastInst(&self) -> *mut AddrSpaceCastInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstExt: ::llvm::value::user::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_AllocaInst(&self) -> *mut AllocaInstInner;

    fn inner(&self) -> *mut AllocaInstInner {
        self.inner_llvm_AllocaInst()
    }
}

pub struct AllocaInst {
    inner: ::core::nonzero::NonZero<*mut AllocaInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AllocaInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AllocaInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AllocaInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for AllocaInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AllocaInstExt for AllocaInst {
    fn inner_llvm_AllocaInst(&self) -> *mut AllocaInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_AtomicCmpXchgInst(&self) -> *mut AtomicCmpXchgInstInner;

    fn inner(&self) -> *mut AtomicCmpXchgInstInner {
        self.inner_llvm_AtomicCmpXchgInst()
    }
}

pub struct AtomicCmpXchgInst {
    inner: ::core::nonzero::NonZero<*mut AtomicCmpXchgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AtomicCmpXchgInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicCmpXchgInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AtomicCmpXchgInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicCmpXchgInstExt for AtomicCmpXchgInst {
    fn inner_llvm_AtomicCmpXchgInst(&self) -> *mut AtomicCmpXchgInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_AtomicRMWInst(&self) -> *mut AtomicRMWInstInner;

    fn inner(&self) -> *mut AtomicRMWInstInner {
        self.inner_llvm_AtomicRMWInst()
    }
}

pub struct AtomicRMWInst {
    inner: ::core::nonzero::NonZero<*mut AtomicRMWInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for AtomicRMWInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for AtomicRMWInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for AtomicRMWInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicRMWInstExt for AtomicRMWInst {
    fn inner_llvm_AtomicRMWInst(&self) -> *mut AtomicRMWInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_BinaryOperator(&self) -> *mut BinaryOperatorInner;

    fn inner(&self) -> *mut BinaryOperatorInner {
        self.inner_llvm_BinaryOperator()
    }
}

pub struct BinaryOperator {
    inner: ::core::nonzero::NonZero<*mut BinaryOperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BinaryOperator {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BinaryOperator {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BinaryOperator {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BinaryOperatorExt for BinaryOperator {
    fn inner_llvm_BinaryOperator(&self) -> *mut BinaryOperatorInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstExt: ::llvm::value::user::CastInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_BitCastInst(&self) -> *mut BitCastInstInner;

    fn inner(&self) -> *mut BitCastInstInner {
        self.inner_llvm_BitCastInst()
    }
}

pub struct BitCastInst {
    inner: ::core::nonzero::NonZero<*mut BitCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BitCastInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BitCastInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BitCastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for BitCastInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for BitCastInst {
    fn inner_llvm_CastInst(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BitCastInstExt for BitCastInst {
    fn inner_llvm_BitCastInst(&self) -> *mut BitCastInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_BranchInst(&self) -> *mut BranchInstInner;

    fn inner(&self) -> *mut BranchInstInner {
        self.inner_llvm_BranchInst()
    }
}

pub struct BranchInst {
    inner: ::core::nonzero::NonZero<*mut BranchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BranchInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BranchInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for BranchInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for BranchInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BranchInstExt for BranchInst {
    fn inner_llvm_BranchInst(&self) -> *mut BranchInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_CallInst(&self) -> *mut CallInstInner;

    fn inner(&self) -> *mut CallInstInner {
        self.inner_llvm_CallInst()
    }
}

pub struct CallInst {
    inner: ::core::nonzero::NonZero<*mut CallInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CallInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CallInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CallInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CallInstExt for CallInst {
    fn inner_llvm_CallInst(&self) -> *mut CallInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstExt: ::llvm::value::user::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_CastInst(&self) -> *mut CastInstInner;

    fn inner(&self) -> *mut CastInstInner {
        self.inner_llvm_CastInst()
    }
}

pub struct CastInst {
    inner: ::core::nonzero::NonZero<*mut CastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CastInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CastInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for CastInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CastInstExt for CastInst {
    fn inner_llvm_CastInst(&self) -> *mut CastInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type CmpInstInner = ::ffi::llvm_CmpInst;

pub trait CmpInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_CmpInst(&self) -> *mut CmpInstInner;

    fn inner(&self) -> *mut CmpInstInner {
        self.inner_llvm_CmpInst()
    }
}

pub struct CmpInst {
    inner: ::core::nonzero::NonZero<*mut CmpInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for CmpInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for CmpInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for CmpInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CmpInstExt for CmpInst {
    fn inner_llvm_CmpInst(&self) -> *mut CmpInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ExtractElementInst(&self) -> *mut ExtractElementInstInner;

    fn inner(&self) -> *mut ExtractElementInstInner {
        self.inner_llvm_ExtractElementInst()
    }
}

pub struct ExtractElementInst {
    inner: ::core::nonzero::NonZero<*mut ExtractElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ExtractElementInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractElementInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ExtractElementInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractElementInstExt for ExtractElementInst {
    fn inner_llvm_ExtractElementInst(&self) -> *mut ExtractElementInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstExt: ::llvm::value::user::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ExtractValueInst(&self) -> *mut ExtractValueInstInner;

    fn inner(&self) -> *mut ExtractValueInstInner {
        self.inner_llvm_ExtractValueInst()
    }
}

pub struct ExtractValueInst {
    inner: ::core::nonzero::NonZero<*mut ExtractValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ExtractValueInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ExtractValueInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ExtractValueInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for ExtractValueInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractValueInstExt for ExtractValueInst {
    fn inner_llvm_ExtractValueInst(&self) -> *mut ExtractValueInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstExt: ::llvm::value::user::CastInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_FPExtInst(&self) -> *mut FPExtInstInner;

    fn inner(&self) -> *mut FPExtInstInner {
        self.inner_llvm_FPExtInst()
    }
}

pub struct FPExtInst {
    inner: ::core::nonzero::NonZero<*mut FPExtInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FPExtInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPExtInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FPExtInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for FPExtInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for FPExtInst {
    fn inner_llvm_CastInst(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPExtInstExt for FPExtInst {
    fn inner_llvm_FPExtInst(&self) -> *mut FPExtInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstExt: ::llvm::value::user::CastInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_FPToSIInst(&self) -> *mut FPToSIInstInner;

    fn inner(&self) -> *mut FPToSIInstInner {
        self.inner_llvm_FPToSIInst()
    }
}

pub struct FPToSIInst {
    inner: ::core::nonzero::NonZero<*mut FPToSIInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FPToSIInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FPToSIInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FPToSIInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for FPToSIInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::CastInstExt for FPToSIInst {
    fn inner_llvm_CastInst(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPToSIInstExt for FPToSIInst {
    fn inner_llvm_FPToSIInst(&self) -> *mut FPToSIInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_FenceInst(&self) -> *mut FenceInstInner;

    fn inner(&self) -> *mut FenceInstInner {
        self.inner_llvm_FenceInst()
    }
}

pub struct FenceInst {
    inner: ::core::nonzero::NonZero<*mut FenceInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for FenceInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for FenceInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for FenceInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FenceInstExt for FenceInst {
    fn inner_llvm_FenceInst(&self) -> *mut FenceInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_GetElementPtrInst(&self) -> *mut GetElementPtrInstInner;

    fn inner(&self) -> *mut GetElementPtrInstInner {
        self.inner_llvm_GetElementPtrInst()
    }
}

pub struct GetElementPtrInst {
    inner: ::core::nonzero::NonZero<*mut GetElementPtrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for GetElementPtrInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GetElementPtrInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for GetElementPtrInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GetElementPtrInstExt for GetElementPtrInst {
    fn inner_llvm_GetElementPtrInst(&self) -> *mut GetElementPtrInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_IndirectBrInst(&self) -> *mut IndirectBrInstInner;

    fn inner(&self) -> *mut IndirectBrInstInner {
        self.inner_llvm_IndirectBrInst()
    }
}

pub struct IndirectBrInst {
    inner: ::core::nonzero::NonZero<*mut IndirectBrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for IndirectBrInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for IndirectBrInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for IndirectBrInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for IndirectBrInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IndirectBrInstExt for IndirectBrInst {
    fn inner_llvm_IndirectBrInst(&self) -> *mut IndirectBrInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_InsertElementInst(&self) -> *mut InsertElementInstInner;

    fn inner(&self) -> *mut InsertElementInstInner {
        self.inner_llvm_InsertElementInst()
    }
}

pub struct InsertElementInst {
    inner: ::core::nonzero::NonZero<*mut InsertElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InsertElementInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertElementInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InsertElementInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertElementInstExt for InsertElementInst {
    fn inner_llvm_InsertElementInst(&self) -> *mut InsertElementInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_InsertValueInst(&self) -> *mut InsertValueInstInner;

    fn inner(&self) -> *mut InsertValueInstInner {
        self.inner_llvm_InsertValueInst()
    }
}

pub struct InsertValueInst {
    inner: ::core::nonzero::NonZero<*mut InsertValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InsertValueInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InsertValueInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InsertValueInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertValueInstExt for InsertValueInst {
    fn inner_llvm_InsertValueInst(&self) -> *mut InsertValueInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type InstructionInner = ::ffi::llvm_Instruction;

pub trait InstructionExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Instruction(&self) -> *mut InstructionInner;

    fn inner(&self) -> *mut InstructionInner {
        self.inner_llvm_Instruction()
    }
}

pub struct Instruction {
    inner: ::core::nonzero::NonZero<*mut InstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Instruction {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Instruction {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InstructionExt for Instruction {
    fn inner_llvm_Instruction(&self) -> *mut InstructionInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_InvokeInst(&self) -> *mut InvokeInstInner;

    fn inner(&self) -> *mut InvokeInstInner {
        self.inner_llvm_InvokeInst()
    }
}

pub struct InvokeInst {
    inner: ::core::nonzero::NonZero<*mut InvokeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InvokeInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for InvokeInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for InvokeInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for InvokeInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InvokeInstExt for InvokeInst {
    fn inner_llvm_InvokeInst(&self) -> *mut InvokeInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_LandingPadInst(&self) -> *mut LandingPadInstInner;

    fn inner(&self) -> *mut LandingPadInstInner {
        self.inner_llvm_LandingPadInst()
    }
}

pub struct LandingPadInst {
    inner: ::core::nonzero::NonZero<*mut LandingPadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for LandingPadInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LandingPadInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for LandingPadInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LandingPadInstExt for LandingPadInst {
    fn inner_llvm_LandingPadInst(&self) -> *mut LandingPadInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstExt: ::llvm::value::user::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_LoadInst(&self) -> *mut LoadInstInner;

    fn inner(&self) -> *mut LoadInstInner {
        self.inner_llvm_LoadInst()
    }
}

pub struct LoadInst {
    inner: ::core::nonzero::NonZero<*mut LoadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for LoadInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for LoadInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for LoadInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for LoadInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LoadInstExt for LoadInst {
    fn inner_llvm_LoadInst(&self) -> *mut LoadInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Operator(&self) -> *mut OperatorInner;

    fn inner(&self) -> *mut OperatorInner {
        self.inner_llvm_Operator()
    }
}

pub struct Operator {
    inner: ::core::nonzero::NonZero<*mut OperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Operator {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Operator {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl OperatorExt for Operator {
    fn inner_llvm_Operator(&self) -> *mut OperatorInner {
        *self.inner
    }
}
impl Operator {
    pub unsafe fn from_inner(inner: *mut OperatorInner, owned: bool) -> Operator {
        Operator {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Operator {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_PHINode(&self) -> *mut PHINodeInner;

    fn inner(&self) -> *mut PHINodeInner {
        self.inner_llvm_PHINode()
    }
}

pub struct PHINode {
    inner: ::core::nonzero::NonZero<*mut PHINodeInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for PHINode {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for PHINode {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for PHINode {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PHINodeExt for PHINode {
    fn inner_llvm_PHINode(&self) -> *mut PHINodeInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ResumeInst(&self) -> *mut ResumeInstInner;

    fn inner(&self) -> *mut ResumeInstInner {
        self.inner_llvm_ResumeInst()
    }
}

pub struct ResumeInst {
    inner: ::core::nonzero::NonZero<*mut ResumeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ResumeInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ResumeInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ResumeInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for ResumeInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ResumeInstExt for ResumeInst {
    fn inner_llvm_ResumeInst(&self) -> *mut ResumeInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ReturnInst(&self) -> *mut ReturnInstInner;

    fn inner(&self) -> *mut ReturnInstInner {
        self.inner_llvm_ReturnInst()
    }
}

pub struct ReturnInst {
    inner: ::core::nonzero::NonZero<*mut ReturnInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ReturnInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ReturnInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ReturnInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for ReturnInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ReturnInstExt for ReturnInst {
    fn inner_llvm_ReturnInst(&self) -> *mut ReturnInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_SelectInst(&self) -> *mut SelectInstInner;

    fn inner(&self) -> *mut SelectInstInner {
        self.inner_llvm_SelectInst()
    }
}

pub struct SelectInst {
    inner: ::core::nonzero::NonZero<*mut SelectInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for SelectInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SelectInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for SelectInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SelectInstExt for SelectInst {
    fn inner_llvm_SelectInst(&self) -> *mut SelectInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ShuffleVectorInst(&self) -> *mut ShuffleVectorInstInner;

    fn inner(&self) -> *mut ShuffleVectorInstInner {
        self.inner_llvm_ShuffleVectorInst()
    }
}

pub struct ShuffleVectorInst {
    inner: ::core::nonzero::NonZero<*mut ShuffleVectorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for ShuffleVectorInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ShuffleVectorInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for ShuffleVectorInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ShuffleVectorInstExt for ShuffleVectorInst {
    fn inner_llvm_ShuffleVectorInst(&self) -> *mut ShuffleVectorInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_StoreInst(&self) -> *mut StoreInstInner;

    fn inner(&self) -> *mut StoreInstInner {
        self.inner_llvm_StoreInst()
    }
}

pub struct StoreInst {
    inner: ::core::nonzero::NonZero<*mut StoreInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for StoreInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for StoreInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for StoreInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl StoreInstExt for StoreInst {
    fn inner_llvm_StoreInst(&self) -> *mut StoreInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_SwitchInst(&self) -> *mut SwitchInstInner;

    fn inner(&self) -> *mut SwitchInstInner {
        self.inner_llvm_SwitchInst()
    }
}

pub struct SwitchInst {
    inner: ::core::nonzero::NonZero<*mut SwitchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for SwitchInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for SwitchInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for SwitchInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for SwitchInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SwitchInstExt for SwitchInst {
    fn inner_llvm_SwitchInst(&self) -> *mut SwitchInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_TerminatorInst(&self) -> *mut TerminatorInstInner;

    fn inner(&self) -> *mut TerminatorInstInner {
        self.inner_llvm_TerminatorInst()
    }
}

pub struct TerminatorInst {
    inner: ::core::nonzero::NonZero<*mut TerminatorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for TerminatorInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for TerminatorInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for TerminatorInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl TerminatorInstExt for TerminatorInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut TerminatorInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionExt: ::llvm::value::user::InstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_UnaryInstruction(&self) -> *mut UnaryInstructionInner;

    fn inner(&self) -> *mut UnaryInstructionInner {
        self.inner_llvm_UnaryInstruction()
    }
}

pub struct UnaryInstruction {
    inner: ::core::nonzero::NonZero<*mut UnaryInstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for UnaryInstruction {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnaryInstruction {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for UnaryInstruction {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnaryInstructionExt for UnaryInstruction {
    fn inner_llvm_UnaryInstruction(&self) -> *mut UnaryInstructionInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstExt: ::llvm::value::user::TerminatorInstExt {
    #[allow(non_snake_case)]
    fn inner_llvm_UnreachableInst(&self) -> *mut UnreachableInstInner;

    fn inner(&self) -> *mut UnreachableInstInner {
        self.inner_llvm_UnreachableInst()
    }
}

pub struct UnreachableInst {
    inner: ::core::nonzero::NonZero<*mut UnreachableInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for UnreachableInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UnreachableInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for UnreachableInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::TerminatorInstExt for UnreachableInst {
    fn inner_llvm_TerminatorInst(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnreachableInstExt for UnreachableInst {
    fn inner_llvm_UnreachableInst(&self) -> *mut UnreachableInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type UserInner = ::ffi::llvm_User;

pub trait UserExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_User(&self) -> *mut UserInner;

    fn inner(&self) -> *mut UserInner {
        self.inner_llvm_User()
    }

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(self.inner_llvm_User());
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::User_getNumOperands(self.inner_llvm_User() as *const ::ffi::llvm_User) as u32;
            ret
        }
    }

    fn get_operand(&self, idx: u32) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::User_getOperand(self.inner_llvm_User() as *const ::ffi::llvm_User, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn replace_uses_of_with(&mut self, from: &::llvm::value::ValueExt, to: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(self.inner_llvm_User(), from.inner_llvm_Value(), to.inner_llvm_Value());
        }
    }

    fn set_operand(&mut self, idx: u32, val: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_setOperand(self.inner_llvm_User(), idx as ::libc::c_uint, val.inner_llvm_Value());
        }
    }
}

pub struct User {
    inner: ::core::nonzero::NonZero<*mut UserInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for User {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UserExt for User {
    fn inner_llvm_User(&self) -> *mut UserInner {
        *self.inner
    }
}
impl User {
    pub unsafe fn from_inner(inner: *mut UserInner, owned: bool) -> User {
        User {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof(v: &::llvm::value::ValueExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::User_classof(v.inner_llvm_Value());
            ret
        }
    }
}
impl Drop for User {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstExt: ::llvm::value::user::UnaryInstructionExt {
    #[allow(non_snake_case)]
    fn inner_llvm_VAArgInst(&self) -> *mut VAArgInstInner;

    fn inner(&self) -> *mut VAArgInstInner {
        self.inner_llvm_VAArgInst()
    }
}

pub struct VAArgInst {
    inner: ::core::nonzero::NonZero<*mut VAArgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for VAArgInst {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for VAArgInst {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::InstructionExt for VAArgInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UnaryInstructionExt for VAArgInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VAArgInstExt for VAArgInst {
    fn inner_llvm_VAArgInst(&self) -> *mut VAArgInstInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
