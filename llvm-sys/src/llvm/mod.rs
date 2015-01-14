pub mod ty;
pub mod value;
pub mod calling_conv;
pub type DataLayoutInner = ::ffi::llvm_DataLayout;

pub trait DataLayoutExt {
    #[allow(non_snake_case)]
    fn inner_llvm_DataLayout(&self) -> *mut DataLayoutInner;

    fn inner(&self) -> *mut DataLayoutInner {
        self.inner_llvm_DataLayout()
    }
}

pub struct DataLayout {
    inner: ::core::nonzero::NonZero<*mut DataLayoutInner>,
}
impl DataLayoutExt for DataLayout {
    fn inner_llvm_DataLayout(&self) -> *mut DataLayoutInner {
        *self.inner
    }
}
impl DataLayout {
    pub unsafe fn from_inner(inner: *mut DataLayoutInner) -> DataLayout {
        DataLayout {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for DataLayout {}
pub type DebugLocInner = ::ffi::llvm_DebugLoc;

pub trait DebugLocExt {
    #[allow(non_snake_case)]
    fn inner_llvm_DebugLoc(&self) -> *mut DebugLocInner;

    fn inner(&self) -> *mut DebugLocInner {
        self.inner_llvm_DebugLoc()
    }

    fn dump(&self, ctx: &::llvm::LLVMContextExt) {
        unsafe {
            ::ffi::llvm::DebugLoc_dump(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc, ctx.inner_llvm_LLVMContext());
        }
    }

    fn get_as_md_node(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getAsMDNode(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc, ctx.inner_llvm_LLVMContext());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_col(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getCol(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_inlined_at(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getInlinedAt(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc, ctx.inner_llvm_LLVMContext());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_line(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getLine(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_scope(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScope(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc, ctx.inner_llvm_LLVMContext());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_scope_node(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScopeNode(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc, ctx.inner_llvm_LLVMContext());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn is_unknown(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_isUnknown(self.inner_llvm_DebugLoc() as *const ::ffi::llvm_DebugLoc);
            ret
        }
    }
}

pub struct DebugLoc {
    inner: ::core::nonzero::NonZero<*mut DebugLocInner>,
}
impl DebugLocExt for DebugLoc {
    fn inner_llvm_DebugLoc(&self) -> *mut DebugLocInner {
        *self.inner
    }
}
impl DebugLoc {
    pub unsafe fn from_inner(inner: *mut DebugLocInner) -> DebugLoc {
        DebugLoc {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn new() -> ::llvm::DebugLoc {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_new();
            if ret.is_null(){
                panic!("::llvm::DebugLoc::new returned a null pointer!");
            }
            ::llvm::DebugLoc::from_inner(ret)
        }
    }
}
impl Copy for DebugLoc {}
pub type LLVMContextInner = ::ffi::llvm_LLVMContext;

pub trait LLVMContextExt {
    #[allow(non_snake_case)]
    fn inner_llvm_LLVMContext(&self) -> *mut LLVMContextInner;

    fn inner(&self) -> *mut LLVMContextInner {
        self.inner_llvm_LLVMContext()
    }
}

pub struct LLVMContext {
    inner: ::core::nonzero::NonZero<*mut LLVMContextInner>,
}
impl LLVMContextExt for LLVMContext {
    fn inner_llvm_LLVMContext(&self) -> *mut LLVMContextInner {
        *self.inner
    }
}
impl LLVMContext {
    pub unsafe fn from_inner(inner: *mut LLVMContextInner) -> LLVMContext {
        LLVMContext {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for LLVMContext {}
pub type ModuleInner = ::ffi::llvm_Module;

pub trait ModuleExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Module(&self) -> *mut ModuleInner;

    fn inner(&self) -> *mut ModuleInner {
        self.inner_llvm_Module()
    }

    fn append_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_appendModuleInlineAsm(self.inner_llvm_Module(), c_asm);
        }
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Module_dump(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Module_getContext(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayout(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_data_layout_str(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayoutStr(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_function(&self, name: &str) -> Option<::llvm::value::user::constant::Function> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getFunction(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Function::from_inner(ret, false))
        }
    }

    fn get_md_kind_id(&self, name: &str) -> u32 {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getMDKindID(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name) as u32;
            ret
        }
    }

    fn get_module_identifier(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleIdentifier(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_module_inline_asm(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleInlineAsm(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_named_value(&self, name: &str) -> Option<::llvm::value::user::constant::GlobalValue> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getNamedValue(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::GlobalValue::from_inner(ret, false))
        }
    }

    fn get_or_insert_function(&mut self, name: &str, ty: &::llvm::ty::FunctionTypeExt) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getOrInsertFunction(self.inner_llvm_Module(), c_name, ty.inner_llvm_FunctionType());
            ::llvm::value::user::constant::Constant::from_inner(ret, false)
        }
    }

    fn get_target_triple(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getTargetTriple(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_type_by_name(&self, name: &str) -> Option<::llvm::ty::StructType> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getTypeByName(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    fn set_data_layout(&mut self, other: &::llvm::DataLayoutExt) {
        unsafe {
            ::ffi::llvm::Module_setDataLayout(self.inner_llvm_Module(), other.inner_llvm_DataLayout());
        }
    }

    fn set_data_layout_str(&mut self, desc: &str) {
        unsafe {
            let c_desc = ::ffi::llvm_StringRef {
                data: desc.as_ptr() as *const ::libc::c_char,
                length: desc.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setDataLayoutStr(self.inner_llvm_Module(), c_desc);
        }
    }

    fn set_module_identifier(&mut self, id: &str) {
        unsafe {
            let c_id = ::ffi::llvm_StringRef {
                data: id.as_ptr() as *const ::libc::c_char,
                length: id.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleIdentifier(self.inner_llvm_Module(), c_id);
        }
    }

    fn set_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleInlineAsm(self.inner_llvm_Module(), c_asm);
        }
    }

    fn set_target_triple(&mut self, triple: &str) {
        unsafe {
            let c_triple = ::ffi::llvm_StringRef {
                data: triple.as_ptr() as *const ::libc::c_char,
                length: triple.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setTargetTriple(self.inner_llvm_Module(), c_triple);
        }
    }
}

pub struct Module {
    inner: ::core::nonzero::NonZero<*mut ModuleInner>,
    owned: bool,
}
impl ModuleExt for Module {
    fn inner_llvm_Module(&self) -> *mut ModuleInner {
        *self.inner
    }
}
impl Module {
    pub unsafe fn from_inner(inner: *mut ModuleInner, owned: bool) -> Module {
        Module {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn new(module_id: &str, context: &::llvm::LLVMContextExt) -> ::llvm::Module {
        unsafe {
            let c_module_id = ::ffi::llvm_StringRef {
                data: module_id.as_ptr() as *const ::libc::c_char,
                length: module_id.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_new(c_module_id, context.inner_llvm_LLVMContext());
            if ret.is_null(){
                panic!("::llvm::Module::new returned a null pointer!");
            }
            ::llvm::Module::from_inner(ret, true)
        }
    }
}
impl Drop for Module {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Module_delete(::llvm::ModuleExt::inner_llvm_Module(self));
            }
        }
    }
}

pub fn get_global_context() -> ::llvm::LLVMContext {
    unsafe {
        let ret = ::ffi::llvm::getGlobalContext();
        ::llvm::LLVMContext::from_inner(ret)
    }
}
