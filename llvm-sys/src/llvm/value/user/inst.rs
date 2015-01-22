pub type AddrSpaceCastInstInner = ::ffi::llvm_AddrSpaceCastInst;

pub trait AddrSpaceCastInstObj: ::llvm::value::user::inst::CastInstObj {
    fn inner(&self) -> *mut AddrSpaceCastInstInner;
}

pub trait AddrSpaceCastInstExt: AddrSpaceCastInstObj {
}
impl<T> AddrSpaceCastInstExt for T where T: AddrSpaceCastInstObj {}

pub struct AddrSpaceCastInst {
    inner: ::core::nonzero::NonZero<*mut AddrSpaceCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for AddrSpaceCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AddrSpaceCastInstObj for AddrSpaceCastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type AllocaInstInner = ::ffi::llvm_AllocaInst;

pub trait AllocaInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    fn inner(&self) -> *mut AllocaInstInner;
}

pub trait AllocaInstExt: AllocaInstObj {
}
impl<T> AllocaInstExt for T where T: AllocaInstObj {}

pub struct AllocaInst {
    inner: ::core::nonzero::NonZero<*mut AllocaInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for AllocaInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AllocaInstObj for AllocaInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type AtomicCmpXchgInstInner = ::ffi::llvm_AtomicCmpXchgInst;

pub trait AtomicCmpXchgInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut AtomicCmpXchgInstInner;
}

pub trait AtomicCmpXchgInstExt: AtomicCmpXchgInstObj {
}
impl<T> AtomicCmpXchgInstExt for T where T: AtomicCmpXchgInstObj {}

pub struct AtomicCmpXchgInst {
    inner: ::core::nonzero::NonZero<*mut AtomicCmpXchgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AtomicCmpXchgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicCmpXchgInstObj for AtomicCmpXchgInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type AtomicRMWInstInner = ::ffi::llvm_AtomicRMWInst;

pub trait AtomicRMWInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut AtomicRMWInstInner;
}

pub trait AtomicRMWInstExt: AtomicRMWInstObj {
}
impl<T> AtomicRMWInstExt for T where T: AtomicRMWInstObj {}

pub struct AtomicRMWInst {
    inner: ::core::nonzero::NonZero<*mut AtomicRMWInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for AtomicRMWInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl AtomicRMWInstObj for AtomicRMWInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type BinaryOperatorInner = ::ffi::llvm_BinaryOperator;

pub trait BinaryOperatorObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut BinaryOperatorInner;
}

pub trait BinaryOperatorExt: BinaryOperatorObj {
}
impl<T> BinaryOperatorExt for T where T: BinaryOperatorObj {}

pub struct BinaryOperator {
    inner: ::core::nonzero::NonZero<*mut BinaryOperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BinaryOperator {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BinaryOperatorObj for BinaryOperator {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type BitCastInstInner = ::ffi::llvm_BitCastInst;

pub trait BitCastInstObj: ::llvm::value::user::inst::CastInstObj {
    fn inner(&self) -> *mut BitCastInstInner;
}

pub trait BitCastInstExt: BitCastInstObj {
}
impl<T> BitCastInstExt for T where T: BitCastInstObj {}

pub struct BitCastInst {
    inner: ::core::nonzero::NonZero<*mut BitCastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for BitCastInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BitCastInstObj for BitCastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type BranchInstInner = ::ffi::llvm_BranchInst;

pub trait BranchInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut BranchInstInner;
}

pub trait BranchInstExt: BranchInstObj {
}
impl<T> BranchInstExt for T where T: BranchInstObj {}

pub struct BranchInst {
    inner: ::core::nonzero::NonZero<*mut BranchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for BranchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BranchInstObj for BranchInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type CallInstInner = ::ffi::llvm_CallInst;

pub trait CallInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut CallInstInner;
}

pub trait CallInstExt: CallInstObj {
}
impl<T> CallInstExt for T where T: CallInstObj {}

pub struct CallInst {
    inner: ::core::nonzero::NonZero<*mut CallInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CallInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CallInstObj for CallInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type CastInstInner = ::ffi::llvm_CastInst;

pub trait CastInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    fn inner(&self) -> *mut CastInstInner;
}

pub trait CastInstExt: CastInstObj {
}
impl<T> CastInstExt for T where T: CastInstObj {}

pub struct CastInst {
    inner: ::core::nonzero::NonZero<*mut CastInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for CastInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CastInstObj for CastInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
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
    fn inner(&self) -> *mut CmpInstInner;
}

pub trait CmpInstExt: CmpInstObj {
}
impl<T> CmpInstExt for T where T: CmpInstObj {}

pub struct CmpInst {
    inner: ::core::nonzero::NonZero<*mut CmpInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for CmpInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CmpInstObj for CmpInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type ExtractElementInstInner = ::ffi::llvm_ExtractElementInst;

pub trait ExtractElementInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut ExtractElementInstInner;
}

pub trait ExtractElementInstExt: ExtractElementInstObj {
}
impl<T> ExtractElementInstExt for T where T: ExtractElementInstObj {}

pub struct ExtractElementInst {
    inner: ::core::nonzero::NonZero<*mut ExtractElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ExtractElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractElementInstObj for ExtractElementInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type ExtractValueInstInner = ::ffi::llvm_ExtractValueInst;

pub trait ExtractValueInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    fn inner(&self) -> *mut ExtractValueInstInner;
}

pub trait ExtractValueInstExt: ExtractValueInstObj {
}
impl<T> ExtractValueInstExt for T where T: ExtractValueInstObj {}

pub struct ExtractValueInst {
    inner: ::core::nonzero::NonZero<*mut ExtractValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for ExtractValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ExtractValueInstObj for ExtractValueInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type FPExtInstInner = ::ffi::llvm_FPExtInst;

pub trait FPExtInstObj: ::llvm::value::user::inst::CastInstObj {
    fn inner(&self) -> *mut FPExtInstInner;
}

pub trait FPExtInstExt: FPExtInstObj {
}
impl<T> FPExtInstExt for T where T: FPExtInstObj {}

pub struct FPExtInst {
    inner: ::core::nonzero::NonZero<*mut FPExtInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for FPExtInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPExtInstObj for FPExtInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type FPToSIInstInner = ::ffi::llvm_FPToSIInst;

pub trait FPToSIInstObj: ::llvm::value::user::inst::CastInstObj {
    fn inner(&self) -> *mut FPToSIInstInner;
}

pub trait FPToSIInstExt: FPToSIInstObj {
}
impl<T> FPToSIInstExt for T where T: FPToSIInstObj {}

pub struct FPToSIInst {
    inner: ::core::nonzero::NonZero<*mut FPToSIInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::CastInstObj for FPToSIInst {
    fn inner(&self) -> *mut ::ffi::llvm_CastInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FPToSIInstObj for FPToSIInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type FenceInstInner = ::ffi::llvm_FenceInst;

pub trait FenceInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut FenceInstInner;
}

pub trait FenceInstExt: FenceInstObj {
}
impl<T> FenceInstExt for T where T: FenceInstObj {}

pub struct FenceInst {
    inner: ::core::nonzero::NonZero<*mut FenceInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for FenceInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FenceInstObj for FenceInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type GetElementPtrInstInner = ::ffi::llvm_GetElementPtrInst;

pub trait GetElementPtrInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut GetElementPtrInstInner;
}

pub trait GetElementPtrInstExt: GetElementPtrInstObj {
}
impl<T> GetElementPtrInstExt for T where T: GetElementPtrInstObj {}

pub struct GetElementPtrInst {
    inner: ::core::nonzero::NonZero<*mut GetElementPtrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for GetElementPtrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GetElementPtrInstObj for GetElementPtrInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type IndirectBrInstInner = ::ffi::llvm_IndirectBrInst;

pub trait IndirectBrInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut IndirectBrInstInner;
}

pub trait IndirectBrInstExt: IndirectBrInstObj {
}
impl<T> IndirectBrInstExt for T where T: IndirectBrInstObj {}

pub struct IndirectBrInst {
    inner: ::core::nonzero::NonZero<*mut IndirectBrInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for IndirectBrInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IndirectBrInstObj for IndirectBrInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type InsertElementInstInner = ::ffi::llvm_InsertElementInst;

pub trait InsertElementInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut InsertElementInstInner;
}

pub trait InsertElementInstExt: InsertElementInstObj {
}
impl<T> InsertElementInstExt for T where T: InsertElementInstObj {}

pub struct InsertElementInst {
    inner: ::core::nonzero::NonZero<*mut InsertElementInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InsertElementInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertElementInstObj for InsertElementInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type InsertValueInstInner = ::ffi::llvm_InsertValueInst;

pub trait InsertValueInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut InsertValueInstInner;
}

pub trait InsertValueInstExt: InsertValueInstObj {
}
impl<T> InsertValueInstExt for T where T: InsertValueInstObj {}

pub struct InsertValueInst {
    inner: ::core::nonzero::NonZero<*mut InsertValueInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InsertValueInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InsertValueInstObj for InsertValueInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
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
    fn inner(&self) -> *mut InstructionInner;
}

pub trait InstructionExt: InstructionObj {

    fn clone(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_clone(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn copy_fast_math_flags<A1: ::llvm::value::user::inst::InstructionObj>(&mut self, inst: &A1) {
        unsafe {
            ::ffi::llvm::Instruction_copyFastMathFlags(::llvm::value::user::inst::InstructionObj::inner(self), ::llvm::value::user::inst::InstructionObj::inner(inst));
        }
    }

    fn drop_unknown_metadata(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_dropUnknownMetadata(::llvm::value::user::inst::InstructionObj::inner(self));
        }
    }

    fn drop_unknown_metadata_from_ids(&mut self, known_i_ds: &[u32]) {
        unsafe {
            let c_known_i_ds = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: known_i_ds.as_ptr(),
                length: known_i_ds.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_dropUnknownMetadataFromIDS(::llvm::value::user::inst::InstructionObj::inner(self), c_known_i_ds);
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_eraseFromParent(::llvm::value::user::inst::InstructionObj::inner(self));
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDataLayout(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_debug_loc(&self) -> ::llvm::DebugLoc {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getDebugLoc(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ::llvm::DebugLoc::from_inner(ret as *mut ::ffi::llvm_DebugLoc)
        }
    }

    fn get_metadata(&self, kind_id: u32) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getMetadata(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, kind_id as ::libc::c_uint);
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
            let ret = ::ffi::llvm::Instruction_getMetadataStr(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, c_kind);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getOpcode(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction) as u32;
            ret
        }
    }

    fn get_parent(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParent(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_getParentMut(::llvm::value::user::inst::InstructionObj::inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn has_allow_reciprocal(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasAllowReciprocal(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadata(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_metadata_other_than_debug_loc(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasMetadataOtherThanDebugLoc(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_infs(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoInfs(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_na_ns(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoNaNs(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_no_signed_zeros(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasNoSignedZeros(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn has_unsafe_algebra(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_hasUnsafeAlgebra(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn insert_after<A1: ::llvm::value::user::inst::InstructionObj>(&mut self, insert_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_insertAfter(::llvm::value::user::inst::InstructionObj::inner(self), ::llvm::value::user::inst::InstructionObj::inner(insert_pos));
        }
    }

    fn insert_before<A1: ::llvm::value::user::inst::InstructionObj>(&mut self, insert_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_insertBefore(::llvm::value::user::inst::InstructionObj::inner(self), ::llvm::value::user::inst::InstructionObj::inner(insert_pos));
        }
    }

    fn is_arithmetic_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isArithmeticShift(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_associative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isAssociative(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_binary_op(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isBinaryOp(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_cast(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCast(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_commutative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isCommutative(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_idempotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdempotent(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_identical_to<A1: ::llvm::value::user::inst::InstructionObj>(&self, inst: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalTo(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::inner(inst));
            ret
        }
    }

    fn is_identical_to_when_defined<A1: ::llvm::value::user::inst::InstructionObj>(&self, inst: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isIdenticalToWhenDefined(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::inner(inst));
            ret
        }
    }

    fn is_logical_shift(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isLogicalShift(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_nilpotent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isNilpotent(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_same_operation_as<A1: ::llvm::value::user::inst::InstructionObj>(&self, inst: &A1, flags: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isSameOperationAs(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::user::inst::InstructionObj::inner(inst), flags as ::libc::c_uint);
            ret
        }
    }

    fn is_shift(&mut self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isShift(::llvm::value::user::inst::InstructionObj::inner(self));
            ret
        }
    }

    fn is_terminator(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isTerminator(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn is_used_outside_of_block<A1: ::llvm::value::BasicBlockObj>(&self, bb: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_isUsedOutsideOfBlock(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction, ::llvm::value::BasicBlockObj::inner(bb));
            ret
        }
    }

    fn may_have_side_effects(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayHaveSideEffects(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_from_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadFromMemory(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_read_or_write_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReadOrWriteMemory(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_return(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayReturn(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_throw(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayThrow(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn may_write_to_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Instruction_mayWriteToMemory(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            ret
        }
    }

    fn move_before<A1: ::llvm::value::user::inst::InstructionObj>(&mut self, move_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::Instruction_moveBefore(::llvm::value::user::inst::InstructionObj::inner(self), ::llvm::value::user::inst::InstructionObj::inner(move_pos));
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Instruction_removeFromParent(::llvm::value::user::inst::InstructionObj::inner(self));
        }
    }

    fn set_debug_loc<A1: ::llvm::DebugLocObj>(&mut self, loc: &A1) {
        unsafe {
            ::ffi::llvm::Instruction_setDebugLoc(::llvm::value::user::inst::InstructionObj::inner(self), ::llvm::DebugLocObj::inner(loc));
        }
    }

    fn set_has_allow_reciprocal(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasAllowReciprocal(::llvm::value::user::inst::InstructionObj::inner(self), val);
        }
    }

    fn set_has_no_infs(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoInfs(::llvm::value::user::inst::InstructionObj::inner(self), val);
        }
    }

    fn set_has_no_na_ns(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoNaNs(::llvm::value::user::inst::InstructionObj::inner(self), val);
        }
    }

    fn set_has_no_signed_zeros(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasNoSignedZeros(::llvm::value::user::inst::InstructionObj::inner(self), val);
        }
    }

    fn set_has_unsafe_algebra(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::Instruction_setHasUnsafeAlgebra(::llvm::value::user::inst::InstructionObj::inner(self), val);
        }
    }

    fn set_metadata<A2: ::llvm::value::MDNodeObj>(&mut self, kind_id: u32, node: &mut A2) {
        unsafe {
            ::ffi::llvm::Instruction_setMetadata(::llvm::value::user::inst::InstructionObj::inner(self), kind_id as ::libc::c_uint, ::llvm::value::MDNodeObj::inner(node));
        }
    }

    fn set_metadata_str<A2: ::llvm::value::MDNodeObj>(&mut self, kind: &str, node: &mut A2) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Instruction_setMetadataStr(::llvm::value::user::inst::InstructionObj::inner(self), c_kind, ::llvm::value::MDNodeObj::inner(node));
        }
    }

    fn user_back(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back(::llvm::value::user::inst::InstructionObj::inner(self) as *const ::ffi::llvm_Instruction);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn user_back_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::Instruction_user_back_mut(::llvm::value::user::inst::InstructionObj::inner(self));
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
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Instruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InstructionObj for Instruction {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type InvokeInstInner = ::ffi::llvm_InvokeInst;

pub trait InvokeInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut InvokeInstInner;
}

pub trait InvokeInstExt: InvokeInstObj {
}
impl<T> InvokeInstExt for T where T: InvokeInstObj {}

pub struct InvokeInst {
    inner: ::core::nonzero::NonZero<*mut InvokeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for InvokeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InvokeInstObj for InvokeInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type LandingPadInstInner = ::ffi::llvm_LandingPadInst;

pub trait LandingPadInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut LandingPadInstInner;
}

pub trait LandingPadInstExt: LandingPadInstObj {
}
impl<T> LandingPadInstExt for T where T: LandingPadInstObj {}

pub struct LandingPadInst {
    inner: ::core::nonzero::NonZero<*mut LandingPadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for LandingPadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LandingPadInstObj for LandingPadInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type LoadInstInner = ::ffi::llvm_LoadInst;

pub trait LoadInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    fn inner(&self) -> *mut LoadInstInner;
}

pub trait LoadInstExt: LoadInstObj {
}
impl<T> LoadInstExt for T where T: LoadInstObj {}

pub struct LoadInst {
    inner: ::core::nonzero::NonZero<*mut LoadInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for LoadInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LoadInstObj for LoadInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type PHINodeInner = ::ffi::llvm_PHINode;

pub trait PHINodeObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut PHINodeInner;
}

pub trait PHINodeExt: PHINodeObj {
}
impl<T> PHINodeExt for T where T: PHINodeObj {}

pub struct PHINode {
    inner: ::core::nonzero::NonZero<*mut PHINodeInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for PHINode {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PHINodeObj for PHINode {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type ResumeInstInner = ::ffi::llvm_ResumeInst;

pub trait ResumeInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut ResumeInstInner;
}

pub trait ResumeInstExt: ResumeInstObj {
}
impl<T> ResumeInstExt for T where T: ResumeInstObj {}

pub struct ResumeInst {
    inner: ::core::nonzero::NonZero<*mut ResumeInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for ResumeInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ResumeInstObj for ResumeInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type ReturnInstInner = ::ffi::llvm_ReturnInst;

pub trait ReturnInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut ReturnInstInner;
}

pub trait ReturnInstExt: ReturnInstObj {
}
impl<T> ReturnInstExt for T where T: ReturnInstObj {}

pub struct ReturnInst {
    inner: ::core::nonzero::NonZero<*mut ReturnInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for ReturnInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ReturnInstObj for ReturnInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type SelectInstInner = ::ffi::llvm_SelectInst;

pub trait SelectInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut SelectInstInner;
}

pub trait SelectInstExt: SelectInstObj {
}
impl<T> SelectInstExt for T where T: SelectInstObj {}

pub struct SelectInst {
    inner: ::core::nonzero::NonZero<*mut SelectInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for SelectInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SelectInstObj for SelectInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type ShuffleVectorInstInner = ::ffi::llvm_ShuffleVectorInst;

pub trait ShuffleVectorInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut ShuffleVectorInstInner;
}

pub trait ShuffleVectorInstExt: ShuffleVectorInstObj {
}
impl<T> ShuffleVectorInstExt for T where T: ShuffleVectorInstObj {}

pub struct ShuffleVectorInst {
    inner: ::core::nonzero::NonZero<*mut ShuffleVectorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for ShuffleVectorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ShuffleVectorInstObj for ShuffleVectorInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type StoreInstInner = ::ffi::llvm_StoreInst;

pub trait StoreInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut StoreInstInner;
}

pub trait StoreInstExt: StoreInstObj {
}
impl<T> StoreInstExt for T where T: StoreInstObj {}

pub struct StoreInst {
    inner: ::core::nonzero::NonZero<*mut StoreInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for StoreInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl StoreInstObj for StoreInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type SwitchInstInner = ::ffi::llvm_SwitchInst;

pub trait SwitchInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut SwitchInstInner;
}

pub trait SwitchInstExt: SwitchInstObj {
}
impl<T> SwitchInstExt for T where T: SwitchInstObj {}

pub struct SwitchInst {
    inner: ::core::nonzero::NonZero<*mut SwitchInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for SwitchInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SwitchInstObj for SwitchInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type TerminatorInstInner = ::ffi::llvm_TerminatorInst;

pub trait TerminatorInstObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut TerminatorInstInner;
}

pub trait TerminatorInstExt: TerminatorInstObj {
}
impl<T> TerminatorInstExt for T where T: TerminatorInstObj {}

pub struct TerminatorInst {
    inner: ::core::nonzero::NonZero<*mut TerminatorInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for TerminatorInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl TerminatorInstObj for TerminatorInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type UnaryInstructionInner = ::ffi::llvm_UnaryInstruction;

pub trait UnaryInstructionObj: ::llvm::value::user::inst::InstructionObj {
    fn inner(&self) -> *mut UnaryInstructionInner;
}

pub trait UnaryInstructionExt: UnaryInstructionObj {
}
impl<T> UnaryInstructionExt for T where T: UnaryInstructionObj {}

pub struct UnaryInstruction {
    inner: ::core::nonzero::NonZero<*mut UnaryInstructionInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for UnaryInstruction {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnaryInstructionObj for UnaryInstruction {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type UnreachableInstInner = ::ffi::llvm_UnreachableInst;

pub trait UnreachableInstObj: ::llvm::value::user::inst::TerminatorInstObj {
    fn inner(&self) -> *mut UnreachableInstInner;
}

pub trait UnreachableInstExt: UnreachableInstObj {
}
impl<T> UnreachableInstExt for T where T: UnreachableInstObj {}

pub struct UnreachableInst {
    inner: ::core::nonzero::NonZero<*mut UnreachableInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::TerminatorInstObj for UnreachableInst {
    fn inner(&self) -> *mut ::ffi::llvm_TerminatorInst {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UnreachableInstObj for UnreachableInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
pub type VAArgInstInner = ::ffi::llvm_VAArgInst;

pub trait VAArgInstObj: ::llvm::value::user::inst::UnaryInstructionObj {
    fn inner(&self) -> *mut VAArgInstInner;
}

pub trait VAArgInstExt: VAArgInstObj {
}
impl<T> VAArgInstExt for T where T: VAArgInstObj {}

pub struct VAArgInst {
    inner: ::core::nonzero::NonZero<*mut VAArgInstInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::InstructionObj for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_Instruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::inst::UnaryInstructionObj for VAArgInst {
    fn inner(&self) -> *mut ::ffi::llvm_UnaryInstruction {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VAArgInstObj for VAArgInst {
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
                ::ffi::llvm::Instruction_delete(::llvm::value::user::inst::InstructionObj::inner(self));
            }
        }
    }
}
