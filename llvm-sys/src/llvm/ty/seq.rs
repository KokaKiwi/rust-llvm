pub type ArrayTypeInner = ::ffi::llvm_ArrayType;

pub trait ArrayTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    unsafe fn get_inner(&self) -> *mut ArrayTypeInner;
}

pub trait ArrayTypeOwned: ArrayTypeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut ArrayTypeInner {
        let inner = ArrayTypeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> ArrayTypeOwned for T where T: ArrayTypeObj + ::core::marker::Sized {}

pub trait ArrayTypeExt: ArrayTypeObj {

    fn get_num_elements(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_getNumElements(::llvm::ty::seq::ArrayTypeObj::get_inner(self) as *const ::ffi::llvm_ArrayType) as u64;
            ret
        }
    }
}
impl<T> ArrayTypeExt for T where T: ArrayTypeObj {}

pub struct ArrayType {
    inner: ::core::nonzero::NonZero<*mut ArrayTypeInner>,
}
impl ::llvm::ty::TypeObj for ArrayType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for ArrayType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for ArrayType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ArrayTypeObj for ArrayType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ArrayTypeInner {
        *self.inner
    }
}
impl ArrayType {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut ArrayTypeInner) -> ArrayType {
        ArrayType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_classof(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(element_type: &mut A1, num_elements: u64) -> Option<::llvm::ty::seq::ArrayType> {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_get(::llvm::ty::TypeObj::get_inner(element_type), num_elements as ::libc::uint64_t);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::ArrayType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_isValidElementType(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }
}
impl Copy for ArrayType {}
pub type PointerTypeInner = ::ffi::llvm_PointerType;

pub trait PointerTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    unsafe fn get_inner(&self) -> *mut PointerTypeInner;
}

pub trait PointerTypeOwned: PointerTypeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut PointerTypeInner {
        let inner = PointerTypeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> PointerTypeOwned for T where T: PointerTypeObj + ::core::marker::Sized {}

pub trait PointerTypeExt: PointerTypeObj {

    fn get_address_space(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getAddressSpace(::llvm::ty::seq::PointerTypeObj::get_inner(self) as *const ::ffi::llvm_PointerType) as u32;
            ret
        }
    }
}
impl<T> PointerTypeExt for T where T: PointerTypeObj {}

pub struct PointerType {
    inner: ::core::nonzero::NonZero<*mut PointerTypeInner>,
}
impl ::llvm::ty::TypeObj for PointerType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for PointerType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for PointerType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PointerTypeObj for PointerType {
    #[inline(always)]
    fn get_inner(&self) -> *mut PointerTypeInner {
        *self.inner
    }
}
impl PointerType {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut PointerTypeInner) -> PointerType {
        PointerType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_classof(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(element_type: &mut A1, address_space: u32) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_get(::llvm::ty::TypeObj::get_inner(element_type), address_space as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_unqual<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(element_type: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getUnqual(::llvm::ty::TypeObj::get_inner(element_type));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_isValidElementType(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }
}
impl Copy for PointerType {}
pub type SequentialTypeInner = ::ffi::llvm_SequentialType;

pub trait SequentialTypeObj: ::llvm::ty::CompositeTypeObj {
    unsafe fn get_inner(&self) -> *mut SequentialTypeInner;
}

pub trait SequentialTypeOwned: SequentialTypeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut SequentialTypeInner {
        let inner = SequentialTypeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> SequentialTypeOwned for T where T: SequentialTypeObj + ::core::marker::Sized {}

pub trait SequentialTypeExt: SequentialTypeObj {

    fn get_element_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_getElementType(::llvm::ty::seq::SequentialTypeObj::get_inner(self) as *const ::ffi::llvm_SequentialType);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }
}
impl<T> SequentialTypeExt for T where T: SequentialTypeObj {}

pub struct SequentialType {
    inner: ::core::nonzero::NonZero<*mut SequentialTypeInner>,
}
impl ::llvm::ty::TypeObj for SequentialType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for SequentialType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SequentialTypeObj for SequentialType {
    #[inline(always)]
    fn get_inner(&self) -> *mut SequentialTypeInner {
        *self.inner
    }
}
impl SequentialType {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut SequentialTypeInner) -> SequentialType {
        SequentialType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_classof(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }
}
impl Copy for SequentialType {}
pub type VectorTypeInner = ::ffi::llvm_VectorType;

pub trait VectorTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    unsafe fn get_inner(&self) -> *mut VectorTypeInner;
}

pub trait VectorTypeOwned: VectorTypeObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut VectorTypeInner {
        let inner = VectorTypeObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> VectorTypeOwned for T where T: VectorTypeObj + ::core::marker::Sized {}

pub trait VectorTypeExt: VectorTypeObj {

    fn get_bit_width(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getBitWidth(::llvm::ty::seq::VectorTypeObj::get_inner(self) as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getNumElements(::llvm::ty::seq::VectorTypeObj::get_inner(self) as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }
}
impl<T> VectorTypeExt for T where T: VectorTypeObj {}

pub struct VectorType {
    inner: ::core::nonzero::NonZero<*mut VectorTypeInner>,
}
impl ::llvm::ty::TypeObj for VectorType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for VectorType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for VectorType {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VectorTypeObj for VectorType {
    #[inline(always)]
    fn get_inner(&self) -> *mut VectorTypeInner {
        *self.inner
    }
}
impl VectorType {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut VectorTypeInner) -> VectorType {
        VectorType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_classof(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1, num_elements: u32) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_get(::llvm::ty::TypeObj::get_inner(ty), num_elements as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_double_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeObj = ::llvm::ty::seq::VectorType>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getDoubleElementsVectorType(::llvm::ty::seq::VectorTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_extended_element_vector_type<A1: ::llvm::ty::seq::VectorTypeObj = ::llvm::ty::seq::VectorType>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getExtendedElementVectorType(::llvm::ty::seq::VectorTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_half_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeObj = ::llvm::ty::seq::VectorType>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getHalfElementsVectorType(::llvm::ty::seq::VectorTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_integer<A1: ::llvm::ty::seq::VectorTypeObj = ::llvm::ty::seq::VectorType>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getInteger(::llvm::ty::seq::VectorTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_truncated_element_vector_type<A1: ::llvm::ty::seq::VectorTypeObj = ::llvm::ty::seq::VectorType>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getTruncatedElementVectorType(::llvm::ty::seq::VectorTypeObj::get_inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj = ::llvm::ty::Type>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_isValidElementType(::llvm::ty::TypeObj::get_inner(ty));
            ret
        }
    }
}
impl Copy for VectorType {}
