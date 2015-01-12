pub mod user;
pub type ArgumentInner = ::ffi::llvm_Argument;

pub trait ArgumentExt: ::llvm::value::ValueExt {

    fn inner(&self) -> *mut ArgumentInner;
}

pub struct Argument {
    inner: *mut ArgumentInner,
}
impl ::llvm::value::ValueExt for Argument {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ArgumentExt for Argument {
    fn inner(&self) -> *mut ArgumentInner {
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
pub type BasicBlockInner = ::ffi::llvm_BasicBlock;

pub trait BasicBlockExt: ::llvm::value::ValueExt {

    fn inner(&self) -> *mut BasicBlockInner;
}

pub struct BasicBlock {
    inner: *mut BasicBlockInner,
}
impl ::llvm::value::ValueExt for BasicBlock {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BasicBlockExt for BasicBlock {
    fn inner(&self) -> *mut BasicBlockInner {
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
pub type InlineAsmInner = ::ffi::llvm_InlineAsm;

pub trait InlineAsmExt: ::llvm::value::ValueExt {

    fn inner(&self) -> *mut InlineAsmInner;
}

pub struct InlineAsm {
    inner: *mut InlineAsmInner,
}
impl ::llvm::value::ValueExt for InlineAsm {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl InlineAsmExt for InlineAsm {
    fn inner(&self) -> *mut InlineAsmInner {
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
pub type MetadataAsValueInner = ::ffi::llvm_MetadataAsValue;

pub trait MetadataAsValueExt: ::llvm::value::ValueExt {

    fn inner(&self) -> *mut MetadataAsValueInner;
}

pub struct MetadataAsValue {
    inner: *mut MetadataAsValueInner,
}
impl ::llvm::value::ValueExt for MetadataAsValue {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl MetadataAsValueExt for MetadataAsValue {
    fn inner(&self) -> *mut MetadataAsValueInner {
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
pub type ValueInner = ::ffi::llvm_Value;

pub trait ValueExt {

    fn inner(&self) -> *mut ValueInner;

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Value_dump(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Value_getContext(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value))
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Value_getName(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_num_uses(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Value_getNumUses(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value) as u32
        }
    }

    fn get_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Value_getType(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value))
        }
    }

    fn get_value_id(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Value_getValueID(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value) as u32
        }
    }

    fn has_n_uses(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasNUses(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value, n as ::libc::c_uint)
        }
    }

    fn has_n_uses_or_more(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasNUsesOrMore(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value, n as ::libc::c_uint)
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasName(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value)
        }
    }

    fn has_one_use(&self) -> bool {
        unsafe {
            ::ffi::llvm::Value_hasOneUse(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value)
        }
    }

    fn is_used_in_basic_block<A1: ::llvm::value::BasicBlockExt>(&self, bb: A1) -> bool {
        unsafe {
            ::ffi::llvm::Value_isUsedInBasicBlock(::llvm::value::ValueExt::inner(self) as *const ::ffi::llvm_Value, ::llvm::value::BasicBlockExt::inner(&bb))
        }
    }

    fn replace_all_uses_with<A1: ::llvm::value::ValueExt>(&mut self, value: A1) {
        unsafe {
            ::ffi::llvm::Value_replaceAllUsesWith(::llvm::value::ValueExt::inner(self), ::llvm::value::ValueExt::inner(&value));
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::Value_setName(::llvm::value::ValueExt::inner(self), c_name);
        }
    }

    fn take_name<A1: ::llvm::value::ValueExt>(&mut self, value: A1) {
        unsafe {
            ::ffi::llvm::Value_takeName(::llvm::value::ValueExt::inner(self), ::llvm::value::ValueExt::inner(&value));
        }
    }
}

pub struct Value {
    inner: *mut ValueInner,
}
impl ValueExt for Value {
    fn inner(&self) -> *mut ValueInner {
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
