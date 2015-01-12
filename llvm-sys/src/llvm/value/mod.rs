pub mod user;
pub type ArgumentInner = ::ffi::llvm_Argument;

pub trait ArgumentExt: ::llvm::value::ValueExt {

    fn inner_llvm_Argument(&self) -> *mut ArgumentInner;
    fn inner(&self) -> *mut ArgumentInner {
        self.inner_llvm_Argument()
    }
}

pub struct Argument {
    inner: *mut ArgumentInner,
}
impl ::llvm::value::ValueExt for Argument {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ArgumentExt for Argument {
    fn inner_llvm_Argument(&self) -> *mut ArgumentInner {
        self.inner
    }
}
impl Argument {
    pub unsafe fn from_inner(inner: *mut ArgumentInner) -> Argument {
        Argument {
            inner: inner,
        }
    }
}
impl Drop for Argument {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
        }
    }
}
pub type BasicBlockInner = ::ffi::llvm_BasicBlock;

pub trait BasicBlockExt: ::llvm::value::ValueExt {

    fn inner_llvm_BasicBlock(&self) -> *mut BasicBlockInner;
    fn inner(&self) -> *mut BasicBlockInner {
        self.inner_llvm_BasicBlock()
    }
}

pub struct BasicBlock {
    inner: *mut BasicBlockInner,
}
impl ::llvm::value::ValueExt for BasicBlock {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BasicBlockExt for BasicBlock {
    fn inner_llvm_BasicBlock(&self) -> *mut BasicBlockInner {
        self.inner
    }
}
impl BasicBlock {
    pub unsafe fn from_inner(inner: *mut BasicBlockInner) -> BasicBlock {
        BasicBlock {
            inner: inner,
        }
    }
}
impl Drop for BasicBlock {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
        }
    }
}
pub type InlineAsmInner = ::ffi::llvm_InlineAsm;

pub trait InlineAsmExt: ::llvm::value::ValueExt {

    fn inner_llvm_InlineAsm(&self) -> *mut InlineAsmInner;
    fn inner(&self) -> *mut InlineAsmInner {
        self.inner_llvm_InlineAsm()
    }
}

pub struct InlineAsm {
    inner: *mut InlineAsmInner,
}
impl ::llvm::value::ValueExt for InlineAsm {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InlineAsmExt for InlineAsm {
    fn inner_llvm_InlineAsm(&self) -> *mut InlineAsmInner {
        self.inner
    }
}
impl InlineAsm {
    pub unsafe fn from_inner(inner: *mut InlineAsmInner) -> InlineAsm {
        InlineAsm {
            inner: inner,
        }
    }
}
impl Drop for InlineAsm {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
        }
    }
}
pub type MetadataAsValueInner = ::ffi::llvm_MetadataAsValue;

pub trait MetadataAsValueExt: ::llvm::value::ValueExt {

    fn inner_llvm_MetadataAsValue(&self) -> *mut MetadataAsValueInner;
    fn inner(&self) -> *mut MetadataAsValueInner {
        self.inner_llvm_MetadataAsValue()
    }
}

pub struct MetadataAsValue {
    inner: *mut MetadataAsValueInner,
}
impl ::llvm::value::ValueExt for MetadataAsValue {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl MetadataAsValueExt for MetadataAsValue {
    fn inner_llvm_MetadataAsValue(&self) -> *mut MetadataAsValueInner {
        self.inner
    }
}
impl MetadataAsValue {
    pub unsafe fn from_inner(inner: *mut MetadataAsValueInner) -> MetadataAsValue {
        MetadataAsValue {
            inner: inner,
        }
    }
}
impl Drop for MetadataAsValue {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
        }
    }
}
pub type ValueInner = ::ffi::llvm_Value;

pub trait ValueExt {

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
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Value_getContext(self.inner_llvm_Value() as *const ::ffi::llvm_Value))
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Value_getName(self.inner_llvm_Value() as *const ::ffi::llvm_Value);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_num_uses(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Value_getNumUses(self.inner_llvm_Value() as *const ::ffi::llvm_Value) as u32
        }
    }

    fn get_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Value_getType(self.inner_llvm_Value() as *const ::ffi::llvm_Value))
        }
    }

    fn get_value_id(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Value_getValueID(self.inner_llvm_Value() as *const ::ffi::llvm_Value) as u32
        }
    }

    fn has_n_uses(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasNUses(self.inner_llvm_Value() as *const ::ffi::llvm_Value, n as ::libc::c_uint)
        }
    }

    fn has_n_uses_or_more(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasNUsesOrMore(self.inner_llvm_Value() as *const ::ffi::llvm_Value, n as ::libc::c_uint)
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasName(self.inner_llvm_Value() as *const ::ffi::llvm_Value)
        }
    }

    fn has_one_use(&self) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasOneUse(self.inner_llvm_Value() as *const ::ffi::llvm_Value)
        }
    }

    fn is_used_in_basic_block(&self, bb: &::llvm::value::BasicBlockExt) -> bool {
        unsafe {
            ::ffi::llvm::Value_isUsedInBasicBlock(self.inner_llvm_Value() as *const ::ffi::llvm_Value, bb.inner_llvm_BasicBlock())
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
    inner: *mut ValueInner,
}
impl ValueExt for Value {
    fn inner_llvm_Value(&self) -> *mut ValueInner {
        self.inner
    }
}
impl Value {
    pub unsafe fn from_inner(inner: *mut ValueInner) -> Value {
        Value {
            inner: inner,
        }
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner_llvm_Value(self));
        }
    }
}
