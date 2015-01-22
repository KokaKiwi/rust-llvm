pub type ArrayTypeInner = ::ffi::llvm_ArrayType;

pub trait ArrayTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    fn inner(&self) -> *mut ArrayTypeInner;
}

pub trait ArrayTypeExt: ArrayTypeObj {

    fn get_num_elements(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_getNumElements(::llvm::ty::seq::ArrayTypeObj::inner(self) as *const ::ffi::llvm_ArrayType) as u64;
            ret
        }
    }
}
impl<T> ArrayTypeExt for T where T: ArrayTypeObj {}

pub struct ArrayType {
    inner: ::core::nonzero::NonZero<*mut ArrayTypeInner>,
}
impl ::llvm::ty::TypeObj for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ArrayTypeObj for ArrayType {
    fn inner(&self) -> *mut ArrayTypeInner {
        *self.inner
    }
}
impl ArrayType {
    pub unsafe fn from_inner(inner: *mut ArrayTypeInner) -> ArrayType {
        ArrayType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj>(element_type: &mut A1, num_elements: u64) -> Option<::llvm::ty::seq::ArrayType> {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_get(::llvm::ty::TypeObj::inner(element_type), num_elements as ::libc::uint64_t);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::ArrayType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_isValidElementType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for ArrayType {}
pub type PointerTypeInner = ::ffi::llvm_PointerType;

pub trait PointerTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    fn inner(&self) -> *mut PointerTypeInner;
}

pub trait PointerTypeExt: PointerTypeObj {

    fn get_address_space(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getAddressSpace(::llvm::ty::seq::PointerTypeObj::inner(self) as *const ::ffi::llvm_PointerType) as u32;
            ret
        }
    }
}
impl<T> PointerTypeExt for T where T: PointerTypeObj {}

pub struct PointerType {
    inner: ::core::nonzero::NonZero<*mut PointerTypeInner>,
}
impl ::llvm::ty::TypeObj for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PointerTypeObj for PointerType {
    fn inner(&self) -> *mut PointerTypeInner {
        *self.inner
    }
}
impl PointerType {
    pub unsafe fn from_inner(inner: *mut PointerTypeInner) -> PointerType {
        PointerType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj>(element_type: &mut A1, address_space: u32) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_get(::llvm::ty::TypeObj::inner(element_type), address_space as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_unqual<A1: ::llvm::ty::TypeObj>(element_type: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getUnqual(::llvm::ty::TypeObj::inner(element_type));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_isValidElementType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for PointerType {}
pub type SequentialTypeInner = ::ffi::llvm_SequentialType;

pub trait SequentialTypeObj: ::llvm::ty::CompositeTypeObj {
    fn inner(&self) -> *mut SequentialTypeInner;
}

pub trait SequentialTypeExt: SequentialTypeObj {

    fn get_element_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_getElementType(::llvm::ty::seq::SequentialTypeObj::inner(self) as *const ::ffi::llvm_SequentialType);
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
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for SequentialType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SequentialTypeObj for SequentialType {
    fn inner(&self) -> *mut SequentialTypeInner {
        *self.inner
    }
}
impl SequentialType {
    pub unsafe fn from_inner(inner: *mut SequentialTypeInner) -> SequentialType {
        SequentialType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for SequentialType {}
pub type VectorTypeInner = ::ffi::llvm_VectorType;

pub trait VectorTypeObj: ::llvm::ty::seq::SequentialTypeObj {
    fn inner(&self) -> *mut VectorTypeInner;
}

pub trait VectorTypeExt: VectorTypeObj {

    fn get_bit_width(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getBitWidth(::llvm::ty::seq::VectorTypeObj::inner(self) as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getNumElements(::llvm::ty::seq::VectorTypeObj::inner(self) as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }
}
impl<T> VectorTypeExt for T where T: VectorTypeObj {}

pub struct VectorType {
    inner: ::core::nonzero::NonZero<*mut VectorTypeInner>,
}
impl ::llvm::ty::TypeObj for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeObj for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VectorTypeObj for VectorType {
    fn inner(&self) -> *mut VectorTypeInner {
        *self.inner
    }
}
impl VectorType {
    pub unsafe fn from_inner(inner: *mut VectorTypeInner) -> VectorType {
        VectorType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj>(ty: &mut A1, num_elements: u32) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_get(::llvm::ty::TypeObj::inner(ty), num_elements as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_double_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeObj>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getDoubleElementsVectorType(::llvm::ty::seq::VectorTypeObj::inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_extended_element_vector_type<A1: ::llvm::ty::seq::VectorTypeObj>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getExtendedElementVectorType(::llvm::ty::seq::VectorTypeObj::inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_half_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeObj>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getHalfElementsVectorType(::llvm::ty::seq::VectorTypeObj::inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_integer<A1: ::llvm::ty::seq::VectorTypeObj>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getInteger(::llvm::ty::seq::VectorTypeObj::inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_truncated_element_vector_type<A1: ::llvm::ty::seq::VectorTypeObj>(ty: &mut A1) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getTruncatedElementVectorType(::llvm::ty::seq::VectorTypeObj::inner(ty));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_isValidElementType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for VectorType {}
