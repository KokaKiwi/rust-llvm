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
    unsafe fn get_inner(&self) -> *mut DataLayoutInner;
}

pub trait DataLayoutOwned: DataLayoutObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut DataLayoutInner {
        let inner = DataLayoutObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> DataLayoutOwned for T where T: DataLayoutObj + ::core::marker::Sized {}

pub trait DataLayoutExt: DataLayoutObj {
}
impl<T> DataLayoutExt for T where T: DataLayoutObj {}

pub struct DataLayout {
    inner: ::core::nonzero::NonZero<*mut DataLayoutInner>,
}
impl DataLayoutObj for DataLayout {
    #[inline(always)]
    fn get_inner(&self) -> *mut DataLayoutInner {
        *self.inner
    }
}
impl DataLayout {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut DataLayoutInner) -> DataLayout {
        DataLayout {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }
}
impl Copy for DataLayout {}
pub type DebugLocInner = ::ffi::llvm_DebugLoc;

pub trait DebugLocObj {
    unsafe fn get_inner(&self) -> *mut DebugLocInner;
}

pub trait DebugLocOwned: DebugLocObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut DebugLocInner {
        let inner = DebugLocObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> DebugLocOwned for T where T: DebugLocObj + ::core::marker::Sized {}

pub trait DebugLocExt: DebugLocObj {

    fn dump<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(&self, ctx: &A1) {
        unsafe {
            ::ffi::llvm::DebugLoc_dump(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::get_inner(ctx));
        }
    }

    fn get_as_md_node<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getAsMDNode(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::get_inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_col(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getCol(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_inlined_at<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getInlinedAt(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::get_inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_line(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getLine(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc) as u32;
            ret
        }
    }

    fn get_scope<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScope(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::get_inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_scope_node<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(&self, ctx: &A1) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_getScopeNode(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc, ::llvm::LLVMContextObj::get_inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn is_unknown(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::DebugLoc_isUnknown(::llvm::DebugLocObj::get_inner(self) as *const ::ffi::llvm_DebugLoc);
            ret
        }
    }
}
impl<T> DebugLocExt for T where T: DebugLocObj {}

pub struct DebugLoc {
    inner: ::core::nonzero::NonZero<*mut DebugLocInner>,
}
impl DebugLocObj for DebugLoc {
    #[inline(always)]
    fn get_inner(&self) -> *mut DebugLocInner {
        *self.inner
    }
}
impl DebugLoc {
    #[inline(always)]
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
    unsafe fn get_inner(&self) -> *mut IRBuilderInner;
}

pub trait IRBuilderOwned: IRBuilderObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut IRBuilderInner {
        let inner = IRBuilderObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> IRBuilderOwned for T where T: IRBuilderObj + ::core::marker::Sized {}

pub trait IRBuilderExt: IRBuilderObj {

    fn create_a_shr<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_a_shr_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAShrByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_add<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAdd(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_addr_space_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAddrSpaceCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_aggregate_ret(&mut self, values: &[&::llvm::value::ValueObj]) -> Option<::llvm::value::user::inst::ReturnInst> {
        unsafe {
            let values_vec: Vec<_> = values.iter().map(|&ty| ::llvm::value::ValueObj::get_inner(ty)).collect();
            let ret = ::ffi::llvm::IRBuilder_CreateAggregateRet(::llvm::IRBuilderObj::get_inner(self), values_vec.as_slice());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::inst::ReturnInst::from_inner(ret, false))
        }
    }

    fn create_aligned_load<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, align: u32, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoad(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), align as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_load_volatile<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, align: u32, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedLoadVolatile(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), align as ::libc::c_uint, is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_aligned_store<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, ptr: &mut A2, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateAlignedStore(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::value::ValueObj::get_inner(ptr), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_alloca<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ty: &mut A1, array_size: Option<&mut A2>, name: Option<&str>) -> ::llvm::value::user::inst::AllocaInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAlloca(::llvm::IRBuilderObj::get_inner(self), ::llvm::ty::TypeObj::get_inner(ty), array_size.map(|array_size| ::llvm::value::ValueObj::get_inner(array_size)).unwrap_or(::std::ptr::null_mut()), c_name);
            ::llvm::value::user::inst::AllocaInst::from_inner(ret, false)
        }
    }

    fn create_and<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAnd(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_and_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateAndByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bin_op<A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, opcode: ::llvm::value::user::inst::BinaryOps, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBinOp(::llvm::IRBuilderObj::get_inner(self), opcode.to_ffi(), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_bit_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateBitCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_br<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, dest: &mut A1) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateBr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(dest));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_call<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, callee: &mut A1, args: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueObj::get_inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCall(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(callee), c_args, c_name);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_cast<A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, opcode: ::llvm::value::user::inst::CastOps, value: &mut A2, dest_ty: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateCast(::llvm::IRBuilderObj::get_inner(self), opcode.to_ffi(), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_cond_br<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock, A3: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, cond: &mut A1, true_block: &mut A2, false_block: &mut A3) -> ::llvm::value::user::inst::BranchInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateCondBr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(cond), ::llvm::value::BasicBlockObj::get_inner(true_block), ::llvm::value::BasicBlockObj::get_inner(false_block));
            ::llvm::value::user::inst::BranchInst::from_inner(ret, false)
        }
    }

    fn create_exact_s_div<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactSDiv(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_exact_u_div<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExactUDiv(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_element<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, vec: &mut A1, idx: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractElement(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(vec), ::llvm::value::ValueObj::get_inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_integer<A1: ::llvm::DataLayoutObj = ::llvm::DataLayout, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::ty::IntegerTypeObj = ::llvm::ty::IntegerType>(&mut self, dl: &A1, from: &mut A2, extracted_ty: &mut A3, offset: u64, name: &str) -> ::llvm::value::Value {
        unsafe {
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateExtractInteger(::llvm::IRBuilderObj::get_inner(self), ::llvm::DataLayoutObj::get_inner(dl), ::llvm::value::ValueObj::get_inner(from), ::llvm::ty::IntegerTypeObj::get_inner(extracted_ty), offset as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_extract_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, agg: &mut A1, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
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
            let ret = ::ffi::llvm::IRBuilder_CreateExtractValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(agg), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_add<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFAdd(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp<A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmp(::llvm::IRBuilderObj::get_inner(self), pred.to_ffi(), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oeq<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOEQ(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_oge<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ogt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOGT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ole<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_olt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpOLT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_one<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpONE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ord<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpORD(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ueq<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUEQ(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uge<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ugt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUGT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ule<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_ult<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpULT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_une<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_cmp_uno<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFCmpUNO(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_div<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFDiv(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_mul<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFMul(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_neg<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFNeg(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_ext<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPExt(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_si<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToSI(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_to_ui<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPToUI(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_fp_trunc<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFPTrunc(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_rem<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFRem(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_f_sub<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateFSub(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
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
            let ret = ::ffi::llvm::IRBuilder_CreateFence(::llvm::IRBuilderObj::get_inner(self), ordering.to_ffi(), synch_scope.to_ffi(), c_name);
            ::llvm::value::user::inst::FenceInst::from_inner(ret, false)
        }
    }

    fn create_gep<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, indexes: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueObj::get_inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateGEP(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), c_indexes, c_name);
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
            let ret = ::ffi::llvm::IRBuilder_CreateGlobalStringPtr(::llvm::IRBuilderObj::get_inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp<A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, pred: ::llvm::value::user::inst::Predicate, lhs: &mut A2, rhs: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmp(::llvm::IRBuilderObj::get_inner(self), pred.to_ffi(), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_eq<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpEQ(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ne<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpNE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sge<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sgt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSGT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_sle<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_slt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpSLT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_uge<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ugt<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpUGT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ule<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULE(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_i_cmp_ult<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateICmpULT(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_in_bounds_gep<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, indexes: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let _tmp_indexes: Vec<_> = indexes.iter().map(|&ty| ::llvm::value::ValueObj::get_inner(ty)).collect();
            let c_indexes = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_indexes.as_ptr(),
                length: indexes.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInBoundsGEP(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_indirect_br<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, addr: &mut A1, num_cases: Option<u32>) -> ::llvm::value::user::inst::IndirectBrInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateIndirectBr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(addr), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::IndirectBrInst::from_inner(ret, false)
        }
    }

    fn create_insert_element<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, vec: &mut A1, new_elt: &mut A2, idx: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInsertElement(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(vec), ::llvm::value::ValueObj::get_inner(new_elt), ::llvm::value::ValueObj::get_inner(idx), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_insert_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, agg: &mut A1, value: &mut A2, indexes: &[u32], name: Option<&str>) -> ::llvm::value::Value {
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
            let ret = ::ffi::llvm::IRBuilder_CreateInsertValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(agg), ::llvm::value::ValueObj::get_inner(value), c_indexes, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, is_signed: bool, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), is_signed, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_int_to_ptr<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIntToPtr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_invoke<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock, A3: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, callee: &mut A1, normal_dest: &mut A2, unwind_dest: &mut A3, args: &[&::llvm::value::ValueObj], name: Option<&str>) -> ::llvm::value::user::inst::InvokeInst {
        unsafe {
            let _tmp_args: Vec<_> = args.iter().map(|&ty| ::llvm::value::ValueObj::get_inner(ty)).collect();
            let c_args = ::ffi::llvm_ArrayRef_llvm_Value_ptr {
                data: _tmp_args.as_ptr(),
                length: args.len() as ::libc::size_t,
            };
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string_const {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateInvoke(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(callee), ::llvm::value::BasicBlockObj::get_inner(normal_dest), ::llvm::value::BasicBlockObj::get_inner(unwind_dest), c_args, c_name);
            ::llvm::value::user::inst::InvokeInst::from_inner(ret, false)
        }
    }

    fn create_is_not_null<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, arg: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNotNull(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_is_null<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, arg: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateIsNull(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(arg), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_l_shr_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLShrByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_landing_pad<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ty: &mut A1, pers_fn: &mut A2, num_clauses: u32, name: Option<&str>) -> ::llvm::value::user::inst::LandingPadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLandingPad(::llvm::IRBuilderObj::get_inner(self), ::llvm::ty::TypeObj::get_inner(ty), ::llvm::value::ValueObj::get_inner(pers_fn), num_clauses as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::LandingPadInst::from_inner(ret, false)
        }
    }

    fn create_load<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoad(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_load_volatile<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, is_volatile: bool, name: Option<&str>) -> ::llvm::value::user::inst::LoadInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateLoadVolatile(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), is_volatile, c_name);
            ::llvm::value::user::inst::LoadInst::from_inner(ret, false)
        }
    }

    fn create_mul<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateMul(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_add<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWAdd(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_mul<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWMul(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_neg<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWNeg(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nsw_sub<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNSWSub(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_add<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWAdd(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_mul<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWMul(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_neg<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWNeg(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_nuw_sub<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNUWSub(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_neg<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNeg(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_not<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateNot(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOr(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_or_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateOrByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_phi<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, ty: &mut A1, num_reserved_values: u32, name: Option<&str>) -> ::llvm::value::user::inst::PHINode {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePHI(::llvm::IRBuilderObj::get_inner(self), ::llvm::ty::TypeObj::get_inner(ty), num_reserved_values as ::libc::c_uint, c_name);
            ::llvm::value::user::inst::PHINode::from_inner(ret, false)
        }
    }

    fn create_pointer_bit_cast_or_addr_space_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerBitCastOrAddrSpaceCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_pointer_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePointerCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_diff<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrDiff(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ptr_to_int<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreatePtrToInt(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_resume<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, exn: &mut A1) -> ::llvm::value::user::inst::ResumeInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateResume(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(exn));
            ::llvm::value::user::inst::ResumeInst::from_inner(ret, false)
        }
    }

    fn create_ret<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRet(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_ret_void(&mut self) -> ::llvm::value::user::inst::ReturnInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateRetVoid(::llvm::IRBuilderObj::get_inner(self));
            ::llvm::value::user::inst::ReturnInst::from_inner(ret, false)
        }
    }

    fn create_s_div<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSDiv(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExt(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_bit_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrBitCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_ext_or_trunc<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSExtOrTrunc(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_si_to_fp<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSIToFP(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_s_rem<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSRem(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_select<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, c: &mut A1, true_value: &mut A2, false_value: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSelect(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(c), ::llvm::value::ValueObj::get_inner(true_value), ::llvm::value::ValueObj::get_inner(false_value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShl(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shl_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShlByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_shuffle_vector<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, v1: &mut A1, p2: &mut A2, mask: &mut A3, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateShuffleVector(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(v1), ::llvm::value::ValueObj::get_inner(p2), ::llvm::value::ValueObj::get_inner(mask), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_store<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, value: &mut A1, ptr: &mut A2, is_volatile: Option<bool>) -> ::llvm::value::user::inst::StoreInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilder_CreateStore(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::value::ValueObj::get_inner(ptr), is_volatile);
            ::llvm::value::user::inst::StoreInst::from_inner(ret, false)
        }
    }

    fn create_struct_gep<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, index: u32, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateStructGEP(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), index as ::libc::c_uint, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_sub<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateSub(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_switch<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, value: &mut A1, dest: &mut A2, num_cases: Option<u32>) -> ::llvm::value::user::inst::SwitchInst {
        unsafe {
            let num_cases = num_cases.unwrap_or(10);
            let ret = ::ffi::llvm::IRBuilder_CreateSwitch(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::value::BasicBlockObj::get_inner(dest), num_cases as ::libc::c_uint);
            ::llvm::value::user::inst::SwitchInst::from_inner(ret, false)
        }
    }

    fn create_trunc<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTrunc(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_trunc_or_bit_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateTruncOrBitCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_div<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUDiv(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_ui_to_fp<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateUIToFP(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_u_rem<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateURem(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_unreachable(&mut self) -> ::llvm::value::user::inst::UnreachableInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_CreateUnreachable(::llvm::IRBuilderObj::get_inner(self));
            ::llvm::value::user::inst::UnreachableInst::from_inner(ret, false)
        }
    }

    fn create_va_arg<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, list: &mut A1, ty: &mut A2, name: Option<&str>) -> ::llvm::value::user::inst::VAArgInst {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVAArg(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(list), ::llvm::ty::TypeObj::get_inner(ty), c_name);
            ::llvm::value::user::inst::VAArgInst::from_inner(ret, false)
        }
    }

    fn create_vector_splat<A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, num_elements: u32, value: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateVectorSplat(::llvm::IRBuilderObj::get_inner(self), num_elements as ::libc::c_uint, ::llvm::value::ValueObj::get_inner(value), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXor(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), ::llvm::value::ValueObj::get_inner(rhs), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_xor_by_value<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, lhs: &mut A1, rhs: u64, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateXorByValue(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(lhs), rhs as ::libc::uint64_t, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExt(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_bit_cast<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrBitCast(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_z_ext_or_trunc<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type>(&mut self, value: &mut A1, dest_ty: &mut A2, name: Option<&str>) -> ::llvm::value::Value {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::IRBuilder_CreateZExtOrTrunc(::llvm::IRBuilderObj::get_inner(self), ::llvm::value::ValueObj::get_inner(value), ::llvm::ty::TypeObj::get_inner(dest_ty), c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn is_name_preserving(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_isNamePreserving(::llvm::IRBuilderObj::get_inner(self) as *const ::ffi::llvm_IRBuilder);
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
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_IRBuilderBase {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IRBuilderObj for IRBuilder {
    #[inline(always)]
    fn get_inner(&self) -> *mut IRBuilderInner {
        *self.inner
    }
}
impl IRBuilder {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut IRBuilderInner, owned: bool) -> IRBuilder {
        IRBuilder {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }


    pub fn new<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(context: &mut A1) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new(::llvm::LLVMContextObj::get_inner(context));
            if ret.is_null() {
                panic!("::llvm::IRBuilder::new returned a null pointer!");
            }
            ::llvm::IRBuilder::from_inner(ret, true)
        }
    }

    pub fn new_in_block<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(bb: &mut A1) -> ::llvm::IRBuilder {
        unsafe {
            let ret = ::ffi::llvm::IRBuilder_new_in_block(::llvm::value::BasicBlockObj::get_inner(bb));
            if ret.is_null() {
                panic!("::llvm::IRBuilder::new_in_block returned a null pointer!");
            }
            ::llvm::IRBuilder::from_inner(ret, true)
        }
    }
}
impl Drop for IRBuilder {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::IRBuilder_delete(::llvm::IRBuilderObj::get_inner(self));
            }
        }
    }
}
pub type IRBuilderBaseInner = ::ffi::llvm_IRBuilderBase;

pub trait IRBuilderBaseObj {
    unsafe fn get_inner(&self) -> *mut IRBuilderBaseInner;
}

pub trait IRBuilderBaseOwned: IRBuilderBaseObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut IRBuilderBaseInner {
        let inner = IRBuilderBaseObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> IRBuilderBaseOwned for T where T: IRBuilderBaseObj + ::core::marker::Sized {}

pub trait IRBuilderBaseExt: IRBuilderBaseObj {

    fn clear_insertion_point(&mut self) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_ClearInsertionPoint(::llvm::IRBuilderBaseObj::get_inner(self));
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
            let ret = ::ffi::llvm::IRBuilderBase_CreateGlobalString(::llvm::IRBuilderBaseObj::get_inner(self), c_str, c_name);
            ::llvm::value::Value::from_inner(ret, false)
        }
    }

    fn create_lifetime_end<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::user::constant::ConstantIntObj = ::llvm::value::user::constant::ConstantInt>(&mut self, ptr: &mut A1, size: Option<&mut A2>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeEnd(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntObj::get_inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_lifetime_start<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::user::constant::ConstantIntObj = ::llvm::value::user::constant::ConstantInt>(&mut self, ptr: &mut A1, size: Option<&mut A2>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_CreateLifetimeStart(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), size.map(|size| ::llvm::value::user::constant::ConstantIntObj::get_inner(size)).unwrap_or(::std::ptr::null_mut()));
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_cpy<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, dst: &mut A1, src: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemCpy(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(dst), ::llvm::value::ValueObj::get_inner(src), ::llvm::value::ValueObj::get_inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_move<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, dst: &mut A1, src: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemMove(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(dst), ::llvm::value::ValueObj::get_inner(src), ::llvm::value::ValueObj::get_inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn create_mem_set<A1: ::llvm::value::ValueObj = ::llvm::value::Value, A2: ::llvm::value::ValueObj = ::llvm::value::Value, A3: ::llvm::value::ValueObj = ::llvm::value::Value>(&mut self, ptr: &mut A1, value: &mut A2, size: &mut A3, align: u32, is_volatile: Option<bool>) -> ::llvm::value::user::inst::CallInst {
        unsafe {
            let is_volatile = is_volatile.unwrap_or(false);
            let ret = ::ffi::llvm::IRBuilderBase_CreateMemSet(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(ptr), ::llvm::value::ValueObj::get_inner(value), ::llvm::value::ValueObj::get_inner(size), align as ::libc::c_uint, is_volatile);
            ::llvm::value::user::inst::CallInst::from_inner(ret, false)
        }
    }

    fn get_insert_block(&self) -> ::llvm::value::BasicBlock {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_GetInsertBlock(::llvm::IRBuilderBaseObj::get_inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::value::BasicBlock::from_inner(ret, false)
        }
    }

    fn set_current_debug_location<A1: ::llvm::DebugLocObj = ::llvm::DebugLoc>(&mut self, loc: &A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetCurrentDebugLocation(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::DebugLocObj::get_inner(loc));
        }
    }

    fn set_default_fp_math_tag<A1: ::llvm::value::MDNodeObj = ::llvm::value::MDNode>(&mut self, fp_math_tag: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetDefaultFPMathTag(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::MDNodeObj::get_inner(fp_math_tag));
        }
    }

    fn set_insert_point<A1: ::llvm::value::BasicBlockObj = ::llvm::value::BasicBlock>(&mut self, bb: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPoint(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::BasicBlockObj::get_inner(bb));
        }
    }

    fn set_insert_point_at_inst<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&mut self, inst: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInsertPointAtInst(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::value::user::inst::InstructionObj::get_inner(inst));
        }
    }

    fn set_inst_debug_location<A1: ::llvm::value::user::inst::InstructionObj = ::llvm::value::user::inst::Instruction>(&self, inst: &mut A1) {
        unsafe {
            ::ffi::llvm::IRBuilderBase_SetInstDebugLocation(::llvm::IRBuilderBaseObj::get_inner(self) as *const ::ffi::llvm_IRBuilderBase, ::llvm::value::user::inst::InstructionObj::get_inner(inst));
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getContext(::llvm::IRBuilderBaseObj::get_inner(self) as *const ::ffi::llvm_IRBuilderBase);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_current_function_return_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getCurrentFunctionReturnType(::llvm::IRBuilderBaseObj::get_inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_default_fp_math_tag(&self) -> Option<::llvm::value::MDNode> {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDefaultFPMathTag(::llvm::IRBuilderBaseObj::get_inner(self) as *const ::ffi::llvm_IRBuilderBase);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::MDNode::from_inner(ret, false))
        }
    }

    fn get_double_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getDoubleTy(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_false(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFalse(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_float_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getFloatTy(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }

    fn get_half_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getHalfTy(::llvm::IRBuilderBaseObj::get_inner(self));
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
            let ret = ::ffi::llvm::IRBuilderBase_getInt(::llvm::IRBuilderBaseObj::get_inner(self), c_value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int1(&mut self, value: bool) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1(::llvm::IRBuilderBaseObj::get_inner(self), value);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16(&mut self, value: u16) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16(::llvm::IRBuilderBaseObj::get_inner(self), value as ::libc::uint16_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int16_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt16Ty(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int1_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt1Ty(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int32(&mut self, value: u32) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32(::llvm::IRBuilderBaseObj::get_inner(self), value as ::libc::uint32_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int32_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt32Ty(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int64(&mut self, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64(::llvm::IRBuilderBaseObj::get_inner(self), value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int64_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt64Ty(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int8(&mut self, value: u8) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8(::llvm::IRBuilderBaseObj::get_inner(self), value as ::libc::uint8_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int8_ptr_ty(&mut self, addr_space: Option<u32>) -> ::llvm::ty::seq::PointerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getInt8PtrTy(::llvm::IRBuilderBaseObj::get_inner(self), addr_space as ::libc::c_uint);
            ::llvm::ty::seq::PointerType::from_inner(ret)
        }
    }

    fn get_int8_ty(&mut self) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getInt8Ty(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_n(&mut self, num_bits: u32, value: u64) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntN(::llvm::IRBuilderBaseObj::get_inner(self), num_bits as ::libc::c_uint, value as ::libc::uint64_t);
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_int_n_ty(&mut self, num_bits: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getIntNTy(::llvm::IRBuilderBaseObj::get_inner(self), num_bits as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_int_ptr_ty<A1: ::llvm::DataLayoutObj = ::llvm::DataLayout>(&mut self, dl: &A1, addr_space: Option<u32>) -> ::llvm::ty::IntegerType {
        unsafe {
            let addr_space = addr_space.unwrap_or(0);
            let ret = ::ffi::llvm::IRBuilderBase_getIntPtrTy(::llvm::IRBuilderBaseObj::get_inner(self), ::llvm::DataLayoutObj::get_inner(dl), addr_space as ::libc::c_uint);
            ::llvm::ty::IntegerType::from_inner(ret)
        }
    }

    fn get_true(&mut self) -> ::llvm::value::user::constant::ConstantInt {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getTrue(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::value::user::constant::ConstantInt::from_inner(ret, false)
        }
    }

    fn get_void_ty(&mut self) -> ::llvm::ty::Type {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_getVoidTy(::llvm::IRBuilderBaseObj::get_inner(self));
            ::llvm::ty::Type::from_inner(ret)
        }
    }
}
impl<T> IRBuilderBaseExt for T where T: IRBuilderBaseObj {}

pub struct IRBuilderBase {
    inner: ::core::nonzero::NonZero<*mut IRBuilderBaseInner>,
}
impl IRBuilderBaseObj for IRBuilderBase {
    #[inline(always)]
    fn get_inner(&self) -> *mut IRBuilderBaseInner {
        *self.inner
    }
}
impl IRBuilderBase {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut IRBuilderBaseInner) -> IRBuilderBase {
        IRBuilderBase {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn new<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(context: &mut A1) -> ::llvm::IRBuilderBase {
        unsafe {
            let ret = ::ffi::llvm::IRBuilderBase_new(::llvm::LLVMContextObj::get_inner(context));
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
    unsafe fn get_inner(&self) -> *mut LLVMContextInner;
}

pub trait LLVMContextOwned: LLVMContextObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut LLVMContextInner {
        let inner = LLVMContextObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> LLVMContextOwned for T where T: LLVMContextObj + ::core::marker::Sized {}

pub trait LLVMContextExt: LLVMContextObj {
}
impl<T> LLVMContextExt for T where T: LLVMContextObj {}

pub struct LLVMContext {
    inner: ::core::nonzero::NonZero<*mut LLVMContextInner>,
}
impl LLVMContextObj for LLVMContext {
    #[inline(always)]
    fn get_inner(&self) -> *mut LLVMContextInner {
        *self.inner
    }
}
impl LLVMContext {
    #[inline(always)]
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
    unsafe fn get_inner(&self) -> *mut ModuleInner;
}

pub trait ModuleOwned: ModuleObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ModuleInner {
        let inner = ModuleObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ModuleOwned for T where T: ModuleObj + ::core::marker::Sized {}

pub trait ModuleExt: ModuleObj {

    fn append_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_appendModuleInlineAsm(::llvm::ModuleObj::get_inner(self), c_asm);
        }
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Module_dump(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Module_getContext(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayout(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_data_layout_str(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getDataLayoutStr(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getFunction(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module, c_name);
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
            let ret = ::ffi::llvm::Module_getMDKindID(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module, c_name) as u32;
            ret
        }
    }

    fn get_module_identifier(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleIdentifier(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_module_inline_asm(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getModuleInlineAsm(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getNamedValue(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::GlobalValue::from_inner(ret, false))
        }
    }

    fn get_or_insert_function<A2: ::llvm::ty::FunctionTypeObj = ::llvm::ty::FunctionType>(&mut self, name: &str, ty: &mut A2) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_getOrInsertFunction(::llvm::ModuleObj::get_inner(self), c_name, ::llvm::ty::FunctionTypeObj::get_inner(ty));
            ::llvm::value::user::constant::Constant::from_inner(ret, false)
        }
    }

    fn get_target_triple(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Module_getTargetTriple(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module);
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
            let ret = ::ffi::llvm::Module_getTypeByName(::llvm::ModuleObj::get_inner(self) as *const ::ffi::llvm_Module, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    fn set_data_layout<A1: ::llvm::DataLayoutObj = ::llvm::DataLayout>(&mut self, other: &A1) {
        unsafe {
            ::ffi::llvm::Module_setDataLayout(::llvm::ModuleObj::get_inner(self), ::llvm::DataLayoutObj::get_inner(other));
        }
    }

    fn set_data_layout_str(&mut self, desc: &str) {
        unsafe {
            let c_desc = ::ffi::llvm_StringRef {
                data: desc.as_ptr() as *const ::libc::c_char,
                length: desc.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setDataLayoutStr(::llvm::ModuleObj::get_inner(self), c_desc);
        }
    }

    fn set_module_identifier(&mut self, id: &str) {
        unsafe {
            let c_id = ::ffi::llvm_StringRef {
                data: id.as_ptr() as *const ::libc::c_char,
                length: id.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleIdentifier(::llvm::ModuleObj::get_inner(self), c_id);
        }
    }

    fn set_module_inline_asm(&mut self, asm: &str) {
        unsafe {
            let c_asm = ::ffi::llvm_StringRef {
                data: asm.as_ptr() as *const ::libc::c_char,
                length: asm.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setModuleInlineAsm(::llvm::ModuleObj::get_inner(self), c_asm);
        }
    }

    fn set_target_triple(&mut self, triple: &str) {
        unsafe {
            let c_triple = ::ffi::llvm_StringRef {
                data: triple.as_ptr() as *const ::libc::c_char,
                length: triple.len() as ::libc::size_t,
            };
            ::ffi::llvm::Module_setTargetTriple(::llvm::ModuleObj::get_inner(self), c_triple);
        }
    }
}
impl<T> ModuleExt for T where T: ModuleObj {}

pub struct Module {
    inner: ::core::nonzero::NonZero<*mut ModuleInner>,
    owned: bool,
}
impl ModuleObj for Module {
    #[inline(always)]
    fn get_inner(&self) -> *mut ModuleInner {
        *self.inner
    }
}
impl Module {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ModuleInner, owned: bool) -> Module {
        Module {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn new<A2: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(module_id: &str, context: &mut A2) -> ::llvm::Module {
        unsafe {
            let c_module_id = ::ffi::llvm_StringRef {
                data: module_id.as_ptr() as *const ::libc::c_char,
                length: module_id.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Module_new(c_module_id, ::llvm::LLVMContextObj::get_inner(context));
            if ret.is_null() {
                panic!("::llvm::Module::new returned a null pointer!");
            }
            ::llvm::Module::from_inner(ret, true)
        }
    }
}
impl Drop for Module {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Module_delete(::llvm::ModuleObj::get_inner(self));
            }
        }
    }
}
pub type ValueSymbolTableInner = ::ffi::llvm_ValueSymbolTable;

pub trait ValueSymbolTableObj {
    unsafe fn get_inner(&self) -> *mut ValueSymbolTableInner;
}

pub trait ValueSymbolTableOwned: ValueSymbolTableObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ValueSymbolTableInner {
        let inner = ValueSymbolTableObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ValueSymbolTableOwned for T where T: ValueSymbolTableObj + ::core::marker::Sized {}

pub trait ValueSymbolTableExt: ValueSymbolTableObj {

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::ValueSymbolTable_dump(::llvm::ValueSymbolTableObj::get_inner(self) as *const ::ffi::llvm_ValueSymbolTable);
        }
    }

    fn empty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_empty(::llvm::ValueSymbolTableObj::get_inner(self) as *const ::ffi::llvm_ValueSymbolTable);
            ret
        }
    }

    fn lookup(&self, name: &str) -> Option<::llvm::value::Value> {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ValueSymbolTable_lookup(::llvm::ValueSymbolTableObj::get_inner(self) as *const ::ffi::llvm_ValueSymbolTable, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn size(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::ValueSymbolTable_size(::llvm::ValueSymbolTableObj::get_inner(self) as *const ::ffi::llvm_ValueSymbolTable) as u32;
            ret
        }
    }
}
impl<T> ValueSymbolTableExt for T where T: ValueSymbolTableObj {}

pub struct ValueSymbolTable {
    inner: ::core::nonzero::NonZero<*mut ValueSymbolTableInner>,
}
impl ValueSymbolTableObj for ValueSymbolTable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ValueSymbolTableInner {
        *self.inner
    }
}
impl ValueSymbolTable {
    #[inline(always)]
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

pub fn verify_function<A1: ::llvm::value::user::constant::FunctionObj = ::llvm::value::user::constant::Function>(function: &A1) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyFunction(::llvm::value::user::constant::FunctionObj::get_inner(function));
        ret
    }
}

pub fn verify_module<A1: ::llvm::ModuleObj = ::llvm::Module>(module: &A1) -> bool {
    unsafe {
        let ret = ::ffi::llvm::verifyModule(::llvm::ModuleObj::get_inner(module));
        ret
    }
}
