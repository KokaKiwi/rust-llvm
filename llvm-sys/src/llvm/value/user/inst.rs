pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstObj: ::llvm::value::user::inst::CastInstObj {
    unsafe fn get_inner(&self) -> *mut AddrSpaceCastInstInner;
}

pub trait AddrSpaceCastInstOwned: AddrSpaceCastInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut AddrSpaceCastInstInner {
        let inner = AddrSpaceCastInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> AddrSpaceCastInstOwned for T where T: AddrSpaceCastInstObj + ::core::marker::Sized {}

pub trait AddrSpaceCastInstExt: AddrSpaceCastInstObj {
}
impl<T> AddrSpaceCastInstExt for T where T: AddrSpaceCastInstObj {}

pub struct AddrSpaceCastInst {
    inner: ::core::nonzero::NonZero<*mut AddrSpaceCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AddrSpaceCastInstObj for AddrSpaceCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut AddrSpaceCastInstInner {
        *self.inner
    }
}
impl AddrSpaceCastInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut AddrSpaceCastInstInner, owned: bool) -> AddrSpaceCastInst {
        AddrSpaceCastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AddrSpaceCastInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    unsafe fn get_inner(&self) -> *mut AllocaInstInner;
}

pub trait AllocaInstOwned: AllocaInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut AllocaInstInner {
        let inner = AllocaInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> AllocaInstOwned for T where T: AllocaInstObj + ::core::marker::Sized {}

pub trait AllocaInstExt: AllocaInstObj {
}
impl<T> AllocaInstExt for T where T: AllocaInstObj {}

pub struct AllocaInst {
    inner: ::core::nonzero::NonZero<*mut AllocaInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AllocaInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AllocaInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AllocaInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for AllocaInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AllocaInstObj for AllocaInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut AllocaInstInner {
        *self.inner
    }
}
impl AllocaInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut AllocaInstInner, owned: bool) -> AllocaInst {
        AllocaInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AllocaInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut AtomicCmpXchgInstInner;
}

pub trait AtomicCmpXchgInstOwned: AtomicCmpXchgInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut AtomicCmpXchgInstInner {
        let inner = AtomicCmpXchgInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> AtomicCmpXchgInstOwned for T where T: AtomicCmpXchgInstObj + ::core::marker::Sized {}

pub trait AtomicCmpXchgInstExt: AtomicCmpXchgInstObj {
}
impl<T> AtomicCmpXchgInstExt for T where T: AtomicCmpXchgInstObj {}

pub struct AtomicCmpXchgInst {
    inner: ::core::nonzero::NonZero<*mut AtomicCmpXchgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AtomicCmpXchgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AtomicCmpXchgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AtomicCmpXchgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicCmpXchgInstObj for AtomicCmpXchgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut AtomicCmpXchgInstInner {
        *self.inner
    }
}
impl AtomicCmpXchgInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut AtomicCmpXchgInstInner, owned: bool) -> AtomicCmpXchgInst {
        AtomicCmpXchgInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AtomicCmpXchgInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut AtomicRMWInstInner;
}

pub trait AtomicRMWInstOwned: AtomicRMWInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut AtomicRMWInstInner {
        let inner = AtomicRMWInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> AtomicRMWInstOwned for T where T: AtomicRMWInstObj + ::core::marker::Sized {}

pub trait AtomicRMWInstExt: AtomicRMWInstObj {
}
impl<T> AtomicRMWInstExt for T where T: AtomicRMWInstObj {}

pub struct AtomicRMWInst {
    inner: ::core::nonzero::NonZero<*mut AtomicRMWInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AtomicRMWInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AtomicRMWInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AtomicRMWInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicRMWInstObj for AtomicRMWInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut AtomicRMWInstInner {
        *self.inner
    }
}
impl AtomicRMWInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut AtomicRMWInstInner, owned: bool) -> AtomicRMWInst {
        AtomicRMWInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for AtomicRMWInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut BinaryOperatorInner;
}

pub trait BinaryOperatorOwned: BinaryOperatorObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut BinaryOperatorInner {
        let inner = BinaryOperatorObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> BinaryOperatorOwned for T where T: BinaryOperatorObj + ::core::marker::Sized {}

pub trait BinaryOperatorExt: BinaryOperatorObj {
}
impl<T> BinaryOperatorExt for T where T: BinaryOperatorObj {}

pub struct BinaryOperator {
    inner: ::core::nonzero::NonZero<*mut BinaryOperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BinaryOperator {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BinaryOperator {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BinaryOperator {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BinaryOperatorObj for BinaryOperator {
    #[inline(always)]
    fn get_inner(&self) -> *mut BinaryOperatorInner {
        *self.inner
    }
}
impl BinaryOperator {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut BinaryOperatorInner, owned: bool) -> BinaryOperator {
        BinaryOperator {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BinaryOperator {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstObj: ::llvm::value::user::inst::CastInstObj {
    unsafe fn get_inner(&self) -> *mut BitCastInstInner;
}

pub trait BitCastInstOwned: BitCastInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut BitCastInstInner {
        let inner = BitCastInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> BitCastInstOwned for T where T: BitCastInstObj + ::core::marker::Sized {}

pub trait BitCastInstExt: BitCastInstObj {
}
impl<T> BitCastInstExt for T where T: BitCastInstObj {}

pub struct BitCastInst {
    inner: ::core::nonzero::NonZero<*mut BitCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BitCastInstObj for BitCastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut BitCastInstInner {
        *self.inner
    }
}
impl BitCastInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut BitCastInstInner, owned: bool) -> BitCastInst {
        BitCastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BitCastInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut BranchInstInner;
}

pub trait BranchInstOwned: BranchInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut BranchInstInner {
        let inner = BranchInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> BranchInstOwned for T where T: BranchInstObj + ::core::marker::Sized {}

pub trait BranchInstExt: BranchInstObj {
}
impl<T> BranchInstExt for T where T: BranchInstObj {}

pub struct BranchInst {
    inner: ::core::nonzero::NonZero<*mut BranchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BranchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BranchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BranchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for BranchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BranchInstObj for BranchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut BranchInstInner {
        *self.inner
    }
}
impl BranchInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut BranchInstInner, owned: bool) -> BranchInst {
        BranchInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BranchInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut CallInstInner;
}

pub trait CallInstOwned: CallInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut CallInstInner {
        let inner = CallInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> CallInstOwned for T where T: CallInstObj + ::core::marker::Sized {}

pub trait CallInstExt: CallInstObj {
}
impl<T> CallInstExt for T where T: CallInstObj {}

pub struct CallInst {
    inner: ::core::nonzero::NonZero<*mut CallInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CallInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CallInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CallInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CallInstObj for CallInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut CallInstInner {
        *self.inner
    }
}
impl CallInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut CallInstInner, owned: bool) -> CallInst {
        CallInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CallInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    unsafe fn get_inner(&self) -> *mut CastInstInner;
}

pub trait CastInstOwned: CastInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut CastInstInner {
        let inner = CastInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> CastInstOwned for T where T: CastInstObj + ::core::marker::Sized {}

pub trait CastInstExt: CastInstObj {
}
impl<T> CastInstExt for T where T: CastInstObj {}

pub struct CastInst {
    inner: ::core::nonzero::NonZero<*mut CastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for CastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CastInstObj for CastInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut CastInstInner {
        *self.inner
    }
}
impl CastInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut CastInstInner, owned: bool) -> CastInst {
        CastInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CastInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub enum Predicate {
    FCMPFALSE,
    FCMPOEQ,
    FCMPOGT,
    FCMPOGE,
    FCMPOLT,
    FCMPOLE,
    FCMPONE,
    FCMPORD,
    FCMPUNO,
    FCMPUEQ,
    FCMPUGT,
    FCMPUGE,
    FCMPULT,
    FCMPULE,
    FCMPUNE,
    FCMPTRUE,
    FIRSTFCMPPREDICATE,
    LASTFCMPPREDICATE,
    BADFCMPPREDICATE,
    ICMPEQ,
    ICMPNE,
    ICMPUGT,
    ICMPUGE,
    ICMPULT,
    ICMPULE,
    ICMPSGT,
    ICMPSGE,
    ICMPSLT,
    ICMPSLE,
    FIRSTICMPPREDICATE,
    LASTICMPPREDICATE,
    BADICMPPREDICATE,
}
impl Predicate {
    pub fn from_ffi(value: ::ffi::llvm_CmpInst_Predicate) -> Predicate {
        match value {
            ::ffi::llvm_CmpInst_Predicate::FCMP_FALSE => Predicate::FCMPFALSE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_OEQ => Predicate::FCMPOEQ,
            ::ffi::llvm_CmpInst_Predicate::FCMP_OGT => Predicate::FCMPOGT,
            ::ffi::llvm_CmpInst_Predicate::FCMP_OGE => Predicate::FCMPOGE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_OLT => Predicate::FCMPOLT,
            ::ffi::llvm_CmpInst_Predicate::FCMP_OLE => Predicate::FCMPOLE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_ONE => Predicate::FCMPONE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_ORD => Predicate::FCMPORD,
            ::ffi::llvm_CmpInst_Predicate::FCMP_UNO => Predicate::FCMPUNO,
            ::ffi::llvm_CmpInst_Predicate::FCMP_UEQ => Predicate::FCMPUEQ,
            ::ffi::llvm_CmpInst_Predicate::FCMP_UGT => Predicate::FCMPUGT,
            ::ffi::llvm_CmpInst_Predicate::FCMP_UGE => Predicate::FCMPUGE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_ULT => Predicate::FCMPULT,
            ::ffi::llvm_CmpInst_Predicate::FCMP_ULE => Predicate::FCMPULE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_UNE => Predicate::FCMPUNE,
            ::ffi::llvm_CmpInst_Predicate::FCMP_TRUE => Predicate::FCMPTRUE,
            ::ffi::llvm_CmpInst_Predicate::BAD_FCMP_PREDICATE => Predicate::BADFCMPPREDICATE,
            ::ffi::llvm_CmpInst_Predicate::ICMP_EQ => Predicate::ICMPEQ,
            ::ffi::llvm_CmpInst_Predicate::ICMP_NE => Predicate::ICMPNE,
            ::ffi::llvm_CmpInst_Predicate::ICMP_UGT => Predicate::ICMPUGT,
            ::ffi::llvm_CmpInst_Predicate::ICMP_UGE => Predicate::ICMPUGE,
            ::ffi::llvm_CmpInst_Predicate::ICMP_ULT => Predicate::ICMPULT,
            ::ffi::llvm_CmpInst_Predicate::ICMP_ULE => Predicate::ICMPULE,
            ::ffi::llvm_CmpInst_Predicate::ICMP_SGT => Predicate::ICMPSGT,
            ::ffi::llvm_CmpInst_Predicate::ICMP_SGE => Predicate::ICMPSGE,
            ::ffi::llvm_CmpInst_Predicate::ICMP_SLT => Predicate::ICMPSLT,
            ::ffi::llvm_CmpInst_Predicate::ICMP_SLE => Predicate::ICMPSLE,
            ::ffi::llvm_CmpInst_Predicate::BAD_ICMP_PREDICATE => Predicate::BADICMPPREDICATE,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_CmpInst_Predicate {
        match self {
            Predicate::FCMPFALSE => ::ffi::llvm_CmpInst_Predicate::FCMP_FALSE,
            Predicate::FCMPOEQ => ::ffi::llvm_CmpInst_Predicate::FCMP_OEQ,
            Predicate::FCMPOGT => ::ffi::llvm_CmpInst_Predicate::FCMP_OGT,
            Predicate::FCMPOGE => ::ffi::llvm_CmpInst_Predicate::FCMP_OGE,
            Predicate::FCMPOLT => ::ffi::llvm_CmpInst_Predicate::FCMP_OLT,
            Predicate::FCMPOLE => ::ffi::llvm_CmpInst_Predicate::FCMP_OLE,
            Predicate::FCMPONE => ::ffi::llvm_CmpInst_Predicate::FCMP_ONE,
            Predicate::FCMPORD => ::ffi::llvm_CmpInst_Predicate::FCMP_ORD,
            Predicate::FCMPUNO => ::ffi::llvm_CmpInst_Predicate::FCMP_UNO,
            Predicate::FCMPUEQ => ::ffi::llvm_CmpInst_Predicate::FCMP_UEQ,
            Predicate::FCMPUGT => ::ffi::llvm_CmpInst_Predicate::FCMP_UGT,
            Predicate::FCMPUGE => ::ffi::llvm_CmpInst_Predicate::FCMP_UGE,
            Predicate::FCMPULT => ::ffi::llvm_CmpInst_Predicate::FCMP_ULT,
            Predicate::FCMPULE => ::ffi::llvm_CmpInst_Predicate::FCMP_ULE,
            Predicate::FCMPUNE => ::ffi::llvm_CmpInst_Predicate::FCMP_UNE,
            Predicate::FCMPTRUE => ::ffi::llvm_CmpInst_Predicate::FCMP_TRUE,
            Predicate::FIRSTFCMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::FCMP_FALSE,
            Predicate::LASTFCMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::FCMP_TRUE,
            Predicate::BADFCMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::BAD_FCMP_PREDICATE,
            Predicate::ICMPEQ => ::ffi::llvm_CmpInst_Predicate::ICMP_EQ,
            Predicate::ICMPNE => ::ffi::llvm_CmpInst_Predicate::ICMP_NE,
            Predicate::ICMPUGT => ::ffi::llvm_CmpInst_Predicate::ICMP_UGT,
            Predicate::ICMPUGE => ::ffi::llvm_CmpInst_Predicate::ICMP_UGE,
            Predicate::ICMPULT => ::ffi::llvm_CmpInst_Predicate::ICMP_ULT,
            Predicate::ICMPULE => ::ffi::llvm_CmpInst_Predicate::ICMP_ULE,
            Predicate::ICMPSGT => ::ffi::llvm_CmpInst_Predicate::ICMP_SGT,
            Predicate::ICMPSGE => ::ffi::llvm_CmpInst_Predicate::ICMP_SGE,
            Predicate::ICMPSLT => ::ffi::llvm_CmpInst_Predicate::ICMP_SLT,
            Predicate::ICMPSLE => ::ffi::llvm_CmpInst_Predicate::ICMP_SLE,
            Predicate::FIRSTICMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::ICMP_EQ,
            Predicate::LASTICMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::ICMP_SLE,
            Predicate::BADICMPPREDICATE => ::ffi::llvm_CmpInst_Predicate::BAD_ICMP_PREDICATE,
        }
    }
}
impl Copy for Predicate {}
pub type CmpInstInner = ::ffi::llvm_CmpInst;

pub trait CmpInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut CmpInstInner;
}

pub trait CmpInstOwned: CmpInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut CmpInstInner {
        let inner = CmpInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> CmpInstOwned for T where T: CmpInstObj + ::core::marker::Sized {}

pub trait CmpInstExt: CmpInstObj {
}
impl<T> CmpInstExt for T where T: CmpInstObj {}

pub struct CmpInst {
    inner: ::core::nonzero::NonZero<*mut CmpInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CmpInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CmpInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CmpInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CmpInstObj for CmpInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut CmpInstInner {
        *self.inner
    }
}
impl CmpInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut CmpInstInner, owned: bool) -> CmpInst {
        CmpInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CmpInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut ExtractElementInstInner;
}

pub trait ExtractElementInstOwned: ExtractElementInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ExtractElementInstInner {
        let inner = ExtractElementInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ExtractElementInstOwned for T where T: ExtractElementInstObj + ::core::marker::Sized {}

pub trait ExtractElementInstExt: ExtractElementInstObj {
}
impl<T> ExtractElementInstExt for T where T: ExtractElementInstObj {}

pub struct ExtractElementInst {
    inner: ::core::nonzero::NonZero<*mut ExtractElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ExtractElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ExtractElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ExtractElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractElementInstObj for ExtractElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ExtractElementInstInner {
        *self.inner
    }
}
impl ExtractElementInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ExtractElementInstInner, owned: bool) -> ExtractElementInst {
        ExtractElementInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ExtractElementInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    unsafe fn get_inner(&self) -> *mut ExtractValueInstInner;
}

pub trait ExtractValueInstOwned: ExtractValueInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ExtractValueInstInner {
        let inner = ExtractValueInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ExtractValueInstOwned for T where T: ExtractValueInstObj + ::core::marker::Sized {}

pub trait ExtractValueInstExt: ExtractValueInstObj {
}
impl<T> ExtractValueInstExt for T where T: ExtractValueInstObj {}

pub struct ExtractValueInst {
    inner: ::core::nonzero::NonZero<*mut ExtractValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ExtractValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ExtractValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ExtractValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for ExtractValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractValueInstObj for ExtractValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ExtractValueInstInner {
        *self.inner
    }
}
impl ExtractValueInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ExtractValueInstInner, owned: bool) -> ExtractValueInst {
        ExtractValueInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ExtractValueInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstObj: ::llvm::value::user::inst::CastInstObj {
    unsafe fn get_inner(&self) -> *mut FPExtInstInner;
}

pub trait FPExtInstOwned: FPExtInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut FPExtInstInner {
        let inner = FPExtInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> FPExtInstOwned for T where T: FPExtInstObj + ::core::marker::Sized {}

pub trait FPExtInstExt: FPExtInstObj {
}
impl<T> FPExtInstExt for T where T: FPExtInstObj {}

pub struct FPExtInst {
    inner: ::core::nonzero::NonZero<*mut FPExtInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPExtInstObj for FPExtInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut FPExtInstInner {
        *self.inner
    }
}
impl FPExtInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut FPExtInstInner, owned: bool) -> FPExtInst {
        FPExtInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FPExtInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstObj: ::llvm::value::user::inst::CastInstObj {
    unsafe fn get_inner(&self) -> *mut FPToSIInstInner;
}

pub trait FPToSIInstOwned: FPToSIInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut FPToSIInstInner {
        let inner = FPToSIInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> FPToSIInstOwned for T where T: FPToSIInstObj + ::core::marker::Sized {}

pub trait FPToSIInstExt: FPToSIInstObj {
}
impl<T> FPToSIInstExt for T where T: FPToSIInstObj {}

pub struct FPToSIInst {
    inner: ::core::nonzero::NonZero<*mut FPToSIInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPToSIInstObj for FPToSIInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut FPToSIInstInner {
        *self.inner
    }
}
impl FPToSIInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut FPToSIInstInner, owned: bool) -> FPToSIInst {
        FPToSIInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FPToSIInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut FenceInstInner;
}

pub trait FenceInstOwned: FenceInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut FenceInstInner {
        let inner = FenceInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> FenceInstOwned for T where T: FenceInstObj + ::core::marker::Sized {}

pub trait FenceInstExt: FenceInstObj {
}
impl<T> FenceInstExt for T where T: FenceInstObj {}

pub struct FenceInst {
    inner: ::core::nonzero::NonZero<*mut FenceInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FenceInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FenceInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FenceInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FenceInstObj for FenceInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut FenceInstInner {
        *self.inner
    }
}
impl FenceInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut FenceInstInner, owned: bool) -> FenceInst {
        FenceInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FenceInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut GetElementPtrInstInner;
}

pub trait GetElementPtrInstOwned: GetElementPtrInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut GetElementPtrInstInner {
        let inner = GetElementPtrInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> GetElementPtrInstOwned for T where T: GetElementPtrInstObj + ::core::marker::Sized {}

pub trait GetElementPtrInstExt: GetElementPtrInstObj {
}
impl<T> GetElementPtrInstExt for T where T: GetElementPtrInstObj {}

pub struct GetElementPtrInst {
    inner: ::core::nonzero::NonZero<*mut GetElementPtrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GetElementPtrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GetElementPtrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for GetElementPtrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GetElementPtrInstObj for GetElementPtrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut GetElementPtrInstInner {
        *self.inner
    }
}
impl GetElementPtrInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut GetElementPtrInstInner, owned: bool) -> GetElementPtrInst {
        GetElementPtrInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for GetElementPtrInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut IndirectBrInstInner;
}

pub trait IndirectBrInstOwned: IndirectBrInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut IndirectBrInstInner {
        let inner = IndirectBrInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> IndirectBrInstOwned for T where T: IndirectBrInstObj + ::core::marker::Sized {}

pub trait IndirectBrInstExt: IndirectBrInstObj {
}
impl<T> IndirectBrInstExt for T where T: IndirectBrInstObj {}

pub struct IndirectBrInst {
    inner: ::core::nonzero::NonZero<*mut IndirectBrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for IndirectBrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for IndirectBrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for IndirectBrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for IndirectBrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IndirectBrInstObj for IndirectBrInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut IndirectBrInstInner {
        *self.inner
    }
}
impl IndirectBrInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut IndirectBrInstInner, owned: bool) -> IndirectBrInst {
        IndirectBrInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for IndirectBrInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut InsertElementInstInner;
}

pub trait InsertElementInstOwned: InsertElementInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut InsertElementInstInner {
        let inner = InsertElementInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> InsertElementInstOwned for T where T: InsertElementInstObj + ::core::marker::Sized {}

pub trait InsertElementInstExt: InsertElementInstObj {
}
impl<T> InsertElementInstExt for T where T: InsertElementInstObj {}

pub struct InsertElementInst {
    inner: ::core::nonzero::NonZero<*mut InsertElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InsertElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InsertElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InsertElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertElementInstObj for InsertElementInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut InsertElementInstInner {
        *self.inner
    }
}
impl InsertElementInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut InsertElementInstInner, owned: bool) -> InsertElementInst {
        InsertElementInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InsertElementInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut InsertValueInstInner;
}

pub trait InsertValueInstOwned: InsertValueInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut InsertValueInstInner {
        let inner = InsertValueInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> InsertValueInstOwned for T where T: InsertValueInstObj + ::core::marker::Sized {}

pub trait InsertValueInstExt: InsertValueInstObj {
}
impl<T> InsertValueInstExt for T where T: InsertValueInstObj {}

pub struct InsertValueInst {
    inner: ::core::nonzero::NonZero<*mut InsertValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InsertValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InsertValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InsertValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertValueInstObj for InsertValueInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut InsertValueInstInner {
        *self.inner
    }
}
impl InsertValueInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut InsertValueInstInner, owned: bool) -> InsertValueInst {
        InsertValueInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InsertValueInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub enum BinaryOps {
    Add,
    FAdd,
    Sub,
    FSub,
    Mul,
    FMul,
    UDiv,
    SDiv,
    FDiv,
    URem,
    SRem,
    FRem,
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}
impl BinaryOps {
    pub fn from_ffi(value: ::ffi::llvm_Instruction_BinaryOps) -> BinaryOps {
        match value {
            ::ffi::llvm_Instruction_BinaryOps::Add => BinaryOps::Add,
            ::ffi::llvm_Instruction_BinaryOps::FAdd => BinaryOps::FAdd,
            ::ffi::llvm_Instruction_BinaryOps::Sub => BinaryOps::Sub,
            ::ffi::llvm_Instruction_BinaryOps::FSub => BinaryOps::FSub,
            ::ffi::llvm_Instruction_BinaryOps::Mul => BinaryOps::Mul,
            ::ffi::llvm_Instruction_BinaryOps::FMul => BinaryOps::FMul,
            ::ffi::llvm_Instruction_BinaryOps::UDiv => BinaryOps::UDiv,
            ::ffi::llvm_Instruction_BinaryOps::SDiv => BinaryOps::SDiv,
            ::ffi::llvm_Instruction_BinaryOps::FDiv => BinaryOps::FDiv,
            ::ffi::llvm_Instruction_BinaryOps::URem => BinaryOps::URem,
            ::ffi::llvm_Instruction_BinaryOps::SRem => BinaryOps::SRem,
            ::ffi::llvm_Instruction_BinaryOps::FRem => BinaryOps::FRem,
            ::ffi::llvm_Instruction_BinaryOps::Shl => BinaryOps::Shl,
            ::ffi::llvm_Instruction_BinaryOps::LShr => BinaryOps::LShr,
            ::ffi::llvm_Instruction_BinaryOps::AShr => BinaryOps::AShr,
            ::ffi::llvm_Instruction_BinaryOps::And => BinaryOps::And,
            ::ffi::llvm_Instruction_BinaryOps::Or => BinaryOps::Or,
            ::ffi::llvm_Instruction_BinaryOps::Xor => BinaryOps::Xor,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Instruction_BinaryOps {
        match self {
            BinaryOps::Add => ::ffi::llvm_Instruction_BinaryOps::Add,
            BinaryOps::FAdd => ::ffi::llvm_Instruction_BinaryOps::FAdd,
            BinaryOps::Sub => ::ffi::llvm_Instruction_BinaryOps::Sub,
            BinaryOps::FSub => ::ffi::llvm_Instruction_BinaryOps::FSub,
            BinaryOps::Mul => ::ffi::llvm_Instruction_BinaryOps::Mul,
            BinaryOps::FMul => ::ffi::llvm_Instruction_BinaryOps::FMul,
            BinaryOps::UDiv => ::ffi::llvm_Instruction_BinaryOps::UDiv,
            BinaryOps::SDiv => ::ffi::llvm_Instruction_BinaryOps::SDiv,
            BinaryOps::FDiv => ::ffi::llvm_Instruction_BinaryOps::FDiv,
            BinaryOps::URem => ::ffi::llvm_Instruction_BinaryOps::URem,
            BinaryOps::SRem => ::ffi::llvm_Instruction_BinaryOps::SRem,
            BinaryOps::FRem => ::ffi::llvm_Instruction_BinaryOps::FRem,
            BinaryOps::Shl => ::ffi::llvm_Instruction_BinaryOps::Shl,
            BinaryOps::LShr => ::ffi::llvm_Instruction_BinaryOps::LShr,
            BinaryOps::AShr => ::ffi::llvm_Instruction_BinaryOps::AShr,
            BinaryOps::And => ::ffi::llvm_Instruction_BinaryOps::And,
            BinaryOps::Or => ::ffi::llvm_Instruction_BinaryOps::Or,
            BinaryOps::Xor => ::ffi::llvm_Instruction_BinaryOps::Xor,
        }
    }
}
impl Copy for BinaryOps {}
pub enum CastOps {
    Trunc,
    ZExt,
    SExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,
    FPTrunc,
    FPExt,
    PtrToInt,
    IntToPtr,
    BitCast,
    AddrSpaceCast,
}
impl CastOps {
    pub fn from_ffi(value: ::ffi::llvm_Instruction_CastOps) -> CastOps {
        match value {
            ::ffi::llvm_Instruction_CastOps::Trunc => CastOps::Trunc,
            ::ffi::llvm_Instruction_CastOps::ZExt => CastOps::ZExt,
            ::ffi::llvm_Instruction_CastOps::SExt => CastOps::SExt,
            ::ffi::llvm_Instruction_CastOps::FPToUI => CastOps::FPToUI,
            ::ffi::llvm_Instruction_CastOps::FPToSI => CastOps::FPToSI,
            ::ffi::llvm_Instruction_CastOps::UIToFP => CastOps::UIToFP,
            ::ffi::llvm_Instruction_CastOps::SIToFP => CastOps::SIToFP,
            ::ffi::llvm_Instruction_CastOps::FPTrunc => CastOps::FPTrunc,
            ::ffi::llvm_Instruction_CastOps::FPExt => CastOps::FPExt,
            ::ffi::llvm_Instruction_CastOps::PtrToInt => CastOps::PtrToInt,
            ::ffi::llvm_Instruction_CastOps::IntToPtr => CastOps::IntToPtr,
            ::ffi::llvm_Instruction_CastOps::BitCast => CastOps::BitCast,
            ::ffi::llvm_Instruction_CastOps::AddrSpaceCast => CastOps::AddrSpaceCast,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Instruction_CastOps {
        match self {
            CastOps::Trunc => ::ffi::llvm_Instruction_CastOps::Trunc,
            CastOps::ZExt => ::ffi::llvm_Instruction_CastOps::ZExt,
            CastOps::SExt => ::ffi::llvm_Instruction_CastOps::SExt,
            CastOps::FPToUI => ::ffi::llvm_Instruction_CastOps::FPToUI,
            CastOps::FPToSI => ::ffi::llvm_Instruction_CastOps::FPToSI,
            CastOps::UIToFP => ::ffi::llvm_Instruction_CastOps::UIToFP,
            CastOps::SIToFP => ::ffi::llvm_Instruction_CastOps::SIToFP,
            CastOps::FPTrunc => ::ffi::llvm_Instruction_CastOps::FPTrunc,
            CastOps::FPExt => ::ffi::llvm_Instruction_CastOps::FPExt,
            CastOps::PtrToInt => ::ffi::llvm_Instruction_CastOps::PtrToInt,
            CastOps::IntToPtr => ::ffi::llvm_Instruction_CastOps::IntToPtr,
            CastOps::BitCast => ::ffi::llvm_Instruction_CastOps::BitCast,
            CastOps::AddrSpaceCast => ::ffi::llvm_Instruction_CastOps::AddrSpaceCast,
        }
    }
}
impl Copy for CastOps {}
pub enum MemoryOps {
    Alloca,
    Load,
    Store,
    GetElementPtr,
    Fence,
    AtomicCmpXchg,
    AtomicRMW,
}
impl MemoryOps {
    pub fn from_ffi(value: ::ffi::llvm_Instruction_MemoryOps) -> MemoryOps {
        match value {
            ::ffi::llvm_Instruction_MemoryOps::Alloca => MemoryOps::Alloca,
            ::ffi::llvm_Instruction_MemoryOps::Load => MemoryOps::Load,
            ::ffi::llvm_Instruction_MemoryOps::Store => MemoryOps::Store,
            ::ffi::llvm_Instruction_MemoryOps::GetElementPtr => MemoryOps::GetElementPtr,
            ::ffi::llvm_Instruction_MemoryOps::Fence => MemoryOps::Fence,
            ::ffi::llvm_Instruction_MemoryOps::AtomicCmpXchg => MemoryOps::AtomicCmpXchg,
            ::ffi::llvm_Instruction_MemoryOps::AtomicRMW => MemoryOps::AtomicRMW,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Instruction_MemoryOps {
        match self {
            MemoryOps::Alloca => ::ffi::llvm_Instruction_MemoryOps::Alloca,
            MemoryOps::Load => ::ffi::llvm_Instruction_MemoryOps::Load,
            MemoryOps::Store => ::ffi::llvm_Instruction_MemoryOps::Store,
            MemoryOps::GetElementPtr => ::ffi::llvm_Instruction_MemoryOps::GetElementPtr,
            MemoryOps::Fence => ::ffi::llvm_Instruction_MemoryOps::Fence,
            MemoryOps::AtomicCmpXchg => ::ffi::llvm_Instruction_MemoryOps::AtomicCmpXchg,
            MemoryOps::AtomicRMW => ::ffi::llvm_Instruction_MemoryOps::AtomicRMW,
        }
    }
}
impl Copy for MemoryOps {}
pub enum OtherOps {
    ICmp,
    FCmp,
    PHI,
    Call,
    Select,
    UserOp1,
    UserOp2,
    VAArg,
    ExtractElement,
    InsertElement,
    ShuffleVector,
    ExtractValue,
    InsertValue,
    LandingPad,
}
impl OtherOps {
    pub fn from_ffi(value: ::ffi::llvm_Instruction_OtherOps) -> OtherOps {
        match value {
            ::ffi::llvm_Instruction_OtherOps::ICmp => OtherOps::ICmp,
            ::ffi::llvm_Instruction_OtherOps::FCmp => OtherOps::FCmp,
            ::ffi::llvm_Instruction_OtherOps::PHI => OtherOps::PHI,
            ::ffi::llvm_Instruction_OtherOps::Call => OtherOps::Call,
            ::ffi::llvm_Instruction_OtherOps::Select => OtherOps::Select,
            ::ffi::llvm_Instruction_OtherOps::UserOp1 => OtherOps::UserOp1,
            ::ffi::llvm_Instruction_OtherOps::UserOp2 => OtherOps::UserOp2,
            ::ffi::llvm_Instruction_OtherOps::VAArg => OtherOps::VAArg,
            ::ffi::llvm_Instruction_OtherOps::ExtractElement => OtherOps::ExtractElement,
            ::ffi::llvm_Instruction_OtherOps::InsertElement => OtherOps::InsertElement,
            ::ffi::llvm_Instruction_OtherOps::ShuffleVector => OtherOps::ShuffleVector,
            ::ffi::llvm_Instruction_OtherOps::ExtractValue => OtherOps::ExtractValue,
            ::ffi::llvm_Instruction_OtherOps::InsertValue => OtherOps::InsertValue,
            ::ffi::llvm_Instruction_OtherOps::LandingPad => OtherOps::LandingPad,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Instruction_OtherOps {
        match self {
            OtherOps::ICmp => ::ffi::llvm_Instruction_OtherOps::ICmp,
            OtherOps::FCmp => ::ffi::llvm_Instruction_OtherOps::FCmp,
            OtherOps::PHI => ::ffi::llvm_Instruction_OtherOps::PHI,
            OtherOps::Call => ::ffi::llvm_Instruction_OtherOps::Call,
            OtherOps::Select => ::ffi::llvm_Instruction_OtherOps::Select,
            OtherOps::UserOp1 => ::ffi::llvm_Instruction_OtherOps::UserOp1,
            OtherOps::UserOp2 => ::ffi::llvm_Instruction_OtherOps::UserOp2,
            OtherOps::VAArg => ::ffi::llvm_Instruction_OtherOps::VAArg,
            OtherOps::ExtractElement => ::ffi::llvm_Instruction_OtherOps::ExtractElement,
            OtherOps::InsertElement => ::ffi::llvm_Instruction_OtherOps::InsertElement,
            OtherOps::ShuffleVector => ::ffi::llvm_Instruction_OtherOps::ShuffleVector,
            OtherOps::ExtractValue => ::ffi::llvm_Instruction_OtherOps::ExtractValue,
            OtherOps::InsertValue => ::ffi::llvm_Instruction_OtherOps::InsertValue,
            OtherOps::LandingPad => ::ffi::llvm_Instruction_OtherOps::LandingPad,
        }
    }
}
impl Copy for OtherOps {}
pub enum TermOps {
    Ret,
    Br,
    Switch,
    IndirectBr,
    Invoke,
    Resume,
    Unreachable,
}
impl TermOps {
    pub fn from_ffi(value: ::ffi::llvm_Instruction_TermOps) -> TermOps {
        match value {
            ::ffi::llvm_Instruction_TermOps::Ret => TermOps::Ret,
            ::ffi::llvm_Instruction_TermOps::Br => TermOps::Br,
            ::ffi::llvm_Instruction_TermOps::Switch => TermOps::Switch,
            ::ffi::llvm_Instruction_TermOps::IndirectBr => TermOps::IndirectBr,
            ::ffi::llvm_Instruction_TermOps::Invoke => TermOps::Invoke,
            ::ffi::llvm_Instruction_TermOps::Resume => TermOps::Resume,
            ::ffi::llvm_Instruction_TermOps::Unreachable => TermOps::Unreachable,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Instruction_TermOps {
        match self {
            TermOps::Ret => ::ffi::llvm_Instruction_TermOps::Ret,
            TermOps::Br => ::ffi::llvm_Instruction_TermOps::Br,
            TermOps::Switch => ::ffi::llvm_Instruction_TermOps::Switch,
            TermOps::IndirectBr => ::ffi::llvm_Instruction_TermOps::IndirectBr,
            TermOps::Invoke => ::ffi::llvm_Instruction_TermOps::Invoke,
            TermOps::Resume => ::ffi::llvm_Instruction_TermOps::Resume,
            TermOps::Unreachable => ::ffi::llvm_Instruction_TermOps::Unreachable,
        }
    }
}
impl Copy for TermOps {}
pub type InstructionInner = ::ffi::llvm_Instruction;

pub trait InstructionObj: ::llvm::value::user::UserObj {
    unsafe fn get_inner(&self) -> *mut InstructionInner;
}

pub trait InstructionOwned: InstructionObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut InstructionInner {
        let inner = InstructionObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> InstructionOwned for T where T: InstructionObj + ::core::marker::Sized {}

pub trait InstructionExt: InstructionObj {

    fn clone(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_clone(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn copy_fast_math_flags<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&mut self, inst: &A1) {
        unsafe {
            ::ffi::llvm::Instruction_copyFastMathFlags(::llvm::value::user::inst::InstructionObj::get_inner(self), ::llvm::value::user::inst::InstructionObj::get_inner(inst));
        }
    }

    fn drop_unknown_metadata(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_dropUnknownMetadata(::llvm::value::user::inst::InstructionObj::get_inner(self));
        }
    }

    fn drop_unknown_metadata_from_ids(&mut self, known_i_ds: &[u32]) {
        unsafe {
            let c_known_i_ds = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: known_i_ds.as_ptr(),
                length: known_i_ds.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_dropUnknownMetadataFromIDS(::llvm::value::user::inst::InstructionObj::get_inner(self), c_known_i_ds);
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_eraseFromParent(::llvm::value::user::inst::InstructionObj::get_inner(self));
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDataLayout(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_debug_loc(&self) -> ::llvm::DebugLoc {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDebugLoc(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ::llvm::DebugLoc::from_inner(ret as *mut ::ffi::llvm_DebugLoc)
        }
    }

    fn get_metadata(&self, kind_id: u32) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getMetadata(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, kind_id as ::libc::c_uint);
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
            let ret = ::ffi::llvm::Instruction_getMetadataStr(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, c_kind);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getOpcode(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction) as u32;
            ret
        }
    }

    fn get_parent(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParent(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParentMut(::llvm::value::user::inst::InstructionObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn has_allow_reciprocal(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasAllowReciprocal(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadata(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata_other_than_debug_loc(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadataOtherThanDebugLoc(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_infs(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoInfs(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_na_ns(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoNaNs(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_signed_zeros(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoSignedZeros(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_unsafe_algebra(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasUnsafeAlgebra(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn insert_after<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&mut self, insert_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_insertAfter(::llvm::value::user::inst::InstructionObj::get_inner(self), ::llvm::value::user::inst::InstructionObj::get_inner(insert_pos));
        }
    }

    fn insert_before<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&mut self, insert_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_insertBefore(::llvm::value::user::inst::InstructionObj::get_inner(self), ::llvm::value::user::inst::InstructionObj::get_inner(insert_pos));
        }
    }

    fn is_arithmetic_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isArithmeticShift(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_associative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isAssociative(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_binary_op(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isBinaryOp(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_cast(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCast(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_commutative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCommutative(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_idempotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdempotent(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_identical_to<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&self, inst: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalTo(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::get_inner(inst));
            ret
        }
    }

    fn is_identical_to_when_defined<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&self, inst: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalToWhenDefined(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::get_inner(inst));
            ret
        }
    }

    fn is_logical_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isLogicalShift(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_nilpotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isNilpotent(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_same_operation_as<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&self, inst: &A1, flags: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isSameOperationAs(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::get_inner(inst), flags as ::libc::c_uint);
            ret
        }
    }

    fn is_shift(&mut self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isShift(::llvm::value::user::inst::InstructionObj::get_inner(self));
            ret
        }
    }

    fn is_terminator(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isTerminator(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_used_outside_of_block<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&self, bb: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isUsedOutsideOfBlock(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::BasicBlockObj::get_inner(bb));
            ret
        }
    }

    fn may_have_side_effects(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayHaveSideEffects(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_from_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadFromMemory(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_or_write_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadOrWriteMemory(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_return(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReturn(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_throw(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayThrow(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_write_to_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayWriteToMemory(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn move_before<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&mut self, move_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_moveBefore(::llvm::value::user::inst::InstructionObj::get_inner(self), ::llvm::value::user::inst::InstructionObj::get_inner(move_pos));
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_removeFromParent(::llvm::value::user::inst::InstructionObj::get_inner(self));
        }
    }

    fn set_debug_loc<A1: ::llvm::DebugLocObj = ::llvm::DebugLoc>(&mut self, loc: &A1) {
        unsafe {
            ::ffi::llvm::Instruction_setDebugLoc(::llvm::value::user::inst::InstructionObj::get_inner(self), ::llvm::DebugLocObj::get_inner(loc));
        }
    }

    fn set_has_allow_reciprocal(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasAllowReciprocal(::llvm::value::user::inst::InstructionObj::get_inner(self), val);
        }
    }

    fn set_has_no_infs(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoInfs(::llvm::value::user::inst::InstructionObj::get_inner(self), val);
        }
    }

    fn set_has_no_na_ns(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoNaNs(::llvm::value::user::inst::InstructionObj::get_inner(self), val);
        }
    }

    fn set_has_no_signed_zeros(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoSignedZeros(::llvm::value::user::inst::InstructionObj::get_inner(self), val);
        }
    }

    fn set_has_unsafe_algebra(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasUnsafeAlgebra(::llvm::value::user::inst::InstructionObj::get_inner(self), val);
        }
    }

    fn set_metadata<A2: ::llvm::value::MDNodeObj = ::llvm::value::MDNode>(&mut self, kind_id: u32, node: &mut A2) {
        unsafe {
            ::ffi::llvm::Instruction_setMetadata(::llvm::value::user::inst::InstructionObj::get_inner(self), kind_id as ::libc::c_uint, ::llvm::value::MDNodeObj::get_inner(node));
        }
    }

    fn set_metadata_str<A2: ::llvm::value::MDNodeObj = ::llvm::value::MDNode>(&mut self, kind: &str, node: &mut A2) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_setMetadataStr(::llvm::value::user::inst::InstructionObj::get_inner(self), c_kind, ::llvm::value::MDNodeObj::get_inner(node));
        }
    }

    fn user_back(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back(::llvm::value::user::inst::InstructionObj::get_inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn user_back_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back_mut(::llvm::value::user::inst::InstructionObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }
}
impl<T> InstructionExt for T where T: InstructionObj {}

pub struct Instruction {
    inner: ::core::nonzero::NonZero<*mut InstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for Instruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Instruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InstructionObj for Instruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut InstructionInner {
        *self.inner
    }
}
impl Instruction {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut InstructionInner, owned: bool) -> Instruction {
        Instruction {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Instruction {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut InvokeInstInner;
}

pub trait InvokeInstOwned: InvokeInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut InvokeInstInner {
        let inner = InvokeInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> InvokeInstOwned for T where T: InvokeInstObj + ::core::marker::Sized {}

pub trait InvokeInstExt: InvokeInstObj {
}
impl<T> InvokeInstExt for T where T: InvokeInstObj {}

pub struct InvokeInst {
    inner: ::core::nonzero::NonZero<*mut InvokeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InvokeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InvokeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InvokeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for InvokeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InvokeInstObj for InvokeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut InvokeInstInner {
        *self.inner
    }
}
impl InvokeInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut InvokeInstInner, owned: bool) -> InvokeInst {
        InvokeInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InvokeInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut LandingPadInstInner;
}

pub trait LandingPadInstOwned: LandingPadInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut LandingPadInstInner {
        let inner = LandingPadInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> LandingPadInstOwned for T where T: LandingPadInstObj + ::core::marker::Sized {}

pub trait LandingPadInstExt: LandingPadInstObj {
}
impl<T> LandingPadInstExt for T where T: LandingPadInstObj {}

pub struct LandingPadInst {
    inner: ::core::nonzero::NonZero<*mut LandingPadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for LandingPadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for LandingPadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for LandingPadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LandingPadInstObj for LandingPadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut LandingPadInstInner {
        *self.inner
    }
}
impl LandingPadInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut LandingPadInstInner, owned: bool) -> LandingPadInst {
        LandingPadInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for LandingPadInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    unsafe fn get_inner(&self) -> *mut LoadInstInner;
}

pub trait LoadInstOwned: LoadInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut LoadInstInner {
        let inner = LoadInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> LoadInstOwned for T where T: LoadInstObj + ::core::marker::Sized {}

pub trait LoadInstExt: LoadInstObj {
}
impl<T> LoadInstExt for T where T: LoadInstObj {}

pub struct LoadInst {
    inner: ::core::nonzero::NonZero<*mut LoadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for LoadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for LoadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for LoadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for LoadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LoadInstObj for LoadInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut LoadInstInner {
        *self.inner
    }
}
impl LoadInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut LoadInstInner, owned: bool) -> LoadInst {
        LoadInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for LoadInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut PHINodeInner;
}

pub trait PHINodeOwned: PHINodeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut PHINodeInner {
        let inner = PHINodeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> PHINodeOwned for T where T: PHINodeObj + ::core::marker::Sized {}

pub trait PHINodeExt: PHINodeObj {
}
impl<T> PHINodeExt for T where T: PHINodeObj {}

pub struct PHINode {
    inner: ::core::nonzero::NonZero<*mut PHINodeInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for PHINode {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for PHINode {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for PHINode {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PHINodeObj for PHINode {
    #[inline(always)]
    fn get_inner(&self) -> *mut PHINodeInner {
        *self.inner
    }
}
impl PHINode {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut PHINodeInner, owned: bool) -> PHINode {
        PHINode {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for PHINode {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut ResumeInstInner;
}

pub trait ResumeInstOwned: ResumeInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ResumeInstInner {
        let inner = ResumeInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ResumeInstOwned for T where T: ResumeInstObj + ::core::marker::Sized {}

pub trait ResumeInstExt: ResumeInstObj {
}
impl<T> ResumeInstExt for T where T: ResumeInstObj {}

pub struct ResumeInst {
    inner: ::core::nonzero::NonZero<*mut ResumeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ResumeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ResumeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ResumeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for ResumeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ResumeInstObj for ResumeInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ResumeInstInner {
        *self.inner
    }
}
impl ResumeInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ResumeInstInner, owned: bool) -> ResumeInst {
        ResumeInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ResumeInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut ReturnInstInner;
}

pub trait ReturnInstOwned: ReturnInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ReturnInstInner {
        let inner = ReturnInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ReturnInstOwned for T where T: ReturnInstObj + ::core::marker::Sized {}

pub trait ReturnInstExt: ReturnInstObj {
}
impl<T> ReturnInstExt for T where T: ReturnInstObj {}

pub struct ReturnInst {
    inner: ::core::nonzero::NonZero<*mut ReturnInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ReturnInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ReturnInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ReturnInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for ReturnInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ReturnInstObj for ReturnInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ReturnInstInner {
        *self.inner
    }
}
impl ReturnInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ReturnInstInner, owned: bool) -> ReturnInst {
        ReturnInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ReturnInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut SelectInstInner;
}

pub trait SelectInstOwned: SelectInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut SelectInstInner {
        let inner = SelectInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> SelectInstOwned for T where T: SelectInstObj + ::core::marker::Sized {}

pub trait SelectInstExt: SelectInstObj {
}
impl<T> SelectInstExt for T where T: SelectInstObj {}

pub struct SelectInst {
    inner: ::core::nonzero::NonZero<*mut SelectInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for SelectInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for SelectInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for SelectInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SelectInstObj for SelectInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut SelectInstInner {
        *self.inner
    }
}
impl SelectInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut SelectInstInner, owned: bool) -> SelectInst {
        SelectInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for SelectInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut ShuffleVectorInstInner;
}

pub trait ShuffleVectorInstOwned: ShuffleVectorInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ShuffleVectorInstInner {
        let inner = ShuffleVectorInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ShuffleVectorInstOwned for T where T: ShuffleVectorInstObj + ::core::marker::Sized {}

pub trait ShuffleVectorInstExt: ShuffleVectorInstObj {
}
impl<T> ShuffleVectorInstExt for T where T: ShuffleVectorInstObj {}

pub struct ShuffleVectorInst {
    inner: ::core::nonzero::NonZero<*mut ShuffleVectorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ShuffleVectorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ShuffleVectorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ShuffleVectorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ShuffleVectorInstObj for ShuffleVectorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ShuffleVectorInstInner {
        *self.inner
    }
}
impl ShuffleVectorInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ShuffleVectorInstInner, owned: bool) -> ShuffleVectorInst {
        ShuffleVectorInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ShuffleVectorInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut StoreInstInner;
}

pub trait StoreInstOwned: StoreInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut StoreInstInner {
        let inner = StoreInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> StoreInstOwned for T where T: StoreInstObj + ::core::marker::Sized {}

pub trait StoreInstExt: StoreInstObj {
}
impl<T> StoreInstExt for T where T: StoreInstObj {}

pub struct StoreInst {
    inner: ::core::nonzero::NonZero<*mut StoreInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for StoreInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for StoreInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for StoreInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl StoreInstObj for StoreInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut StoreInstInner {
        *self.inner
    }
}
impl StoreInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut StoreInstInner, owned: bool) -> StoreInst {
        StoreInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for StoreInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut SwitchInstInner;
}

pub trait SwitchInstOwned: SwitchInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut SwitchInstInner {
        let inner = SwitchInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> SwitchInstOwned for T where T: SwitchInstObj + ::core::marker::Sized {}

pub trait SwitchInstExt: SwitchInstObj {
}
impl<T> SwitchInstExt for T where T: SwitchInstObj {}

pub struct SwitchInst {
    inner: ::core::nonzero::NonZero<*mut SwitchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for SwitchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for SwitchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for SwitchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for SwitchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SwitchInstObj for SwitchInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut SwitchInstInner {
        *self.inner
    }
}
impl SwitchInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut SwitchInstInner, owned: bool) -> SwitchInst {
        SwitchInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for SwitchInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut TerminatorInstInner;
}

pub trait TerminatorInstOwned: TerminatorInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut TerminatorInstInner {
        let inner = TerminatorInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> TerminatorInstOwned for T where T: TerminatorInstObj + ::core::marker::Sized {}

pub trait TerminatorInstExt: TerminatorInstObj {
}
impl<T> TerminatorInstExt for T where T: TerminatorInstObj {}

pub struct TerminatorInst {
    inner: ::core::nonzero::NonZero<*mut TerminatorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for TerminatorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for TerminatorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for TerminatorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl TerminatorInstObj for TerminatorInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut TerminatorInstInner {
        *self.inner
    }
}
impl TerminatorInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut TerminatorInstInner, owned: bool) -> TerminatorInst {
        TerminatorInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for TerminatorInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionObj: ::llvm::value::user::inst::InstructionObj {
    unsafe fn get_inner(&self) -> *mut UnaryInstructionInner;
}

pub trait UnaryInstructionOwned: UnaryInstructionObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut UnaryInstructionInner {
        let inner = UnaryInstructionObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> UnaryInstructionOwned for T where T: UnaryInstructionObj + ::core::marker::Sized {}

pub trait UnaryInstructionExt: UnaryInstructionObj {
}
impl<T> UnaryInstructionExt for T where T: UnaryInstructionObj {}

pub struct UnaryInstruction {
    inner: ::core::nonzero::NonZero<*mut UnaryInstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for UnaryInstruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for UnaryInstruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for UnaryInstruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnaryInstructionObj for UnaryInstruction {
    #[inline(always)]
    fn get_inner(&self) -> *mut UnaryInstructionInner {
        *self.inner
    }
}
impl UnaryInstruction {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut UnaryInstructionInner, owned: bool) -> UnaryInstruction {
        UnaryInstruction {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for UnaryInstruction {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    unsafe fn get_inner(&self) -> *mut UnreachableInstInner;
}

pub trait UnreachableInstOwned: UnreachableInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut UnreachableInstInner {
        let inner = UnreachableInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> UnreachableInstOwned for T where T: UnreachableInstObj + ::core::marker::Sized {}

pub trait UnreachableInstExt: UnreachableInstObj {
}
impl<T> UnreachableInstExt for T where T: UnreachableInstObj {}

pub struct UnreachableInst {
    inner: ::core::nonzero::NonZero<*mut UnreachableInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for UnreachableInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for UnreachableInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for UnreachableInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for UnreachableInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnreachableInstObj for UnreachableInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut UnreachableInstInner {
        *self.inner
    }
}
impl UnreachableInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut UnreachableInstInner, owned: bool) -> UnreachableInst {
        UnreachableInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for UnreachableInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    unsafe fn get_inner(&self) -> *mut VAArgInstInner;
}

pub trait VAArgInstOwned: VAArgInstObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut VAArgInstInner {
        let inner = VAArgInstObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> VAArgInstOwned for T where T: VAArgInstObj + ::core::marker::Sized {}

pub trait VAArgInstExt: VAArgInstObj {
}
impl<T> VAArgInstExt for T where T: VAArgInstObj {}

pub struct VAArgInst {
    inner: ::core::nonzero::NonZero<*mut VAArgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for VAArgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for VAArgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for VAArgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for VAArgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VAArgInstObj for VAArgInst {
    #[inline(always)]
    fn get_inner(&self) -> *mut VAArgInstInner {
        *self.inner
    }
}
impl VAArgInst {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut VAArgInstInner, owned: bool) -> VAArgInst {
        VAArgInst {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for VAArgInst {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::get_inner(self));
            }
        }
    }
}
