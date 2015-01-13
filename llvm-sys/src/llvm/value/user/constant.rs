pub type BlockAddressInner = ::ffi::llvm_BlockAddress;

pub trait BlockAddressExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_BlockAddress(&self) -> *mut BlockAddressInner;

    fn inner(&self) -> *mut BlockAddressInner {
        self.inner_llvm_BlockAddress()
    }
}

pub struct BlockAddress {
    inner: *mut BlockAddressInner,
}
impl ::llvm::value::ValueExt for BlockAddress {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for BlockAddress {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for BlockAddress {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl BlockAddressExt for BlockAddress {
    fn inner_llvm_BlockAddress(&self) -> *mut BlockAddressInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantInner = ::ffi::llvm_Constant;

pub trait ConstantExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Constant(&self) -> *mut ConstantInner;

    fn inner(&self) -> *mut ConstantInner {
        self.inner_llvm_Constant()
    }

    fn can_trap(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_canTrap(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::Constant_destroyConstant(self.inner_llvm_Constant());
        }
    }

    fn get_aggregate_element(&self, elt: u32) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAggregateElement(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant, elt as ::libc::c_uint))
        }
    }

    fn get_aggregate_element_constant(&self, elt: &::llvm::value::user::constant::ConstantExt) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAggregateElementConstant(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant, elt.inner_llvm_Constant()))
        }
    }

    fn get_splat_value(&self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getSplatValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant))
        }
    }

    fn is_all_ones_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isAllOnesValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_constant_used(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isConstantUsed(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_dll_import_dependent(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isDLLImportDependent(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_min_signed_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isMinSignedValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_negative_zero_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isNegativeZeroValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_null_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isNullValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_thread_dependent(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isThreadDependent(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn is_zero_value(&self) -> bool {
        unsafe {
            ::ffi::llvm::Constant_isZeroValue(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant)
        }
    }

    fn remove_dead_constant_users(&self) {
        unsafe {
            ::ffi::llvm::Constant_removeDeadConstantUsers(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant);
        }
    }

    fn strip_pointer_casts(&self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_stripPointerCasts(self.inner_llvm_Constant() as *const ::ffi::llvm_Constant) as *mut ::ffi::llvm_Constant)
        }
    }

    fn strip_pointer_casts_mut(&mut self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_stripPointerCastsMut(self.inner_llvm_Constant()))
        }
    }
}

pub struct Constant {
    inner: *mut ConstantInner,
}
impl ::llvm::value::ValueExt for Constant {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Constant {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantExt for Constant {
    fn inner_llvm_Constant(&self) -> *mut ConstantInner {
        self.inner
    }
}
impl Constant {
    pub unsafe fn from_inner(inner: *mut ConstantInner) -> Constant {
        Constant {
            inner: inner,
        }
    }

    pub fn classof(v: &::llvm::value::ValueExt) -> bool {
        unsafe {
            ::ffi::llvm::Constant_classof(v.inner_llvm_Value())
        }
    }

    pub fn get_all_ones_value(ty: &::llvm::ty::TypeExt) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getAllOnesValue(ty.inner_llvm_Type()))
        }
    }

    pub fn get_integer_value(ty: &::llvm::ty::TypeExt, value: (u32, &[u64])) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let c_value = ::ffi::llvm_APInt {
                num_bits: value.0 as ::libc::c_uint,
                value: ::ffi::llvm_ArrayRef__libc_uint64_t {
                data: value.1.as_ptr(),
                length: value.1.len() as ::libc::size_t,
            },
            };
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getIntegerValue(ty.inner_llvm_Type(), c_value))
        }
    }

    pub fn get_null_value(ty: &::llvm::ty::TypeExt) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::Constant_getNullValue(ty.inner_llvm_Type()))
        }
    }
}
impl Drop for Constant {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantAggregateZeroInner = ::ffi::llvm_ConstantAggregateZero;

pub trait ConstantAggregateZeroExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantAggregateZero(&self) -> *mut ConstantAggregateZeroInner;

    fn inner(&self) -> *mut ConstantAggregateZeroInner {
        self.inner_llvm_ConstantAggregateZero()
    }
}

pub struct ConstantAggregateZero {
    inner: *mut ConstantAggregateZeroInner,
}
impl ::llvm::value::ValueExt for ConstantAggregateZero {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantAggregateZero {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantAggregateZero {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantAggregateZeroExt for ConstantAggregateZero {
    fn inner_llvm_ConstantAggregateZero(&self) -> *mut ConstantAggregateZeroInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantArrayInner = ::ffi::llvm_ConstantArray;

pub trait ConstantArrayExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantArray(&self) -> *mut ConstantArrayInner;

    fn inner(&self) -> *mut ConstantArrayInner {
        self.inner_llvm_ConstantArray()
    }

    fn get_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::ConstantArray_getType(self.inner_llvm_ConstantArray() as *const ::ffi::llvm_ConstantArray))
        }
    }
}

pub struct ConstantArray {
    inner: *mut ConstantArrayInner,
}
impl ::llvm::value::ValueExt for ConstantArray {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantArray {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantArray {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantArrayExt for ConstantArray {
    fn inner_llvm_ConstantArray(&self) -> *mut ConstantArrayInner {
        self.inner
    }
}
impl ConstantArray {
    pub unsafe fn from_inner(inner: *mut ConstantArrayInner) -> ConstantArray {
        ConstantArray {
            inner: inner,
        }
    }

    pub fn classof(v: &::llvm::value::ValueExt) -> bool {
        unsafe {
            ::ffi::llvm::ConstantArray_classof(v.inner_llvm_Value())
        }
    }

    pub fn get(ty: &::llvm::ty::seq::ArrayTypeExt, values: &[&::llvm::value::user::constant::ConstantExt]) -> ::llvm::value::user::constant::Constant {
        unsafe {
            let _tmp_values: Vec<_> = values.iter().map(|&ty| ty.inner_llvm_Constant()).collect();
            let c_values = ::ffi::llvm_ArrayRef_llvm_Constant_ptr {
                data: _tmp_values.as_ptr(),
                length: values.len() as ::libc::size_t,
            };
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::ConstantArray_get(ty.inner_llvm_ArrayType(), c_values))
        }
    }
}
impl Drop for ConstantArray {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantDataArrayInner = ::ffi::llvm_ConstantDataArray;

pub trait ConstantDataArrayExt: ::llvm::value::user::constant::ConstantDataSequentialExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantDataArray(&self) -> *mut ConstantDataArrayInner;

    fn inner(&self) -> *mut ConstantDataArrayInner {
        self.inner_llvm_ConstantDataArray()
    }
}

pub struct ConstantDataArray {
    inner: *mut ConstantDataArrayInner,
}
impl ::llvm::value::ValueExt for ConstantDataArray {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataArray {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataArray {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialExt for ConstantDataArray {
    fn inner_llvm_ConstantDataSequential(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataArrayExt for ConstantDataArray {
    fn inner_llvm_ConstantDataArray(&self) -> *mut ConstantDataArrayInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantDataSequentialInner = ::ffi::llvm_ConstantDataSequential;

pub trait ConstantDataSequentialExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantDataSequential(&self) -> *mut ConstantDataSequentialInner;

    fn inner(&self) -> *mut ConstantDataSequentialInner {
        self.inner_llvm_ConstantDataSequential()
    }
}

pub struct ConstantDataSequential {
    inner: *mut ConstantDataSequentialInner,
}
impl ::llvm::value::ValueExt for ConstantDataSequential {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataSequential {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataSequential {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataSequentialExt for ConstantDataSequential {
    fn inner_llvm_ConstantDataSequential(&self) -> *mut ConstantDataSequentialInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantDataVectorInner = ::ffi::llvm_ConstantDataVector;

pub trait ConstantDataVectorExt: ::llvm::value::user::constant::ConstantDataSequentialExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantDataVector(&self) -> *mut ConstantDataVectorInner;

    fn inner(&self) -> *mut ConstantDataVectorInner {
        self.inner_llvm_ConstantDataVector()
    }
}

pub struct ConstantDataVector {
    inner: *mut ConstantDataVectorInner,
}
impl ::llvm::value::ValueExt for ConstantDataVector {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantDataVector {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantDataVector {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantDataSequentialExt for ConstantDataVector {
    fn inner_llvm_ConstantDataSequential(&self) -> *mut ::ffi::llvm_ConstantDataSequential {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantDataVectorExt for ConstantDataVector {
    fn inner_llvm_ConstantDataVector(&self) -> *mut ConstantDataVectorInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantExprInner = ::ffi::llvm_ConstantExpr;

pub trait ConstantExprExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantExpr(&self) -> *mut ConstantExprInner;

    fn inner(&self) -> *mut ConstantExprInner {
        self.inner_llvm_ConstantExpr()
    }
}

pub struct ConstantExpr {
    inner: *mut ConstantExprInner,
}
impl ::llvm::value::ValueExt for ConstantExpr {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantExpr {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantExpr {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantExprExt for ConstantExpr {
    fn inner_llvm_ConstantExpr(&self) -> *mut ConstantExprInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantFPInner = ::ffi::llvm_ConstantFP;

pub trait ConstantFPExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantFP(&self) -> *mut ConstantFPInner;

    fn inner(&self) -> *mut ConstantFPInner {
        self.inner_llvm_ConstantFP()
    }
}

pub struct ConstantFP {
    inner: *mut ConstantFPInner,
}
impl ::llvm::value::ValueExt for ConstantFP {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantFP {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantFP {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantFPExt for ConstantFP {
    fn inner_llvm_ConstantFP(&self) -> *mut ConstantFPInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantIntInner = ::ffi::llvm_ConstantInt;

pub trait ConstantIntExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantInt(&self) -> *mut ConstantIntInner;

    fn inner(&self) -> *mut ConstantIntInner {
        self.inner_llvm_ConstantInt()
    }
}

pub struct ConstantInt {
    inner: *mut ConstantIntInner,
}
impl ::llvm::value::ValueExt for ConstantInt {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantInt {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantInt {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantIntExt for ConstantInt {
    fn inner_llvm_ConstantInt(&self) -> *mut ConstantIntInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantPointerNullInner = ::ffi::llvm_ConstantPointerNull;

pub trait ConstantPointerNullExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantPointerNull(&self) -> *mut ConstantPointerNullInner;

    fn inner(&self) -> *mut ConstantPointerNullInner {
        self.inner_llvm_ConstantPointerNull()
    }
}

pub struct ConstantPointerNull {
    inner: *mut ConstantPointerNullInner,
}
impl ::llvm::value::ValueExt for ConstantPointerNull {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantPointerNull {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantPointerNull {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantPointerNullExt for ConstantPointerNull {
    fn inner_llvm_ConstantPointerNull(&self) -> *mut ConstantPointerNullInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantStructInner = ::ffi::llvm_ConstantStruct;

pub trait ConstantStructExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantStruct(&self) -> *mut ConstantStructInner;

    fn inner(&self) -> *mut ConstantStructInner {
        self.inner_llvm_ConstantStruct()
    }
}

pub struct ConstantStruct {
    inner: *mut ConstantStructInner,
}
impl ::llvm::value::ValueExt for ConstantStruct {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantStruct {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantStruct {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantStructExt for ConstantStruct {
    fn inner_llvm_ConstantStruct(&self) -> *mut ConstantStructInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type ConstantVectorInner = ::ffi::llvm_ConstantVector;

pub trait ConstantVectorExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ConstantVector(&self) -> *mut ConstantVectorInner;

    fn inner(&self) -> *mut ConstantVectorInner {
        self.inner_llvm_ConstantVector()
    }
}

pub struct ConstantVector {
    inner: *mut ConstantVectorInner,
}
impl ::llvm::value::ValueExt for ConstantVector {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for ConstantVector {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for ConstantVector {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ConstantVectorExt for ConstantVector {
    fn inner_llvm_ConstantVector(&self) -> *mut ConstantVectorInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
pub type FunctionInner = ::ffi::llvm_Function;

pub trait FunctionExt: ::llvm::value::user::constant::GlobalObjectExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Function(&self) -> *mut FunctionInner;

    fn inner(&self) -> *mut FunctionInner {
        self.inner_llvm_Function()
    }

    fn add_fn_attr(&mut self, kind: &str) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Function_addFnAttr(self.inner_llvm_Function(), c_kind);
        }
    }

    fn add_fn_attr_with_value(&mut self, kind: &str, val: &str) {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            let c_val = ::ffi::llvm_StringRef {
                data: val.as_ptr() as *const ::libc::c_char,
                length: val.len() as ::libc::size_t,
            };
            ::ffi::llvm::Function_addFnAttrWithValue(self.inner_llvm_Function(), c_kind, c_val);
        }
    }

    fn cannot_duplicate(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_cannotDuplicate(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn clear_gc(&mut self) {
        unsafe {
            ::ffi::llvm::Function_clearGC(self.inner_llvm_Function());
        }
    }

    fn copy_attributes_from(&mut self, src: &::llvm::value::user::constant::GlobalValueExt) {
        unsafe {
            ::ffi::llvm::Function_copyAttributesFrom(self.inner_llvm_Function(), src.inner_llvm_GlobalValue());
        }
    }

    fn delete_body(&mut self) {
        unsafe {
            ::ffi::llvm::Function_deleteBody(self.inner_llvm_Function());
        }
    }

    fn does_not_access_memory(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotAccessMemory(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn does_not_access_memory_param(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotAccessMemoryParam(self.inner_llvm_Function() as *const ::ffi::llvm_Function, n as ::libc::c_uint)
        }
    }

    fn does_not_alias(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotAlias(self.inner_llvm_Function() as *const ::ffi::llvm_Function, n as ::libc::c_uint)
        }
    }

    fn does_not_capture(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotCapture(self.inner_llvm_Function() as *const ::ffi::llvm_Function, n as ::libc::c_uint)
        }
    }

    fn does_not_return(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotReturn(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn does_not_throw(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_doesNotThrow(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Function_eraseFromParent(self.inner_llvm_Function());
        }
    }

    fn get_calling_conv(&self) -> ::llvm::calling_conv::ID {
        unsafe {
            ::llvm::calling_conv::ID::from_ffi(::ffi::llvm::Function_getCallingConv(self.inner_llvm_Function() as *const ::ffi::llvm_Function))
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Function_getContext(self.inner_llvm_Function() as *const ::ffi::llvm_Function))
        }
    }

    fn get_dereferenceable_bytes(&self, idx: u32) -> u64 {
        unsafe {
            ::ffi::llvm::Function_getDereferenceableBytes(self.inner_llvm_Function() as *const ::ffi::llvm_Function, idx as ::libc::c_uint) as u64
        }
    }

    fn get_function_type(&self) -> ::llvm::ty::FunctionType {
        unsafe {
            ::llvm::ty::FunctionType::from_inner(::ffi::llvm::Function_getFunctionType(self.inner_llvm_Function() as *const ::ffi::llvm_Function))
        }
    }

    fn get_intrinsic_id(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Function_getIntrinsicID(self.inner_llvm_Function() as *const ::ffi::llvm_Function) as u32
        }
    }

    fn get_param_alignment(&self, idx: u32) -> u32 {
        unsafe {
            ::ffi::llvm::Function_getParamAlignment(self.inner_llvm_Function() as *const ::ffi::llvm_Function, idx as ::libc::c_uint) as u32
        }
    }

    fn get_return_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Function_getReturnType(self.inner_llvm_Function() as *const ::ffi::llvm_Function))
        }
    }

    fn has_fn_attr(&self, kind: &str) -> bool {
        unsafe {
            let c_kind = ::ffi::llvm_StringRef {
                data: kind.as_ptr() as *const ::libc::c_char,
                length: kind.len() as ::libc::size_t,
            };
            ::ffi::llvm::Function_hasFnAttr(self.inner_llvm_Function() as *const ::ffi::llvm_Function, c_kind)
        }
    }

    fn has_gc(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_hasGC(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn has_struct_ret_attr(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_hasStructRetAttr(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn has_uw_table(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_hasUWTable(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn is_intrinsic(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_isIntrinsic(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_isVarArg(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn needs_unwind_table_entry(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_needsUnwindTableEntry(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn only_reads_memory(&self) -> bool {
        unsafe {
            ::ffi::llvm::Function_onlyReadsMemory(self.inner_llvm_Function() as *const ::ffi::llvm_Function)
        }
    }

    fn only_reads_memory_param(&self, n: u32) -> bool {
        unsafe {
            ::ffi::llvm::Function_onlyReadsMemoryParam(self.inner_llvm_Function() as *const ::ffi::llvm_Function, n as ::libc::c_uint)
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::Function_removeFromParent(self.inner_llvm_Function());
        }
    }

    fn set_calling_conv(&mut self, cc: ::llvm::calling_conv::ID) {
        unsafe {
            ::ffi::llvm::Function_setCallingConv(self.inner_llvm_Function(), cc.to_ffi());
        }
    }

    fn set_cannot_duplicate(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setCannotDuplicate(self.inner_llvm_Function());
        }
    }

    fn set_does_not_access_memory(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAccessMemory(self.inner_llvm_Function());
        }
    }

    fn set_does_not_access_memory_param(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAccessMemoryParam(self.inner_llvm_Function(), n as ::libc::c_uint);
        }
    }

    fn set_does_not_alias(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotAlias(self.inner_llvm_Function(), n as ::libc::c_uint);
        }
    }

    fn set_does_not_capture(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotCapture(self.inner_llvm_Function(), n as ::libc::c_uint);
        }
    }

    fn set_does_not_return(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotReturn(self.inner_llvm_Function());
        }
    }

    fn set_does_not_throw(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setDoesNotThrow(self.inner_llvm_Function());
        }
    }

    fn set_has_uw_table(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setHasUWTable(self.inner_llvm_Function());
        }
    }

    fn set_only_reads_memory(&mut self) {
        unsafe {
            ::ffi::llvm::Function_setOnlyReadsMemory(self.inner_llvm_Function());
        }
    }

    fn set_only_reads_memory_param(&mut self, n: u32) {
        unsafe {
            ::ffi::llvm::Function_setOnlyReadsMemoryParam(self.inner_llvm_Function(), n as ::libc::c_uint);
        }
    }
}

pub struct Function {
    inner: *mut FunctionInner,
}
impl ::llvm::value::ValueExt for Function {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Function {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for Function {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueExt for Function {
    fn inner_llvm_GlobalValue(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalObjectExt for Function {
    fn inner_llvm_GlobalObject(&self) -> *mut ::ffi::llvm_GlobalObject {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FunctionExt for Function {
    fn inner_llvm_Function(&self) -> *mut FunctionInner {
        self.inner
    }
}
impl Function {
    pub unsafe fn from_inner(inner: *mut FunctionInner) -> Function {
        Function {
            inner: inner,
        }
    }

    pub fn create(ty: &::llvm::ty::FunctionTypeExt, linkage: ::llvm::value::user::constant::LinkageTypes) -> ::llvm::value::user::constant::Function {
        unsafe {
            ::llvm::value::user::constant::Function::from_inner(::ffi::llvm::Function_Create(ty.inner_llvm_FunctionType(), linkage.to_ffi()))
        }
    }

    pub fn create_with_name(ty: &::llvm::ty::FunctionTypeExt, linkage: ::llvm::value::user::constant::LinkageTypes, name: &str) -> ::llvm::value::user::constant::Function {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::value::user::constant::Function::from_inner(::ffi::llvm::Function_CreateWithName(ty.inner_llvm_FunctionType(), linkage.to_ffi(), c_name))
        }
    }

    pub fn classof(val: &::llvm::value::ValueExt) -> bool {
        unsafe {
            ::ffi::llvm::Function_classof(val.inner_llvm_Value())
        }
    }
}
impl Drop for Function {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Function_delete(::llvm::value::user::constant::FunctionExt::inner_llvm_Function(self));
        }
    }
}
pub type GlobalAliasInner = ::ffi::llvm_GlobalAlias;

pub trait GlobalAliasExt: ::llvm::value::user::constant::GlobalValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_GlobalAlias(&self) -> *mut GlobalAliasInner;

    fn inner(&self) -> *mut GlobalAliasInner {
        self.inner_llvm_GlobalAlias()
    }
}

pub struct GlobalAlias {
    inner: *mut GlobalAliasInner,
}
impl ::llvm::value::ValueExt for GlobalAlias {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GlobalAlias {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for GlobalAlias {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueExt for GlobalAlias {
    fn inner_llvm_GlobalValue(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GlobalAliasExt for GlobalAlias {
    fn inner_llvm_GlobalAlias(&self) -> *mut GlobalAliasInner {
        self.inner
    }
}
impl GlobalAlias {
    pub unsafe fn from_inner(inner: *mut GlobalAliasInner) -> GlobalAlias {
        GlobalAlias {
            inner: inner,
        }
    }
}
impl Drop for GlobalAlias {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueExt::inner_llvm_GlobalValue(self));
        }
    }
}
pub type GlobalObjectInner = ::ffi::llvm_GlobalObject;

pub trait GlobalObjectExt: ::llvm::value::user::constant::GlobalValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_GlobalObject(&self) -> *mut GlobalObjectInner;

    fn inner(&self) -> *mut GlobalObjectInner {
        self.inner_llvm_GlobalObject()
    }

    fn set_section(&mut self, s: &str) {
        unsafe {
            let c_s = ::ffi::llvm_StringRef {
                data: s.as_ptr() as *const ::libc::c_char,
                length: s.len() as ::libc::size_t,
            };
            ::ffi::llvm::GlobalObject_setSection(self.inner_llvm_GlobalObject(), c_s);
        }
    }
}

pub struct GlobalObject {
    inner: *mut GlobalObjectInner,
}
impl ::llvm::value::ValueExt for GlobalObject {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GlobalObject {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for GlobalObject {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueExt for GlobalObject {
    fn inner_llvm_GlobalValue(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GlobalObjectExt for GlobalObject {
    fn inner_llvm_GlobalObject(&self) -> *mut GlobalObjectInner {
        self.inner
    }
}
impl GlobalObject {
    pub unsafe fn from_inner(inner: *mut GlobalObjectInner) -> GlobalObject {
        GlobalObject {
            inner: inner,
        }
    }
}
impl Drop for GlobalObject {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueExt::inner_llvm_GlobalValue(self));
        }
    }
}
#[derive(Copy)]
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
pub type GlobalValueInner = ::ffi::llvm_GlobalValue;

pub trait GlobalValueExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_GlobalValue(&self) -> *mut GlobalValueInner;

    fn inner(&self) -> *mut GlobalValueInner {
        self.inner_llvm_GlobalValue()
    }

    fn copy_attributes_from(&mut self, src: &::llvm::value::user::constant::GlobalValueExt) {
        unsafe {
            ::ffi::llvm::GlobalValue_copyAttributesFrom(self.inner_llvm_GlobalValue(), src.inner_llvm_GlobalValue());
        }
    }

    fn destroy_constant(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_destroyConstant(self.inner_llvm_GlobalValue());
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_eraseFromParent(self.inner_llvm_GlobalValue());
        }
    }

    fn get_alignment(&self) -> u32 {
        unsafe {
            ::ffi::llvm::GlobalValue_getAlignment(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue) as u32
        }
    }

    fn get_data_layout(&self) -> ::llvm::DataLayout {
        unsafe {
            ::llvm::DataLayout::from_inner(::ffi::llvm::GlobalValue_getDataLayout(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue) as *mut ::ffi::llvm_DataLayout)
        }
    }

    fn get_parent(&self) -> ::llvm::Module {
        unsafe {
            ::llvm::Module::from_inner(::ffi::llvm::GlobalValue_getParent(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue) as *mut ::ffi::llvm_Module)
        }
    }

    fn get_parent_mut(&mut self) -> ::llvm::Module {
        unsafe {
            ::llvm::Module::from_inner(::ffi::llvm::GlobalValue_getParentMut(self.inner_llvm_GlobalValue()))
        }
    }

    fn get_type(&self) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::GlobalValue_getType(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue))
        }
    }

    fn has_appending_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasAppendingLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_available_externally_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasAvailableExternallyLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_common_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasCommonLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_dll_export_storage_class(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasDLLExportStorageClass(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_dll_import_storage_class(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasDLLImportStorageClass(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_default_visibility(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasDefaultVisibility(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_external_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasExternalLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_external_weak_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasExternalWeakLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_hidden_visibility(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasHiddenVisibility(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_internal_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasInternalLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_link_once_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasLinkOnceLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_local_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasLocalLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_private_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasPrivateLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_protected_visibility(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasProtectedVisibility(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_section(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasSection(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_unnamed_addr(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasUnnamedAddr(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_weak_any_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasWeakAnyLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_weak_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasWeakLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn has_weak_odr_linkage(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_hasWeakODRLinkage(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn is_declaration(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_isDeclaration(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn is_discardable_if_unused(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_isDiscardableIfUnused(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn is_thread_local(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_isThreadLocal(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn is_weak_for_linker(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_isWeakForLinker(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn may_be_overridden(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalValue_mayBeOverridden(self.inner_llvm_GlobalValue() as *const ::ffi::llvm_GlobalValue)
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalValue_removeFromParent(self.inner_llvm_GlobalValue());
        }
    }

    fn set_thread_local(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalValue_setThreadLocal(self.inner_llvm_GlobalValue(), val);
        }
    }

    fn set_unnamed_addr(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalValue_setUnnamedAddr(self.inner_llvm_GlobalValue(), val);
        }
    }
}

pub struct GlobalValue {
    inner: *mut GlobalValueInner,
}
impl ::llvm::value::ValueExt for GlobalValue {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GlobalValue {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for GlobalValue {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GlobalValueExt for GlobalValue {
    fn inner_llvm_GlobalValue(&self) -> *mut GlobalValueInner {
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
            ::ffi::llvm::GlobalValue_delete(::llvm::value::user::constant::GlobalValueExt::inner_llvm_GlobalValue(self));
        }
    }
}
pub type GlobalVariableInner = ::ffi::llvm_GlobalVariable;

pub trait GlobalVariableExt: ::llvm::value::user::constant::GlobalObjectExt {
    #[allow(non_snake_case)]
    fn inner_llvm_GlobalVariable(&self) -> *mut GlobalVariableInner;

    fn inner(&self) -> *mut GlobalVariableInner {
        self.inner_llvm_GlobalVariable()
    }

    fn copy_attributes_from(&mut self, src: &::llvm::value::user::constant::GlobalValueExt) {
        unsafe {
            ::ffi::llvm::GlobalVariable_copyAttributesFrom(self.inner_llvm_GlobalVariable(), src.inner_llvm_GlobalValue());
        }
    }

    fn erase_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalVariable_eraseFromParent(self.inner_llvm_GlobalVariable());
        }
    }

    fn get_initializer(&self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::GlobalVariable_getInitializer(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable) as *mut ::ffi::llvm_Constant)
        }
    }

    fn get_initializer_mut(&mut self) -> ::llvm::value::user::constant::Constant {
        unsafe {
            ::llvm::value::user::constant::Constant::from_inner(::ffi::llvm::GlobalVariable_getInitializerMut(self.inner_llvm_GlobalVariable()))
        }
    }

    fn has_definitive_initializer(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalVariable_hasDefinitiveInitializer(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable)
        }
    }

    fn has_initializer(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalVariable_hasInitializer(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable)
        }
    }

    fn has_unique_initializer(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalVariable_hasUniqueInitializer(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable)
        }
    }

    fn is_constant(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalVariable_isConstant(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable)
        }
    }

    fn is_externally_initialized(&self) -> bool {
        unsafe {
            ::ffi::llvm::GlobalVariable_isExternallyInitialized(self.inner_llvm_GlobalVariable() as *const ::ffi::llvm_GlobalVariable)
        }
    }

    fn remove_from_parent(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalVariable_removeFromParent(self.inner_llvm_GlobalVariable());
        }
    }

    fn set_constant(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setConstant(self.inner_llvm_GlobalVariable(), val);
        }
    }

    fn set_externally_initialized(&mut self, val: bool) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setExternallyInitialized(self.inner_llvm_GlobalVariable(), val);
        }
    }

    fn set_initializer(&mut self, init_val: &::llvm::value::user::constant::ConstantExt) {
        unsafe {
            ::ffi::llvm::GlobalVariable_setInitializer(self.inner_llvm_GlobalVariable(), init_val.inner_llvm_Constant());
        }
    }
}

pub struct GlobalVariable {
    inner: *mut GlobalVariableInner,
}
impl ::llvm::value::ValueExt for GlobalVariable {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for GlobalVariable {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for GlobalVariable {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalValueExt for GlobalVariable {
    fn inner_llvm_GlobalValue(&self) -> *mut ::ffi::llvm_GlobalValue {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::GlobalObjectExt for GlobalVariable {
    fn inner_llvm_GlobalObject(&self) -> *mut ::ffi::llvm_GlobalObject {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl GlobalVariableExt for GlobalVariable {
    fn inner_llvm_GlobalVariable(&self) -> *mut GlobalVariableInner {
        self.inner
    }
}
impl GlobalVariable {
    pub unsafe fn from_inner(inner: *mut GlobalVariableInner) -> GlobalVariable {
        GlobalVariable {
            inner: inner,
        }
    }

    pub fn new(ty: &::llvm::ty::TypeExt, is_constant: bool, linkage: ::llvm::value::user::constant::LinkageTypes) -> ::llvm::value::user::constant::GlobalVariable {
        unsafe {
            ::llvm::value::user::constant::GlobalVariable::from_inner(::ffi::llvm::GlobalVariable_new(ty.inner_llvm_Type(), is_constant, linkage.to_ffi()))
        }
    }

    pub fn new_with_module(module: &::llvm::ModuleExt, ty: &::llvm::ty::TypeExt, is_constant: bool, linkage: ::llvm::value::user::constant::LinkageTypes, initializer: &::llvm::value::user::constant::ConstantExt) -> ::llvm::value::user::constant::GlobalVariable {
        unsafe {
            ::llvm::value::user::constant::GlobalVariable::from_inner(::ffi::llvm::GlobalVariable_newWithModule(module.inner_llvm_Module(), ty.inner_llvm_Type(), is_constant, linkage.to_ffi(), initializer.inner_llvm_Constant()))
        }
    }
}
impl Drop for GlobalVariable {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::GlobalVariable_delete(::llvm::value::user::constant::GlobalVariableExt::inner_llvm_GlobalVariable(self));
        }
    }
}
pub type UndefValueInner = ::ffi::llvm_UndefValue;

pub trait UndefValueExt: ::llvm::value::user::constant::ConstantExt {
    #[allow(non_snake_case)]
    fn inner_llvm_UndefValue(&self) -> *mut UndefValueInner;

    fn inner(&self) -> *mut UndefValueInner {
        self.inner_llvm_UndefValue()
    }
}

pub struct UndefValue {
    inner: *mut UndefValueInner,
}
impl ::llvm::value::ValueExt for UndefValue {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for UndefValue {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::constant::ConstantExt for UndefValue {
    fn inner_llvm_Constant(&self) -> *mut ::ffi::llvm_Constant {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl UndefValueExt for UndefValue {
    fn inner_llvm_UndefValue(&self) -> *mut UndefValueInner {
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
            ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
        }
    }
}
