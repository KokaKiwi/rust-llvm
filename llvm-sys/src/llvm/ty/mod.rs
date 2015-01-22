pub mod seq;
pub type CompositeTypeInner = ::ffi::llvm_CompositeType;

pub trait CompositeTypeObj: ::llvm::ty::TypeObj {
    fn inner(&self) -> *mut CompositeTypeInner;
}

pub trait CompositeTypeExt: CompositeTypeObj {

    fn get_type_at_index(&mut self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::CompositeType_getTypeAtIndex(::llvm::ty::CompositeTypeObj::inner(self), idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn index_valid(&self, idx: u32) -> bool {
        unsafe {
            let ret = ::ffi::llvm::CompositeType_indexValid(::llvm::ty::CompositeTypeObj::inner(self) as *const ::ffi::llvm_CompositeType, idx as ::libc::c_uint);
            ret
        }
    }
}
impl<T> CompositeTypeExt for T where T: CompositeTypeObj {}

pub struct CompositeType {
    inner: ::core::nonzero::NonZero<*mut CompositeTypeInner>,
}
impl ::llvm::ty::TypeObj for CompositeType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CompositeTypeObj for CompositeType {
    fn inner(&self) -> *mut CompositeTypeInner {
        *self.inner
    }
}
impl CompositeType {
    pub unsafe fn from_inner(inner: *mut CompositeTypeInner) -> CompositeType {
        CompositeType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::CompositeType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for CompositeType {}
pub type FunctionTypeInner = ::ffi::llvm_FunctionType;

pub trait FunctionTypeObj: ::llvm::ty::TypeObj {
    fn inner(&self) -> *mut FunctionTypeInner;
}

pub trait FunctionTypeExt: FunctionTypeObj {

    fn get_num_params(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_getNumParams(::llvm::ty::FunctionTypeObj::inner(self) as *const ::ffi::llvm_FunctionType) as u32;
            ret
        }
    }

    fn get_param_type(&self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_getParamType(::llvm::ty::FunctionTypeObj::inner(self) as *const ::ffi::llvm_FunctionType, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_return_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_getReturnType(::llvm::ty::FunctionTypeObj::inner(self) as *const ::ffi::llvm_FunctionType);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_isVarArg(::llvm::ty::FunctionTypeObj::inner(self) as *const ::ffi::llvm_FunctionType);
            ret
        }
    }
}
impl<T> FunctionTypeExt for T where T: FunctionTypeObj {}

pub struct FunctionType {
    inner: ::core::nonzero::NonZero<*mut FunctionTypeInner>,
}
impl ::llvm::ty::TypeObj for FunctionType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FunctionTypeObj for FunctionType {
    fn inner(&self) -> *mut FunctionTypeInner {
        *self.inner
    }
}
impl FunctionType {
    pub unsafe fn from_inner(inner: *mut FunctionTypeInner) -> FunctionType {
        FunctionType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::ty::TypeObj>(result: &mut A1, params: &[&::llvm::ty::TypeObj], is_var_arg: bool) -> Option<::llvm::ty::FunctionType> {
        unsafe {
            let _tmp_params: Vec<_> = params.iter().map(|&ty| ::llvm::ty::TypeObj::inner(ty)).collect();
            let c_params = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_params.as_ptr(),
                length: params.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::FunctionType_get(::llvm::ty::TypeObj::inner(result), c_params, is_var_arg);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::FunctionType::from_inner(ret))
        }
    }

    pub fn is_valid_argument_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_isValidArgumentType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn is_valid_return_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::FunctionType_isValidReturnType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for FunctionType {}
pub type IntegerTypeInner = ::ffi::llvm_IntegerType;

pub trait IntegerTypeObj: ::llvm::ty::TypeObj {
    fn inner(&self) -> *mut IntegerTypeInner;
}

pub trait IntegerTypeExt: IntegerTypeObj {

    fn get_bit_mask(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_getBitMask(::llvm::ty::IntegerTypeObj::inner(self) as *const ::ffi::llvm_IntegerType) as u64;
            ret
        }
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_getBitWidth(::llvm::ty::IntegerTypeObj::inner(self) as *const ::ffi::llvm_IntegerType) as u32;
            ret
        }
    }

    fn get_sign_bit(&self) -> u64 {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_getSignBit(::llvm::ty::IntegerTypeObj::inner(self) as *const ::ffi::llvm_IntegerType) as u64;
            ret
        }
    }

    fn is_power_of2_byte_width(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_isPowerOf2ByteWidth(::llvm::ty::IntegerTypeObj::inner(self) as *const ::ffi::llvm_IntegerType);
            ret
        }
    }
}
impl<T> IntegerTypeExt for T where T: IntegerTypeObj {}

pub struct IntegerType {
    inner: ::core::nonzero::NonZero<*mut IntegerTypeInner>,
}
impl ::llvm::ty::TypeObj for IntegerType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl IntegerTypeObj for IntegerType {
    fn inner(&self) -> *mut IntegerTypeInner {
        *self.inner
    }
}
impl IntegerType {
    pub unsafe fn from_inner(inner: *mut IntegerTypeInner) -> IntegerType {
        IntegerType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn get<A1: ::llvm::LLVMContextObj>(ctx: &mut A1, num_bits: u32) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::IntegerType_get(::llvm::LLVMContextObj::inner(ctx), num_bits as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }
}
impl Copy for IntegerType {}
pub type StructTypeInner = ::ffi::llvm_StructType;

pub trait StructTypeObj: ::llvm::ty::CompositeTypeObj {
    fn inner(&self) -> *mut StructTypeInner;
}

pub trait StructTypeExt: StructTypeObj {

    fn get_element_type(&self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::StructType_getElementType(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::StructType_getName(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::StructType_getNumElements(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType) as u32;
            ret
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_hasName(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            ret
        }
    }

    fn is_layout_identical<A1: ::llvm::ty::StructTypeObj>(&self, other: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isLayoutIdentical(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType, ::llvm::ty::StructTypeObj::inner(other));
            ret
        }
    }

    fn is_literal(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isLiteral(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            ret
        }
    }

    fn is_opaque(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isOpaque(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            ret
        }
    }

    fn is_packed(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isPacked(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            ret
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isSized(::llvm::ty::StructTypeObj::inner(self) as *const ::ffi::llvm_StructType);
            ret
        }
    }

    fn set_body(&mut self, elements: &[&::llvm::ty::TypeObj]) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ::llvm::ty::TypeObj::inner(ty)).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBody(::llvm::ty::StructTypeObj::inner(self), c_elements);
        }
    }

    fn set_body_packed(&mut self, elements: &[&::llvm::ty::TypeObj], is_packed: bool) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ::llvm::ty::TypeObj::inner(ty)).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBodyPacked(::llvm::ty::StructTypeObj::inner(self), c_elements, is_packed);
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setName(::llvm::ty::StructTypeObj::inner(self), c_name);
        }
    }
}
impl<T> StructTypeExt for T where T: StructTypeObj {}

pub struct StructType {
    inner: ::core::nonzero::NonZero<*mut StructTypeInner>,
}
impl ::llvm::ty::TypeObj for StructType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeObj for StructType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl StructTypeObj for StructType {
    fn inner(&self) -> *mut StructTypeInner {
        *self.inner
    }
}
impl StructType {
    pub unsafe fn from_inner(inner: *mut StructTypeInner) -> StructType {
        StructType {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeObj>(ty: &A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_classof(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }

    pub fn create<A1: ::llvm::LLVMContextObj>(ctx: &mut A1, elements: &[&::llvm::ty::TypeObj], name: &str) -> Option<::llvm::ty::StructType> {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ::llvm::ty::TypeObj::inner(ty)).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::StructType_create(::llvm::LLVMContextObj::inner(ctx), c_elements, c_name);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    pub fn create_packed<A1: ::llvm::LLVMContextObj>(ctx: &mut A1, elements: &[&::llvm::ty::TypeObj], name: &str, is_packed: bool) -> Option<::llvm::ty::StructType> {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ::llvm::ty::TypeObj::inner(ty)).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            let ret = ::ffi::llvm::StructType_createPacked(::llvm::LLVMContextObj::inner(ctx), c_elements, c_name, is_packed);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::StructType::from_inner(ret))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeObj>(ty: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::StructType_isValidElementType(::llvm::ty::TypeObj::inner(ty));
            ret
        }
    }
}
impl Copy for StructType {}
pub enum TypeID {
    VoidTyID,
    HalfTyID,
    FloatTyID,
    DoubleTyID,
    X86FP80TyID,
    FP128TyID,
    PPCFP128TyID,
    LabelTyID,
    MetadataTyID,
    X86MMXTyID,
    IntegerTyID,
    FunctionTyID,
    StructTyID,
    ArrayTyID,
    PointerTyID,
    VectorTyID,
}
impl TypeID {
    pub fn from_ffi(value: ::ffi::llvm_Type_TypeID) -> TypeID {
        match value {
            ::ffi::llvm_Type_TypeID::VoidTyID => TypeID::VoidTyID,
            ::ffi::llvm_Type_TypeID::HalfTyID => TypeID::HalfTyID,
            ::ffi::llvm_Type_TypeID::FloatTyID => TypeID::FloatTyID,
            ::ffi::llvm_Type_TypeID::DoubleTyID => TypeID::DoubleTyID,
            ::ffi::llvm_Type_TypeID::X86_FP80TyID => TypeID::X86FP80TyID,
            ::ffi::llvm_Type_TypeID::FP128TyID => TypeID::FP128TyID,
            ::ffi::llvm_Type_TypeID::PPC_FP128TyID => TypeID::PPCFP128TyID,
            ::ffi::llvm_Type_TypeID::LabelTyID => TypeID::LabelTyID,
            ::ffi::llvm_Type_TypeID::MetadataTyID => TypeID::MetadataTyID,
            ::ffi::llvm_Type_TypeID::X86_MMXTyID => TypeID::X86MMXTyID,
            ::ffi::llvm_Type_TypeID::IntegerTyID => TypeID::IntegerTyID,
            ::ffi::llvm_Type_TypeID::FunctionTyID => TypeID::FunctionTyID,
            ::ffi::llvm_Type_TypeID::StructTyID => TypeID::StructTyID,
            ::ffi::llvm_Type_TypeID::ArrayTyID => TypeID::ArrayTyID,
            ::ffi::llvm_Type_TypeID::PointerTyID => TypeID::PointerTyID,
            ::ffi::llvm_Type_TypeID::VectorTyID => TypeID::VectorTyID,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_Type_TypeID {
        match self {
            TypeID::VoidTyID => ::ffi::llvm_Type_TypeID::VoidTyID,
            TypeID::HalfTyID => ::ffi::llvm_Type_TypeID::HalfTyID,
            TypeID::FloatTyID => ::ffi::llvm_Type_TypeID::FloatTyID,
            TypeID::DoubleTyID => ::ffi::llvm_Type_TypeID::DoubleTyID,
            TypeID::X86FP80TyID => ::ffi::llvm_Type_TypeID::X86_FP80TyID,
            TypeID::FP128TyID => ::ffi::llvm_Type_TypeID::FP128TyID,
            TypeID::PPCFP128TyID => ::ffi::llvm_Type_TypeID::PPC_FP128TyID,
            TypeID::LabelTyID => ::ffi::llvm_Type_TypeID::LabelTyID,
            TypeID::MetadataTyID => ::ffi::llvm_Type_TypeID::MetadataTyID,
            TypeID::X86MMXTyID => ::ffi::llvm_Type_TypeID::X86_MMXTyID,
            TypeID::IntegerTyID => ::ffi::llvm_Type_TypeID::IntegerTyID,
            TypeID::FunctionTyID => ::ffi::llvm_Type_TypeID::FunctionTyID,
            TypeID::StructTyID => ::ffi::llvm_Type_TypeID::StructTyID,
            TypeID::ArrayTyID => ::ffi::llvm_Type_TypeID::ArrayTyID,
            TypeID::PointerTyID => ::ffi::llvm_Type_TypeID::PointerTyID,
            TypeID::VectorTyID => ::ffi::llvm_Type_TypeID::VectorTyID,
        }
    }
}
impl Copy for TypeID {}
pub type TypeInner = ::ffi::llvm_Type;

pub trait TypeObj {
    fn inner(&self) -> *mut TypeInner;
}

pub trait TypeExt: TypeObj {

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Type_dump(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
        }
    }

    fn get_contained_type(&self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getContainedType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            let ret = ::ffi::llvm::Type_getContext(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ::llvm::LLVMContext::from_inner(ret)
        }
    }

    fn get_function_num_params(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Type_getFunctionNumParams(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type) as u32;
            ret
        }
    }

    fn get_function_param_type(&self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getFunctionParamType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_num_contained_types(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Type_getNumContainedTypes(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type) as u32;
            ret
        }
    }

    fn get_pointer_address_space(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Type_getPointerAddressSpace(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type) as u32;
            ret
        }
    }

    fn get_pointer_element_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getPointerElementType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_pointer_to(&mut self, idx: u32) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getPointerTo(::llvm::ty::TypeObj::inner(self), idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    fn get_sequential_element_type(&self) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getSequentialElementType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_struct_element_type(&self, idx: u32) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getStructElementType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    fn get_struct_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Type_getStructName(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            let ret = ::core::str::from_utf8_unchecked(::core::mem::transmute(::core::slice::from_raw_buf(&ret.data, ret.length as usize)));
            ret
        }
    }

    fn get_struct_num_elements(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Type_getStructNumElements(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type) as u32;
            ret
        }
    }

    fn get_type_id(&self) -> ::llvm::ty::TypeID {
        unsafe {
            let ret = ::llvm::ty::TypeID::from_ffi(::ffi::llvm::Type_getTypeID(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type));
            ret
        }
    }

    fn is_aggregate_type(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isAggregateType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_array_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isArrayTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_double_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isDoubleTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_empty_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isEmptyTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_fp128_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFP128Ty(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_fp_or_fp_vector_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFPOrFPVectorTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_first_class_type(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFirstClassType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_float_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFloatTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_floating_point_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFloatingPointTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_function_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFunctionTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_function_var_arg(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isFunctionVarArg(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_half_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isHalfTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_int_or_int_vector_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isIntOrIntVectorTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_integer_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isIntegerTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_label_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isLabelTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_metadata_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isMetadataTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_ppc_fp128_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isPPC_FP128Ty(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_pointer_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isPointerTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_ptr_or_ptr_vector_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isPtrOrPtrVectorTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_single_value_type(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isSingleValueType(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isSized(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_struct_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isStructTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_vector_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isVectorTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_void_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isVoidTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_x86_fp80_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isX86_FP80Ty(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }

    fn is_x86_mmx_ty(&self) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Type_isX86_MMXTy(::llvm::ty::TypeObj::inner(self) as *const ::ffi::llvm_Type);
            ret
        }
    }
}
impl<T> TypeExt for T where T: TypeObj {}

pub struct Type {
    inner: ::core::nonzero::NonZero<*mut TypeInner>,
}
impl TypeObj for Type {
    fn inner(&self) -> *mut TypeInner {
        *self.inner
    }
}
impl Type {
    pub unsafe fn from_inner(inner: *mut TypeInner) -> Type {
        Type {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn get_double_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getDoublePtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_double_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getDoubleTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_fp128_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getFP128PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_fp128_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getFP128Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_float_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getFloatPtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_float_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getFloatTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_half_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getHalfPtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_half_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getHalfTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_int16_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt16PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int16_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt16Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_int1_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt1PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int1_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt1Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_int32_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt32PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int32_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt32Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_int64_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt64PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int64_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt64Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_int8_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt8PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int8_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getInt8Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_int_n_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1, size: u32) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getIntNPtrTy(::llvm::LLVMContextObj::inner(ctx), size as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_int_n_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1, size: u32) -> Option<::llvm::ty::IntegerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getIntNTy(::llvm::LLVMContextObj::inner(ctx), size as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::IntegerType::from_inner(ret))
        }
    }

    pub fn get_label_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getLabelTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_metadata_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getMetadataTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_ppc_fp128_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getPPC_FP128PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_ppc_fp128_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getPPC_FP128Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_void_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getVoidTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_x86_fp80_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getX86_FP80PtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_x86_fp80_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getX86_FP80Ty(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }

    pub fn get_x86_mmx_ptr_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::seq::PointerType> {
        unsafe {
            let ret = ::ffi::llvm::Type_getX86_MMXPtrTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::seq::PointerType::from_inner(ret))
        }
    }

    pub fn get_x86_mmx_ty<A1: ::llvm::LLVMContextObj>(ctx: &mut A1) -> Option<::llvm::ty::Type> {
        unsafe {
            let ret = ::ffi::llvm::Type_getX86_MMXTy(::llvm::LLVMContextObj::inner(ctx));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::ty::Type::from_inner(ret))
        }
    }
}
impl Copy for Type {}
