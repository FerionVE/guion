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

impl<'a,E> WQueryResponder<'_,'a,E> where E: Env {
    #[inline]
    pub fn new<T>(respond_into: &mut Option<T::Result<'a>>) -> Self where T: WQuery<E> + ?Sized {
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
    pub fn try_downcast<'s,T>(&'s mut self) -> Option<&'s mut Option<T::Result<'a>>> where 'a: 's, T: WQuery<E> + ?Sized {
        if TypeId::of::<T>() == self.type_id {
            Some(unsafe {
                self.response.cast::<Option<T::Result<'a>>>().as_mut()
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

impl<'a,'sa,Q,G,E> WQueryResponderGeneric<'sa,'a,Q,G,E> where E: Env, Q: WQueryGeneric<E> + ?Sized, G: ?Sized {
    #[inline]
    pub fn new(respond_into: &'sa mut Option<Q::Result<'a,G>>) -> Self {
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
    pub fn try_downcast<'s,T>(&'s mut self) -> Option<&'s mut Option<T::Result<'a,G>>> where 'a: 's, T: WQueryGeneric<E> + ?Sized {
        if TypeId::of::<T>() == TypeId::of::<Q>() {
            Some(unsafe {
                let resp: &'s mut Option<Q::Result<'a,G>> = self.response;
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
