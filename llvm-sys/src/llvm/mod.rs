pub mod value;
pub mod calling_conv;
pub mod ty;
pub mod pass;
pub enum AtomicOrdering {
    NotAtomic,
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}
impl AtomicOrdering {
    pub fn from_ffi(value: ::ffi::llvm_AtomicOrdering) -> AtomicOrdering {
        match value {
            ::ffi::llvm_AtomicOrdering::NotAtomic => AtomicOrdering::NotAtomic,
            ::ffi::llvm_AtomicOrdering::Unordered => AtomicOrdering::Unordered,
            ::ffi::llvm_AtomicOrdering::Monotonic => AtomicOrdering::Monotonic,
            ::ffi::llvm_AtomicOrdering::Acquire => AtomicOrdering::Acquire,
            ::ffi::llvm_AtomicOrdering::Release => AtomicOrdering::Release,
            ::ffi::llvm_AtomicOrdering::AcquireRelease => AtomicOrdering::AcquireRelease,
            ::ffi::llvm_AtomicOrdering::SequentiallyConsistent => AtomicOrdering::SequentiallyConsistent,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_AtomicOrdering {
        match self {
            AtomicOrdering::NotAtomic => ::ffi::llvm_AtomicOrdering::NotAtomic,
            AtomicOrdering::Unordered => ::ffi::llvm_AtomicOrdering::Unordered,
            AtomicOrdering::Monotonic => ::ffi::llvm_AtomicOrdering::Monotonic,
            AtomicOrdering::Acquire => ::ffi::llvm_AtomicOrdering::Acquire,
            AtomicOrdering::Release => ::ffi::llvm_AtomicOrdering::Release,
            AtomicOrdering::AcquireRelease => ::ffi::llvm_AtomicOrdering::AcquireRelease,
            AtomicOrdering::SequentiallyConsistent => ::ffi::llvm_AtomicOrdering::SequentiallyConsistent,
        }
    }
}
impl Copy for AtomicOrdering {}
pub enum SynchronizationScope {
    SingleThread,
    CrossThread,
}
impl SynchronizationScope {
    pub fn from_ffi(value: ::ffi::llvm_SynchronizationScope) -> SynchronizationScope {
        match value {
            ::ffi::llvm_SynchronizationScope::SingleThread => SynchronizationScope::SingleThread,
            ::ffi::llvm_SynchronizationScope::CrossThread => SynchronizationScope::CrossThread,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_SynchronizationScope {
        match self {
            SynchronizationScope::SingleThread => ::ffi::llvm_SynchronizationScope::SingleThread,
            SynchronizationScope::CrossThread => ::ffi::llvm_SynchronizationScope::CrossThread,
        }
    }
}
impl Copy for SynchronizationScope {}
pub type DataLayoutInner = ::ffi::llvm_DataLayout;

pub trait DataLayoutExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut DataLayoutInner;
}

pub struct DataLayout {
    inner: ::core::nonzero::NonZero<*mut DataLayoutInner>,
}
impl DataLayoutExt for DataLayout {
    fn inner(&self) -> *mut DataLayoutInner {
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
    fn inner(&self) -> *mut DebugLocInner;

    fn dump(&self, ctx: &::llvm::LLVMContextExt) {
        unsafe {
            ::ffi::llvm::DebugLoc_dump(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextExt::inner(ctx));
        }
    }

    fn get_as_md_node(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getAsMDNode(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextExt::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_col(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getCol(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_inlined_at(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getInlinedAt(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextExt::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_line(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getLine(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_scope(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScope(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextExt::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_scope_node(&self, ctx: &::llvm::LLVMContextExt) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScopeNode(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextExt::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn is_unknown(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_isUnknown(::llvm::DebugLocExt::inner(self) as *const ::ffi::llvm_DebugLoc);
            ret
        }
    }
}

pub struct DebugLoc {
    inner: ::core::nonzero::NonZero<*mut DebugLocInner>,
}
impl DebugLocExt for DebugLoc {
    fn inner(&self) -> *mut DebugLocInner {
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
            if ret.is_null() {
                panic!("::llvm::DebugLoc::new returned a null pointer!");
            }
            ::llvm::DebugLoc::from_inner(ret)
        }
    }
}
impl Copy for DebugLoc {}
pub type FunctionPassManagerInner = ::ffi::llvm_FunctionPassManager;

pub trait FunctionPassManagerExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut FunctionPassManagerInner;
}

pub struct FunctionPassManager {
    inner: ::core::nonzero::NonZero<*mut FunctionPassManagerInner>,
}
impl FunctionPassManagerExt for FunctionPassManager {
    fn inner(&self) -> *mut FunctionPassManagerInner {
        *self.inner
    }
}
impl FunctionPassManager {
    pub unsafe fn from_inner(inner: *mut FunctionPassManagerInner) -> FunctionPassManager {
        FunctionPassManager {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for FunctionPassManager {}
pub type IRBuilderInner = ::ffi::llvm_IRBuilder;

pub trait IRBuilderExt: ::llvm::IRBuilderBaseExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut IRBuilderInner;

    fn create_a_shr(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_a_shr_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShrByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_add(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAdd(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_addr_space_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAddrSpaceCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_aggregate_ret(&mut self, values: &[&::llvm::value::ValueExt]) -> Option<::llvm::value::user::inst::ReturnInst> {
        unsafe {
            let values_vec: Vec<_> = values.iter().map(|&ty| ::llvm::value::ValueExt::inner(ty)).collect();
            let ret = ::ffi::llvm::IRBuilder_CreateAggregateRet(::llvm::IRBuilderExt::inner(self), values_vec.as_slice());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::ReturnInst::from_inner(ret, false))
        }
    }

    fn create_aligned_load(&mut self, ptr: &::llvm::value::ValueExt, align: u32, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoad(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), align as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_load_volatile(&mut self, ptr: &::llvm::value::ValueExt, align: u32, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoadVolatile(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), align as ::libc::c_uint, is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_store(&mut self, value: &::llvm::value::ValueExt, ptr: &::llvm::value::ValueExt, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedStore(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::value::ValueExt::inner(ptr), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_alloca(&mut self, ty: &::llvm::ty::TypeExt, array_size: Option<&::llvm::value::ValueExt>, name: Option<&str>) -> ::llvm::value::user::inst::AllocaInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlloca(::llvm::IRBuilderExt::inner(self), ::llvm::ty::TypeExt::inner(ty), array_size.map(|array_size| ::llvm::value::ValueExt::inner(array_size)).unwrap_or(::std::ptr::null_mut()), c_name);
            ::llvm::value::user::inst::AllocaInst::from_inner(ret, false)
        }
    }

    fn create_and(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAnd(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_and_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAndByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bin_op(&mut self, opcode: ::llvm::value::user::inst::BinaryOps, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBinOp(::llvm::IRBuilderExt::inner(self), opcode.to_ffi(), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bit_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBitCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_br(&mut self, dest: &::llvm::value::BasicBlockExt) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateBr(::llvm::IRBuilderExt::inner(self), ::llvm::value::BasicBlockExt::inner(dest));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_call(&mut self, callee: &::llvm::value::ValueExt, args: &[&::llvm::value::ValueExt], name: Option<&str>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueExt::inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCall(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(callee), c_args, c_name);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_cast(&mut self, opcode: ::llvm::value::user::inst::CastOps, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCast(::llvm::IRBuilderExt::inner(self), opcode.to_ffi(), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_cond_br(&mut self, cond: &::llvm::value::ValueExt, true_block: &::llvm::value::BasicBlockExt, false_block: &::llvm::value::BasicBlockExt) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateCondBr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(cond), ::llvm::value::BasicBlockExt::inner(true_block), ::llvm::value::BasicBlockExt::inner(false_block));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_exact_s_div(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactSDiv(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_exact_u_div(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactUDiv(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_element(&mut self, vec: &::llvm::value::ValueExt, idx: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractElement(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(vec), ::llvm::value::ValueExt::inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_integer(&mut self, dl: &::llvm::DataLayoutExt, from: &::llvm::value::ValueExt, extracted_ty: &::llvm::ty::IntegerTypeExt, offset: u64, name: &str) -> ::llvm::value::Value {
        unsafe {
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractInteger(::llvm::IRBuilderExt::inner(self), ::llvm::DataLayoutExt::inner(dl), ::llvm::value::ValueExt::inner(from), ::llvm::ty::IntegerTypeExt::inner(extracted_ty), offset as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_value(&mut self, agg: &::llvm::value::ValueExt, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let c_indexes = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(agg), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_add(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFAdd(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmp(::llvm::IRBuilderExt::inner(self), pred.to_ffi(), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oeq(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOEQ(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oge(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ogt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ole(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_olt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_one(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpONE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ord(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpORD(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ueq(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUEQ(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uge(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ugt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ule(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ult(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_une(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uno(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNO(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_div(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFDiv(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_mul(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFMul(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_neg(&mut self, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFNeg(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_ext(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPExt(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_si(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToSI(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_ui(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToUI(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_trunc(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPTrunc(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_rem(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFRem(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_sub(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFSub(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fence(&mut self, ordering: ::llvm::AtomicOrdering, synch_scope: Option<::llvm::SynchronizationScope>, name: Option<&str>) -> ::llvm::value::user::inst::FenceInst {
        unsafe {
            let synch_scope = synch_scope.unwrap_or(SynchronizationScope::CrossThread);
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFence(::llvm::IRBuilderExt::inner(self), ordering.to_ffi(), synch_scope.to_ffi(), c_name);
            ::llvm::value::user::inst::FenceInst::from_inner(ret, false)
        }
    }

    fn create_gep(&mut self, ptr: &::llvm::value::ValueExt, indexes: &[&::llvm::value::ValueExt], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueExt::inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateGEP(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_global_string_ptr(&mut self, str: &str, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let c_str = ::ffi::llvm_StringRef {
                data: str.as_ptr() as *const ::libc::c_char,
                length: str.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateGlobalStringPtr(::llvm::IRBuilderExt::inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmp(::llvm::IRBuilderExt::inner(self), pred.to_ffi(), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_eq(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpEQ(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ne(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpNE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sge(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sgt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sle(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_slt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_uge(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ugt(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ule(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULE(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ult(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULT(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_in_bounds_gep(&mut self, ptr: &::llvm::value::ValueExt, indexes: &[&::llvm::value::ValueExt], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueExt::inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInBoundsGEP(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_indirect_br(&mut self, addr: &::llvm::value::ValueExt, num_cases: Option<u32>) -> ::llvm::value::user::inst::IndirectBrInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateIndirectBr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(addr), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::IndirectBrInst::from_inner(ret, false)
        }
    }

    fn create_insert_element(&mut self, vec: &::llvm::value::ValueExt, new_elt: &::llvm::value::ValueExt, idx: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInsertElement(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(vec), ::llvm::value::ValueExt::inner(new_elt), ::llvm::value::ValueExt::inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_insert_value(&mut self, agg: &::llvm::value::ValueExt, value: &::llvm::value::ValueExt, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let c_indexes = ::ffi::llvm_ArrayRef__libc_c_uint {
                data: indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInsertValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(agg), ::llvm::value::ValueExt::inner(value), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, is_signed: bool, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), is_signed, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_to_ptr(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntToPtr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_invoke(&mut self, callee: &::llvm::value::ValueExt, normal_dest: &::llvm::value::BasicBlockExt, unwind_dest: &::llvm::value::BasicBlockExt, args: &[&::llvm::value::ValueExt], name: Option<&str>) -> ::llvm::value::user::inst::InvokeInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueExt::inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string_const {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInvoke(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(callee), ::llvm::value::BasicBlockExt::inner(normal_dest), ::llvm::value::BasicBlockExt::inner(unwind_dest), c_args, c_name);
            ::llvm::value::user::inst::InvokeInst::from_inner(ret, false)
        }
    }

    fn create_is_not_null(&mut self, arg: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNotNull(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_is_null(&mut self, arg: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNull(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShrByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_landing_pad(&mut self, ty: &::llvm::ty::TypeExt, pers_fn: &::llvm::value::ValueExt, num_clauses: u32, name: Option<&str>) -> ::llvm::value::user::inst::LandingPadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLandingPad(::llvm::IRBuilderExt::inner(self), ::llvm::ty::TypeExt::inner(ty), ::llvm::value::ValueExt::inner(pers_fn), num_clauses as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LandingPadInst::from_inner(ret, false)
        }
    }

    fn create_load(&mut self, ptr: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoad(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_load_volatile(&mut self, ptr: &::llvm::value::ValueExt, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoadVolatile(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_mul(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateMul(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_add(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWAdd(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_mul(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWMul(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_neg(&mut self, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWNeg(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_sub(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWSub(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_add(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWAdd(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_mul(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWMul(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_neg(&mut self, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWNeg(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_sub(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWSub(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_neg(&mut self, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNeg(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_not(&mut self, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNot(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOr(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOrByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_phi(&mut self, ty: &::llvm::ty::TypeExt, num_reserved_values: u32, name: Option<&str>) -> ::llvm::value::user::inst::PHINode {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePHI(::llvm::IRBuilderExt::inner(self), ::llvm::ty::TypeExt::inner(ty), num_reserved_values as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::PHINode::from_inner(ret, false)
        }
    }

    fn create_pointer_bit_cast_or_addr_space_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerBitCastOrAddrSpaceCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_pointer_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_diff(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrDiff(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_to_int(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrToInt(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_resume(&mut self, exn: &::llvm::value::ValueExt) -> ::llvm::value::user::inst::ResumeInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateResume(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(exn));
            ::llvm::value::user::inst::ResumeInst::from_inner(ret, false)
        }
    }

    fn create_ret(&mut self, value: &::llvm::value::ValueExt) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRet(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_ret_void(&mut self) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRetVoid(::llvm::IRBuilderExt::inner(self));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_s_div(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSDiv(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExt(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_bit_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrBitCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_trunc(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrTrunc(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_si_to_fp(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSIToFP(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_rem(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSRem(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_select(&mut self, c: &::llvm::value::ValueExt, true_value: &::llvm::value::ValueExt, false_value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSelect(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(c), ::llvm::value::ValueExt::inner(true_value), ::llvm::value::ValueExt::inner(false_value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShl(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShlByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shuffle_vector(&mut self, v1: &::llvm::value::ValueExt, p2: &::llvm::value::ValueExt, mask: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShuffleVector(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(v1), ::llvm::value::ValueExt::inner(p2), ::llvm::value::ValueExt::inner(mask), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_store(&mut self, value: &::llvm::value::ValueExt, ptr: &::llvm::value::ValueExt, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateStore(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::value::ValueExt::inner(ptr), is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_struct_gep(&mut self, ptr: &::llvm::value::ValueExt, index: u32, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateStructGEP(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(ptr), index as ::libc::c_uint, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_sub(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSub(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_switch(&mut self, value: &::llvm::value::ValueExt, dest: &::llvm::value::BasicBlockExt, num_cases: Option<u32>) -> ::llvm::value::user::inst::SwitchInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateSwitch(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::value::BasicBlockExt::inner(dest), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::SwitchInst::from_inner(ret, false)
        }
    }

    fn create_trunc(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTrunc(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_trunc_or_bit_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTruncOrBitCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_div(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUDiv(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ui_to_fp(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUIToFP(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_rem(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateURem(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_unreachable(&mut self) -> ::llvm::value::user::inst::UnreachableInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateUnreachable(::llvm::IRBuilderExt::inner(self));
            ::llvm::value::user::inst::UnreachableInst::from_inner(ret, false)
        }
    }

    fn create_va_arg(&mut self, list: &::llvm::value::ValueExt, ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::user::inst::VAArgInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVAArg(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(list), ::llvm::ty::TypeExt::inner(ty), c_name);
            ::llvm::value::user::inst::VAArgInst::from_inner(ret, false)
        }
    }

    fn create_vector_splat(&mut self, num_elements: u32, value: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVectorSplat(::llvm::IRBuilderExt::inner(self), num_elements as ::libc::c_uint, ::llvm::value::ValueExt::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor(&mut self, lhs: &::llvm::value::ValueExt, rhs: &::llvm::value::ValueExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXor(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), ::llvm::value::ValueExt::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor_by_value(&mut self, lhs: &::llvm::value::ValueExt, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXorByValue(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExt(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_bit_cast(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrBitCast(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_trunc(&mut self, value: &::llvm::value::ValueExt, dest_ty: &::llvm::ty::TypeExt, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrTrunc(::llvm::IRBuilderExt::inner(self), ::llvm::value::ValueExt::inner(value), ::llvm::ty::TypeExt::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn is_name_preserving(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_isNamePreserving(::llvm::IRBuilderExt::inner(self) as *const ::ffi::llvm_IRBuilder);
            ret
        }
    }
}

pub struct IRBuilder {
    inner: ::core::nonzero::NonZero<*mut IRBuilderInner>,
    owned: bool,
}
impl ::llvm::IRBuilderBaseExt for IRBuilder {
    fn inner(&self) -> *mut ::ffi::llvm_IRBuilderBase {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IRBuilderExt for IRBuilder {
    fn inner(&self) -> *mut IRBuilderInner {
        *self.inner
    }
}
impl IRBuilder {
    pub unsafe fn from_inner(inner: *mut IRBuilderInner, owned: bool) -> IRBuilder {
        IRBuilder {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }


    pub fn new(context: &::llvm::LLVMContextExt) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new(::llvm::LLVMContextExt::inner(context));
            if ret.is_null() {
                panic!("::llvm::IRBuilder::new returned a null pointer!");
            }
            ::llvm::IRBuilder::from_inner(ret, true)
        }
    }

    pub fn new_in_block(bb: &::llvm::value::BasicBlockExt) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new_in_block(::llvm::value::BasicBlockExt::inner(bb));
            if ret.is_null() {
                panic!("::llvm::IRBuilder::new_in_block returned a null pointer!");
            }
            ::llvm::IRBuilder::from_inner(ret, true)
        }
    }
}
impl Drop for IRBuilder {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::IRBuilder_delete(::llvm::IRBuilderExt::inner(self));
            }
        }
    }
}
pub type IRBuilderBaseInner = ::ffi::llvm_IRBuilderBase;

pub trait IRBuilderBaseExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut IRBuilderBaseInner;

    fn clear_insertion_point(&mut self) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_ClearInsertionPoint(::llvm::IRBuilderBaseExt::inner(self));
        }
    }

    fn create_global_string(&mut self, str: &str, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let c_str = ::ffi::llvm_StringRef {
                data: str.as_ptr() as *const ::libc::c_char,
                length: str.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilderBase_CreateGlobalString(::llvm::IRBuilderBaseExt::inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_lifetime_end(&mut self, ptr: &::llvm::value::ValueExt, size: Option<&::llvm::value::user::constant::ConstantIntExt>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeEnd(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::ValueExt::inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntExt::inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_lifetime_start(&mut self, ptr: &::llvm::value::ValueExt, size: Option<&::llvm::value::user::constant::ConstantIntExt>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeStart(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::ValueExt::inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntExt::inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_cpy(&mut self, dst: &::llvm::value::ValueExt, src: &::llvm::value::ValueExt, size: &::llvm::value::ValueExt, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemCpy(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::ValueExt::inner(dst), ::llvm::value::ValueExt::inner(src), ::llvm::value::ValueExt::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_move(&mut self, dst: &::llvm::value::ValueExt, src: &::llvm::value::ValueExt, size: &::llvm::value::ValueExt, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemMove(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::ValueExt::inner(dst), ::llvm::value::ValueExt::inner(src), ::llvm::value::ValueExt::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_set(&mut self, ptr: &::llvm::value::ValueExt, value: &::llvm::value::ValueExt, size: &::llvm::value::ValueExt, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemSet(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::ValueExt::inner(ptr), ::llvm::value::ValueExt::inner(value), ::llvm::value::ValueExt::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn get_insert_block(&self) -> ::llvm::value::BasicBlock {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_GetInsertBlock(::llvm::IRBuilderBaseExt::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::value::BasicBlock::from_inner(ret, false)
        }
    }

    fn set_current_debug_location(&mut self, loc: &::llvm::DebugLocExt) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetCurrentDebugLocation(::llvm::IRBuilderBaseExt::inner(self), ::llvm::DebugLocExt::inner(loc));
        }
    }

    fn set_default_fp_math_tag(&mut self, fp_math_tag: &::llvm::value::MDNodeExt) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetDefaultFPMathTag(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::MDNodeExt::inner(fp_math_tag));
        }
    }

    fn set_insert_point(&mut self, bb: &::llvm::value::BasicBlockExt) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPoint(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::BasicBlockExt::inner(bb));
        }
    }

    fn set_insert_point_at_inst(&mut self, inst: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPointAtInst(::llvm::IRBuilderBaseExt::inner(self), ::llvm::value::user::inst::InstructionExt::inner(inst));
        }
    }

    fn set_inst_debug_location(&self, inst: &::llvm::value::user::inst::InstructionExt) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInstDebugLocation(::llvm::IRBuilderBaseExt::inner(self) as *const ::ffi::llvm_IRBuilderBase, ::llvm::value::user::inst::InstructionExt::inner(inst));
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getContext(::llvm::IRBuilderBaseExt::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_current_function_return_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getCurrentFunctionReturnType(::llvm::IRBuilderBaseExt::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_default_fp_math_tag(&self) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDefaultFPMathTag(::llvm::IRBuilderBaseExt::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_double_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDoubleTy(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_false(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFalse(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_float_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFloatTy(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_half_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getHalfTy(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_int(&mut self, value: (u32, &[u64])) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let c_value = ::ffi::llvm_APInt {
                num_bits: value.0 as ::libc::c_uint,
                value: ::ffi::llvm_ArrayRef__libc_uint64_t {
                data: value.1.as_ptr(),
                length: value.1.len() as ::libc::size_t,
            },
            };
            let ret = ::ffi::llvm::IRBuilderBase_getInt(::llvm::IRBuilderBaseExt::inner(self), c_value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int1(&mut self, value: bool) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1(::llvm::IRBuilderBaseExt::inner(self), value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16(&mut self, value: u16) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16(::llvm::IRBuilderBaseExt::inner(self), value as ::libc::uint16_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16Ty(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int1_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1Ty(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int32(&mut self, value: u32) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32(::llvm::IRBuilderBaseExt::inner(self), value as ::libc::uint32_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int32_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32Ty(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int64(&mut self, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64(::llvm::IRBuilderBaseExt::inner(self), value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int64_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64Ty(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int8(&mut self, value: u8) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8(::llvm::IRBuilderBaseExt::inner(self), value as ::libc::uint8_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int8_ptr_ty(&mut self, addr_space: Option<u32>) -> ::llvm::ty::seq::PointerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getInt8PtrTy(::llvm::IRBuilderBaseExt::inner(self), addr_space as ::libc::c_uint);
            ::llvm::ty::seq::PointerType::from_inner(ret)
        }
    }

    fn get_int8_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8Ty(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_n(&mut self, num_bits: u32, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntN(::llvm::IRBuilderBaseExt::inner(self), num_bits as ::libc::c_uint, value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int_n_ty(&mut self, num_bits: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntNTy(::llvm::IRBuilderBaseExt::inner(self), num_bits as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_ptr_ty(&mut self, dl: &::llvm::DataLayoutExt, addr_space: Option<u32>) -> ::llvm::ty::IntegerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getIntPtrTy(::llvm::IRBuilderBaseExt::inner(self), ::llvm::DataLayoutExt::inner(dl), addr_space as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_true(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getTrue(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_void_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getVoidTy(::llvm::IRBuilderBaseExt::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }
}

pub struct IRBuilderBase {
    inner: ::core::nonzero::NonZero<*mut IRBuilderBaseInner>,
}
impl IRBuilderBaseExt for IRBuilderBase {
    fn inner(&self) -> *mut IRBuilderBaseInner {
        *self.inner
    }
}
impl IRBuilderBase {
    pub unsafe fn from_inner(inner: *mut IRBuilderBaseInner) -> IRBuilderBase {
        IRBuilderBase {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn new(context: &::llvm::LLVMContextExt) -> ::llvm::IRBuilderBase {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_new(::llvm::LLVMContextExt::inner(context));
            if ret.is_null() {
                panic!("::llvm::IRBuilderBase::new returned a null pointer!");
            }
            ::llvm::IRBuilderBase::from_inner(ret)
        }
    }
}
impl Copy for IRBuilderBase {}
pub type LLVMContextInner = ::ffi::llvm_LLVMContext;

pub trait LLVMContextExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut LLVMContextInner;
}

pub struct LLVMContext {
    inner: ::core::nonzero::NonZero<*mut LLVMContextInner>,
}
impl LLVMContextExt for LLVMContext {
    fn inner(&self) -> *mut LLVMContextInner {
        *self.inner
    }
}
impl LLVMContext {
    pub unsafe fn from_inner(inner: *mut LLVMContextInner) -> LLVMContext {
        LLVMContext {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn delete() -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::LLVMContext_delete();
            if ret.is_null() {
                panic!("::llvm::LLVMContext::delete returned a null pointer!");
            }
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    pub fn new() -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::LLVMContext_new();
            if ret.is_null() {
                panic!("::llvm::LLVMContext::new returned a null pointer!");
            }
            ::llvm::LLVMContext::from_inner(ret)
        }
    }
}
impl Copy for LLVMContext {}
pub type ModuleInner = ::ffi::llvm_Module;

pub trait ModuleExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ModuleInner;

    fn append_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_appendModuleInlineAsm(::llvm::ModuleExt::inner(self), c_asm);
        }
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Module_dump(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Module_getContext(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayout(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_data_layout_str(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayoutStr(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getFunction(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module, c_name);
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
            let ret = ::ffi::llvm::Module_getMDKindID(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module, c_name) as u32;
            ret
        }
    }

    fn get_module_identifier(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleIdentifier(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_module_inline_asm(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleInlineAsm(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getNamedValue(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module, c_name);
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
            let ret = ::ffi::llvm::Module_getOrInsertFunction(::llvm::ModuleExt::inner(self), c_name, ::llvm::ty::FunctionTypeExt::inner(ty));
            ::llvm::value::user::constant::Constant::from_inner(ret, false)
        }
    }

    fn get_target_triple(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getTargetTriple(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getTypeByName(::llvm::ModuleExt::inner(self) as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    fn set_data_layout(&mut self, other: &::llvm::DataLayoutExt) {
        unsafe {
            ::ffi::llvm::Module_setDataLayout(::llvm::ModuleExt::inner(self), ::llvm::DataLayoutExt::inner(other));
        }
    }

    fn set_data_layout_str(&mut self, desc: &str) {
        unsafe {
            let c_desc = ::ffi::llvm_StringRef {
                data: desc.as_ptr() as *const ::libc::c_char,
                length: desc.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setDataLayoutStr(::llvm::ModuleExt::inner(self), c_desc);
        }
    }

    fn set_module_identifier(&mut self, id: &str) {
        unsafe {
            let c_id = ::ffi::llvm_StringRef {
                data: id.as_ptr() as *const ::libc::c_char,
                length: id.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleIdentifier(::llvm::ModuleExt::inner(self), c_id);
        }
    }

    fn set_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleInlineAsm(::llvm::ModuleExt::inner(self), c_asm);
        }
    }

    fn set_target_triple(&mut self, triple: &str) {
        unsafe {
            let c_triple = ::ffi::llvm_StringRef {
                data: triple.as_ptr() as *const ::libc::c_char,
                length: triple.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setTargetTriple(::llvm::ModuleExt::inner(self), c_triple);
        }
    }
}

pub struct Module {
    inner: ::core::nonzero::NonZero<*mut ModuleInner>,
    owned: bool,
}
impl ModuleExt for Module {
    fn inner(&self) -> *mut ModuleInner {
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
            let ret = ::ffi::llvm::Module_new(c_module_id, ::llvm::LLVMContextExt::inner(context));
            if ret.is_null() {
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
                ::ffi::llvm::Module_delete(::llvm::ModuleExt::inner(self));
            }
        }
    }
}
pub type ModulePassManagerInner = ::ffi::llvm_ModulePassManager;

pub trait ModulePassManagerExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ModulePassManagerInner;
}

pub struct ModulePassManager {
    inner: ::core::nonzero::NonZero<*mut ModulePassManagerInner>,
}
impl ModulePassManagerExt for ModulePassManager {
    fn inner(&self) -> *mut ModulePassManagerInner {
        *self.inner
    }
}
impl ModulePassManager {
    pub unsafe fn from_inner(inner: *mut ModulePassManagerInner) -> ModulePassManager {
        ModulePassManager {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for ModulePassManager {}
pub type PassManagerBuilderInner = ::ffi::llvm_PassManagerBuilder;

pub trait PassManagerBuilderExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut PassManagerBuilderInner;
}

pub struct PassManagerBuilder {
    inner: ::core::nonzero::NonZero<*mut PassManagerBuilderInner>,
}
impl PassManagerBuilderExt for PassManagerBuilder {
    fn inner(&self) -> *mut PassManagerBuilderInner {
        *self.inner
    }
}
impl PassManagerBuilder {
    pub unsafe fn from_inner(inner: *mut PassManagerBuilderInner) -> PassManagerBuilder {
        PassManagerBuilder {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for PassManagerBuilder {}
pub type ValueSymbolTableInner = ::ffi::llvm_ValueSymbolTable;

pub trait ValueSymbolTableExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ValueSymbolTableInner;

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::ValueSymbolTable_dump(::llvm::ValueSymbolTableExt::inner(self) as *const ::ffi::llvm_ValueSymbolTable);
        }
    }

    fn empty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_empty(::llvm::ValueSymbolTableExt::inner(self) as *const ::ffi::llvm_ValueSymbolTable);
            ret
        }
    }

    fn lookup(&self, name: &str) -> Option<::llvm::value::Value> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ValueSymbolTable_lookup(::llvm::ValueSymbolTableExt::inner(self) as *const ::ffi::llvm_ValueSymbolTable, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn size(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_size(::llvm::ValueSymbolTableExt::inner(self) as *const ::ffi::llvm_ValueSymbolTable) as u32;
            ret
        }
    }
}

pub struct ValueSymbolTable {
    inner: ::core::nonzero::NonZero<*mut ValueSymbolTableInner>,
}
impl ValueSymbolTableExt for ValueSymbolTable {
    fn inner(&self) -> *mut ValueSymbolTableInner {
        *self.inner
    }
}
impl ValueSymbolTable {
    pub unsafe fn from_inner(inner: *mut ValueSymbolTableInner) -> ValueSymbolTable {
        ValueSymbolTable {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn delete() -> ::llvm::ValueSymbolTable {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_delete();
            if ret.is_null() {
                panic!("::llvm::ValueSymbolTable::delete returned a null pointer!");
            }
            ::llvm::ValueSymbolTable::from_inner(ret)
        }
    }

    pub fn new() -> ::llvm::ValueSymbolTable {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_new();
            if ret.is_null() {
                panic!("::llvm::ValueSymbolTable::new returned a null pointer!");
            }
            ::llvm::ValueSymbolTable::from_inner(ret)
        }
    }
}
impl Copy for ValueSymbolTable {}

pub fn get_global_context() -> ::llvm::LLVMContext {
    unsafe {
        let ret = ::ffi::llvm::getGlobalContext();
        ::llvm::LLVMContext::from_inner(ret)
    }
}

pub fn verify_function(function: &::llvm::value::user::constant::FunctionExt) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyFunction(::llvm::value::user::constant::FunctionExt::inner(function));
        ret
    }
}

pub fn verify_module(module: &::llvm::ModuleExt) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyModule(::llvm::ModuleExt::inner(module));
        ret
    }
}
