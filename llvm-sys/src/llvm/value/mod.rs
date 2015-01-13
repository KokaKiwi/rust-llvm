pub mod user;
pub type ArgumentInner = ::ffi::llvm_Argument;

pub trait ArgumentExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Argument(&self) -> *mut ArgumentInner;

    fn inner(&self) -> *mut ArgumentInner {
        self.inner_llvm_Argument()
    }
}

pub struct Argument {
    inner: ::core::nonzero::NonZero<*mut ArgumentInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Argument {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ArgumentExt for Argument {
    fn inner_llvm_Argument(&self) -> *mut ArgumentInner {
        *self.inner
    }
}
impl Argument {
    pub unsafe fn from_inner(inner: *mut ArgumentInner, owned: bool) -> Argument {
        Argument {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Argument {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
            }
        }
    }
}
pub type BasicBlockInner = ::ffi::llvm_BasicBlock;

pub trait BasicBlockExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_BasicBlock(&self) -> *mut BasicBlockInner;

    fn inner(&self) -> *mut BasicBlockInner {
        self.inner_llvm_BasicBlock()
    }
}

pub struct BasicBlock {
    inner: ::core::nonzero::NonZero<*mut BasicBlockInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for BasicBlock {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BasicBlockExt for BasicBlock {
    fn inner_llvm_BasicBlock(&self) -> *mut BasicBlockInner {
        *self.inner
    }
}
impl BasicBlock {
    pub unsafe fn from_inner(inner: *mut BasicBlockInner, owned: bool) -> BasicBlock {
        BasicBlock {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BasicBlock {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
            }
        }
    }
}
pub type InlineAsmInner = ::ffi::llvm_InlineAsm;

pub trait InlineAsmExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_InlineAsm(&self) -> *mut InlineAsmInner;

    fn inner(&self) -> *mut InlineAsmInner {
        self.inner_llvm_InlineAsm()
    }
}

pub struct InlineAsm {
    inner: ::core::nonzero::NonZero<*mut InlineAsmInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for InlineAsm {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl InlineAsmExt for InlineAsm {
    fn inner_llvm_InlineAsm(&self) -> *mut InlineAsmInner {
        *self.inner
    }
}
impl InlineAsm {
    pub unsafe fn from_inner(inner: *mut InlineAsmInner, owned: bool) -> InlineAsm {
        InlineAsm {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for InlineAsm {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
            }
        }
    }
}
pub type MetadataAsValueInner = ::ffi::llvm_MetadataAsValue;

pub trait MetadataAsValueExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_MetadataAsValue(&self) -> *mut MetadataAsValueInner;

    fn inner(&self) -> *mut MetadataAsValueInner {
        self.inner_llvm_MetadataAsValue()
    }
}

pub struct MetadataAsValue {
    inner: ::core::nonzero::NonZero<*mut MetadataAsValueInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for MetadataAsValue {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl MetadataAsValueExt for MetadataAsValue {
    fn inner_llvm_MetadataAsValue(&self) -> *mut MetadataAsValueInner {
        *self.inner
    }
}
impl MetadataAsValue {
    pub unsafe fn from_inner(inner: *mut MetadataAsValueInner, owned: bool) -> MetadataAsValue {
        MetadataAsValue {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for MetadataAsValue {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
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

pub trait ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Value(&self) -> *mut ValueInner;

    fn inner(&self) -> *mut ValueInner {
        self.inner_llvm_Value()
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Value_dump(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Value_getContext(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Value_getName(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_num_uses(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Value_getNumUses(self.inner_llvm_Value() as *const ::ffi::llvm_Value) as u32;
            ret
        }
    }

    fn get_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Value_getType(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_value_id(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Value_getValueID(self.inner_llvm_Value() as *const ::ffi::llvm_Value) as u32;
            ret
        }
    }

    fn has_n_uses(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasNUses(self.inner_llvm_Value() as *const ::ffi::llvm_Value, n as ::libc::c_uint);
            ret
        }
    }

    fn has_n_uses_or_more(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasNUsesOrMore(self.inner_llvm_Value() as *const ::ffi::llvm_Value, n as ::libc::c_uint);
            ret
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasName(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            ret
        }
    }

    fn has_one_use(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_hasOneUse(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            ret
        }
    }

    fn is_used_in_basic_block(&self, bb: &::llvm::value::BasicBlockExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Value_isUsedInBasicBlock(self.inner_llvm_Value() as *const ::ffi::llvm_Value, bb.inner_llvm_BasicBlock());
            ret
        }
    }

    fn mutate_type(&mut self, ty: &::llvm::ty::TypeExt) {
        unsafe {
            ::ffi::llvm::Value_mutateType(self.inner_llvm_Value(), ty.inner_llvm_Type());
        }
    }

    fn replace_all_uses_with(&mut self, value: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::Value_replaceAllUsesWith(self.inner_llvm_Value(), value.inner_llvm_Value());
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::Value_setName(self.inner_llvm_Value(), c_name);
        }
    }

    fn take_name(&mut self, value: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::Value_takeName(self.inner_llvm_Value(), value.inner_llvm_Value());
        }
    }
}

pub struct Value {
    inner: ::core::nonzero::NonZero<*mut ValueInner>,
    owned: bool,
}
impl ValueExt for Value {
    fn inner_llvm_Value(&self) -> *mut ValueInner {
        *self.inner
    }
}
impl Value {
    pub unsafe fn from_inner(inner: *mut ValueInner, owned: bool) -> Value {
        Value {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
            }
        }
    }
}
