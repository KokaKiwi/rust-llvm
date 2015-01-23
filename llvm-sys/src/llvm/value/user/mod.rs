pub mod constant;
pub mod inst;
pub type OperatorInner = ::ffi::llvm_Operator;

pub trait OperatorObj: ::llvm::value::user::UserObj {
    unsafe fn get_inner(&self) -> *mut OperatorInner;
}

pub trait OperatorOwned: OperatorObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut OperatorInner {
        let inner = OperatorObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> OperatorOwned for T where T: OperatorObj + ::core::marker::Sized {}

pub trait OperatorExt: OperatorObj {

    fn get_opcode(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Operator_getOpcode(::llvm::value::user::OperatorObj::get_inner(self) as *const ::ffi::llvm_Operator) as u32;
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
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ::llvm::value::user::UserObj for Operator {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_User {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl OperatorObj for Operator {
    #[inline(always)]
    fn get_inner(&self) -> *mut OperatorInner {
        *self.inner
    }
}
impl Operator {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut OperatorInner, owned: bool) -> Operator {
        Operator {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Operator {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
pub type UseInner = ::ffi::llvm_Use;

pub trait UseObj {
    unsafe fn get_inner(&self) -> *mut UseInner;
}

pub trait UseOwned: UseObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut UseInner {
        let inner = UseObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> UseOwned for T where T: UseObj + ::core::marker::Sized {}

pub trait UseExt: UseObj {

    fn get(&self) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::Use_get(::llvm::value::user::UseObj::get_inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn get_next(&self) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_getNext(::llvm::value::user::UseObj::get_inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::Use::from_inner(ret))
        }
    }

    fn get_operand_no(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::Use_getOperandNo(::llvm::value::user::UseObj::get_inner(self) as *const ::ffi::llvm_Use) as u32;
            ret
        }
    }

    fn get_user(&self) -> Option<::llvm::value::user::User> {
        unsafe {
            let ret = ::ffi::llvm::Use_getUser(::llvm::value::user::UseObj::get_inner(self) as *const ::ffi::llvm_Use);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::user::User::from_inner(ret, false))
        }
    }

    fn set<A1: ::llvm::value::ValueObj>(&mut self, val: &mut A1) {
        unsafe {
            ::ffi::llvm::Use_set(::llvm::value::user::UseObj::get_inner(self), ::llvm::value::ValueObj::get_inner(val));
        }
    }

    fn swap<A1: ::llvm::value::user::UseObj>(&mut self, rhs: &mut A1) {
        unsafe {
            ::ffi::llvm::Use_swap(::llvm::value::user::UseObj::get_inner(self), ::llvm::value::user::UseObj::get_inner(rhs));
        }
    }
}
impl<T> UseExt for T where T: UseObj {}

pub struct Use {
    inner: ::core::nonzero::NonZero<*mut UseInner>,
}
impl UseObj for Use {
    #[inline(always)]
    fn get_inner(&self) -> *mut UseInner {
        *self.inner
    }
}
impl Use {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut UseInner) -> Use {
        Use {
            inner: ::core::nonzero::NonZero::new(inner),
        }
    }

    pub fn init_tags<A1: ::llvm::value::user::UseObj, A2: ::llvm::value::user::UseObj>(start: &mut A1, stop: &mut A2) -> Option<::llvm::value::user::Use> {
        unsafe {
            let ret = ::ffi::llvm::Use_initTags(::llvm::value::user::UseObj::get_inner(start), ::llvm::value::user::UseObj::get_inner(stop));
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
    unsafe fn get_inner(&self) -> *mut UserInner;
}

pub trait UserOwned: UserObj + ::core::marker::Sized {
    #[inline(always)]
    unsafe fn move_inner(self) -> *mut UserInner {
        let inner = UserObj::get_inner(&self);
        ::core::mem::forget(self);
        return inner;
    }
}
impl<T> UserOwned for T where T: UserObj + ::core::marker::Sized {}

pub trait UserExt: UserObj {

    fn drop_all_references(&mut self) {
        unsafe {
            ::ffi::llvm::User_dropAllReferences(::llvm::value::user::UserObj::get_inner(self));
        }
    }

    fn get_num_operands(&self) -> u32 {
        unsafe {
            let ret = ::ffi::llvm::User_getNumOperands(::llvm::value::user::UserObj::get_inner(self) as *const ::ffi::llvm_User) as u32;
            ret
        }
    }

    fn get_operand(&self, idx: u32) -> Option<::llvm::value::Value> {
        unsafe {
            let ret = ::ffi::llvm::User_getOperand(::llvm::value::user::UserObj::get_inner(self) as *const ::ffi::llvm_User, idx as ::libc::c_uint);
            if ret.is_null() {
                return None;
            }
            Some(::llvm::value::Value::from_inner(ret, false))
        }
    }

    fn replace_uses_of_with<A1: ::llvm::value::ValueObj, A2: ::llvm::value::ValueObj>(&mut self, from: &mut A1, to: &mut A2) {
        unsafe {
            ::ffi::llvm::User_replaceUsesOfWith(::llvm::value::user::UserObj::get_inner(self), ::llvm::value::ValueObj::get_inner(from), ::llvm::value::ValueObj::get_inner(to));
        }
    }

    fn set_operand<A2: ::llvm::value::ValueObj>(&mut self, idx: u32, val: &mut A2) {
        unsafe {
            ::ffi::llvm::User_setOperand(::llvm::value::user::UserObj::get_inner(self), idx as ::libc::c_uint, ::llvm::value::ValueObj::get_inner(val));
        }
    }
}
impl<T> UserExt for T where T: UserObj {}

pub struct User {
    inner: ::core::nonzero::NonZero<*mut UserInner>,
    owned: bool,
}
impl ::llvm::value::ValueObj for User {
    #[inline(always)]
    fn get_inner(&self) -> *mut ::ffi::llvm_Value {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl UserObj for User {
    #[inline(always)]
    fn get_inner(&self) -> *mut UserInner {
        *self.inner
    }
}
impl User {
    #[inline(always)]
    pub unsafe fn from_inner(inner: *mut UserInner, owned: bool) -> User {
        User {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }

    pub fn classof<A1: ::llvm::value::ValueObj>(v: &mut A1) -> bool {
        unsafe {
            let ret = ::ffi::llvm::User_classof(::llvm::value::ValueObj::get_inner(v));
            ret
        }
    }
}
impl Drop for User {
    #[inline(always)]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::User_delete(::llvm::value::user::UserObj::get_inner(self));
            }
        }
    }
}
