pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstExt: ::llvm::value::user::inst::CastInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for AddrSpaceCastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for AddrSpaceCastInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for AddrSpaceCastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for AllocaInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for AllocaInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for AtomicCmpXchgInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for AtomicRMWInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for BinaryOperator {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstExt: ::llvm::value::user::inst::CastInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for BitCastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for BitCastInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for BitCastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for BranchInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for BranchInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for CallInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for CastInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for CastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type CmpInstInner = ::ffi::llvm_CmpInst;

pub trait CmpInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for CmpInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for ExtractElementInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for ExtractValueInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for ExtractValueInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstExt: ::llvm::value::user::inst::CastInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for FPExtInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for FPExtInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for FPExtInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstExt: ::llvm::value::user::inst::CastInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for FPToSIInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for FPToSIInst {
    fn inner_llvm_UnaryInstruction(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstExt for FPToSIInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for FenceInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for GetElementPtrInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for IndirectBrInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for IndirectBrInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for InsertElementInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for InsertValueInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
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

    fn clone(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_clone(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn copy_fast_math_flags(&mut self, inst: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_copyFastMathFlags(self.inner_llvm_Instruction(), inst.inner_llvm_Instruction());
        }
    }

    fn drop_unknown_metadata(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_dropUnknownMetadata(self.inner_llvm_Instruction());
        }
    }

    fn drop_unknown_metadata_from_ids(&mut self, known_i_ds: &[u32]) {
        unsafe {
            let c_known_i_ds = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: known_i_ds.as_ptr(),
                length: known_i_ds.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_dropUnknownMetadataFromIDS(self.inner_llvm_Instruction(), c_known_i_ds);
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_eraseFromParent(self.inner_llvm_Instruction());
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDataLayout(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_debug_loc(&self) -> ::llvm::DebugLoc {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDebugLoc(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ::llvm::DebugLoc::from_inner(ret as *mut ::ffi::llvm_DebugLoc)
        }
    }

    fn get_metadata(&self, kind_id: u32) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getMetadata(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, kind_id as ::libc::c_uint);
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
            let ret = ::ffi::llvm::Instruction_getMetadataStr(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, c_kind);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getOpcode(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction) as u32;
            ret
        }
    }

    fn get_parent(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParent(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParentMut(self.inner_llvm_Instruction());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn has_allow_reciprocal(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasAllowReciprocal(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadata(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata_other_than_debug_loc(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadataOtherThanDebugLoc(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_infs(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoInfs(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_na_ns(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoNaNs(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_signed_zeros(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoSignedZeros(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_unsafe_algebra(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasUnsafeAlgebra(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn insert_after(&mut self, insert_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_insertAfter(self.inner_llvm_Instruction(), insert_pos.inner_llvm_Instruction());
        }
    }

    fn insert_before(&mut self, insert_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_insertBefore(self.inner_llvm_Instruction(), insert_pos.inner_llvm_Instruction());
        }
    }

    fn is_arithmetic_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isArithmeticShift(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_associative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isAssociative(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_binary_op(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isBinaryOp(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_cast(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCast(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_commutative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCommutative(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_idempotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdempotent(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_identical_to(&self, inst: &::llvm::value::user::inst::InstructionExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalTo(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, inst.inner_llvm_Instruction());
            ret
        }
    }

    fn is_identical_to_when_defined(&self, inst: &::llvm::value::user::inst::InstructionExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalToWhenDefined(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, inst.inner_llvm_Instruction());
            ret
        }
    }

    fn is_logical_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isLogicalShift(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_nilpotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isNilpotent(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_same_operation_as(&self, inst: &::llvm::value::user::inst::InstructionExt, flags: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isSameOperationAs(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, inst.inner_llvm_Instruction(), flags as ::libc::c_uint);
            ret
        }
    }

    fn is_shift(&mut self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isShift(self.inner_llvm_Instruction());
            ret
        }
    }

    fn is_terminator(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isTerminator(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_used_outside_of_block(&self, bb: &::llvm::value::BasicBlockExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isUsedOutsideOfBlock(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction, bb.inner_llvm_BasicBlock());
            ret
        }
    }

    fn may_have_side_effects(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayHaveSideEffects(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_from_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadFromMemory(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_or_write_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadOrWriteMemory(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_return(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReturn(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_throw(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayThrow(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_write_to_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayWriteToMemory(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn move_before(&mut self, move_pos: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::Instruction_moveBefore(self.inner_llvm_Instruction(), move_pos.inner_llvm_Instruction());
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_removeFromParent(self.inner_llvm_Instruction());
        }
    }

    fn set_debug_loc(&mut self, loc: &::llvm::DebugLocExt) {
        unsafe {
            ::ffi::llvm::Instruction_setDebugLoc(self.inner_llvm_Instruction(), loc.inner_llvm_DebugLoc());
        }
    }

    fn set_has_allow_reciprocal(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasAllowReciprocal(self.inner_llvm_Instruction(), val);
        }
    }

    fn set_has_no_infs(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoInfs(self.inner_llvm_Instruction(), val);
        }
    }

    fn set_has_no_na_ns(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoNaNs(self.inner_llvm_Instruction(), val);
        }
    }

    fn set_has_no_signed_zeros(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoSignedZeros(self.inner_llvm_Instruction(), val);
        }
    }

    fn set_has_unsafe_algebra(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasUnsafeAlgebra(self.inner_llvm_Instruction(), val);
        }
    }

    fn set_metadata(&mut self, kind_id: u32, node: &::llvm::value::MDNodeExt) {
        unsafe {
            ::ffi::llvm::Instruction_setMetadata(self.inner_llvm_Instruction(), kind_id as ::libc::c_uint, node.inner_llvm_MDNode());
        }
    }

    fn set_metadata_str(&mut self, kind: &str, node: &::llvm::value::MDNodeExt) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_setMetadataStr(self.inner_llvm_Instruction(), c_kind, node.inner_llvm_MDNode());
        }
    }

    fn user_back(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back(self.inner_llvm_Instruction() as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn user_back_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back_mut(self.inner_llvm_Instruction());
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for InvokeInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for InvokeInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for LandingPadInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for LoadInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for LoadInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for PHINode {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for ResumeInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for ResumeInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for ReturnInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for ReturnInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for SelectInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for ShuffleVectorInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for StoreInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for SwitchInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for SwitchInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for TerminatorInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionExt: ::llvm::value::user::inst::InstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for UnaryInstruction {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstExt: ::llvm::value::user::inst::TerminatorInstExt {
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
impl ::llvm::value::user::inst::InstructionExt for UnreachableInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstExt for UnreachableInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstExt: ::llvm::value::user::inst::UnaryInstructionExt {
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
impl ::llvm::value::user::inst::InstructionExt for VAArgInst {
    fn inner_llvm_Instruction(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionExt for VAArgInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionExt::inner_llvm_Instruction(self));
            }
        }
    }
}
