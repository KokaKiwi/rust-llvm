pub type BlockAddressInner = ::ffi::llvm_BlockAddress;

pub trait BlockAddressExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut BlockAddressInner;
}

pub struct BlockAddress {
    inner: *mut BlockAddressInner,
}
impl ::llvm::value::ValueExt for BlockAddress {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BlockAddress {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for BlockAddress {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BlockAddressExt for BlockAddress {
    fn inner(&self) -> *mut BlockAddressInner {
        self.inner
    }
}
impl BlockAddress {
    pub unsafe fn from_inner(inner: *mut BlockAddressInner) -> BlockAddress {
        BlockAddress {
            inner: inner,
        }
    }
}
impl Drop for BlockAddress {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantInner = ::ffi::llvm_Constant;

pub trait ConstantExt: ::llvm::value::user::UserExt {

    fn inner(&self) -> *mut ConstantInner;

    fn can_trap(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_canTrap(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::Constant_destroyConstant(::llvm::value::user::constant::ConstantExt::inner(self));
        }
    }

    fn get_aggregate_element(&self, elt: u32) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAggregateElement(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant, elt as ::libc::c_uint))
        }
    }

    fn get_aggregate_element_constant<A1: ::llvm::value::user::constant::ConstantExt>(&self, elt: A1) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAggregateElementConstant(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant, ::llvm::value::user::constant::ConstantExt::inner(&elt)))
        }
    }

    fn get_splat_value(&self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getSplatValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant))
        }
    }

    fn is_all_ones_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isAllOnesValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_constant_used(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isConstantUsed(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_dll_import_dependent(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isDLLImportDependent(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_min_signed_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isMinSignedValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_negative_zero_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isNegativeZeroValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_null_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isNullValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_thread_dependent(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isThreadDependent(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn is_zero_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isZeroValue(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant)
        }
    }

    fn remove_dead_constant_users(&self) {
        unsafe {
            ::ffi::llvm::Constant_removeDeadConstantUsers(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant);
        }
    }

    fn strip_pointer_casts(&self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_stripPointerCasts(::llvm::value::user::constant::ConstantExt::inner(self) as *const ::ffi::llvm_Constant) as *mut ::ffi::llvm_Constant)
        }
    }

    fn strip_pointer_casts_mut(&mut self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_stripPointerCastsMut(::llvm::value::user::constant::ConstantExt::inner(self)))
        }
    }
}

pub struct Constant {
    inner: *mut ConstantInner,
}
impl ::llvm::value::ValueExt for Constant {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Constant {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantExt for Constant {
    fn inner(&self) -> *mut ConstantInner {
        self.inner
    }
}
impl Constant {
    pub unsafe fn from_inner(inner: *mut ConstantInner) -> Constant {
        Constant {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueExt>(v: A1) -> bool {
        unsafe {
            ::ffi::llvm::Constant_classof(::llvm::value::ValueExt::inner(&v))
        }
    }

    pub fn get_all_ones_value<A1: ::llvm::ty::TypeExt>(ty: A1) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAllOnesValue(::llvm::ty::TypeExt::inner(&ty)))
        }
    }

    pub fn get_integer_value<A1: ::llvm::ty::TypeExt>(ty: A1, value: (u32, &[u64])) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_value = ::ffi::llvm_APInt {
                num_bits: value.0 as ::libc::c_uint,
                value: ::ffi::llvm_ArrayRef__libc_uint64_t {
                data: value.1.as_ptr(),
                length: value.1.len() as ::libc::size_t,
            },
            };
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getIntegerValue(::llvm::ty::TypeExt::inner(&ty), c_value))
        }
    }

    pub fn get_null_value<A1: ::llvm::ty::TypeExt>(ty: A1) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getNullValue(::llvm::ty::TypeExt::inner(&ty)))
        }
    }
}
impl Drop for Constant {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantAggregateZeroInner = ::ffi::llvm_ConstantAggregateZero;

pub trait ConstantAggregateZeroExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantAggregateZeroInner;
}

pub struct ConstantAggregateZero {
    inner: *mut ConstantAggregateZeroInner,
}
impl ::llvm::value::ValueExt for ConstantAggregateZero {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantAggregateZero {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantAggregateZero {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantAggregateZeroExt for ConstantAggregateZero {
    fn inner(&self) -> *mut ConstantAggregateZeroInner {
        self.inner
    }
}
impl ConstantAggregateZero {
    pub unsafe fn from_inner(inner: *mut ConstantAggregateZeroInner) -> ConstantAggregateZero {
        ConstantAggregateZero {
            inner: inner,
        }
    }
}
impl Drop for ConstantAggregateZero {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantArrayInner = ::ffi::llvm_ConstantArray;

pub trait ConstantArrayExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantArrayInner;
}

pub struct ConstantArray {
    inner: *mut ConstantArrayInner,
}
impl ::llvm::value::ValueExt for ConstantArray {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantArray {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantArray {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantArrayExt for ConstantArray {
    fn inner(&self) -> *mut ConstantArrayInner {
        self.inner
    }
}
impl ConstantArray {
    pub unsafe fn from_inner(inner: *mut ConstantArrayInner) -> ConstantArray {
        ConstantArray {
            inner: inner,
        }
    }
}
impl Drop for ConstantArray {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantDataArrayInner = ::ffi::llvm_ConstantDataArray;

pub trait ConstantDataArrayExt: ::llvm::value::user::constant::ConstantDataSequentialExt {

    fn inner(&self) -> *mut ConstantDataArrayInner;
}

pub struct ConstantDataArray {
    inner: *mut ConstantDataArrayInner,
}
impl ::llvm::value::ValueExt for ConstantDataArray {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataArray {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataArray {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialExt for ConstantDataArray {
    fn inner(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataArrayExt for ConstantDataArray {
    fn inner(&self) -> *mut ConstantDataArrayInner {
        self.inner
    }
}
impl ConstantDataArray {
    pub unsafe fn from_inner(inner: *mut ConstantDataArrayInner) -> ConstantDataArray {
        ConstantDataArray {
            inner: inner,
        }
    }
}
impl Drop for ConstantDataArray {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantDataSequentialInner = ::ffi::llvm_ConstantDataSequential;

pub trait ConstantDataSequentialExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantDataSequentialInner;
}

pub struct ConstantDataSequential {
    inner: *mut ConstantDataSequentialInner,
}
impl ::llvm::value::ValueExt for ConstantDataSequential {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataSequential {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataSequential {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataSequentialExt for ConstantDataSequential {
    fn inner(&self) -> *mut ConstantDataSequentialInner {
        self.inner
    }
}
impl ConstantDataSequential {
    pub unsafe fn from_inner(inner: *mut ConstantDataSequentialInner) -> ConstantDataSequential {
        ConstantDataSequential {
            inner: inner,
        }
    }
}
impl Drop for ConstantDataSequential {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantDataVectorInner = ::ffi::llvm_ConstantDataVector;

pub trait ConstantDataVectorExt: ::llvm::value::user::constant::ConstantDataSequentialExt {

    fn inner(&self) -> *mut ConstantDataVectorInner;
}

pub struct ConstantDataVector {
    inner: *mut ConstantDataVectorInner,
}
impl ::llvm::value::ValueExt for ConstantDataVector {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataVector {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataVector {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialExt for ConstantDataVector {
    fn inner(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataVectorExt for ConstantDataVector {
    fn inner(&self) -> *mut ConstantDataVectorInner {
        self.inner
    }
}
impl ConstantDataVector {
    pub unsafe fn from_inner(inner: *mut ConstantDataVectorInner) -> ConstantDataVector {
        ConstantDataVector {
            inner: inner,
        }
    }
}
impl Drop for ConstantDataVector {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantExprInner = ::ffi::llvm_ConstantExpr;

pub trait ConstantExprExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantExprInner;
}

pub struct ConstantExpr {
    inner: *mut ConstantExprInner,
}
impl ::llvm::value::ValueExt for ConstantExpr {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantExpr {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantExpr {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantExprExt for ConstantExpr {
    fn inner(&self) -> *mut ConstantExprInner {
        self.inner
    }
}
impl ConstantExpr {
    pub unsafe fn from_inner(inner: *mut ConstantExprInner) -> ConstantExpr {
        ConstantExpr {
            inner: inner,
        }
    }
}
impl Drop for ConstantExpr {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantFPInner = ::ffi::llvm_ConstantFP;

pub trait ConstantFPExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantFPInner;
}

pub struct ConstantFP {
    inner: *mut ConstantFPInner,
}
impl ::llvm::value::ValueExt for ConstantFP {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantFP {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantFP {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantFPExt for ConstantFP {
    fn inner(&self) -> *mut ConstantFPInner {
        self.inner
    }
}
impl ConstantFP {
    pub unsafe fn from_inner(inner: *mut ConstantFPInner) -> ConstantFP {
        ConstantFP {
            inner: inner,
        }
    }
}
impl Drop for ConstantFP {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantIntInner = ::ffi::llvm_ConstantInt;

pub trait ConstantIntExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantIntInner;
}

pub struct ConstantInt {
    inner: *mut ConstantIntInner,
}
impl ::llvm::value::ValueExt for ConstantInt {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantInt {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantInt {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantIntExt for ConstantInt {
    fn inner(&self) -> *mut ConstantIntInner {
        self.inner
    }
}
impl ConstantInt {
    pub unsafe fn from_inner(inner: *mut ConstantIntInner) -> ConstantInt {
        ConstantInt {
            inner: inner,
        }
    }
}
impl Drop for ConstantInt {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantPointerNullInner = ::ffi::llvm_ConstantPointerNull;

pub trait ConstantPointerNullExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantPointerNullInner;
}

pub struct ConstantPointerNull {
    inner: *mut ConstantPointerNullInner,
}
impl ::llvm::value::ValueExt for ConstantPointerNull {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantPointerNull {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantPointerNull {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantPointerNullExt for ConstantPointerNull {
    fn inner(&self) -> *mut ConstantPointerNullInner {
        self.inner
    }
}
impl ConstantPointerNull {
    pub unsafe fn from_inner(inner: *mut ConstantPointerNullInner) -> ConstantPointerNull {
        ConstantPointerNull {
            inner: inner,
        }
    }
}
impl Drop for ConstantPointerNull {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantStructInner = ::ffi::llvm_ConstantStruct;

pub trait ConstantStructExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantStructInner;
}

pub struct ConstantStruct {
    inner: *mut ConstantStructInner,
}
impl ::llvm::value::ValueExt for ConstantStruct {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantStruct {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantStruct {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantStructExt for ConstantStruct {
    fn inner(&self) -> *mut ConstantStructInner {
        self.inner
    }
}
impl ConstantStruct {
    pub unsafe fn from_inner(inner: *mut ConstantStructInner) -> ConstantStruct {
        ConstantStruct {
            inner: inner,
        }
    }
}
impl Drop for ConstantStruct {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type ConstantVectorInner = ::ffi::llvm_ConstantVector;

pub trait ConstantVectorExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut ConstantVectorInner;
}

pub struct ConstantVector {
    inner: *mut ConstantVectorInner,
}
impl ::llvm::value::ValueExt for ConstantVector {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantVector {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantVector {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantVectorExt for ConstantVector {
    fn inner(&self) -> *mut ConstantVectorInner {
        self.inner
    }
}
impl ConstantVector {
    pub unsafe fn from_inner(inner: *mut ConstantVectorInner) -> ConstantVector {
        ConstantVector {
            inner: inner,
        }
    }
}
impl Drop for ConstantVector {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type GlobalValueInner = ::ffi::llvm_GlobalValue;

pub trait GlobalValueExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut GlobalValueInner;
}

pub struct GlobalValue {
    inner: *mut GlobalValueInner,
}
impl ::llvm::value::ValueExt for GlobalValue {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GlobalValue {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for GlobalValue {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GlobalValueExt for GlobalValue {
    fn inner(&self) -> *mut GlobalValueInner {
        self.inner
    }
}
impl GlobalValue {
    pub unsafe fn from_inner(inner: *mut GlobalValueInner) -> GlobalValue {
        GlobalValue {
            inner: inner,
        }
    }
}
impl Drop for GlobalValue {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
pub type UndefValueInner = ::ffi::llvm_UndefValue;

pub trait UndefValueExt: ::llvm::value::user::constant::ConstantExt {

    fn inner(&self) -> *mut UndefValueInner;
}

pub struct UndefValue {
    inner: *mut UndefValueInner,
}
impl ::llvm::value::ValueExt for UndefValue {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UndefValue {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for UndefValue {
    fn inner(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl UndefValueExt for UndefValue {
    fn inner(&self) -> *mut UndefValueInner {
        self.inner
    }
}
impl UndefValue {
    pub unsafe fn from_inner(inner: *mut UndefValueInner) -> UndefValue {
        UndefValue {
            inner: inner,
        }
    }
}
impl Drop for UndefValue {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
        }
    }
}
