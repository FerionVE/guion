//! helper functions for limiting lifetimes
use super::*;

/*pub unsafe trait ShortLT: Sized {
    type Dest: Sized;

    fn short_lt(self) -> Self::Dest;
}

/*pub unsafe trait ShortLTBase {
    type _Dest: Sized;

    fn _short_lt(self) -> Self::_Dest;
}

unsafe impl<T> ShortLTBase for T where T: ShortLT + Sized, T::Dest: Sized {
    type _Dest = T::Dest;

    fn _short_lt(self) -> Self::_Dest {
        unsafe {
            std::mem::transmute::<Self,T::_Dest>(self)
        }
    }
}*/

macro_rules! simp {
    () => {
        fn short_lt(self) -> Self::Dest {
            unsafe {
                std::mem::transmute::<Self,Self::Dest>(self)
            }
        }
    };
}

unsafe impl<'l,'s,E> ShortLT for Resolvable<'l,E> where 'l: 's, E: Env {
    type Dest = Resolvable<'s,E>;

    fn short_lt(self) -> Resolvable<'s,E> {
        short_resolvable(self)
    }
}*/

impl<'l:'s,'s,E: Env> Resolvable<'l,E> {
    pub fn short_lt(self) -> Resolvable<'s,E> {
        match self {
            Resolvable::Widget(w) => Resolvable::Widget(w.short_lt()),
            Resolvable::Path(p) => Resolvable::Path(p),
        }
    }
}
pub trait ShortResolvableRef<'l,'s,'y,E> where 'l: 's, 'l: 'y, 's: 'y, E: Env {
    fn short_lt(self) -> &'y Resolvable<'s,E>;
}
impl<'l,'s,'y,E> ShortResolvableRef<'l,'s,'y,E> for &'y Resolvable<'l,E> where 'l: 's, 'l: 'y, 's: 'y, E: Env {
    fn short_lt(self) -> &'y Resolvable<'s,E> {
        unsafe{
            std::mem::transmute::<&'y Resolvable<'l,E>,&'y Resolvable<'s,E>>(self)
        }
    }
}
impl<'l:'s,'s,E: Env> ResolvableMut<'l,E> {
    pub fn short_lt(self) -> ResolvableMut<'s,E> {
        match self {
            ResolvableMut::Widget(w) => ResolvableMut::Widget(w.short_lt()),
            ResolvableMut::Path(p) => ResolvableMut::Path(p),
        }
    }
}

pub trait ShortResolvableVec<'l,'s,E> where 'l: 's, E: Env {
    fn short_lt(self) -> Vec<Resolvable<'s,E>>;
}
impl<'l,'s,E> ShortResolvableVec<'l,'s,E> for Vec<Resolvable<'l,E>> where 'l: 's, E: Env {
    fn short_lt(self) -> Vec<Resolvable<'s,E>> {
        unsafe{
            std::mem::transmute::<Vec<Resolvable<'l,E>>,Vec<Resolvable<'s,E>>>(self)
        }
    }
}

impl<'l:'s,'s,E: Env> Resolved<'l,E> {
    pub fn short_lt(self) -> Resolved<'s,E> {
        Resolved{
            wref: self.wref.short_lt(),
            path: self.path,
            stor: self.stor,
        }
    }
}
impl<'l:'s,'s,E: Env> ResolvedMut<'l,E> {
    pub fn short_lt(self) -> ResolvedMut<'s,E> {
        ResolvedMut{
            wref: self.wref.short_lt(),
            path: self.path,
        }
    }
}
pub trait ShortResolvedRef<'l,'s,'y,E> where 'l: 's, 'l: 'y, 's: 'y, E: Env {
    fn short_lt(self) -> &'y Resolved<'s,E>;
}
impl<'l,'s,'y,E> ShortResolvedRef<'l,'s,'y,E> for &'y Resolved<'l,E> where 'l: 's, 'l: 'y, 's: 'y, E: Env {
    fn short_lt(self) -> &'y Resolved<'s,E> {
        unsafe{
            std::mem::transmute::<&'y Resolved<'l,E>,&'y Resolved<'s,E>>(self)
        }
    }
}

pub trait ShortWidgetRef<'l,'s,E> where 'l: 's, E: Env {
    fn short_lt(self) -> WidgetRef<'s,E>;
}
impl<'l,'s,E> ShortWidgetRef<'l,'s,E> for WidgetRef<'l,E> where 'l: 's, E: Env {
    fn short_lt(self) -> WidgetRef<'s,E> {
        unsafe{
            std::mem::transmute::<WidgetRef<'l,E>,WidgetRef<'s,E>>(self)
        }
    }
}
pub trait ShortWidgetRefMut<'l,'s,E> where 'l: 's, E: Env {
    fn short_lt(self) -> WidgetRefMut<'s,E>;
}
impl<'l,'s,E> ShortWidgetRefMut<'l,'s,E> for WidgetRefMut<'l,E> where 'l: 's, E: Env {
    fn short_lt(self) -> WidgetRefMut<'s,E> {
        unsafe{
            std::mem::transmute::<WidgetRefMut<'l,E>,WidgetRefMut<'s,E>>(self)
        }
    }
}

pub trait ShortRefWidgetRef<'l,'s,'y,E> where 'l: 's, 's: 'y, 'l: 'y, E: Env {
    fn short_lt(self) -> &'y dyn Widget<'s,E>;
}
impl<'l,'s,'y,E> ShortRefWidgetRef<'l,'s,'y,E> for &'y dyn Widget<'l,E> where 'l: 's, 's: 'y, 'l: 'y, E: Env {
    fn short_lt(self) -> &'y dyn Widget<'s,E> {
        unsafe{
            std::mem::transmute::<&'y dyn Widget<'l,E>,&'y dyn Widget<'s,E>>(self)
        }
    }
}
pub trait ShortRefWidgetMut<'l,'s,'y,E> where 'l: 's, 's: 'y, 'l: 'y, E: Env {
    fn short_lt(self) -> &'y mut dyn WidgetMut<'s,E>;
}
impl<'l,'s,'y,E> ShortRefWidgetMut<'l,'s,'y,E> for &'y mut dyn WidgetMut<'l,E> where 'l: 's, 's: 'y, 'l: 'y, E: Env {
    fn short_lt(self) -> &'y mut dyn WidgetMut<'s,E> {
        unsafe{
            std::mem::transmute::<&'y mut dyn WidgetMut<'l,E>,&'y mut dyn WidgetMut<'s,E>>(self)
        }
    }
}
