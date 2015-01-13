pub mod seq;
pub type CompositeTypeInner = ::ffi::llvm_CompositeType;

pub trait CompositeTypeExt: ::llvm::ty::TypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_CompositeType(&self) -> *mut CompositeTypeInner;

    fn inner(&self) -> *mut CompositeTypeInner {
        self.inner_llvm_CompositeType()
    }

    fn get_type_at_index(&mut self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::CompositeType_getTypeAtIndex(self.inner_llvm_CompositeType(), idx as ::libc::c_uint))
        }
    }

    fn index_valid(&self, idx: u32) -> bool {
        unsafe {
            ::ffi::llvm::CompositeType_indexValid(self.inner_llvm_CompositeType() as *const ::ffi::llvm_CompositeType, idx as ::libc::c_uint)
        }
    }
}

pub struct CompositeType {
    inner: *mut CompositeTypeInner,
}
impl ::llvm::ty::TypeExt for CompositeType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl CompositeTypeExt for CompositeType {
    fn inner_llvm_CompositeType(&self) -> *mut CompositeTypeInner {
        self.inner
    }
}
impl CompositeType {
    pub unsafe fn from_inner(inner: *mut CompositeTypeInner) -> CompositeType {
        CompositeType {
            inner: inner,
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::CompositeType_classof(ty.inner_llvm_Type())
        }
    }
}
impl Copy for CompositeType {}
pub type FunctionTypeInner = ::ffi::llvm_FunctionType;

pub trait FunctionTypeExt: ::llvm::ty::TypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_FunctionType(&self) -> *mut FunctionTypeInner;

    fn inner(&self) -> *mut FunctionTypeInner {
        self.inner_llvm_FunctionType()
    }

    fn get_num_params(&self) -> u32 {
        unsafe {
            ::ffi::llvm::FunctionType_getNumParams(self.inner_llvm_FunctionType() as *const ::ffi::llvm_FunctionType) as u32
        }
    }

    fn get_param_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::FunctionType_getParamType(self.inner_llvm_FunctionType() as *const ::ffi::llvm_FunctionType, idx as ::libc::c_uint))
        }
    }

    fn get_return_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::FunctionType_getReturnType(self.inner_llvm_FunctionType() as *const ::ffi::llvm_FunctionType))
        }
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isVarArg(self.inner_llvm_FunctionType() as *const ::ffi::llvm_FunctionType)
        }
    }
}

pub struct FunctionType {
    inner: *mut FunctionTypeInner,
}
impl ::llvm::ty::TypeExt for FunctionType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FunctionTypeExt for FunctionType {
    fn inner_llvm_FunctionType(&self) -> *mut FunctionTypeInner {
        self.inner
    }
}
impl FunctionType {
    pub unsafe fn from_inner(inner: *mut FunctionTypeInner) -> FunctionType {
        FunctionType {
            inner: inner,
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_classof(ty.inner_llvm_Type())
        }
    }

    pub fn get(result: &::llvm::ty::TypeExt, params: &[&::llvm::ty::TypeExt], is_var_arg: bool) -> ::llvm::ty::FunctionType {
        unsafe {
            let _tmp_params: Vec<_> = params.iter().map(|&ty| ty.inner_llvm_Type()).collect();
            let c_params = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_params.as_ptr(),
                length: params.len() as ::libc::size_t,
            };
            ::llvm::ty::FunctionType::from_inner(::ffi::llvm::FunctionType_get(result.inner_llvm_Type(), c_params, is_var_arg))
        }
    }

    pub fn is_valid_argument_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isValidArgumentType(ty.inner_llvm_Type())
        }
    }

    pub fn is_valid_return_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isValidReturnType(ty.inner_llvm_Type())
        }
    }
}
impl Copy for FunctionType {}
pub type IntegerTypeInner = ::ffi::llvm_IntegerType;

pub trait IntegerTypeExt: ::llvm::ty::TypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_IntegerType(&self) -> *mut IntegerTypeInner;

    fn inner(&self) -> *mut IntegerTypeInner {
        self.inner_llvm_IntegerType()
    }

    fn get_bit_mask(&self) -> u64 {
        unsafe {
            ::ffi::llvm::IntegerType_getBitMask(self.inner_llvm_IntegerType() as *const ::ffi::llvm_IntegerType) as u64
        }
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            ::ffi::llvm::IntegerType_getBitWidth(self.inner_llvm_IntegerType() as *const ::ffi::llvm_IntegerType) as u32
        }
    }

    fn get_sign_bit(&self) -> u64 {
        unsafe {
            ::ffi::llvm::IntegerType_getSignBit(self.inner_llvm_IntegerType() as *const ::ffi::llvm_IntegerType) as u64
        }
    }

    fn is_power_of2_byte_width(&self) -> bool {
        unsafe {
            ::ffi::llvm::IntegerType_isPowerOf2ByteWidth(self.inner_llvm_IntegerType() as *const ::ffi::llvm_IntegerType)
        }
    }
}

pub struct IntegerType {
    inner: *mut IntegerTypeInner,
}
impl ::llvm::ty::TypeExt for IntegerType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl IntegerTypeExt for IntegerType {
    fn inner_llvm_IntegerType(&self) -> *mut IntegerTypeInner {
        self.inner
    }
}
impl IntegerType {
    pub unsafe fn from_inner(inner: *mut IntegerTypeInner) -> IntegerType {
        IntegerType {
            inner: inner,
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::IntegerType_classof(ty.inner_llvm_Type())
        }
    }

    pub fn get(ctx: &::llvm::LLVMContextExt, num_bits: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::IntegerType_get(ctx.inner_llvm_LLVMContext(), num_bits as ::libc::c_uint))
        }
    }
}
impl Copy for IntegerType {}
pub type StructTypeInner = ::ffi::llvm_StructType;

pub trait StructTypeExt: ::llvm::ty::CompositeTypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_StructType(&self) -> *mut StructTypeInner;

    fn inner(&self) -> *mut StructTypeInner {
        self.inner_llvm_StructType()
    }

    fn get_element_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::StructType_getElementType(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType, idx as ::libc::c_uint))
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::StructType_getName(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            ::ffi::llvm::StructType_getNumElements(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType) as u32
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_hasName(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType)
        }
    }

    fn is_layout_identical(&self, other: &::llvm::ty::StructTypeExt) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isLayoutIdentical(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType, other.inner_llvm_StructType())
        }
    }

    fn is_literal(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isLiteral(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType)
        }
    }

    fn is_opaque(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isOpaque(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType)
        }
    }

    fn is_packed(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isPacked(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType)
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isSized(self.inner_llvm_StructType() as *const ::ffi::llvm_StructType)
        }
    }

    fn set_body(&mut self, elements: &[&::llvm::ty::TypeExt]) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ty.inner_llvm_Type()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBody(self.inner_llvm_StructType(), c_elements);
        }
    }

    fn set_body_packed(&mut self, elements: &[&::llvm::ty::TypeExt], is_packed: bool) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ty.inner_llvm_Type()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBodyPacked(self.inner_llvm_StructType(), c_elements, is_packed);
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setName(self.inner_llvm_StructType(), c_name);
        }
    }
}

pub struct StructType {
    inner: *mut StructTypeInner,
}
impl ::llvm::ty::TypeExt for StructType {
    fn inner_llvm_Type(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for StructType {
    fn inner_llvm_CompositeType(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl StructTypeExt for StructType {
    fn inner_llvm_StructType(&self) -> *mut StructTypeInner {
        self.inner
    }
}
impl StructType {
    pub unsafe fn from_inner(inner: *mut StructTypeInner) -> StructType {
        StructType {
            inner: inner,
        }
    }

    pub fn classof(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::StructType_classof(ty.inner_llvm_Type())
        }
    }

    pub fn create(ctx: &::llvm::LLVMContextExt, elements: &[&::llvm::ty::TypeExt], name: &str) -> ::llvm::ty::StructType {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ty.inner_llvm_Type()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::ty::StructType::from_inner(::ffi::llvm::StructType_create(ctx.inner_llvm_LLVMContext(), c_elements, c_name))
        }
    }

    pub fn create_packed(ctx: &::llvm::LLVMContextExt, elements: &[&::llvm::ty::TypeExt], name: &str, is_packed: bool) -> ::llvm::ty::StructType {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|&ty| ty.inner_llvm_Type()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::ty::StructType::from_inner(::ffi::llvm::StructType_createPacked(ctx.inner_llvm_LLVMContext(), c_elements, c_name, is_packed))
        }
    }

    pub fn is_valid_element_type(ty: &::llvm::ty::TypeExt) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isValidElementType(ty.inner_llvm_Type())
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

pub trait TypeExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Type(&self) -> *mut TypeInner;

    fn inner(&self) -> *mut TypeInner {
        self.inner_llvm_Type()
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Type_dump(self.inner_llvm_Type() as *const ::ffi::llvm_Type);
        }
    }

    fn get_contained_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getContainedType(self.inner_llvm_Type() as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Type_getContext(self.inner_llvm_Type() as *const ::ffi::llvm_Type))
        }
    }

    fn get_function_num_params(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getFunctionNumParams(self.inner_llvm_Type() as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_function_param_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFunctionParamType(self.inner_llvm_Type() as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_num_contained_types(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getNumContainedTypes(self.inner_llvm_Type() as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_pointer_address_space(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getPointerAddressSpace(self.inner_llvm_Type() as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_pointer_element_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getPointerElementType(self.inner_llvm_Type() as *const ::ffi::llvm_Type))
        }
    }

    fn get_pointer_to(&mut self, idx: u32) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getPointerTo(self.inner_llvm_Type(), idx as ::libc::c_uint))
        }
    }

    fn get_sequential_element_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getSequentialElementType(self.inner_llvm_Type() as *const ::ffi::llvm_Type))
        }
    }

    fn get_struct_element_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getStructElementType(self.inner_llvm_Type() as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_struct_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Type_getStructName(self.inner_llvm_Type() as *const ::ffi::llvm_Type);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_struct_num_elements(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getStructNumElements(self.inner_llvm_Type() as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_type_id(&self) -> ::llvm::ty::TypeID {
        unsafe {
            ::llvm::ty::TypeID::from_ffi(::ffi::llvm::Type_getTypeID(self.inner_llvm_Type() as *const ::ffi::llvm_Type))
        }
    }

    fn is_aggregate_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isAggregateType(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_array_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isArrayTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_double_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isDoubleTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_empty_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isEmptyTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_fp128_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFP128Ty(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_fp_or_fp_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFPOrFPVectorTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_first_class_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFirstClassType(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_float_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFloatTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_floating_point_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFloatingPointTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_function_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFunctionTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_function_var_arg(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFunctionVarArg(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_half_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isHalfTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_int_or_int_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isIntOrIntVectorTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_integer_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isIntegerTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_label_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isLabelTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_metadata_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isMetadataTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_ppc_fp128_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPPC_FP128Ty(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_pointer_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPointerTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_ptr_or_ptr_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPtrOrPtrVectorTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_single_value_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isSingleValueType(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isSized(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_struct_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isStructTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isVectorTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_void_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isVoidTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_x86_fp80_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isX86_FP80Ty(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }

    fn is_x86_mmx_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isX86_MMXTy(self.inner_llvm_Type() as *const ::ffi::llvm_Type)
        }
    }
}

pub struct Type {
    inner: *mut TypeInner,
}
impl TypeExt for Type {
    fn inner_llvm_Type(&self) -> *mut TypeInner {
        self.inner
    }
}
impl Type {
    pub unsafe fn from_inner(inner: *mut TypeInner) -> Type {
        Type {
            inner: inner,
        }
    }

    pub fn get_double_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getDoublePtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_double_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getDoubleTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_fp128_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getFP128PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_fp128_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFP128Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_float_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getFloatPtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_float_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFloatTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_half_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getHalfPtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_half_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getHalfTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int16_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt16PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int16_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt16Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int1_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt1PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int1_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt1Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int32_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt32PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int32_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt32Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int64_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt64PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int64_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt64Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int8_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt8PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int8_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt8Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_int_n_ptr_ty(ctx: &::llvm::LLVMContextExt, size: u32) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getIntNPtrTy(ctx.inner_llvm_LLVMContext(), size as ::libc::c_uint))
        }
    }

    pub fn get_int_n_ty(ctx: &::llvm::LLVMContextExt, size: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getIntNTy(ctx.inner_llvm_LLVMContext(), size as ::libc::c_uint))
        }
    }

    pub fn get_label_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getLabelTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_metadata_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getMetadataTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_ppc_fp128_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getPPC_FP128PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_ppc_fp128_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getPPC_FP128Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_void_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getVoidTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_x86_fp80_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getX86_FP80PtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_x86_fp80_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getX86_FP80Ty(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_x86_mmx_ptr_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getX86_MMXPtrTy(ctx.inner_llvm_LLVMContext()))
        }
    }

    pub fn get_x86_mmx_ty(ctx: &::llvm::LLVMContextExt) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getX86_MMXTy(ctx.inner_llvm_LLVMContext()))
        }
    }
}
impl Copy for Type {}
