pub type ConstantInner = ::ffi::llvm_Constant;

pub trait ConstantExt: ::llvm::value::user::UserExt {

    fn inner(&self) -> *mut ConstantInner;
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
}
impl Drop for Constant {
    fn drop(&mut self) {
        unsafe {
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
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
            ::ffi::llvm::Value_delete(::llvm::value::ValueExt::inner(self));
        }
    }
}
