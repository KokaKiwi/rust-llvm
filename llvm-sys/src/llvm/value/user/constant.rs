pub type BlockAddressInner = ::ffi::llvm_BlockAddress;

pub trait BlockAddressObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut BlockAddressInner;
}

pub trait BlockAddressOwned: BlockAddressObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut BlockAddressInner {
        let inner = BlockAddressObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> BlockAddressOwned for T where T: BlockAddressObj + ::core::marker::Sized {}

pub trait BlockAddressExt: BlockAddressObj {

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::BlockAddress_destroyConstant(::llvm::value::user::constant::BlockAddressObj::get_inner(self));
        }
    }

    fn get_basic_block(&self) -> Option<::llvm::value::BasicBlock> {
        unsafe {
            let ret = ::ffi::llvm::BlockAddress_getBasicBlock(::llvm::value::user::constant::BlockAddressObj::get_inner(self) as *const ::ffi::llvm_BlockAddress);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::BasicBlock::from_inner(ret, false))
        }
    }

    fn get_function(&self) -> Option<::llvm::value::user::constant::Function> {
        unsafe {
            let ret = ::ffi::llvm::BlockAddress_getFunction(::llvm::value::user::constant::BlockAddressObj::get_inner(self) as *const ::ffi::llvm_BlockAddress);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Function::from_inner(ret, false))
        }
    }
}
impl<T> BlockAddressExt for T where T: BlockAddressObj {}

pub struct BlockAddress {
    inner: ::core::nonzero::NonZero<*mut BlockAddressInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for BlockAddress {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for BlockAddress {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for BlockAddress {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BlockAddressObj for BlockAddress {
    #[inline(always)]
    fn get_inner(&self) -> *mut BlockAddressInner {
        *self.inner
    }
}
impl BlockAddress {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut BlockAddressInner, owned: bool) -> BlockAddress {
        BlockAddress {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BlockAddress {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantInner = ::ffi::llvm_Constant;

pub trait ConstantObj: ::llvm::value::user::UserObj {
    unsafe fn get_inner(&self) -> *mut ConstantInner;
}

pub trait ConstantOwned: ConstantObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantInner {
        let inner = ConstantObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantOwned for T where T: ConstantObj + ::core::marker::Sized {}

pub trait ConstantExt: ConstantObj {

    fn can_trap(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_canTrap(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::Constant_destroyConstant(::llvm::value::user::constant::ConstantObj::get_inner(self));
        }
    }

    fn get_aggregate_element(&self, elt: u32) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_getAggregateElement(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant, elt as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    fn get_aggregate_element_constant<A1: ::llvm::value::user::constant::ConstantObj = ::llvm::value::user::constant::Constant>(&self, elt: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_getAggregateElementConstant(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant, ::llvm::value::user::constant::ConstantObj::get_inner(elt));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    fn get_splat_value(&self) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_getSplatValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    fn is_all_ones_value(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isAllOnesValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_constant_used(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isConstantUsed(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_dll_import_dependent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isDLLImportDependent(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_min_signed_value(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isMinSignedValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_negative_zero_value(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isNegativeZeroValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_null_value(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isNullValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_thread_dependent(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isThreadDependent(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn is_zero_value(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_isZeroValue(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            ret
        }
    }

    fn remove_dead_constant_users(&self) {
        unsafe {
            ::ffi::llvm::Constant_removeDeadConstantUsers(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
        }
    }

    fn strip_pointer_casts(&self) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_stripPointerCasts(::llvm::value::user::constant::ConstantObj::get_inner(self) as *const ::ffi::llvm_Constant);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret as *mut ::ffi::llvm_Constant, false))
        }
    }

    fn strip_pointer_casts_mut(&mut self) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_stripPointerCastsMut(::llvm::value::user::constant::ConstantObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }
}
impl<T> ConstantExt for T where T: ConstantObj {}

pub struct Constant {
    inner: ::core::nonzero::NonZero<*mut ConstantInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for Constant {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Constant {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantObj for Constant {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantInner {
        *self.inner
    }
}
impl Constant {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantInner, owned: bool) -> Constant {
        Constant {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(v: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Constant_classof(::llvm::value::ValueObj::get_inner(v));
            ret
        }
    }

    pub fn get_all_ones_value<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_getAllOnesValue(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_integer_value<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, value: (u32, &[u64])) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let c_value = ::ffi::llvm_APInt {
                num_bits: value.0 as ::libc::c_uint,
                value: ::ffi::llvm_ArrayRef__libc_uint64_t {
                data: value.1.as_ptr(),
                length: value.1.len() as ::libc::size_t,
            },
            };
            let ret = ::ffi::llvm::Constant_getIntegerValue(::llvm::ty::TypeObj::get_inner(ty), c_value);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_null_value<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::Constant_getNullValue(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }
}
impl Drop for Constant {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantAggregateZeroInner = ::ffi::llvm_ConstantAggregateZero;

pub trait ConstantAggregateZeroObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantAggregateZeroInner;
}

pub trait ConstantAggregateZeroOwned: ConstantAggregateZeroObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantAggregateZeroInner {
        let inner = ConstantAggregateZeroObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantAggregateZeroOwned for T where T: ConstantAggregateZeroObj + ::core::marker::Sized {}

pub trait ConstantAggregateZeroExt: ConstantAggregateZeroObj {
}
impl<T> ConstantAggregateZeroExt for T where T: ConstantAggregateZeroObj {}

pub struct ConstantAggregateZero {
    inner: ::core::nonzero::NonZero<*mut ConstantAggregateZeroInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantAggregateZero {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantAggregateZero {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantAggregateZero {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantAggregateZeroObj for ConstantAggregateZero {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantAggregateZeroInner {
        *self.inner
    }
}
impl ConstantAggregateZero {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantAggregateZeroInner, owned: bool) -> ConstantAggregateZero {
        ConstantAggregateZero {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantAggregateZero {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantArrayInner = ::ffi::llvm_ConstantArray;

pub trait ConstantArrayObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantArrayInner;
}

pub trait ConstantArrayOwned: ConstantArrayObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantArrayInner {
        let inner = ConstantArrayObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantArrayOwned for T where T: ConstantArrayObj + ::core::marker::Sized {}

pub trait ConstantArrayExt: ConstantArrayObj {

    fn get_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::ConstantArray_getType(::llvm::value::user::constant::ConstantArrayObj::get_inner(self) as *const ::ffi::llvm_ConstantArray);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }
}
impl<T> ConstantArrayExt for T where T: ConstantArrayObj {}

pub struct ConstantArray {
    inner: ::core::nonzero::NonZero<*mut ConstantArrayInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantArrayObj for ConstantArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantArrayInner {
        *self.inner
    }
}
impl ConstantArray {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantArrayInner, owned: bool) -> ConstantArray {
        ConstantArray {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(v: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantArray_classof(::llvm::value::ValueObj::get_inner(v));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::seq::ArrayTypeObj = ::llvm::ty::seq::ArrayType>(ty: &mut A1, values: &[&::llvm::value::user::constant::ConstantObj]) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let _tmp_values: Vec<_> = values.iter().map(|&ty| ::llvm::value::user::constant::ConstantObj::get_inner(ty)).collect();
            let c_values = ::ffi::llvm_ArrayRef_llvm_Constant_ptr {
                data: _tmp_values.as_ptr(),
                length: values.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ConstantArray_get(::llvm::ty::seq::ArrayTypeObj::get_inner(ty), c_values);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }
}
impl Drop for ConstantArray {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantDataArrayInner = ::ffi::llvm_ConstantDataArray;

pub trait ConstantDataArrayObj: ::llvm::value::user::constant::ConstantDataSequentialObj {
    unsafe fn get_inner(&self) -> *mut ConstantDataArrayInner;
}

pub trait ConstantDataArrayOwned: ConstantDataArrayObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantDataArrayInner {
        let inner = ConstantDataArrayObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantDataArrayOwned for T where T: ConstantDataArrayObj + ::core::marker::Sized {}

pub trait ConstantDataArrayExt: ConstantDataArrayObj {
}
impl<T> ConstantDataArrayExt for T where T: ConstantDataArrayObj {}

pub struct ConstantDataArray {
    inner: ::core::nonzero::NonZero<*mut ConstantDataArrayInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantDataArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantDataArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantDataArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialObj for ConstantDataArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataArrayObj for ConstantDataArray {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantDataArrayInner {
        *self.inner
    }
}
impl ConstantDataArray {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantDataArrayInner, owned: bool) -> ConstantDataArray {
        ConstantDataArray {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantDataArray {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantDataSequentialInner = ::ffi::llvm_ConstantDataSequential;

pub trait ConstantDataSequentialObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantDataSequentialInner;
}

pub trait ConstantDataSequentialOwned: ConstantDataSequentialObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantDataSequentialInner {
        let inner = ConstantDataSequentialObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantDataSequentialOwned for T where T: ConstantDataSequentialObj + ::core::marker::Sized {}

pub trait ConstantDataSequentialExt: ConstantDataSequentialObj {
}
impl<T> ConstantDataSequentialExt for T where T: ConstantDataSequentialObj {}

pub struct ConstantDataSequential {
    inner: ::core::nonzero::NonZero<*mut ConstantDataSequentialInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantDataSequential {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantDataSequential {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantDataSequential {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataSequentialObj for ConstantDataSequential {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantDataSequentialInner {
        *self.inner
    }
}
impl ConstantDataSequential {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantDataSequentialInner, owned: bool) -> ConstantDataSequential {
        ConstantDataSequential {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantDataSequential {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantDataVectorInner = ::ffi::llvm_ConstantDataVector;

pub trait ConstantDataVectorObj: ::llvm::value::user::constant::ConstantDataSequentialObj {
    unsafe fn get_inner(&self) -> *mut ConstantDataVectorInner;
}

pub trait ConstantDataVectorOwned: ConstantDataVectorObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantDataVectorInner {
        let inner = ConstantDataVectorObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantDataVectorOwned for T where T: ConstantDataVectorObj + ::core::marker::Sized {}

pub trait ConstantDataVectorExt: ConstantDataVectorObj {
}
impl<T> ConstantDataVectorExt for T where T: ConstantDataVectorObj {}

pub struct ConstantDataVector {
    inner: ::core::nonzero::NonZero<*mut ConstantDataVectorInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantDataVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantDataVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantDataVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialObj for ConstantDataVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataVectorObj for ConstantDataVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantDataVectorInner {
        *self.inner
    }
}
impl ConstantDataVector {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantDataVectorInner, owned: bool) -> ConstantDataVector {
        ConstantDataVector {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantDataVector {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantExprInner = ::ffi::llvm_ConstantExpr;

pub trait ConstantExprObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantExprInner;
}

pub trait ConstantExprOwned: ConstantExprObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantExprInner {
        let inner = ConstantExprObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantExprOwned for T where T: ConstantExprObj + ::core::marker::Sized {}

pub trait ConstantExprExt: ConstantExprObj {
}
impl<T> ConstantExprExt for T where T: ConstantExprObj {}

pub struct ConstantExpr {
    inner: ::core::nonzero::NonZero<*mut ConstantExprInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantExpr {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantExpr {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantExpr {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantExprObj for ConstantExpr {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantExprInner {
        *self.inner
    }
}
impl ConstantExpr {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantExprInner, owned: bool) -> ConstantExpr {
        ConstantExpr {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantExpr {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantFPInner = ::ffi::llvm_ConstantFP;

pub trait ConstantFPObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantFPInner;
}

pub trait ConstantFPOwned: ConstantFPObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantFPInner {
        let inner = ConstantFPObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantFPOwned for T where T: ConstantFPObj + ::core::marker::Sized {}

pub trait ConstantFPExt: ConstantFPObj {

    fn is_exactly_value_float(&self, val: f64) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_isExactlyValueFloat(::llvm::value::user::constant::ConstantFPObj::get_inner(self) as *const ::ffi::llvm_ConstantFP, val as ::libc::c_double);
            ret
        }
    }

    fn is_na_n(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_isNaN(::llvm::value::user::constant::ConstantFPObj::get_inner(self) as *const ::ffi::llvm_ConstantFP);
            ret
        }
    }

    fn is_negative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_isNegative(::llvm::value::user::constant::ConstantFPObj::get_inner(self) as *const ::ffi::llvm_ConstantFP);
            ret
        }
    }

    fn is_zero(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_isZero(::llvm::value::user::constant::ConstantFPObj::get_inner(self) as *const ::ffi::llvm_ConstantFP);
            ret
        }
    }
}
impl<T> ConstantFPExt for T where T: ConstantFPObj {}

pub struct ConstantFP {
    inner: ::core::nonzero::NonZero<*mut ConstantFPInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantFP {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantFP {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantFP {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantFPObj for ConstantFP {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantFPInner {
        *self.inner
    }
}
impl ConstantFP {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantFPInner, owned: bool) -> ConstantFP {
        ConstantFP {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(v: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_classof(::llvm::value::ValueObj::get_inner(v));
            ret
        }
    }

    pub fn from_str<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, val: &str) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let c_val = ::ffi::llvm_StringRef {
                data: val.as_ptr() as *const ::libc::c_char,
                length: val.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ConstantFP_fromStr(::llvm::ty::TypeObj::get_inner(ty), c_val);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, val: f64) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_get(::llvm::ty::TypeObj::get_inner(ty), val as ::libc::c_double);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_infinity<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_getInfinity(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_negative_zero<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_getNegativeZero(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_zero_value_for_negation<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantFP_getZeroValueForNegation(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }
}
impl Drop for ConstantFP {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantIntInner = ::ffi::llvm_ConstantInt;

pub trait ConstantIntObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantIntInner;
}

pub trait ConstantIntOwned: ConstantIntObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantIntInner {
        let inner = ConstantIntObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantIntOwned for T where T: ConstantIntObj + ::core::marker::Sized {}

pub trait ConstantIntExt: ConstantIntObj {

    fn equals_int(&self, val: u64) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_equalsInt(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt, val as ::libc::uint64_t);
            ret
        }
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getBitWidth(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt) as u32;
            ret
        }
    }

    fn get_s_ext_value(&self) -> i64 {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getSExtValue(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt) as i64;
            ret
        }
    }

    fn get_type(&self) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getType(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    fn get_z_ext_value(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getZExtValue(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt) as u64;
            ret
        }
    }

    fn is_max_value(&self, is_signed: bool) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isMaxValue(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt, is_signed);
            ret
        }
    }

    fn is_min_value(&self, is_signed: bool) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isMinValue(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt, is_signed);
            ret
        }
    }

    fn is_minus_one(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isMinusOne(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt);
            ret
        }
    }

    fn is_negative(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isNegative(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt);
            ret
        }
    }

    fn is_one(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isOne(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt);
            ret
        }
    }

    fn is_zero(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isZero(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt);
            ret
        }
    }

    fn uge(&self, num: u64) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_uge(::llvm::value::user::constant::ConstantIntObj::get_inner(self) as *const ::ffi::llvm_ConstantInt, num as ::libc::uint64_t);
            ret
        }
    }
}
impl<T> ConstantIntExt for T where T: ConstantIntObj {}

pub struct ConstantInt {
    inner: ::core::nonzero::NonZero<*mut ConstantIntInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantInt {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantInt {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantInt {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantIntObj for ConstantInt {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantIntInner {
        *self.inner
    }
}
impl ConstantInt {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantIntInner, owned: bool) -> ConstantInt {
        ConstantInt {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(val: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_classof(::llvm::value::ValueObj::get_inner(val));
            ret
        }
    }

    pub fn from_ap_int<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(context: &mut A1, val: (u32, &[u64])) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let c_val = ::ffi::llvm_APInt {
                num_bits: val.0 as ::libc::c_uint,
                value: ::ffi::llvm_ArrayRef__libc_uint64_t {
                data: val.1.as_ptr(),
                length: val.1.len() as ::libc::size_t,
            },
            };
            let ret = ::ffi::llvm::ConstantInt_fromAPInt(::llvm::LLVMContextObj::get_inner(context), c_val);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn from_str<A1: ::llvm::ty::IntegerTypeObj = ::llvm::ty::IntegerType>(ty: &mut A1, str: &str, radix: u8) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let c_str = ::ffi::llvm_StringRef {
                data: str.as_ptr() as *const ::libc::c_char,
                length: str.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::ConstantInt_fromStr(::llvm::ty::IntegerTypeObj::get_inner(ty), c_str, radix as ::libc::uint8_t);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn get<A1: ::llvm::ty::IntegerTypeObj = ::llvm::ty::IntegerType>(ty: &mut A1, value: u64) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_get(::llvm::ty::IntegerTypeObj::get_inner(ty), value as ::libc::uint64_t);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn get_false<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getFalse(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_false_with_context<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(context: &mut A1) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getFalseWithContext(::llvm::LLVMContextObj::get_inner(context));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn get_signed<A1: ::llvm::ty::IntegerTypeObj = ::llvm::ty::IntegerType>(ty: &mut A1, value: u64, is_signed: bool) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getSigned(::llvm::ty::IntegerTypeObj::get_inner(ty), value as ::libc::uint64_t, is_signed);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn get_true<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getTrue(::llvm::ty::TypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    pub fn get_true_with_context<A1: ::llvm::LLVMContextObj = ::llvm::LLVMContext>(context: &mut A1) -> Option<::llvm::value::user::constant::ConstantInt> {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_getTrueWithContext(::llvm::LLVMContextObj::get_inner(context));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantInt::from_inner(ret, false))
        }
    }

    pub fn is_signed_value_valid_for_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, val: i64) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isSignedValueValidForType(::llvm::ty::TypeObj::get_inner(ty), val as ::libc::int64_t);
            ret
        }
    }

    pub fn is_value_valid_for_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, val: u64) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantInt_isValueValidForType(::llvm::ty::TypeObj::get_inner(ty), val as ::libc::uint64_t);
            ret
        }
    }
}
impl Drop for ConstantInt {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantPointerNullInner = ::ffi::llvm_ConstantPointerNull;

pub trait ConstantPointerNullObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantPointerNullInner;
}

pub trait ConstantPointerNullOwned: ConstantPointerNullObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantPointerNullInner {
        let inner = ConstantPointerNullObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantPointerNullOwned for T where T: ConstantPointerNullObj + ::core::marker::Sized {}

pub trait ConstantPointerNullExt: ConstantPointerNullObj {

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::ConstantPointerNull_destroyConstant(::llvm::value::user::constant::ConstantPointerNullObj::get_inner(self));
        }
    }

    fn get_type(&self) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::ConstantPointerNull_getType(::llvm::value::user::constant::ConstantPointerNullObj::get_inner(self) as *const ::ffi::llvm_ConstantPointerNull);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }
}
impl<T> ConstantPointerNullExt for T where T: ConstantPointerNullObj {}

pub struct ConstantPointerNull {
    inner: ::core::nonzero::NonZero<*mut ConstantPointerNullInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantPointerNull {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantPointerNull {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantPointerNull {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantPointerNullObj for ConstantPointerNull {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantPointerNullInner {
        *self.inner
    }
}
impl ConstantPointerNull {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantPointerNullInner, owned: bool) -> ConstantPointerNull {
        ConstantPointerNull {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(val: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ConstantPointerNull_classof(::llvm::value::ValueObj::get_inner(val));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::seq::PointerTypeObj = ::llvm::ty::seq::PointerType>(ty: &mut A1) -> Option<::llvm::value::user::constant::ConstantPointerNull> {
        unsafe {
            let ret = ::ffi::llvm::ConstantPointerNull_get(::llvm::ty::seq::PointerTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::ConstantPointerNull::from_inner(ret, false))
        }
    }
}
impl Drop for ConstantPointerNull {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantStructInner = ::ffi::llvm_ConstantStruct;

pub trait ConstantStructObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantStructInner;
}

pub trait ConstantStructOwned: ConstantStructObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantStructInner {
        let inner = ConstantStructObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantStructOwned for T where T: ConstantStructObj + ::core::marker::Sized {}

pub trait ConstantStructExt: ConstantStructObj {
}
impl<T> ConstantStructExt for T where T: ConstantStructObj {}

pub struct ConstantStruct {
    inner: ::core::nonzero::NonZero<*mut ConstantStructInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantStruct {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantStruct {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantStruct {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantStructObj for ConstantStruct {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantStructInner {
        *self.inner
    }
}
impl ConstantStruct {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantStructInner, owned: bool) -> ConstantStruct {
        ConstantStruct {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantStruct {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type ConstantVectorInner = ::ffi::llvm_ConstantVector;

pub trait ConstantVectorObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut ConstantVectorInner;
}

pub trait ConstantVectorOwned: ConstantVectorObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ConstantVectorInner {
        let inner = ConstantVectorObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ConstantVectorOwned for T where T: ConstantVectorObj + ::core::marker::Sized {}

pub trait ConstantVectorExt: ConstantVectorObj {
}
impl<T> ConstantVectorExt for T where T: ConstantVectorObj {}

pub struct ConstantVector {
    inner: ::core::nonzero::NonZero<*mut ConstantVectorInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for ConstantVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for ConstantVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for ConstantVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ConstantVectorObj for ConstantVector {
    #[inline(always)]
    fn get_inner(&self) -> *mut ConstantVectorInner {
        *self.inner
    }
}
impl ConstantVector {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ConstantVectorInner, owned: bool) -> ConstantVector {
        ConstantVector {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ConstantVector {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type FunctionInner = ::ffi::llvm_Function;

pub trait FunctionObj: ::llvm::value::user::constant::GlobalObjectObj {
    unsafe fn get_inner(&self) -> *mut FunctionInner;
}

pub trait FunctionOwned: FunctionObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut FunctionInner {
        let inner = FunctionObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> FunctionOwned for T where T: FunctionObj + ::core::marker::Sized {}

pub trait FunctionExt: FunctionObj {

    fn add_fn_attr(&mut self, kind: &str, val: Option<&str>) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            let val = val.unwrap_or("");
            let c_val = ::ffi::llvm_StringRef {
                data: val.as_ptr() as *const ::libc::c_char,
                length: val.len() as ::libc::size_t,
            };
            ::ffi::llvm::Function_addFnAttr(::llvm::value::user::constant::FunctionObj::get_inner(self), c_kind, c_val);
        }
    }

    fn cannot_duplicate(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_cannotDuplicate(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn clear_gc(&mut self) {
        unsafe {
            ::ffi::llvm::Function_clearGC(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn copy_attributes_from<A1: ::llvm::value::user::constant::GlobalValueObj = ::llvm::value::user::constant::GlobalValue>(&mut self, src: &mut A1) {
        unsafe {
            ::ffi::llvm::Function_copyAttributesFrom(::llvm::value::user::constant::FunctionObj::get_inner(self), ::llvm::value::user::constant::GlobalValueObj::get_inner(src));
        }
    }

    fn delete_body(&mut self) {
        unsafe {
            ::ffi::llvm::Function_deleteBody(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn does_not_access_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotAccessMemory(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn does_not_access_memory_param(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotAccessMemoryParam(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, n as ::libc::c_uint);
            ret
        }
    }

    fn does_not_alias(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotAlias(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, n as ::libc::c_uint);
            ret
        }
    }

    fn does_not_capture(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotCapture(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, n as ::libc::c_uint);
            ret
        }
    }

    fn does_not_return(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotReturn(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn does_not_throw(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_doesNotThrow(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Function_eraseFromParent(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn get_calling_conv(&self) -> ::llvm::calling_conv::ID {
        unsafe {
            let ret = ::llvm::calling_conv::ID::from_ffi(::ffi::llvm::Function_getCallingConv(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function));
            ret
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Function_getContext(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_dereferenceable_bytes(&self, idx: u32) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::Function_getDereferenceableBytes(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, idx as ::libc::c_uint) as u64;
            ret
        }
    }

    fn get_function_type(&self) -> Option<::llvm::ty::FunctionType> {
        unsafe {
            let ret = ::ffi::llvm::Function_getFunctionType(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::FunctionType::from_inner(ret))
        }
    }

    fn get_intrinsic_id(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Function_getIntrinsicID(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function) as u32;
            ret
        }
    }

    fn get_param_alignment(&self, idx: u32) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Function_getParamAlignment(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, idx as ::libc::c_uint) as u32;
            ret
        }
    }

    fn get_return_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Function_getReturnType(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn has_fn_attr(&self, kind: &str) -> bool {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Function_hasFnAttr(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, c_kind);
            ret
        }
    }

    fn has_gc(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_hasGC(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn has_struct_ret_attr(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_hasStructRetAttr(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn has_uw_table(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_hasUWTable(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn is_intrinsic(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_isIntrinsic(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_isVarArg(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn needs_unwind_table_entry(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_needsUnwindTableEntry(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn only_reads_memory(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_onlyReadsMemory(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function);
            ret
        }
    }

    fn only_reads_memory_param(&self, n: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_onlyReadsMemoryParam(::llvm::value::user::constant::FunctionObj::get_inner(self) as *const ::ffi::llvm_Function, n as ::libc::c_uint);
            ret
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Function_removeFromParent(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_calling_conv(&mut self, cc: ::llvm::calling_conv::ID) {
        unsafe {
            ::ffi::llvm::Function_setCallingConv(::llvm::value::user::constant::FunctionObj::get_inner(self), cc.to_ffi());
        }
    }

    fn set_cannot_duplicate(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setCannotDuplicate(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_does_not_access_memory(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAccessMemory(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_does_not_access_memory_param(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAccessMemoryParam(::llvm::value::user::constant::FunctionObj::get_inner(self), n as ::libc::c_uint);
        }
    }

    fn set_does_not_alias(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAlias(::llvm::value::user::constant::FunctionObj::get_inner(self), n as ::libc::c_uint);
        }
    }

    fn set_does_not_capture(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotCapture(::llvm::value::user::constant::FunctionObj::get_inner(self), n as ::libc::c_uint);
        }
    }

    fn set_does_not_return(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotReturn(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_does_not_throw(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotThrow(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_has_uw_table(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setHasUWTable(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_only_reads_memory(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setOnlyReadsMemory(::llvm::value::user::constant::FunctionObj::get_inner(self));
        }
    }

    fn set_only_reads_memory_param(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setOnlyReadsMemoryParam(::llvm::value::user::constant::FunctionObj::get_inner(self), n as ::libc::c_uint);
        }
    }
}
impl<T> FunctionExt for T where T: FunctionObj {}

pub struct Function {
    inner: ::core::nonzero::NonZero<*mut FunctionInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalObjectObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalObject {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FunctionObj for Function {
    #[inline(always)]
    fn get_inner(&self) -> *mut FunctionInner {
        *self.inner
    }
}
impl Function {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut FunctionInner, owned: bool) -> Function {
        Function {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn create<A1: ::llvm::ty::FunctionTypeObj = ::llvm::ty::FunctionType, A4: ::llvm::ModuleObj = ::llvm::Module>(ty: &mut A1, linkage: ::llvm::value::user::constant::LinkageTypes, name: Option<&str>, module: Option<&mut A4>) -> ::llvm::value::user::constant::Function {
        unsafe {
            let name = name.unwrap_or("");
            let c_name = ::ffi::std_string {
                data: name.as_ptr() as *mut ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::Function_Create(::llvm::ty::FunctionTypeObj::get_inner(ty), linkage.to_ffi(), c_name, module.map(|module| ::llvm::ModuleObj::get_inner(module)).unwrap_or(::std::ptr::null_mut()));
            if ret.is_null() {
                panic!("::llvm::Function::Create returned a null pointer!");
            }
            ::llvm::value::user::constant::Function::from_inner(ret, false)
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj = ::llvm::value::Value>(val: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Function_classof(::llvm::value::ValueObj::get_inner(val));
            ret
        }
    }
}
impl Drop for Function {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Function_delete(::llvm::value::user::constant::FunctionObj::get_inner(self));
            }
        }
    }
}
pub type GlobalAliasInner = ::ffi::llvm_GlobalAlias;

pub trait GlobalAliasObj: ::llvm::value::user::constant::GlobalValueObj {
    unsafe fn get_inner(&self) -> *mut GlobalAliasInner;
}

pub trait GlobalAliasOwned: GlobalAliasObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut GlobalAliasInner {
        let inner = GlobalAliasObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> GlobalAliasOwned for T where T: GlobalAliasObj + ::core::marker::Sized {}

pub trait GlobalAliasExt: GlobalAliasObj {
}
impl<T> GlobalAliasExt for T where T: GlobalAliasObj {}

pub struct GlobalAlias {
    inner: ::core::nonzero::NonZero<*mut GlobalAliasInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GlobalAlias {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GlobalAlias {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for GlobalAlias {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueObj for GlobalAlias {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GlobalAliasObj for GlobalAlias {
    #[inline(always)]
    fn get_inner(&self) -> *mut GlobalAliasInner {
        *self.inner
    }
}
impl GlobalAlias {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut GlobalAliasInner, owned: bool) -> GlobalAlias {
        GlobalAlias {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for GlobalAlias {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
            }
        }
    }
}
pub type GlobalObjectInner = ::ffi::llvm_GlobalObject;

pub trait GlobalObjectObj: ::llvm::value::user::constant::GlobalValueObj {
    unsafe fn get_inner(&self) -> *mut GlobalObjectInner;
}

pub trait GlobalObjectOwned: GlobalObjectObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut GlobalObjectInner {
        let inner = GlobalObjectObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> GlobalObjectOwned for T where T: GlobalObjectObj + ::core::marker::Sized {}

pub trait GlobalObjectExt: GlobalObjectObj {

    fn set_section(&mut self, s: &str) {
        unsafe {
            let c_s = ::ffi::llvm_StringRef {
                data: s.as_ptr() as *const ::libc::c_char,
                length: s.len() as ::libc::size_t,
            };
            ::ffi::llvm::GlobalObject_setSection(::llvm::value::user::constant::GlobalObjectObj::get_inner(self), c_s);
        }
    }
}
impl<T> GlobalObjectExt for T where T: GlobalObjectObj {}

pub struct GlobalObject {
    inner: ::core::nonzero::NonZero<*mut GlobalObjectInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GlobalObject {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GlobalObject {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for GlobalObject {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueObj for GlobalObject {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GlobalObjectObj for GlobalObject {
    #[inline(always)]
    fn get_inner(&self) -> *mut GlobalObjectInner {
        *self.inner
    }
}
impl GlobalObject {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut GlobalObjectInner, owned: bool) -> GlobalObject {
        GlobalObject {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for GlobalObject {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
            }
        }
    }
}
pub enum LinkageTypes {
    ExternalLinkage,
    AvailableExternallyLinkage,
    LinkOnceAnyLinkage,
    LinkOnceODRLinkage,
    WeakAnyLinkage,
    WeakODRLinkage,
    AppendingLinkage,
    InternalLinkage,
    PrivateLinkage,
    ExternalWeakLinkage,
    CommonLinkage,
}
impl LinkageTypes {
    pub fn from_ffi(value: ::ffi::llvm_GlobalValue_LinkageTypes) -> LinkageTypes {
        match value {
            ::ffi::llvm_GlobalValue_LinkageTypes::ExternalLinkage => LinkageTypes::ExternalLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::AvailableExternallyLinkage => LinkageTypes::AvailableExternallyLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::LinkOnceAnyLinkage => LinkageTypes::LinkOnceAnyLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::LinkOnceODRLinkage => LinkageTypes::LinkOnceODRLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::WeakAnyLinkage => LinkageTypes::WeakAnyLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::WeakODRLinkage => LinkageTypes::WeakODRLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::AppendingLinkage => LinkageTypes::AppendingLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::InternalLinkage => LinkageTypes::InternalLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::PrivateLinkage => LinkageTypes::PrivateLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::ExternalWeakLinkage => LinkageTypes::ExternalWeakLinkage,
            ::ffi::llvm_GlobalValue_LinkageTypes::CommonLinkage => LinkageTypes::CommonLinkage,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_GlobalValue_LinkageTypes {
        match self {
            LinkageTypes::ExternalLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::ExternalLinkage,
            LinkageTypes::AvailableExternallyLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::AvailableExternallyLinkage,
            LinkageTypes::LinkOnceAnyLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::LinkOnceAnyLinkage,
            LinkageTypes::LinkOnceODRLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::LinkOnceODRLinkage,
            LinkageTypes::WeakAnyLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::WeakAnyLinkage,
            LinkageTypes::WeakODRLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::WeakODRLinkage,
            LinkageTypes::AppendingLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::AppendingLinkage,
            LinkageTypes::InternalLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::InternalLinkage,
            LinkageTypes::PrivateLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::PrivateLinkage,
            LinkageTypes::ExternalWeakLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::ExternalWeakLinkage,
            LinkageTypes::CommonLinkage => ::ffi::llvm_GlobalValue_LinkageTypes::CommonLinkage,
        }
    }
}
impl Copy for LinkageTypes {}
pub type GlobalValueInner = ::ffi::llvm_GlobalValue;

pub trait GlobalValueObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut GlobalValueInner;
}

pub trait GlobalValueOwned: GlobalValueObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut GlobalValueInner {
        let inner = GlobalValueObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> GlobalValueOwned for T where T: GlobalValueObj + ::core::marker::Sized {}

pub trait GlobalValueExt: GlobalValueObj {

    fn copy_attributes_from<A1: ::llvm::value::user::constant::GlobalValueObj = ::llvm::value::user::constant::GlobalValue>(&mut self, src: &mut A1) {
        unsafe {
            ::ffi::llvm::GlobalValue_copyAttributesFrom(::llvm::value::user::constant::GlobalValueObj::get_inner(self), ::llvm::value::user::constant::GlobalValueObj::get_inner(src));
        }
    }

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_destroyConstant(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_eraseFromParent(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
        }
    }

    fn get_alignment(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_getAlignment(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue) as u32;
            ret
        }
    }

    fn get_data_layout(&self) -> Option<::llvm::DataLayout> {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_getDataLayout(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::DataLayout::from_inner(ret as *mut ::ffi::llvm_DataLayout))
        }
    }

    fn get_parent(&self) -> Option<::llvm::Module> {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_getParent(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::Module::from_inner(ret as *mut ::ffi::llvm_Module, false))
        }
    }

    fn get_parent_mut(&mut self) -> Option<::llvm::Module> {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_getParentMut(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::Module::from_inner(ret, false))
        }
    }

    fn get_type(&self) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_getType(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    fn has_appending_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasAppendingLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_available_externally_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasAvailableExternallyLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_common_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasCommonLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_dll_export_storage_class(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasDLLExportStorageClass(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_dll_import_storage_class(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasDLLImportStorageClass(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_default_visibility(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasDefaultVisibility(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_external_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasExternalLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_external_weak_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasExternalWeakLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_hidden_visibility(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasHiddenVisibility(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_internal_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasInternalLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_link_once_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasLinkOnceLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_local_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasLocalLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_private_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasPrivateLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_protected_visibility(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasProtectedVisibility(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_section(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasSection(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_unnamed_addr(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasUnnamedAddr(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_weak_any_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasWeakAnyLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_weak_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasWeakLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn has_weak_odr_linkage(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_hasWeakODRLinkage(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn is_declaration(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_isDeclaration(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn is_discardable_if_unused(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_isDiscardableIfUnused(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn is_thread_local(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_isThreadLocal(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn is_weak_for_linker(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_isWeakForLinker(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn may_be_overridden(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalValue_mayBeOverridden(::llvm::value::user::constant::GlobalValueObj::get_inner(self) as *const ::ffi::llvm_GlobalValue);
            ret
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_removeFromParent(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
        }
    }

    fn set_thread_local(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalValue_setThreadLocal(::llvm::value::user::constant::GlobalValueObj::get_inner(self), val);
        }
    }

    fn set_unnamed_addr(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalValue_setUnnamedAddr(::llvm::value::user::constant::GlobalValueObj::get_inner(self), val);
        }
    }
}
impl<T> GlobalValueExt for T where T: GlobalValueObj {}

pub struct GlobalValue {
    inner: ::core::nonzero::NonZero<*mut GlobalValueInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GlobalValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GlobalValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for GlobalValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GlobalValueObj for GlobalValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut GlobalValueInner {
        *self.inner
    }
}
impl GlobalValue {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut GlobalValueInner, owned: bool) -> GlobalValue {
        GlobalValue {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for GlobalValue {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueObj::get_inner(self));
            }
        }
    }
}
pub type GlobalVariableInner = ::ffi::llvm_GlobalVariable;

pub trait GlobalVariableObj: ::llvm::value::user::constant::GlobalObjectObj {
    unsafe fn get_inner(&self) -> *mut GlobalVariableInner;
}

pub trait GlobalVariableOwned: GlobalVariableObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut GlobalVariableInner {
        let inner = GlobalVariableObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> GlobalVariableOwned for T where T: GlobalVariableObj + ::core::marker::Sized {}

pub trait GlobalVariableExt: GlobalVariableObj {

    fn copy_attributes_from<A1: ::llvm::value::user::constant::GlobalValueObj = ::llvm::value::user::constant::GlobalValue>(&mut self, src: &mut A1) {
        unsafe {
            ::ffi::llvm::GlobalVariable_copyAttributesFrom(::llvm::value::user::constant::GlobalVariableObj::get_inner(self), ::llvm::value::user::constant::GlobalValueObj::get_inner(src));
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalVariable_eraseFromParent(::llvm::value::user::constant::GlobalVariableObj::get_inner(self));
        }
    }

    fn get_initializer(&self) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_getInitializer(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret as *mut ::ffi::llvm_Constant, false))
        }
    }

    fn get_initializer_mut(&mut self) -> Option<::llvm::value::user::constant::Constant> {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_getInitializerMut(::llvm::value::user::constant::GlobalVariableObj::get_inner(self));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::constant::Constant::from_inner(ret, false))
        }
    }

    fn has_definitive_initializer(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_hasDefinitiveInitializer(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            ret
        }
    }

    fn has_initializer(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_hasInitializer(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            ret
        }
    }

    fn has_unique_initializer(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_hasUniqueInitializer(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            ret
        }
    }

    fn is_constant(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_isConstant(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            ret
        }
    }

    fn is_externally_initialized(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_isExternallyInitialized(::llvm::value::user::constant::GlobalVariableObj::get_inner(self) as *const ::ffi::llvm_GlobalVariable);
            ret
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalVariable_removeFromParent(::llvm::value::user::constant::GlobalVariableObj::get_inner(self));
        }
    }

    fn set_constant(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setConstant(::llvm::value::user::constant::GlobalVariableObj::get_inner(self), val);
        }
    }

    fn set_externally_initialized(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setExternallyInitialized(::llvm::value::user::constant::GlobalVariableObj::get_inner(self), val);
        }
    }

    fn set_initializer<A1: ::llvm::value::user::constant::ConstantObj = ::llvm::value::user::constant::Constant>(&mut self, init_val: &mut A1) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setInitializer(::llvm::value::user::constant::GlobalVariableObj::get_inner(self), ::llvm::value::user::constant::ConstantObj::get_inner(init_val));
        }
    }
}
impl<T> GlobalVariableExt for T where T: GlobalVariableObj {}

pub struct GlobalVariable {
    inner: ::core::nonzero::NonZero<*mut GlobalVariableInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalObjectObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_GlobalObject {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl GlobalVariableObj for GlobalVariable {
    #[inline(always)]
    fn get_inner(&self) -> *mut GlobalVariableInner {
        *self.inner
    }
}
impl GlobalVariable {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut GlobalVariableInner, owned: bool) -> GlobalVariable {
        GlobalVariable {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn new<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, is_constant: bool, linkage: ::llvm::value::user::constant::LinkageTypes) -> ::llvm::value::user::constant::GlobalVariable {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_new(::llvm::ty::TypeObj::get_inner(ty), is_constant, linkage.to_ffi());
            if ret.is_null() {
                panic!("::llvm::GlobalVariable::new returned a null pointer!");
            }
            ::llvm::value::user::constant::GlobalVariable::from_inner(ret, true)
        }
    }

    pub fn new_with_module<A1: ::llvm::ModuleObj = ::llvm::Module, A2: ::llvm::ty::TypeObj = ::llvm::ty::Type, A5: ::llvm::value::user::constant::ConstantObj = ::llvm::value::user::constant::Constant>(module: &mut A1, ty: &mut A2, is_constant: bool, linkage: ::llvm::value::user::constant::LinkageTypes, initializer: &mut A5) -> ::llvm::value::user::constant::GlobalVariable {
        unsafe {
            let ret = ::ffi::llvm::GlobalVariable_newWithModule(::llvm::ModuleObj::get_inner(module), ::llvm::ty::TypeObj::get_inner(ty), is_constant, linkage.to_ffi(), ::llvm::value::user::constant::ConstantObj::get_inner(initializer));
            if ret.is_null() {
                panic!("::llvm::GlobalVariable::newWithModule returned a null pointer!");
            }
            ::llvm::value::user::constant::GlobalVariable::from_inner(ret, true)
        }
    }
}
impl Drop for GlobalVariable {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::GlobalVariable_delete(::llvm::value::user::constant::GlobalVariableObj::get_inner(self));
            }
        }
    }
}
pub type UndefValueInner = ::ffi::llvm_UndefValue;

pub trait UndefValueObj: ::llvm::value::user::constant::ConstantObj {
    unsafe fn get_inner(&self) -> *mut UndefValueInner;
}

pub trait UndefValueOwned: UndefValueObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut UndefValueInner {
        let inner = UndefValueObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> UndefValueOwned for T where T: UndefValueObj + ::core::marker::Sized {}

pub trait UndefValueExt: UndefValueObj {
}
impl<T> UndefValueExt for T where T: UndefValueObj {}

pub struct UndefValue {
    inner: ::core::nonzero::NonZero<*mut UndefValueInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for UndefValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for UndefValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantObj for UndefValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UndefValueObj for UndefValue {
    #[inline(always)]
    fn get_inner(&self) -> *mut UndefValueInner {
        *self.inner
    }
}
impl UndefValue {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut UndefValueInner, owned: bool) -> UndefValue {
        UndefValue {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for UndefValue {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
