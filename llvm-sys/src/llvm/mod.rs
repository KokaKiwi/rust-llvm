pub mod ty;
pub mod value;
pub type DataLayoutInner = ::ffi::llvm_DataLayout;

pub trait DataLayoutExt {

    fn inner_llvm_DataLayout(&self) -> *mut DataLayoutInner;
    fn inner(&self) -> *mut DataLayoutInner {
        self.inner_llvm_DataLayout()
    }
}

pub struct DataLayout {
    inner: *mut DataLayoutInner,
}
impl DataLayoutExt for DataLayout {
    fn inner_llvm_DataLayout(&self) -> *mut DataLayoutInner {
        self.inner
    }
}
impl DataLayout {
    pub unsafe fn from_inner(inner: *mut DataLayoutInner) -> DataLayout {
        DataLayout {
            inner: inner,
        }
    }
}
impl Copy for DataLayout {
}
pub type LLVMContextInner = ::ffi::llvm_LLVMContext;

pub trait LLVMContextExt {

    fn inner_llvm_LLVMContext(&self) -> *mut LLVMContextInner;
    fn inner(&self) -> *mut LLVMContextInner {
        self.inner_llvm_LLVMContext()
    }
}

pub struct LLVMContext {
    inner: *mut LLVMContextInner,
}
impl LLVMContextExt for LLVMContext {
    fn inner_llvm_LLVMContext(&self) -> *mut LLVMContextInner {
        self.inner
    }
}
impl LLVMContext {
    pub unsafe fn from_inner(inner: *mut LLVMContextInner) -> LLVMContext {
        LLVMContext {
            inner: inner,
        }
    }
}
impl Copy for LLVMContext {
}
pub type ModuleInner = ::ffi::llvm_Module;

pub trait ModuleExt {

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
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Module_getContext(self.inner_llvm_Module() as *const ::ffi::llvm_Module))
        }
    }

    fn get_data_layout(&self) -> ::llvm::DataLayout {
        unsafe {
            ::llvm::DataLayout::from_inner(::ffi::llvm::Module_getDataLayout(self.inner_llvm_Module() as *const ::ffi::llvm_Module) as *mut ::ffi::llvm_DataLayout)
        }
    }

    fn get_data_layout_str(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayoutStr(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_md_kind_id(&self, name: &str) -> u32 {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_getMDKindID(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name) as u32
        }
    }

    fn get_module_identifier(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleIdentifier(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_module_inline_asm(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleInlineAsm(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_named_value(&self, name: &str) -> ::llvm::value::user::constant::GlobalValue {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::value::user::constant::GlobalValue::from_inner(::ffi::llvm::Module_getNamedValue(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name))
        }
    }

    fn get_or_insert_function(&mut self, name: &str, ty: &::llvm::ty::FunctionTypeExt) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Module_getOrInsertFunction(self.inner_llvm_Module(), c_name, ty.inner_llvm_FunctionType()))
        }
    }

    fn get_target_triple(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getTargetTriple(self.inner_llvm_Module() as *const ::ffi::llvm_Module);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_type_by_name(&self, name: &str) -> ::llvm::ty::StructType {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::ty::StructType::from_inner(::ffi::llvm::Module_getTypeByName(self.inner_llvm_Module() as *const ::ffi::llvm_Module, c_name))
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
    inner: *mut ModuleInner,
}
impl ModuleExt for Module {
    fn inner_llvm_Module(&self) -> *mut ModuleInner {
        self.inner
    }
}
impl Module {
    pub unsafe fn from_inner(inner: *mut ModuleInner) -> Module {
        Module {
            inner: inner,
        }
    }

    pub fn new(module_id: &str, context: &::llvm::LLVMContextExt) -> ::llvm::Module {
        unsafe {
            let c_module_id = ::ffi::llvm_StringRef {
                data: module_id.as_ptr() as *const ::libc::c_char,
                length: module_id.len() as ::libc::size_t,
            };
            ::llvm::Module::from_inner(::ffi::llvm::Module_new(c_module_id, context.inner_llvm_LLVMContext()))
        }
    }
}
impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Module_delete(::llvm::ModuleExt::inner_llvm_Module(self));
        }
    }
}

pub fn get_global_context() -> ::llvm::LLVMContext {
    unsafe {
        ::llvm::LLVMContext::from_inner(::ffi::llvm::getGlobalContext())
    }
}
