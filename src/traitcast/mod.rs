use std::any::TypeId;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::env::Env;

pub trait WQuery<E>: 'static {
    type Result<'a>: Sized + 'a;

    //fn covariant<'a,'s>(v: Self::Result<'a>) -> Self::Result<'s> where 'a: 's;
}
pub trait WQueryGeneric<E>: 'static {
    type Result<'a,G>: Sized + 'a where G: ?Sized + 'a;
}

pub struct WQueryResponder<'s,'a,E> where 'a: 's {
    type_id: TypeId, // T
    response: NonNull<()>, // &'s mut Option<T::Result<'a>>
    _p: PhantomData<(&'s mut &'a mut (),E)>, // invariant 'a
}

impl<'s,'a,E> WQueryResponder<'s,'a,E> where E: Env {
    #[inline]
    pub fn new<T>(respond_into: &'s mut Option<T::Result<'a>>) -> Self where T: WQuery<E> + ?Sized {
        Self {
            type_id: TypeId::of::<T>(),
            response: NonNull::<Option<T::Result<'a>>>::from(respond_into).cast::<()>(),
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn try_respond<T>(&mut self, respond: impl FnOnce() -> T::Result<'a>) -> bool where T: WQuery<E> + ?Sized {
        self.try_downcast::<T>().map(#[inline] |v| *v = Some(respond()) ).is_some()
    }

    #[inline]
    pub fn try_respond_2<T>(&mut self, respond: impl FnOnce(&mut Option<T::Result<'a>>)) -> bool where T: WQuery<E> + ?Sized {
        self.try_downcast::<T>().map(respond).is_some()
    }

    /// non zero cost respond ops should be extracted into a fn/closure or use try_respond, so that this part can be inlined.
    #[inline]
    pub fn try_downcast<'ss,T>(&'ss mut self) -> Option<&'ss mut Option<T::Result<'a>>> where 's: 'ss, T: WQuery<E> + ?Sized {
        if TypeId::of::<T>() == self.type_id {
            Some(unsafe {
                let resp: &'s mut Option<T::Result<'a>> = self.response.cast::<Option<T::Result<'a>>>().as_mut();
                resp
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn fork(&mut self) -> WQueryResponder<'_,'a,E> {
        WQueryResponder { 
            type_id: self.type_id,
            response: self.response,
            _p: PhantomData,
        }
    }
}

pub struct WQueryResponderGeneric<'s,'a,Q,G,E> where 'a: 's, Q: WQueryGeneric<E> + ?Sized, G: ?Sized + 'a {
    response: &'s mut Option<Q::Result<'a,G>>,
    _p: PhantomData<(&'s mut &'a mut G,&'s mut Q,E)>, // invariant 'a
}

impl<'a,'s,Q,G,E> WQueryResponderGeneric<'s,'a,Q,G,E> where E: Env, Q: WQueryGeneric<E> + ?Sized, G: ?Sized {
    #[inline]
    pub fn new(respond_into: &'s mut Option<Q::Result<'a,G>>) -> Self {
        Self {
            response: respond_into,
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn try_respond<T>(&mut self, fun: impl FnOnce() -> T::Result<'a,G>) -> bool where T: WQueryGeneric<E> + ?Sized {
        self.try_downcast::<T>().map(#[inline] |v| *v = Some(fun()) ).is_some()
    }

    #[inline]
    pub fn try_respond_2<T>(&mut self, fun: impl FnOnce(&mut Option<T::Result<'a,G>>)) -> bool where T: WQueryGeneric<E> + ?Sized {
        self.try_downcast::<T>().map(fun).is_some()
    }

    #[inline]
    pub fn try_downcast<'ss,T>(&'ss mut self) -> Option<&'ss mut Option<T::Result<'a,G>>> where 's: 'ss, T: WQueryGeneric<E> + ?Sized {
        if TypeId::of::<T>() == TypeId::of::<Q>() {
            Some(unsafe {
                let resp: &'ss mut Option<Q::Result<'a,G>> = self.response;
                &mut *(resp as *mut _ as *mut Option<T::Result<'a,G>>)
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn fork(&mut self) -> WQueryResponderGeneric<'_,'a,Q,G,E> {
        WQueryResponderGeneric { 
            response: self.response,
            _p: PhantomData,
        }
    }
}

// pub struct DowncastQuery<T>(PhantomData<T>) where T: ?Sized + 'static;

// pub struct DowncastMutQuery<T>(PhantomData<T>) where T: ?Sized + 'static;

// impl<T,E> WQuery<E> for DowncastQuery<T> where T: ?Sized + 'static {
//     type Result<'a> = &'a T;
// }

// impl<T,E> WQuery<E> for DowncastMutQuery<T> where T: ?Sized + 'static {
//     type Result<'a> = &'a mut T;
// }

pub struct DowncastResponder<'s,'a,E> where 'a: 's {
    type_id: TypeId, // T
    response: NonNull<()>, // &'s mut Option<&'a T>
    _p: PhantomData<(&'s mut &'a mut (),E)>, // invariant 'a
}

impl<'s,'a,E> DowncastResponder<'s,'a,E> where E: Env {
    #[inline]
    pub fn new<T>(respond_into: &'s mut Option<&'a T>) -> Self where T: ?Sized + 'static {
        Self {
            type_id: TypeId::of::<T>(),
            response: NonNull::<Option<&'a T>>::from(respond_into).cast::<()>(),
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn try_downcast<'ss,T>(&'ss mut self) -> Option<&'ss mut Option<&'a T>> where 's: 'ss, T: ?Sized + 'static {
        if TypeId::of::<T>() == self.type_id {
            Some(unsafe {
                let resp: &'s mut Option<&'a T> = self.response.cast::<Option<&'a T>>().as_mut();
                resp
            })
        } else {
            None
        }
    }
}


pub struct DowncastMutResponder<'s,'a,E> where 'a: 's {
    type_id: TypeId, // T
    response: NonNull<()>, // &'s mut Option<&'a mut T>
    _p: PhantomData<(&'s mut &'a mut (),E)>, // invariant 'a
}

impl<'s,'a,E> DowncastMutResponder<'s,'a,E> where E: Env {
    #[inline]
    pub fn new<T>(respond_into: &'s mut Option<&'a mut T>) -> Self where T: ?Sized + 'static {
        Self {
            type_id: TypeId::of::<T>(),
            response: NonNull::<Option<&'a mut T>>::from(respond_into).cast::<()>(),
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn try_downcast<'ss,T>(&'ss mut self) -> Option<&'ss mut Option<&'a mut T>> where 's: 'ss, T: ?Sized + 'static {
        if TypeId::of::<T>() == self.type_id {
            Some(unsafe {
                let resp: &'s mut Option<&'a mut T> = self.response.cast::<Option<&'a mut T>>().as_mut();
                resp
            })
        } else {
            None
        }
    }
}
