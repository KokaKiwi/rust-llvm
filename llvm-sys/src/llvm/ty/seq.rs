pub type SequentialTypeInner = ::ffi::llvm_SequentialType;

pub trait SequentialTypeExt: ::llvm::ty::CompositeTypeExt {

    fn inner(&self) -> *mut SequentialTypeInner;

    fn get_element_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::SequentialType_getElementType(::llvm::ty::seq::SequentialTypeExt::inner(self) as *const ::ffi::llvm_SequentialType))
        }
    }
}

pub struct SequentialType {
    inner: *mut SequentialTypeInner,
}
impl ::llvm::ty::TypeExt for SequentialType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for SequentialType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl SequentialTypeExt for SequentialType {
    fn inner(&self) -> *mut SequentialTypeInner {
        self.inner
    }
}
impl SequentialType {
    pub unsafe fn from_inner(inner: *mut SequentialTypeInner) -> SequentialType {
        SequentialType {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::SequentialType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for SequentialType {
}
pub type ArrayTypeInner = ::ffi::llvm_ArrayType;

pub trait ArrayTypeExt: ::llvm::ty::seq::SequentialTypeExt {

    fn inner(&self) -> *mut ArrayTypeInner;

    fn get_num_elements(&self) -> u64 {
        unsafe {
            ::ffi::llvm::ArrayType_getNumElements(::llvm::ty::seq::ArrayTypeExt::inner(self) as *const ::ffi::llvm_ArrayType) as u64
        }
    }
}

pub struct ArrayType {
    inner: *mut ArrayTypeInner,
}
impl ::llvm::ty::TypeExt for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for ArrayType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ArrayTypeExt for ArrayType {
    fn inner(&self) -> *mut ArrayTypeInner {
        self.inner
    }
}
impl ArrayType {
    pub unsafe fn from_inner(inner: *mut ArrayTypeInner) -> ArrayType {
        ArrayType {
            inner: inner,
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::ArrayType_isValidElementType(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get<A1: ::llvm::ty::TypeExt>(element_type: A1, num_elements: u64) -> ::llvm::ty::seq::ArrayType {
        unsafe {
            ::llvm::ty::seq::ArrayType::from_inner(::ffi::llvm::ArrayType_get(::llvm::ty::TypeExt::inner(&element_type), num_elements as ::libc::uint64_t))
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::ArrayType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for ArrayType {
}
pub type PointerTypeInner = ::ffi::llvm_PointerType;

pub trait PointerTypeExt: ::llvm::ty::seq::SequentialTypeExt {

    fn inner(&self) -> *mut PointerTypeInner;

    fn get_address_space(&self) -> u32 {
        unsafe {
            ::ffi::llvm::PointerType_getAddressSpace(::llvm::ty::seq::PointerTypeExt::inner(self) as *const ::ffi::llvm_PointerType) as u32
        }
    }
}

pub struct PointerType {
    inner: *mut PointerTypeInner,
}
impl ::llvm::ty::TypeExt for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for PointerType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl PointerTypeExt for PointerType {
    fn inner(&self) -> *mut PointerTypeInner {
        self.inner
    }
}
impl PointerType {
    pub unsafe fn from_inner(inner: *mut PointerTypeInner) -> PointerType {
        PointerType {
            inner: inner,
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::PointerType_isValidElementType(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get<A1: ::llvm::ty::TypeExt>(element_type: A1, address_space: u32) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::PointerType_get(::llvm::ty::TypeExt::inner(&element_type), address_space as ::libc::c_uint))
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::PointerType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get_unqual<A1: ::llvm::ty::TypeExt>(element_type: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::PointerType_getUnqual(::llvm::ty::TypeExt::inner(&element_type)))
        }
    }
}
impl Copy for PointerType {
}
pub type VectorTypeInner = ::ffi::llvm_VectorType;

pub trait VectorTypeExt: ::llvm::ty::seq::SequentialTypeExt {

    fn inner(&self) -> *mut VectorTypeInner;

    fn get_num_elements(&self) -> u32 {
        unsafe {
            ::ffi::llvm::VectorType_getNumElements(::llvm::ty::seq::VectorTypeExt::inner(self) as *const ::ffi::llvm_VectorType) as u32
        }
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            ::ffi::llvm::VectorType_getBitWidth(::llvm::ty::seq::VectorTypeExt::inner(self) as *const ::ffi::llvm_VectorType) as u32
        }
    }
}

pub struct VectorType {
    inner: *mut VectorTypeInner,
}
impl ::llvm::ty::TypeExt for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::seq::SequentialTypeExt for VectorType {
    fn inner(&self) -> *mut ::ffi::llvm_SequentialType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl VectorTypeExt for VectorType {
    fn inner(&self) -> *mut VectorTypeInner {
        self.inner
    }
}
impl VectorType {
    pub unsafe fn from_inner(inner: *mut VectorTypeInner) -> VectorType {
        VectorType {
            inner: inner,
        }
    }

    pub fn get_double_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeExt>(ty: A1) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_getDoubleElementsVectorType(::llvm::ty::seq::VectorTypeExt::inner(&ty)))
        }
    }

    pub fn get_truncated_element_vector_type<A1: ::llvm::ty::seq::VectorTypeExt>(ty: A1) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_getTruncatedElementVectorType(::llvm::ty::seq::VectorTypeExt::inner(&ty)))
        }
    }

    pub fn get_extended_element_vector_type<A1: ::llvm::ty::seq::VectorTypeExt>(ty: A1) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_getExtendedElementVectorType(::llvm::ty::seq::VectorTypeExt::inner(&ty)))
        }
    }

    pub fn get_integer<A1: ::llvm::ty::seq::VectorTypeExt>(ty: A1) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_getInteger(::llvm::ty::seq::VectorTypeExt::inner(&ty)))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::VectorType_isValidElementType(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get<A1: ::llvm::ty::TypeExt>(ty: A1, num_elements: u32) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_get(::llvm::ty::TypeExt::inner(&ty), num_elements as ::libc::c_uint))
        }
    }

    pub fn get_half_elements_vector_type<A1: ::llvm::ty::seq::VectorTypeExt>(ty: A1) -> ::llvm::ty::seq::VectorType {
        unsafe {
            ::llvm::ty::seq::VectorType::from_inner(::ffi::llvm::VectorType_getHalfElementsVectorType(::llvm::ty::seq::VectorTypeExt::inner(&ty)))
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::VectorType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for VectorType {
}
