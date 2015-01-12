pub mod seq;
pub type CompositeTypeInner = ::ffi::llvm_CompositeType;

pub trait CompositeTypeExt: ::llvm::ty::TypeExt {

    fn inner(&self) -> *mut CompositeTypeInner;

    fn get_type_at_index(&mut self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::CompositeType_getTypeAtIndex(::llvm::ty::CompositeTypeExt::inner(self), idx as ::libc::c_uint))
        }
    }

    fn index_valid(&self, idx: u32) -> bool {
        unsafe {
            ::ffi::llvm::CompositeType_indexValid(::llvm::ty::CompositeTypeExt::inner(self) as *const ::ffi::llvm_CompositeType, idx as ::libc::c_uint)
        }
    }
}

pub struct CompositeType {
    inner: *mut CompositeTypeInner,
}
impl ::llvm::ty::TypeExt for CompositeType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl CompositeTypeExt for CompositeType {
    fn inner(&self) -> *mut CompositeTypeInner {
        self.inner
    }
}
impl CompositeType {
    pub unsafe fn from_inner(inner: *mut CompositeTypeInner) -> CompositeType {
        CompositeType {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::CompositeType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for CompositeType {
}
pub type FunctionTypeInner = ::ffi::llvm_FunctionType;

pub trait FunctionTypeExt: ::llvm::ty::TypeExt {

    fn inner(&self) -> *mut FunctionTypeInner;

    fn get_num_params(&self) -> u32 {
        unsafe {
            ::ffi::llvm::FunctionType_getNumParams(::llvm::ty::FunctionTypeExt::inner(self) as *const ::ffi::llvm_FunctionType) as u32
        }
    }

    fn get_param_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::FunctionType_getParamType(::llvm::ty::FunctionTypeExt::inner(self) as *const ::ffi::llvm_FunctionType, idx as ::libc::c_uint))
        }
    }

    fn get_return_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::FunctionType_getReturnType(::llvm::ty::FunctionTypeExt::inner(self) as *const ::ffi::llvm_FunctionType))
        }
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isVarArg(::llvm::ty::FunctionTypeExt::inner(self) as *const ::ffi::llvm_FunctionType)
        }
    }
}

pub struct FunctionType {
    inner: *mut FunctionTypeInner,
}
impl ::llvm::ty::TypeExt for FunctionType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl FunctionTypeExt for FunctionType {
    fn inner(&self) -> *mut FunctionTypeInner {
        self.inner
    }
}
impl FunctionType {
    pub unsafe fn from_inner(inner: *mut FunctionTypeInner) -> FunctionType {
        FunctionType {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get<A1: ::llvm::ty::TypeExt>(result: A1, params: &[&::llvm::ty::TypeExt], is_var_arg: bool) -> ::llvm::ty::FunctionType {
        unsafe {
            let _tmp_params: Vec<_> = params.iter().map(|ty| ty.inner()).collect();
            let c_params = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_params.as_ptr(),
                length: params.len() as ::libc::size_t,
            };
            ::llvm::ty::FunctionType::from_inner(::ffi::llvm::FunctionType_get(::llvm::ty::TypeExt::inner(&result), c_params, is_var_arg))
        }
    }

    pub fn is_valid_argument_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isValidArgumentType(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn is_valid_return_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::FunctionType_isValidReturnType(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for FunctionType {
}
pub type IntegerTypeInner = ::ffi::llvm_IntegerType;

pub trait IntegerTypeExt: ::llvm::ty::TypeExt {

    fn inner(&self) -> *mut IntegerTypeInner;

    fn get_bit_mask(&self) -> u64 {
        unsafe {
            ::ffi::llvm::IntegerType_getBitMask(::llvm::ty::IntegerTypeExt::inner(self) as *const ::ffi::llvm_IntegerType) as u64
        }
    }

    fn get_bit_width(&self) -> u32 {
        unsafe {
            ::ffi::llvm::IntegerType_getBitWidth(::llvm::ty::IntegerTypeExt::inner(self) as *const ::ffi::llvm_IntegerType) as u32
        }
    }

    fn get_sign_bit(&self) -> u64 {
        unsafe {
            ::ffi::llvm::IntegerType_getSignBit(::llvm::ty::IntegerTypeExt::inner(self) as *const ::ffi::llvm_IntegerType) as u64
        }
    }

    fn is_power_of2_byte_width(&self) -> bool {
        unsafe {
            ::ffi::llvm::IntegerType_isPowerOf2ByteWidth(::llvm::ty::IntegerTypeExt::inner(self) as *const ::ffi::llvm_IntegerType)
        }
    }
}

pub struct IntegerType {
    inner: *mut IntegerTypeInner,
}
impl ::llvm::ty::TypeExt for IntegerType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl IntegerTypeExt for IntegerType {
    fn inner(&self) -> *mut IntegerTypeInner {
        self.inner
    }
}
impl IntegerType {
    pub unsafe fn from_inner(inner: *mut IntegerTypeInner) -> IntegerType {
        IntegerType {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::IntegerType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn get<A1: ::llvm::LLVMContextExt>(ctx: A1, num_bits: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::IntegerType_get(::llvm::LLVMContextExt::inner(&ctx), num_bits as ::libc::c_uint))
        }
    }
}
impl Copy for IntegerType {
}
pub type StructTypeInner = ::ffi::llvm_StructType;

pub trait StructTypeExt: ::llvm::ty::CompositeTypeExt {

    fn inner(&self) -> *mut StructTypeInner;

    fn get_element_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::StructType_getElementType(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType, idx as ::libc::c_uint))
        }
    }

    fn get_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::StructType_getName(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_num_elements(&self) -> u32 {
        unsafe {
            ::ffi::llvm::StructType_getNumElements(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType) as u32
        }
    }

    fn has_name(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_hasName(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType)
        }
    }

    fn is_layout_identical<A1: ::llvm::ty::StructTypeExt>(&self, other: A1) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isLayoutIdentical(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType, ::llvm::ty::StructTypeExt::inner(&other))
        }
    }

    fn is_literal(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isLiteral(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType)
        }
    }

    fn is_opaque(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isOpaque(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType)
        }
    }

    fn is_packed(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isPacked(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType)
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isSized(::llvm::ty::StructTypeExt::inner(self) as *const ::ffi::llvm_StructType)
        }
    }

    fn set_body(&mut self, elements: &[&::llvm::ty::TypeExt]) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|ty| ty.inner()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBody(::llvm::ty::StructTypeExt::inner(self), c_elements);
        }
    }

    fn set_body_packed(&mut self, elements: &[&::llvm::ty::TypeExt], is_packed: bool) {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|ty| ty.inner()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setBodyPacked(::llvm::ty::StructTypeExt::inner(self), c_elements, is_packed);
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::ffi::llvm::StructType_setName(::llvm::ty::StructTypeExt::inner(self), c_name);
        }
    }
}

pub struct StructType {
    inner: *mut StructTypeInner,
}
impl ::llvm::ty::TypeExt for StructType {
    fn inner(&self) -> *mut ::ffi::llvm_Type {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::ty::CompositeTypeExt for StructType {
    fn inner(&self) -> *mut ::ffi::llvm_CompositeType {
        unsafe {
            ::std::mem::transmute(self.inner)
        }
    }
}
impl StructTypeExt for StructType {
    fn inner(&self) -> *mut StructTypeInner {
        self.inner
    }
}
impl StructType {
    pub unsafe fn from_inner(inner: *mut StructTypeInner) -> StructType {
        StructType {
            inner: inner,
        }
    }

    pub fn classof<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::StructType_classof(::llvm::ty::TypeExt::inner(&ty))
        }
    }

    pub fn create<A1: ::llvm::LLVMContextExt>(ctx: A1, elements: &[&::llvm::ty::TypeExt], name: &str) -> ::llvm::ty::StructType {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|ty| ty.inner()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::ty::StructType::from_inner(::ffi::llvm::StructType_create(::llvm::LLVMContextExt::inner(&ctx), c_elements, c_name))
        }
    }

    pub fn create_packed<A1: ::llvm::LLVMContextExt>(ctx: A1, elements: &[&::llvm::ty::TypeExt], name: &str, is_packed: bool) -> ::llvm::ty::StructType {
        unsafe {
            let _tmp_elements: Vec<_> = elements.iter().map(|ty| ty.inner()).collect();
            let c_elements = ::ffi::llvm_ArrayRef_llvm_Type_ptr {
                data: _tmp_elements.as_ptr(),
                length: elements.len() as ::libc::size_t,
            };
            let c_name = ::ffi::llvm_StringRef {
                data: name.as_ptr() as *const ::libc::c_char,
                length: name.len() as ::libc::size_t,
            };
            ::llvm::ty::StructType::from_inner(::ffi::llvm::StructType_createPacked(::llvm::LLVMContextExt::inner(&ctx), c_elements, c_name, is_packed))
        }
    }

    pub fn is_valid_element_type<A1: ::llvm::ty::TypeExt>(ty: A1) -> bool {
        unsafe {
            ::ffi::llvm::StructType_isValidElementType(::llvm::ty::TypeExt::inner(&ty))
        }
    }
}
impl Copy for StructType {
}
pub type TypeInner = ::ffi::llvm_Type;

pub trait TypeExt {

    fn inner(&self) -> *mut TypeInner;

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Type_dump(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type);
        }
    }

    fn get_contained_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getContainedType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_context(&self) -> ::llvm::LLVMContext {
        unsafe {
            ::llvm::LLVMContext::from_inner(::ffi::llvm::Type_getContext(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type))
        }
    }

    fn get_function_num_params(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getFunctionNumParams(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_function_param_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFunctionParamType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_num_contained_types(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getNumContainedTypes(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_pointer_address_space(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getPointerAddressSpace(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_pointer_element_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getPointerElementType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type))
        }
    }

    fn get_pointer_to(&mut self, idx: u32) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getPointerTo(::llvm::ty::TypeExt::inner(self), idx as ::libc::c_uint))
        }
    }

    fn get_sequential_element_type(&self) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getSequentialElementType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type))
        }
    }

    fn get_struct_element_type(&self, idx: u32) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getStructElementType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type, idx as ::libc::c_uint))
        }
    }

    fn get_struct_name(&self) -> &str {
        unsafe {
            let ret = ::ffi::llvm::Type_getStructName(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type);
            ::std::str::from_utf8_unchecked(::std::mem::transmute(::std::slice::from_raw_buf(&ret.data, ret.length as usize)))
        }
    }

    fn get_struct_num_elements(&self) -> u32 {
        unsafe {
            ::ffi::llvm::Type_getStructNumElements(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type) as u32
        }
    }

    fn get_type_id(&self) -> ::ffi::llvm_Type_TypeID {
        unsafe {
            ::ffi::llvm::Type_getTypeID(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_aggregate_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isAggregateType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_array_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isArrayTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_double_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isDoubleTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_empty_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isEmptyTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_fp128_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFP128Ty(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_fp_or_fp_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFPOrFPVectorTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_first_class_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFirstClassType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_float_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFloatTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_floating_point_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFloatingPointTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_function_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFunctionTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_function_var_arg(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isFunctionVarArg(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_half_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isHalfTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_int_or_int_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isIntOrIntVectorTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_integer_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isIntegerTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_label_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isLabelTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_metadata_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isMetadataTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_ppc_fp128_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPPC_FP128Ty(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_pointer_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPointerTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_ptr_or_ptr_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isPtrOrPtrVectorTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_single_value_type(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isSingleValueType(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isSized(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_struct_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isStructTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_vector_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isVectorTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_void_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isVoidTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_x86_fp80_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isX86_FP80Ty(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }

    fn is_x86_mmx_ty(&self) -> bool {
        unsafe {
            ::ffi::llvm::Type_isX86_MMXTy(::llvm::ty::TypeExt::inner(self) as *const ::ffi::llvm_Type)
        }
    }
}

pub struct Type {
    inner: *mut TypeInner,
}
impl TypeExt for Type {
    fn inner(&self) -> *mut TypeInner {
        self.inner
    }
}
impl Type {
    pub unsafe fn from_inner(inner: *mut TypeInner) -> Type {
        Type {
            inner: inner,
        }
    }

    pub fn get_double_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getDoublePtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_double_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getDoubleTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_fp128_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getFP128PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_fp128_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFP128Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_float_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getFloatPtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_float_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getFloatTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_half_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getHalfPtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_half_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getHalfTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int16_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt16PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int16_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt16Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int1_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt1PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int1_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt1Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int32_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt32PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int32_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt32Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int64_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt64PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int64_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt64Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int8_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getInt8PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int8_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getInt8Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_int_n_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1, size: u32) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getIntNPtrTy(::llvm::LLVMContextExt::inner(&ctx), size as ::libc::c_uint))
        }
    }

    pub fn get_int_n_ty<A1: ::llvm::LLVMContextExt>(ctx: A1, size: u32) -> ::llvm::ty::IntegerType {
        unsafe {
            ::llvm::ty::IntegerType::from_inner(::ffi::llvm::Type_getIntNTy(::llvm::LLVMContextExt::inner(&ctx), size as ::libc::c_uint))
        }
    }

    pub fn get_label_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getLabelTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_metadata_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getMetadataTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_ppc_fp128_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getPPC_FP128PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_ppc_fp128_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getPPC_FP128Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_void_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getVoidTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_x86_fp80_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getX86_FP80PtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_x86_fp80_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getX86_FP80Ty(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_x86_mmx_ptr_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::seq::PointerType {
        unsafe {
            ::llvm::ty::seq::PointerType::from_inner(::ffi::llvm::Type_getX86_MMXPtrTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }

    pub fn get_x86_mmx_ty<A1: ::llvm::LLVMContextExt>(ctx: A1) -> ::llvm::ty::Type {
        unsafe {
            ::llvm::ty::Type::from_inner(::ffi::llvm::Type_getX86_MMXTy(::llvm::LLVMContextExt::inner(&ctx)))
        }
    }
}
impl Copy for Type {
}
