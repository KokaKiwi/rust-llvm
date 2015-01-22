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

pub trait DataLayoutObj {
    fn inner(&self) -> *mut DataLayoutInner;
}

pub trait DataLayoutExt: DataLayoutObj {
}
impl<T> DataLayoutExt for T where T: DataLayoutObj {}

pub struct DataLayout {
    inner: ::core::nonzero::NonZero<*mut DataLayoutInner>,
}
impl DataLayoutObj for DataLayout {
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

pub trait DebugLocObj {
    fn inner(&self) -> *mut DebugLocInner;
}

pub trait DebugLocExt: DebugLocObj {

    fn dump<A1: ::llvm::LLVMContextObj>(&self, ctx: &A1) {
        unsafe {
            ::ffi::llvm::DebugLoc_dump(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::inner(ctx));
        }
    }

    fn get_as_md_node<A1: ::llvm::LLVMContextObj>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getAsMDNode(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_col(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getCol(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_inlined_at<A1: ::llvm::LLVMContextObj>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getInlinedAt(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_line(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getLine(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_scope<A1: ::llvm::LLVMContextObj>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScope(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_scope_node<A1: ::llvm::LLVMContextObj>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScopeNode(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn is_unknown(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_isUnknown(::llvm::DebugLocObj::inner(self) as *const ::ffi::llvm_DebugLoc);
            ret
        }
    }
}
impl<T> DebugLocExt for T where T: DebugLocObj {}

pub struct DebugLoc {
    inner: ::core::nonzero::NonZero<*mut DebugLocInner>,
}
impl DebugLocObj for DebugLoc {
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
pub type IRBuilderInner = ::ffi::llvm_IRBuilder;

pub trait IRBuilderObj: ::llvm::IRBuilderBaseObj {
    fn inner(&self) -> *mut IRBuilderInner;
}

pub trait IRBuilderExt: IRBuilderObj {

    fn create_a_shr<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_a_shr_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShrByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_add<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAdd(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_addr_space_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAddrSpaceCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_aggregate_ret(&mut self, values: &[&::llvm::value::ValueObj]) -> Option<::llvm::value::user::inst::ReturnInst> {
        unsafe {
            let values_vec: Vec<_> = values.iter().map(|&ty| ::llvm::value::ValueObj::inner(ty)).collect();
            let ret = ::ffi::llvm::IRBuilder_CreateAggregateRet(::llvm::IRBuilderObj::inner(self), values_vec.as_slice());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::ReturnInst::from_inner(ret, false))
        }
    }

    fn create_aligned_load<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, align: u32, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoad(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), align as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_load_volatile<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, align: u32, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoadVolatile(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), align as ::libc::c_uint, is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_store<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, value: &mut A1, ptr: &mut A2, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedStore(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::value::ValueObj::inner(ptr), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_alloca<A1: ::llvm::ty::TypeObj, A2: ::llvm::value::ValueObj>(&mut self, ty: &mut A1, array_size: Option<&mut A2>, name: Option<&str>) -> ::llvm::value::user::inst::AllocaInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlloca(::llvm::IRBuilderObj::inner(self), ::llvm::ty::TypeObj::inner(ty), array_size.map(|array_size| ::llvm::value::ValueObj::inner(array_size)).unwrap_or(::std::ptr::null_mut()), c_name);
            ::llvm::value::user::inst::AllocaInst::from_inner(ret, false)
        }
    }

    fn create_and<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAnd(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_and_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAndByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bin_op<A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, opcode: ::llvm::value::user::inst::BinaryOps, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBinOp(::llvm::IRBuilderObj::inner(self), opcode.to_ffi(), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bit_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBitCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_br<A1: ::llvm::value::BasicBlockObj>(&mut self, dest: &mut A1) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateBr(::llvm::IRBuilderObj::inner(self), ::llvm::value::BasicBlockObj::inner(dest));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_call<A1: ::llvm::value::ValueObj>(&mut self, callee: &mut A1, args: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueObj::inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCall(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(callee), c_args, c_name);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_cast<A2: ::llvm::value::ValueObj, A3: ::llvm::ty::TypeObj>(&mut self, opcode: ::llvm::value::user::inst::CastOps, value: &mut A2, dest_ty: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCast(::llvm::IRBuilderObj::inner(self), opcode.to_ffi(), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_cond_br<A1: ::llvm::value::ValueObj, A2: ::llvm::value::BasicBlockObj, A3: ::llvm::value::BasicBlockObj>(&mut self, cond: &mut A1, true_block: &mut A2, false_block: &mut A3) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateCondBr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(cond), ::llvm::value::BasicBlockObj::inner(true_block), ::llvm::value::BasicBlockObj::inner(false_block));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_exact_s_div<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactSDiv(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_exact_u_div<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactUDiv(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_element<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, vec: &mut A1, idx: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractElement(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(vec), ::llvm::value::ValueObj::inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_integer<A1: ::llvm::DataLayoutObj, A2: ::llvm::value::ValueObj, A3: ::llvm::ty::IntegerTypeObj>(&mut self, dl: &A1, from: &mut A2, extracted_ty: &mut A3, offset: u64, name: &str) -> ::llvm::value::Value {
        unsafe {
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractInteger(::llvm::IRBuilderObj::inner(self), ::llvm::DataLayoutObj::inner(dl), ::llvm::value::ValueObj::inner(from), ::llvm::ty::IntegerTypeObj::inner(extracted_ty), offset as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_value<A1: ::llvm::value::ValueObj>(&mut self, agg: &mut A1, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
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
            let ret = ::ffi::llvm::IRBuilder_CreateExtractValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(agg), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_add<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFAdd(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp<A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmp(::llvm::IRBuilderObj::inner(self), pred.to_ffi(), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oeq<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOEQ(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oge<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ogt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ole<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_olt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_one<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpONE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ord<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpORD(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ueq<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUEQ(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uge<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ugt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ule<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ult<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_une<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uno<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNO(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_div<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFDiv(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_mul<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFMul(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_neg<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFNeg(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_ext<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPExt(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_si<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToSI(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_ui<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToUI(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_trunc<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPTrunc(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_rem<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFRem(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_sub<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFSub(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
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
            let ret = ::ffi::llvm::IRBuilder_CreateFence(::llvm::IRBuilderObj::inner(self), ordering.to_ffi(), synch_scope.to_ffi(), c_name);
            ::llvm::value::user::inst::FenceInst::from_inner(ret, false)
        }
    }

    fn create_gep<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, indexes: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueObj::inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateGEP(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), c_indexes, c_name);
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
            let ret = ::ffi::llvm::IRBuilder_CreateGlobalStringPtr(::llvm::IRBuilderObj::inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp<A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmp(::llvm::IRBuilderObj::inner(self), pred.to_ffi(), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_eq<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpEQ(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ne<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpNE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sge<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sgt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sle<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_slt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_uge<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ugt<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ule<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULE(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ult<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULT(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_in_bounds_gep<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, indexes: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueObj::inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInBoundsGEP(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_indirect_br<A1: ::llvm::value::ValueObj>(&mut self, addr: &mut A1, num_cases: Option<u32>) -> ::llvm::value::user::inst::IndirectBrInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateIndirectBr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(addr), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::IndirectBrInst::from_inner(ret, false)
        }
    }

    fn create_insert_element<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, vec: &mut A1, new_elt: &mut A2, idx: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInsertElement(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(vec), ::llvm::value::ValueObj::inner(new_elt), ::llvm::value::ValueObj::inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_insert_value<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, agg: &mut A1, value: &mut A2, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
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
            let ret = ::ffi::llvm::IRBuilder_CreateInsertValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(agg), ::llvm::value::ValueObj::inner(value), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, is_signed: bool, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), is_signed, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_to_ptr<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntToPtr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_invoke<A1: ::llvm::value::ValueObj, A2: ::llvm::value::BasicBlockObj, A3: ::llvm::value::BasicBlockObj>(&mut self, callee: &mut A1, normal_dest: &mut A2, unwind_dest: &mut A3, args: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::user::inst::InvokeInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueObj::inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string_const {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInvoke(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(callee), ::llvm::value::BasicBlockObj::inner(normal_dest), ::llvm::value::BasicBlockObj::inner(unwind_dest), c_args, c_name);
            ::llvm::value::user::inst::InvokeInst::from_inner(ret, false)
        }
    }

    fn create_is_not_null<A1: ::llvm::value::ValueObj>(&mut self, arg: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNotNull(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_is_null<A1: ::llvm::value::ValueObj>(&mut self, arg: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNull(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShrByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_landing_pad<A1: ::llvm::ty::TypeObj, A2: ::llvm::value::ValueObj>(&mut self, ty: &mut A1, pers_fn: &mut A2, num_clauses: u32, name: Option<&str>) -> ::llvm::value::user::inst::LandingPadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLandingPad(::llvm::IRBuilderObj::inner(self), ::llvm::ty::TypeObj::inner(ty), ::llvm::value::ValueObj::inner(pers_fn), num_clauses as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LandingPadInst::from_inner(ret, false)
        }
    }

    fn create_load<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoad(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_load_volatile<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoadVolatile(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_mul<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateMul(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_add<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWAdd(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_mul<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWMul(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_neg<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWNeg(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_sub<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWSub(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_add<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWAdd(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_mul<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWMul(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_neg<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWNeg(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_sub<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWSub(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_neg<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNeg(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_not<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNot(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOr(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOrByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_phi<A1: ::llvm::ty::TypeObj>(&mut self, ty: &mut A1, num_reserved_values: u32, name: Option<&str>) -> ::llvm::value::user::inst::PHINode {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePHI(::llvm::IRBuilderObj::inner(self), ::llvm::ty::TypeObj::inner(ty), num_reserved_values as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::PHINode::from_inner(ret, false)
        }
    }

    fn create_pointer_bit_cast_or_addr_space_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerBitCastOrAddrSpaceCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_pointer_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_diff<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrDiff(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_to_int<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrToInt(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_resume<A1: ::llvm::value::ValueObj>(&mut self, exn: &mut A1) -> ::llvm::value::user::inst::ResumeInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateResume(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(exn));
            ::llvm::value::user::inst::ResumeInst::from_inner(ret, false)
        }
    }

    fn create_ret<A1: ::llvm::value::ValueObj>(&mut self, value: &mut A1) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRet(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_ret_void(&mut self) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRetVoid(::llvm::IRBuilderObj::inner(self));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_s_div<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSDiv(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExt(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_bit_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrBitCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_trunc<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrTrunc(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_si_to_fp<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSIToFP(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_rem<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSRem(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_select<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, c: &mut A1, true_value: &mut A2, false_value: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSelect(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(c), ::llvm::value::ValueObj::inner(true_value), ::llvm::value::ValueObj::inner(false_value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShl(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShlByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shuffle_vector<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, v1: &mut A1, p2: &mut A2, mask: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShuffleVector(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(v1), ::llvm::value::ValueObj::inner(p2), ::llvm::value::ValueObj::inner(mask), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_store<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, value: &mut A1, ptr: &mut A2, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateStore(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::value::ValueObj::inner(ptr), is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_struct_gep<A1: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, index: u32, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateStructGEP(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(ptr), index as ::libc::c_uint, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_sub<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSub(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_switch<A1: ::llvm::value::ValueObj, A2: ::llvm::value::BasicBlockObj>(&mut self, value: &mut A1, dest: &mut A2, num_cases: Option<u32>) -> ::llvm::value::user::inst::SwitchInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateSwitch(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::value::BasicBlockObj::inner(dest), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::SwitchInst::from_inner(ret, false)
        }
    }

    fn create_trunc<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTrunc(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_trunc_or_bit_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTruncOrBitCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_div<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUDiv(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ui_to_fp<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUIToFP(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_rem<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateURem(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_unreachable(&mut self) -> ::llvm::value::user::inst::UnreachableInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateUnreachable(::llvm::IRBuilderObj::inner(self));
            ::llvm::value::user::inst::UnreachableInst::from_inner(ret, false)
        }
    }

    fn create_va_arg<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, list: &mut A1, ty: &mut A2, name: Option<&str>) -> ::llvm::value::user::inst::VAArgInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVAArg(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(list), ::llvm::ty::TypeObj::inner(ty), c_name);
            ::llvm::value::user::inst::VAArgInst::from_inner(ret, false)
        }
    }

    fn create_vector_splat<A2: ::llvm::value::ValueObj>(&mut self, num_elements: u32, value: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVectorSplat(::llvm::IRBuilderObj::inner(self), num_elements as ::libc::c_uint, ::llvm::value::ValueObj::inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXor(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), ::llvm::value::ValueObj::inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor_by_value<A1: ::llvm::value::ValueObj>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXorByValue(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExt(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_bit_cast<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrBitCast(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_trunc<A1: ::llvm::value::ValueObj, A2: ::llvm::ty::TypeObj>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrTrunc(::llvm::IRBuilderObj::inner(self), ::llvm::value::ValueObj::inner(value), ::llvm::ty::TypeObj::inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn is_name_preserving(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_isNamePreserving(::llvm::IRBuilderObj::inner(self) as *const ::ffi::llvm_IRBuilder);
            ret
        }
    }
}
impl<T> IRBuilderExt for T where T: IRBuilderObj {}

pub struct IRBuilder {
    inner: ::core::nonzero::NonZero<*mut IRBuilderInner>,
    owned: bool,
}
impl ::llvm::IRBuilderBaseObj for IRBuilder {
    fn inner(&self) -> *mut ::ffi::llvm_IRBuilderBase {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IRBuilderObj for IRBuilder {
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


    pub fn new<A1: ::llvm::LLVMContextObj>(context: &mut A1) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new(::llvm::LLVMContextObj::inner(context));
            if ret.is_null() {
                panic!("::llvm::IRBuilder::new returned a null pointer!");
            }
            ::llvm::IRBuilder::from_inner(ret, true)
        }
    }

    pub fn new_in_block<A1: ::llvm::value::BasicBlockObj>(bb: &mut A1) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new_in_block(::llvm::value::BasicBlockObj::inner(bb));
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
                ::ffi::llvm::IRBuilder_delete(::llvm::IRBuilderObj::inner(self));
            }
        }
    }
}
pub type IRBuilderBaseInner = ::ffi::llvm_IRBuilderBase;

pub trait IRBuilderBaseObj {
    fn inner(&self) -> *mut IRBuilderBaseInner;
}

pub trait IRBuilderBaseExt: IRBuilderBaseObj {

    fn clear_insertion_point(&mut self) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_ClearInsertionPoint(::llvm::IRBuilderBaseObj::inner(self));
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
            let ret = ::ffi::llvm::IRBuilderBase_CreateGlobalString(::llvm::IRBuilderBaseObj::inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_lifetime_end<A1: ::llvm::value::ValueObj, A2: ::llvm::value::user::constant::ConstantIntObj>(&mut self, ptr: &mut A1, size: Option<&mut A2>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeEnd(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::ValueObj::inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntObj::inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_lifetime_start<A1: ::llvm::value::ValueObj, A2: ::llvm::value::user::constant::ConstantIntObj>(&mut self, ptr: &mut A1, size: Option<&mut A2>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeStart(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::ValueObj::inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntObj::inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_cpy<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, dst: &mut A1, src: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemCpy(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::ValueObj::inner(dst), ::llvm::value::ValueObj::inner(src), ::llvm::value::ValueObj::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_move<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, dst: &mut A1, src: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemMove(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::ValueObj::inner(dst), ::llvm::value::ValueObj::inner(src), ::llvm::value::ValueObj::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_set<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj, A3: ::llvm::value::ValueObj>(&mut self, ptr: &mut A1, value: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemSet(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::ValueObj::inner(ptr), ::llvm::value::ValueObj::inner(value), ::llvm::value::ValueObj::inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn get_insert_block(&self) -> ::llvm::value::BasicBlock {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_GetInsertBlock(::llvm::IRBuilderBaseObj::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::value::BasicBlock::from_inner(ret, false)
        }
    }

    fn set_current_debug_location<A1: ::llvm::DebugLocObj>(&mut self, loc: &A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetCurrentDebugLocation(::llvm::IRBuilderBaseObj::inner(self), ::llvm::DebugLocObj::inner(loc));
        }
    }

    fn set_default_fp_math_tag<A1: ::llvm::value::MDNodeObj>(&mut self, fp_math_tag: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetDefaultFPMathTag(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::MDNodeObj::inner(fp_math_tag));
        }
    }

    fn set_insert_point<A1: ::llvm::value::BasicBlockObj>(&mut self, bb: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPoint(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::BasicBlockObj::inner(bb));
        }
    }

    fn set_insert_point_at_inst<A1: ::llvm::value::user::inst::InstructionObj>(&mut self, inst: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPointAtInst(::llvm::IRBuilderBaseObj::inner(self), ::llvm::value::user::inst::InstructionObj::inner(inst));
        }
    }

    fn set_inst_debug_location<A1: ::llvm::value::user::inst::InstructionObj>(&self, inst: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInstDebugLocation(::llvm::IRBuilderBaseObj::inner(self) as *const ::ffi::llvm_IRBuilderBase, ::llvm::value::user::inst::InstructionObj::inner(inst));
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getContext(::llvm::IRBuilderBaseObj::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_current_function_return_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getCurrentFunctionReturnType(::llvm::IRBuilderBaseObj::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_default_fp_math_tag(&self) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDefaultFPMathTag(::llvm::IRBuilderBaseObj::inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_double_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDoubleTy(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_false(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFalse(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_float_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFloatTy(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_half_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getHalfTy(::llvm::IRBuilderBaseObj::inner(self));
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
            let ret = ::ffi::llvm::IRBuilderBase_getInt(::llvm::IRBuilderBaseObj::inner(self), c_value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int1(&mut self, value: bool) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1(::llvm::IRBuilderBaseObj::inner(self), value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16(&mut self, value: u16) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16(::llvm::IRBuilderBaseObj::inner(self), value as ::libc::uint16_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16Ty(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int1_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1Ty(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int32(&mut self, value: u32) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32(::llvm::IRBuilderBaseObj::inner(self), value as ::libc::uint32_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int32_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32Ty(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int64(&mut self, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64(::llvm::IRBuilderBaseObj::inner(self), value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int64_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64Ty(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int8(&mut self, value: u8) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8(::llvm::IRBuilderBaseObj::inner(self), value as ::libc::uint8_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int8_ptr_ty(&mut self, addr_space: Option<u32>) -> ::llvm::ty::seq::PointerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getInt8PtrTy(::llvm::IRBuilderBaseObj::inner(self), addr_space as ::libc::c_uint);
            ::llvm::ty::seq::PointerType::from_inner(ret)
        }
    }

    fn get_int8_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8Ty(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_n(&mut self, num_bits: u32, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntN(::llvm::IRBuilderBaseObj::inner(self), num_bits as ::libc::c_uint, value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int_n_ty(&mut self, num_bits: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntNTy(::llvm::IRBuilderBaseObj::inner(self), num_bits as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_ptr_ty<A1: ::llvm::DataLayoutObj>(&mut self, dl: &A1, addr_space: Option<u32>) -> ::llvm::ty::IntegerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getIntPtrTy(::llvm::IRBuilderBaseObj::inner(self), ::llvm::DataLayoutObj::inner(dl), addr_space as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_true(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getTrue(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_void_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getVoidTy(::llvm::IRBuilderBaseObj::inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }
}
impl<T> IRBuilderBaseExt for T where T: IRBuilderBaseObj {}

pub struct IRBuilderBase {
    inner: ::core::nonzero::NonZero<*mut IRBuilderBaseInner>,
}
impl IRBuilderBaseObj for IRBuilderBase {
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

    pub fn new<A1: ::llvm::LLVMContextObj>(context: &mut A1) -> ::llvm::IRBuilderBase {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_new(::llvm::LLVMContextObj::inner(context));
            if ret.is_null() {
                panic!("::llvm::IRBuilderBase::new returned a null pointer!");
            }
            ::llvm::IRBuilderBase::from_inner(ret)
        }
    }
}
impl Copy for IRBuilderBase {}
pub type LLVMContextInner = ::ffi::llvm_LLVMContext;

pub trait LLVMContextObj {
    fn inner(&self) -> *mut LLVMContextInner;
}

pub trait LLVMContextExt: LLVMContextObj {
}
impl<T> LLVMContextExt for T where T: LLVMContextObj {}

pub struct LLVMContext {
    inner: ::core::nonzero::NonZero<*mut LLVMContextInner>,
}
impl LLVMContextObj for LLVMContext {
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

pub trait ModuleObj {
    fn inner(&self) -> *mut ModuleInner;
}

pub trait ModuleExt: ModuleObj {

    fn append_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_appendModuleInlineAsm(::llvm::ModuleObj::inner(self), c_asm);
        }
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Module_dump(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Module_getContext(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayout(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_data_layout_str(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayoutStr(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getFunction(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module, c_name);
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
            let ret = ::ffi::llvm::Module_getMDKindID(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module, c_name) as u32;
            ret
        }
    }

    fn get_module_identifier(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleIdentifier(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_module_inline_asm(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleInlineAsm(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getNamedValue(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::GlobalValue::from_inner(ret, false))
        }
    }

    fn get_or_insert_function<A2: ::llvm::ty::FunctionTypeObj>(&mut self, name: &str, ty: &mut A2) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getOrInsertFunction(::llvm::ModuleObj::inner(self), c_name, ::llvm::ty::FunctionTypeObj::inner(ty));
            ::llvm::value::user::constant::Constant::from_inner(ret, false)
        }
    }

    fn get_target_triple(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getTargetTriple(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getTypeByName(::llvm::ModuleObj::inner(self) as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    fn set_data_layout<A1: ::llvm::DataLayoutObj>(&mut self, other: &A1) {
        unsafe {
            ::ffi::llvm::Module_setDataLayout(::llvm::ModuleObj::inner(self), ::llvm::DataLayoutObj::inner(other));
        }
    }

    fn set_data_layout_str(&mut self, desc: &str) {
        unsafe {
            let c_desc = ::ffi::llvm_StringRef {
                data: desc.as_ptr() as *const ::libc::c_char,
                length: desc.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setDataLayoutStr(::llvm::ModuleObj::inner(self), c_desc);
        }
    }

    fn set_module_identifier(&mut self, id: &str) {
        unsafe {
            let c_id = ::ffi::llvm_StringRef {
                data: id.as_ptr() as *const ::libc::c_char,
                length: id.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleIdentifier(::llvm::ModuleObj::inner(self), c_id);
        }
    }

    fn set_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleInlineAsm(::llvm::ModuleObj::inner(self), c_asm);
        }
    }

    fn set_target_triple(&mut self, triple: &str) {
        unsafe {
            let c_triple = ::ffi::llvm_StringRef {
                data: triple.as_ptr() as *const ::libc::c_char,
                length: triple.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setTargetTriple(::llvm::ModuleObj::inner(self), c_triple);
        }
    }
}
impl<T> ModuleExt for T where T: ModuleObj {}

pub struct Module {
    inner: ::core::nonzero::NonZero<*mut ModuleInner>,
    owned: bool,
}
impl ModuleObj for Module {
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

    pub fn new<A2: ::llvm::LLVMContextObj>(module_id: &str, context: &mut A2) -> ::llvm::Module {
        unsafe {
            let c_module_id = ::ffi::llvm_StringRef {
                data: module_id.as_ptr() as *const ::libc::c_char,
                length: module_id.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_new(c_module_id, ::llvm::LLVMContextObj::inner(context));
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
                ::ffi::llvm::Module_delete(::llvm::ModuleObj::inner(self));
            }
        }
    }
}
pub type ValueSymbolTableInner = ::ffi::llvm_ValueSymbolTable;

pub trait ValueSymbolTableObj {
    fn inner(&self) -> *mut ValueSymbolTableInner;
}

pub trait ValueSymbolTableExt: ValueSymbolTableObj {

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::ValueSymbolTable_dump(::llvm::ValueSymbolTableObj::inner(self) as *const ::ffi::llvm_ValueSymbolTable);
        }
    }

    fn empty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_empty(::llvm::ValueSymbolTableObj::inner(self) as *const ::ffi::llvm_ValueSymbolTable);
            ret
        }
    }

    fn lookup(&self, name: &str) -> Option<::llvm::value::Value> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ValueSymbolTable_lookup(::llvm::ValueSymbolTableObj::inner(self) as *const ::ffi::llvm_ValueSymbolTable, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn size(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_size(::llvm::ValueSymbolTableObj::inner(self) as *const ::ffi::llvm_ValueSymbolTable) as u32;
            ret
        }
    }
}
impl<T> ValueSymbolTableExt for T where T: ValueSymbolTableObj {}

pub struct ValueSymbolTable {
    inner: ::core::nonzero::NonZero<*mut ValueSymbolTableInner>,
}
impl ValueSymbolTableObj for ValueSymbolTable {
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

pub fn verify_function<A1: ::llvm::value::user::constant::FunctionObj>(function: &A1) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyFunction(::llvm::value::user::constant::FunctionObj::inner(function));
        ret
    }
}

pub fn verify_module<A1: ::llvm::ModuleObj>(module: &A1) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyModule(::llvm::ModuleObj::inner(module));
        ret
    }
}
