pub type ArrayTypeInner = ::ffi::llvm_ArrayType;

pub trait ArrayTypeExt: ::llvm::ty::seq::SequentialTypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_ArrayType(&self) -> *mut ArrayTypeInner;

    fn inner(&self) -> *mut ArrayTypeInner {
        self.inner_llvm_ArrayType()
    }

    fn get_num_elements(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_getNumElements(self.inner_llvm_ArrayType() as *const ::ffi::llvm_ArrayType) as u64;
            ret
        }
    }
}

pub struct ArrayType {
    inner: ::core::nonzero::NonZero<*mut ArrayTypeInner>,
}
impl ::llvm::ty::TypeExt for ArrayType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for ArrayType {
    fn inner_llvm_CompositeType(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for ArrayType {
    fn inner_llvm_SequentialType(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ArrayTypeExt for ArrayType {
    fn inner_llvm_ArrayType(&self) -> *mut ArrayTypeInner {
        *self.inner
    }
}
impl ArrayType {
    pub unsafe fn from_inner(inner: *mut ArrayTypeInner) -> ArrayType {
        ArrayType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_classof(ty.inner_llvm_Type());
            ret
        }
    }

    pub fn get(element_type: &::llvm::ty::TypeExt, num_elements: u64) -> Option<::llvm::ty::seq::ArrayType> {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_get(element_type.inner_llvm_Type(), num_elements as ::libc::uint64_t);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::ArrayType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::ArrayType_isValidElementType(ty.inner_llvm_Type());
            ret
        }
    }
}
impl Copy for ArrayType {}
pub type PointerTypeInner = ::ffi::llvm_PointerType;

pub trait PointerTypeExt: ::llvm::ty::seq::SequentialTypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_PointerType(&self) -> *mut PointerTypeInner;

    fn inner(&self) -> *mut PointerTypeInner {
        self.inner_llvm_PointerType()
    }

    fn get_address_space(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getAddressSpace(self.inner_llvm_PointerType() as *const ::ffi::llvm_PointerType) as u32;
            ret
        }
    }
}

pub struct PointerType {
    inner: ::core::nonzero::NonZero<*mut PointerTypeInner>,
}
impl ::llvm::ty::TypeExt for PointerType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for PointerType {
    fn inner_llvm_CompositeType(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for PointerType {
    fn inner_llvm_SequentialType(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl PointerTypeExt for PointerType {
    fn inner_llvm_PointerType(&self) -> *mut PointerTypeInner {
        *self.inner
    }
}
impl PointerType {
    pub unsafe fn from_inner(inner: *mut PointerTypeInner) -> PointerType {
        PointerType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_classof(ty.inner_llvm_Type());
            ret
        }
    }

    pub fn get(element_type: &::llvm::ty::TypeExt, address_space: u32) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_get(element_type.inner_llvm_Type(), address_space as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_unqual(element_type: &::llvm::ty::TypeExt) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::PointerType_getUnqual(element_type.inner_llvm_Type());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::PointerType_isValidElementType(ty.inner_llvm_Type());
            ret
        }
    }
}
impl Copy for PointerType {}
pub type SequentialTypeInner = ::ffi::llvm_SequentialType;

pub trait SequentialTypeExt: ::llvm::ty::CompositeTypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_SequentialType(&self) -> *mut SequentialTypeInner;

    fn inner(&self) -> *mut SequentialTypeInner {
        self.inner_llvm_SequentialType()
    }

    fn get_element_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_getElementType(self.inner_llvm_SequentialType() as *const ::ffi::llvm_SequentialType);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }
}

pub struct SequentialType {
    inner: ::core::nonzero::NonZero<*mut SequentialTypeInner>,
}
impl ::llvm::ty::TypeExt for SequentialType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for SequentialType {
    fn inner_llvm_CompositeType(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl SequentialTypeExt for SequentialType {
    fn inner_llvm_SequentialType(&self) -> *mut SequentialTypeInner {
        *self.inner
    }
}
impl SequentialType {
    pub unsafe fn from_inner(inner: *mut SequentialTypeInner) -> SequentialType {
        SequentialType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::SequentialType_classof(ty.inner_llvm_Type());
            ret
        }
    }
}
impl Copy for SequentialType {}
pub type VectorTypeInner = ::ffi::llvm_VectorType;

pub trait VectorTypeExt: ::llvm::ty::seq::SequentialTypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_VectorType(&self) -> *mut VectorTypeInner;

    fn inner(&self) -> *mut VectorTypeInner {
        self.inner_llvm_VectorType()
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getBitWidth(self.inner_llvm_VectorType() as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getNumElements(self.inner_llvm_VectorType() as *const ::ffi::llvm_VectorType) as u32;
            ret
        }
    }
}

pub struct VectorType {
    inner: ::core::nonzero::NonZero<*mut VectorTypeInner>,
}
impl ::llvm::ty::TypeExt for VectorType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for VectorType {
    fn inner_llvm_CompositeType(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for VectorType {
    fn inner_llvm_SequentialType(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl VectorTypeExt for VectorType {
    fn inner_llvm_VectorType(&self) -> *mut VectorTypeInner {
        *self.inner
    }
}
impl VectorType {
    pub unsafe fn from_inner(inner: *mut VectorTypeInner) -> VectorType {
        VectorType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_classof(ty.inner_llvm_Type());
            ret
        }
    }

    pub fn get(ty: &::llvm::ty::TypeExt, num_elements: u32) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_get(ty.inner_llvm_Type(), num_elements as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_double_elements_vector_type(ty: &::llvm::ty::seq::VectorTypeExt) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getDoubleElementsVectorType(ty.inner_llvm_VectorType());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_extended_element_vector_type(ty: &::llvm::ty::seq::VectorTypeExt) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getExtendedElementVectorType(ty.inner_llvm_VectorType());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_half_elements_vector_type(ty: &::llvm::ty::seq::VectorTypeExt) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getHalfElementsVectorType(ty.inner_llvm_VectorType());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_integer(ty: &::llvm::ty::seq::VectorTypeExt) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getInteger(ty.inner_llvm_VectorType());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn get_truncated_element_vector_type(ty: &::llvm::ty::seq::VectorTypeExt) -> Option<::llvm::ty::seq::VectorType> {
        unsafe {
            let ret = ::ffi::llvm::VectorType_getTruncatedElementVectorType(ty.inner_llvm_VectorType());
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::VectorType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::VectorType_isValidElementType(ty.inner_llvm_Type());
            ret
        }
    }
}
impl Copy for VectorType {}
