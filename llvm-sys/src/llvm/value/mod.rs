pub mod user;
pub type ArgumentInner = ::ffi::llvm_Argument;

pub trait ArgumentObj: ::llvm::value::ValueObj {
    unsafe fn get_inner(&self) -> *mut ArgumentInner;
}

pub trait ArgumentOwned: ArgumentObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ArgumentInner {
        let inner = ArgumentObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ArgumentOwned for T where T: ArgumentObj + ::core::marker::Sized {}

pub trait ArgumentExt: ArgumentObj {
}
impl<T> ArgumentExt for T where T: ArgumentObj {}

pub struct Argument {
    inner: ::core::nonzero::NonZero<*mut ArgumentInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for Argument {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ArgumentObj for Argument {
    #[inline(always)]
    fn get_inner(&self) -> *mut ArgumentInner {
        *self.inner
    }
}
impl Argument {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ArgumentInner, owned: bool) -> Argument {
        Argument {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Argument {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueObj::get_inner(self));
            }
        }
    }
}
pub type BasicBlockInner = ::ffi::llvm_BasicBlock;

pub trait BasicBlockObj: ::llvm::value::ValueObj {
    unsafe fn get_inner(&self) -> *mut BasicBlockInner;
}

pub trait BasicBlockOwned: BasicBlockObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut BasicBlockInner {
        let inner = BasicBlockObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> BasicBlockOwned for T where T: BasicBlockObj + ::core::marker::Sized {}

pub trait BasicBlockExt: BasicBlockObj {

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::BasicBlock_dropAllReferences(::llvm::value::BasicBlockObj::get_inner(self));
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::BasicBlock_eraseFromParent(::llvm::value::BasicBlockObj::get_inner(self));
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getDataLayout(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_first_non_phi(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHI(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn get_first_non_phi_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHIMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn get_first_non_phi_or_dbg(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHIOrDbg(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn get_first_non_phi_or_dbg_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHIOrDbgMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn get_first_non_phi_or_dbg_or_lifetime(&self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHIOrDbgOrLifetime(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret as *mut ::ffi::llvm_Instruction, false))
        }
    }

    fn get_first_non_phi_or_dbg_or_lifetime_mut(&mut self) -> Option<::llvm::value::user::inst::Instruction> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getFirstNonPHIOrDbgOrLifetimeMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::Instruction::from_inner(ret, false))
        }
    }

    fn get_landing_pad_inst(&self) -> Option<::llvm::value::user::inst::LandingPadInst> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getLandingPadInst(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::LandingPadInst::from_inner(ret as *mut ::ffi::llvm_LandingPadInst, false))
        }
    }

    fn get_landing_pad_inst_mut(&mut self) -> Option<::llvm::value::user::inst::LandingPadInst> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getLandingPadInstMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::LandingPadInst::from_inner(ret, false))
        }
    }

    fn get_parent(&self) -> Option<::llvm::value::user::constant::Function> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getParent(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Function::from_inner(ret as *mut ::ffi::llvm_Function, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::value::user::constant::Function> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getParentMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Function::from_inner(ret, false))
        }
    }

    fn get_single_predecessor(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getSinglePredecessor(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_single_predecessor_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getSinglePredecessorMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn get_terminator(&self) -> Option<::llvm::value::user::inst::TerminatorInst> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getTerminator(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::TerminatorInst::from_inner(ret as *mut ::ffi::llvm_TerminatorInst, false))
        }
    }

    fn get_terminator_mut(&mut self) -> Option<::llvm::value::user::inst::TerminatorInst> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getTerminatorMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::TerminatorInst::from_inner(ret, false))
        }
    }

    fn get_unique_predecessor(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getUniquePredecessor(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret as *mut ::ffi::llvm_BasicBlock, false))
        }
    }

    fn get_unique_predecessor_mut(&mut self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getUniquePredecessorMut(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn get_value_symbol_table(&mut self) -> Option<::llvm::ValueSymbolTable> {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_getValueSymbolTable(::llvm::value::BasicBlockObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ValueSymbolTable::from_inner(ret))
        }
    }

    fn has_address_taken(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_hasAddressTaken(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            ret
        }
    }

    fn is_landing_pad(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_isLandingPad(::llvm::value::BasicBlockObj::get_inner(self) as *const ::ffi::llvm_BasicBlock);
            ret
        }
    }

    fn move_after<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, move_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::BasicBlock_moveAfter(::llvm::value::BasicBlockObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(move_pos));
        }
    }

    fn move_before<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, move_pos: &mut A1) {
        unsafe {
            ::ffi::llvm::BasicBlock_moveBefore(::llvm::value::BasicBlockObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(move_pos));
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::BasicBlock_removeFromParent(::llvm::value::BasicBlockObj::get_inner(self));
        }
    }

    fn remove_predecessor<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, pred: &mut A1, dont_delete_useless_ph_is: Option<bool>) {
        unsafe {
            let dont_delete_useless_ph_is = dont_delete_useless_ph_is.unwrap_or(false);
            ::ffi::llvm::BasicBlock_removePredecessor(::llvm::value::BasicBlockObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(pred), dont_delete_useless_ph_is);
        }
    }

    fn replace_successors_phi_uses_with<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, new: &mut A1) {
        unsafe {
            ::ffi::llvm::BasicBlock_replaceSuccessorsPhiUsesWith(::llvm::value::BasicBlockObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(new));
        }
    }
}
impl<T> BasicBlockExt for T where T: BasicBlockObj {}

pub struct BasicBlock {
    inner: ::core::nonzero::NonZero<*mut BasicBlockInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BasicBlock {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BasicBlockObj for BasicBlock {
    #[inline(always)]
    fn get_inner(&self) -> *mut BasicBlockInner {
        *self.inner
    }
}
impl BasicBlock {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut BasicBlockInner, owned: bool) -> BasicBlock {
        BasicBlock {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn create<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext, A3: ::llvm::value::user::constant::FunctionObj = ::llvm::value::user::constant::Function, A4: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(context: &mut A1, name: Option<&str>, parent: Option<&mut A3>, insert_before: Option<&mut A4>) -> ::llvm::value::BasicBlock {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::BasicBlock_Create(::llvm::LLVMContextObj::get_inner(context), c_name, parent.map(|parent| ::llvm::value::user::constant::FunctionObj::get_inner(parent)).unwrap_or(::std::ptr::null_mut()), insert_before.map(|insert_before| ::llvm::value::BasicBlockObj::get_inner(insert_before)).unwrap_or(::std::ptr::null_mut()));
            if ret.is_null() {
                panic!("::llvm::BasicBlock::Create returned a null pointer!");
            }
            ::llvm::value::BasicBlock::from_inner(ret, false)
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(val: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::BasicBlock_classof(::llvm::value::ValueObj::get_inner(val));
            ret
        }
    }
}
impl Drop for BasicBlock {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::BasicBlock_delete(::llvm::value::BasicBlockObj::get_inner(self));
            }
        }
    }
}
pub type InlineAsmInner = ::ffi::llvm_InlineAsm;

pub trait InlineAsmObj: ::llvm::value::ValueObj {
    unsafe fn get_inner(&self) -> *mut InlineAsmInner;
}

pub trait InlineAsmOwned: InlineAsmObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut InlineAsmInner {
        let inner = InlineAsmObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> InlineAsmOwned for T where T: InlineAsmObj + ::core::marker::Sized {}

pub trait InlineAsmExt: InlineAsmObj {
}
impl<T> InlineAsmExt for T where T: InlineAsmObj {}

pub struct InlineAsm {
    inner: ::core::nonzero::NonZero<*mut InlineAsmInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for InlineAsm {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InlineAsmObj for InlineAsm {
    #[inline(always)]
    fn get_inner(&self) -> *mut InlineAsmInner {
        *self.inner
    }
}
impl InlineAsm {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut InlineAsmInner, owned: bool) -> InlineAsm {
        InlineAsm {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InlineAsm {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueObj::get_inner(self));
            }
        }
    }
}
pub type MDNodeInner = ::ffi::llvm_MDNode;

pub trait MDNodeObj: ::llvm::value::ValueObj {
    unsafe fn get_inner(&self) -> *mut MDNodeInner;
}

pub trait MDNodeOwned: MDNodeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut MDNodeInner {
        let inner = MDNodeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> MDNodeOwned for T where T: MDNodeObj + ::core::marker::Sized {}

pub trait MDNodeExt: MDNodeObj {
}
impl<T> MDNodeExt for T where T: MDNodeObj {}

pub struct MDNode {
    inner: ::core::nonzero::NonZero<*mut MDNodeInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for MDNode {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl MDNodeObj for MDNode {
    #[inline(always)]
    fn get_inner(&self) -> *mut MDNodeInner {
        *self.inner
    }
}
impl MDNode {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut MDNodeInner, owned: bool) -> MDNode {
        MDNode {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for MDNode {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueObj::get_inner(self));
            }
        }
    }
}
pub type MDStringInner = ::ffi::llvm_MDString;

pub trait MDStringObj: ::llvm::value::ValueObj {
    unsafe fn get_inner(&self) -> *mut MDStringInner;
}

pub trait MDStringOwned: MDStringObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut MDStringInner {
        let inner = MDStringObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> MDStringOwned for T where T: MDStringObj + ::core::marker::Sized {}

pub trait MDStringExt: MDStringObj {
}
impl<T> MDStringExt for T where T: MDStringObj {}

pub struct MDString {
    inner: ::core::nonzero::NonZero<*mut MDStringInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for MDString {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl MDStringObj for MDString {
    #[inline(always)]
    fn get_inner(&self) -> *mut MDStringInner {
        *self.inner
    }
}
impl MDString {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut MDStringInner, owned: bool) -> MDString {
        MDString {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for MDString {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueObj::get_inner(self));
            }
        }
    }
}
pub enum ValueTy {
    ArgumentVal,
    BasicBlockVal,
    FunctionVal,
    GlobalAliasVal,
    GlobalVariableVal,
    UndefValueVal,
    BlockAddressVal,
    ConstantExprVal,
    ConstantAggregateZeroVal,
    ConstantDataArrayVal,
    ConstantDataVectorVal,
    ConstantIntVal,
    ConstantFPVal,
    ConstantArrayVal,
    ConstantStructVal,
    ConstantVectorVal,
    ConstantPointerNullVal,
    MetadataAsValueVal,
    InlineAsmVal,
    InstructionVal,
    ConstantFirstVal,
    ConstantLastVal,
}
impl ValueTy {
    pub fn from_ffi(value: ::ffi::llvm_Value_ValueTy) -> ValueTy {
        match value {
            ::ffi::llvm_Value_ValueTy::ArgumentVal => ValueTy::ArgumentVal,
            ::ffi::llvm_Value_ValueTy::BasicBlockVal => ValueTy::BasicBlockVal,
            ::ffi::llvm_Value_ValueTy::FunctionVal => ValueTy::FunctionVal,
            ::ffi::llvm_Value_ValueTy::GlobalAliasVal => ValueTy::GlobalAliasVal,
            ::ffi::llvm_Value_ValueTy::GlobalVariableVal => ValueTy::GlobalVariableVal,
            ::ffi::llvm_Value_ValueTy::UndefValueVal => ValueTy::UndefValueVal,
            ::ffi::llvm_Value_ValueTy::BlockAddressVal => ValueTy::BlockAddressVal,
            ::ffi::llvm_Value_ValueTy::ConstantExprVal => ValueTy::ConstantExprVal,
            ::ffi::llvm_Value_ValueTy::ConstantAggregateZeroVal => ValueTy::ConstantAggregateZeroVal,
            ::ffi::llvm_Value_ValueTy::ConstantDataArrayVal => ValueTy::ConstantDataArrayVal,
            ::ffi::llvm_Value_ValueTy::ConstantDataVectorVal => ValueTy::ConstantDataVectorVal,
            ::ffi::llvm_Value_ValueTy::ConstantIntVal => ValueTy::ConstantIntVal,
            ::ffi::llvm_Value_ValueTy::ConstantFPVal => ValueTy::ConstantFPVal,
            ::ffi::llvm_Value_ValueTy::ConstantArrayVal => ValueTy::ConstantArrayVal,
            ::ffi::llvm_Value_ValueTy::ConstantStructVal => ValueTy::ConstantStructVal,
            ::ffi::llvm_Value_ValueTy::ConstantVectorVal => ValueTy::ConstantVectorVal,
            ::ffi::llvm_Value_ValueTy::ConstantPointerNullVal => ValueTy::ConstantPointerNullVal,
            ::ffi::llvm_Value_ValueTy::MetadataAsValueVal => ValueTy::MetadataAsValueVal,
            ::ffi::llvm_Value_ValueTy::InlineAsmVal => ValueTy::InlineAsmVal,
            ::ffi::llvm_Value_ValueTy::InstructionVal => ValueTy::InstructionVal,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Value_ValueTy {
        match self {
            ValueTy::ArgumentVal => ::ffi::llvm_Value_ValueTy::ArgumentVal,
            ValueTy::BasicBlockVal => ::ffi::llvm_Value_ValueTy::BasicBlockVal,
            ValueTy::FunctionVal => ::ffi::llvm_Value_ValueTy::FunctionVal,
            ValueTy::GlobalAliasVal => ::ffi::llvm_Value_ValueTy::GlobalAliasVal,
            ValueTy::GlobalVariableVal => ::ffi::llvm_Value_ValueTy::GlobalVariableVal,
            ValueTy::UndefValueVal => ::ffi::llvm_Value_ValueTy::UndefValueVal,
            ValueTy::BlockAddressVal => ::ffi::llvm_Value_ValueTy::BlockAddressVal,
            ValueTy::ConstantExprVal => ::ffi::llvm_Value_ValueTy::ConstantExprVal,
            ValueTy::ConstantAggregateZeroVal => ::ffi::llvm_Value_ValueTy::ConstantAggregateZeroVal,
            ValueTy::ConstantDataArrayVal => ::ffi::llvm_Value_ValueTy::ConstantDataArrayVal,
            ValueTy::ConstantDataVectorVal => ::ffi::llvm_Value_ValueTy::ConstantDataVectorVal,
            ValueTy::ConstantIntVal => ::ffi::llvm_Value_ValueTy::ConstantIntVal,
            ValueTy::ConstantFPVal => ::ffi::llvm_Value_ValueTy::ConstantFPVal,
            ValueTy::ConstantArrayVal => ::ffi::llvm_Value_ValueTy::ConstantArrayVal,
            ValueTy::ConstantStructVal => ::ffi::llvm_Value_ValueTy::ConstantStructVal,
            ValueTy::ConstantVectorVal => ::ffi::llvm_Value_ValueTy::ConstantVectorVal,
            ValueTy::ConstantPointerNullVal => ::ffi::llvm_Value_ValueTy::ConstantPointerNullVal,
            ValueTy::MetadataAsValueVal => ::ffi::llvm_Value_ValueTy::MetadataAsValueVal,
            ValueTy::InlineAsmVal => ::ffi::llvm_Value_ValueTy::InlineAsmVal,
            ValueTy::InstructionVal => ::ffi::llvm_Value_ValueTy::InstructionVal,
            ValueTy::ConstantFirstVal => ::ffi::llvm_Value_ValueTy::FunctionVal,
            ValueTy::ConstantLastVal => ::ffi::llvm_Value_ValueTy::ConstantPointerNullVal,
        }
    }
}
impl Copy for ValueTy {}
pub type ValueInner = ::ffi::llvm_Value;

pub trait ValueObj {
    unsafe fn get_inner(&self) -> *mut ValueInner;
}

pub trait ValueOwned: ValueObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ValueInner {
        let inner = ValueObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ValueOwned for T where T: ValueObj + ::core::marker::Sized {}

pub trait ValueExt: ValueObj {

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Value_dump(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Value_getContext(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Value_getName(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_num_uses(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Value_getNumUses(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value) as u32;
            ret
        }
    }

    fn get_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Value_getType(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_value_id(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Value_getValueID(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value) as u32;
            ret
        }
    }

    fn has_n_uses(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasNUses(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value, n as ::libc::c_uint);
            ret
        }
    }

    fn has_n_uses_or_more(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasNUsesOrMore(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value, n as ::libc::c_uint);
            ret
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasName(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
            ret
        }
    }

    fn has_one_use(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasOneUse(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value);
            ret
        }
    }

    fn is_used_in_basic_block<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&self, bb: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_isUsedInBasicBlock(::llvm::value::ValueObj::get_inner(self) as *const ::ffi::llvm_Value, ::llvm::value::BasicBlockObj::get_inner(bb));
            ret
        }
    }

    fn mutate_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, ty: &mut A1) {
        unsafe {
            ::ffi::llvm::Value_mutateType(::llvm::value::ValueObj::get_inner(self), ::llvm::ty::TypeObj::get_inner(ty));
        }
    }

    fn replace_all_uses_with<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1) {
        unsafe {
            ::ffi::llvm::Value_replaceAllUsesWith(::llvm::value::ValueObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value));
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::Value_setName(::llvm::value::ValueObj::get_inner(self), c_name);
        }
    }

    fn take_name<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1) {
        unsafe {
            ::ffi::llvm::Value_takeName(::llvm::value::ValueObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value));
        }
    }
}
impl<T> ValueExt for T where T: ValueObj {}

pub struct Value {
    inner: ::core::nonzero::NonZero<*mut ValueInner>,
    owned: bool,
}
impl ValueObj for Value {
    #[inline(always)]
    fn get_inner(&self) -> *mut ValueInner {
        *self.inner
    }
}
impl Value {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ValueInner, owned: bool) -> Value {
        Value {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Value {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueObj::get_inner(self));
            }
        }
    }
}
