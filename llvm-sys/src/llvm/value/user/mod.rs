pub mod constant;
pub mod inst;
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorObj: ::llvm::value::user::UserObj {
    fn inner(&self) -> *mut OperatorInner;
}

pub trait OperatorExt: OperatorObj {

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Operator_getOpcode(::llvm::value::user::OperatorObj::inner(self) as *const ::ffi::llvm_Operator) as u32;
            ret
        }
    }
}
impl<T> OperatorExt for T where T: OperatorObj {}

pub struct Operator {
    inner: ::core::nonzero::NonZero<*mut OperatorInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Operator {
    fn inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl OperatorObj for Operator {
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
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::inner(self));
            }
        }
    }
}
pub type UseInner = ::ffi::llvm_Use;

pub trait UseObj {
    fn inner(&self) -> *mut UseInner;
}

pub trait UseExt: UseObj {

    fn get(&self) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::Use_get(::llvm::value::user::UseObj::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn get_next(&self) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_getNext(::llvm::value::user::UseObj::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::Use::from_inner(ret))
        }
    }

    fn get_operand_no(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Use_getOperandNo(::llvm::value::user::UseObj::inner(self) as *const ::ffi::llvm_Use) as u32;
            ret
        }
    }

    fn get_user(&self) -> Option<::llvm::value::user::User> {
        unsafe {
            let ret = ::ffi::llvm::Use_getUser(::llvm::value::user::UseObj::inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::User::from_inner(ret, false))
        }
    }

    fn set<A1: ::llvm::value::ValueObj>(&mut self, val: &mut A1) {
        unsafe {
            ::ffi::llvm::Use_set(::llvm::value::user::UseObj::inner(self), ::llvm::value::ValueObj::inner(val));
        }
    }

    fn swap<A1: ::llvm::value::user::UseObj>(&mut self, rhs: &mut A1) {
        unsafe {
            ::ffi::llvm::Use_swap(::llvm::value::user::UseObj::inner(self), ::llvm::value::user::UseObj::inner(rhs));
        }
    }
}
impl<T> UseExt for T where T: UseObj {}

pub struct Use {
    inner: ::core::nonzero::NonZero<*mut UseInner>,
}
impl UseObj for Use {
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

    pub fn init_tags<A1: ::llvm::value::user::UseObj, A2: ::llvm::value::user::UseObj>(start: &mut A1, stop: &mut A2) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_initTags(::llvm::value::user::UseObj::inner(start), ::llvm::value::user::UseObj::inner(stop));
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::Use::from_inner(ret))
        }
    }
}
impl Copy for Use {}
pub type UserInner = ::ffi::llvm_User;

pub trait UserObj: ::llvm::value::ValueObj {
    fn inner(&self) -> *mut UserInner;
}

pub trait UserExt: UserObj {

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(::llvm::value::user::UserObj::inner(self));
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::User_getNumOperands(::llvm::value::user::UserObj::inner(self) as *const ::ffi::llvm_User) as u32;
            ret
        }
    }

    fn get_operand(&self, idx: u32) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::User_getOperand(::llvm::value::user::UserObj::inner(self) as *const ::ffi::llvm_User, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn replace_uses_of_with<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, from: &mut A1, to: &mut A2) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(::llvm::value::user::UserObj::inner(self), ::llvm::value::ValueObj::inner(from), ::llvm::value::ValueObj::inner(to));
        }
    }

    fn set_operand<A2: ::llvm::value::ValueObj>(&mut self, idx: u32, val: &mut A2) {
        unsafe {
            ::ffi::llvm::User_setOperand(::llvm::value::user::UserObj::inner(self), idx as ::libc::c_uint, ::llvm::value::ValueObj::inner(val));
        }
    }
}
impl<T> UserExt for T where T: UserObj {}

pub struct User {
    inner: ::core::nonzero::NonZero<*mut UserInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for User {
    fn inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UserObj for User {
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

    pub fn classof<A1: ::llvm::value::ValueObj>(v: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::User_classof(::llvm::value::ValueObj::inner(v));
            ret
        }
    }
}
impl Drop for User {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::inner(self));
            }
        }
    }
}
