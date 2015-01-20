pub mod constant;
pub mod inst;
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut OperatorInner;

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Operator_getOpcode(::llvm::value::user::OperatorExt::inner(self) as *const ::ffi::llvm_Operator) as u32;
            ret
        }
    }
}

pub struct Operator {
    inner: ::core::nonzero::NonZero<*mut OperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl OperatorExt for Operator {
    fn inner(&self) -> *mut OperatorInner {
        *self.inner
    }
}
impl Operator {
    pub unsafe fn from_inner(inner: *mut OperatorInner, owned: bool) -> Operator {
        Operator {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Operator {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
            }
        }
    }
}
pub type UseInner = ::ffi::llvm_Use;

pub trait UseExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut UseInner;

    fn get(&self) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::Use_get(::llvm::value::user::UseExt::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn get_next(&self) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_getNext(::llvm::value::user::UseExt::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::Use::from_inner(ret))
        }
    }

    fn get_operand_no(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Use_getOperandNo(::llvm::value::user::UseExt::inner(self) as *const ::ffi::llvm_Use) as u32;
            ret
        }
    }

    fn get_user(&self) -> Option<::llvm::value::user::User> {
        unsafe {
            let ret = ::ffi::llvm::Use_getUser(::llvm::value::user::UseExt::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::User::from_inner(ret, false))
        }
    }

    fn set(&mut self, val: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::Use_set(::llvm::value::user::UseExt::inner(self), ::llvm::value::ValueExt::inner(val));
        }
    }

    fn swap(&mut self, rhs: &::llvm::value::user::UseExt) {
        unsafe {
            ::ffi::llvm::Use_swap(::llvm::value::user::UseExt::inner(self), ::llvm::value::user::UseExt::inner(rhs));
        }
    }
}

pub struct Use {
    inner: ::core::nonzero::NonZero<*mut UseInner>,
}
impl UseExt for Use {
    fn inner(&self) -> *mut UseInner {
        *self.inner
    }
}
impl Use {
    pub unsafe fn from_inner(inner: *mut UseInner) -> Use {
        Use {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn init_tags(start: &::llvm::value::user::UseExt, stop: &::llvm::value::user::UseExt) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_initTags(::llvm::value::user::UseExt::inner(start), ::llvm::value::user::UseExt::inner(stop));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::Use::from_inner(ret))
        }
    }
}
impl Copy for Use {}
pub type UserInner = ::ffi::llvm_User;

pub trait UserExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut UserInner;

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(::llvm::value::user::UserExt::inner(self));
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::User_getNumOperands(::llvm::value::user::UserExt::inner(self) as *const ::ffi::llvm_User) as u32;
            ret
        }
    }

    fn get_operand(&self, idx: u32) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::User_getOperand(::llvm::value::user::UserExt::inner(self) as *const ::ffi::llvm_User, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn replace_uses_of_with(&mut self, from: &::llvm::value::ValueExt, to: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(::llvm::value::user::UserExt::inner(self), ::llvm::value::ValueExt::inner(from), ::llvm::value::ValueExt::inner(to));
        }
    }

    fn set_operand(&mut self, idx: u32, val: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_setOperand(::llvm::value::user::UserExt::inner(self), idx as ::libc::c_uint, ::llvm::value::ValueExt::inner(val));
        }
    }
}

pub struct User {
    inner: ::core::nonzero::NonZero<*mut UserInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for User {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UserExt for User {
    fn inner(&self) -> *mut UserInner {
        *self.inner
    }
}
impl User {
    pub unsafe fn from_inner(inner: *mut UserInner, owned: bool) -> User {
        User {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof(v: &::llvm::value::ValueExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::User_classof(::llvm::value::ValueExt::inner(v));
            ret
        }
    }
}
impl Drop for User {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner(self));
            }
        }
    }
}
