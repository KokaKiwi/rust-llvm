pub mod constant;
pub mod inst;
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorExt: ::llvm::value::user::UserExt {
    #[allow(non_snake_case)]
    fn inner_llvm_Operator(&self) -> *mut OperatorInner;

    fn inner(&self) -> *mut OperatorInner {
        self.inner_llvm_Operator()
    }
}

pub struct Operator {
    inner: ::core::nonzero::NonZero<*mut OperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for Operator {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserExt for Operator {
    fn inner_llvm_User(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl OperatorExt for Operator {
    fn inner_llvm_Operator(&self) -> *mut OperatorInner {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
pub type UserInner = ::ffi::llvm_User;

pub trait UserExt: ::llvm::value::ValueExt {
    #[allow(non_snake_case)]
    fn inner_llvm_User(&self) -> *mut UserInner;

    fn inner(&self) -> *mut UserInner {
        self.inner_llvm_User()
    }

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(self.inner_llvm_User());
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::User_getNumOperands(self.inner_llvm_User() as *const ::ffi::llvm_User) as u32;
            ret
        }
    }

    fn get_operand(&self, idx: u32) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::User_getOperand(self.inner_llvm_User() as *const ::ffi::llvm_User, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn replace_uses_of_with(&mut self, from: &::llvm::value::ValueExt, to: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(self.inner_llvm_User(), from.inner_llvm_Value(), to.inner_llvm_Value());
        }
    }

    fn set_operand(&mut self, idx: u32, val: &::llvm::value::ValueExt) {
        unsafe {
            ::ffi::llvm::User_setOperand(self.inner_llvm_User(), idx as ::libc::c_uint, val.inner_llvm_Value());
        }
    }
}

pub struct User {
    inner: ::core::nonzero::NonZero<*mut UserInner>,
    owned: bool,
}
impl ::llvm::value::ValueExt for User {
    fn inner_llvm_Value(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UserExt for User {
    fn inner_llvm_User(&self) -> *mut UserInner {
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
            let ret = ::ffi::llvm::User_classof(v.inner_llvm_Value());
            ret
        }
    }
}
impl Drop for User {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserExt::inner_llvm_User(self));
            }
        }
    }
}
