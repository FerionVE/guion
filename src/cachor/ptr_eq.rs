use std::any::TypeId;
use std::rc::Rc;
use std::sync::Arc;

use super::{AsCachorOf, AsCachor};

//TODO prevent use with interior mutable T
//TODO remove Rc impl as no Send + Sync
/// Cachor by ptr_eq faster than deep eq
/// 
/// Implemented on e.g. `&'static T`, `Rc<T>` `Arc<T>`
/// 
/// On e.g. &'static T it will only compare the pointer address, on e.g. Rc/Arc it will refclone it and only use Rc/Arc::ptr_eq.
/// 
/// If there are different pointers with identical T content, it will still invalidate, so it performs best if there are no different pointers with identical content.
/// 
/// Do NOT use this with types with interior mutability, changes to the interior would NOT be detected (so it BREAKS caching)
/// 
#[derive(Clone,Copy)]
pub struct CachorPtrEq;
/// Cachor by ptr_eq faster than deep eq
/// 
/// Implemented on e.g. `&'static T`, `Rc<T>` `Arc<T>`
/// 
/// On e.g. &'static T it will only compare the pointer address, on e.g. Rc/Arc it will refclone it and only use Rc/Arc::ptr_eq.
/// 
/// If there are different pointers with identical T content, it will still invalidate, so it performs best if there are no different pointers with identical content.
/// 
/// Do NOT use this with types with interior mutability, changes to the interior would NOT be detected (so it BREAKS caching)
/// 
#[derive(Clone,Copy)]
pub struct CachorPtrEqRef;
/// Cachor by ptr_eq faster than deep eq
/// 
/// Implemented on e.g. `&'static T`, `Rc<T>` `Arc<T>`
/// 
/// On e.g. &'static T it will only compare the pointer address, on e.g. Rc/Arc it will refclone it and only use Rc/Arc::ptr_eq.
/// 
/// If there are different pointers with identical T content, it will still invalidate, so it performs best if there are no different pointers with identical content.
/// 
/// Do NOT use this with types with interior mutability, changes to the interior would NOT be detected (so it BREAKS caching)
/// 
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct CachorPtrEqV<T>(pub T) where T: ?Sized;
/// Cachor by ptr_eq faster than deep eq
/// 
/// Implemented on e.g. `&'static T`, `Rc<T>` `Arc<T>`
/// 
/// On e.g. &'static T it will only compare the pointer address, on e.g. Rc/Arc it will refclone it and only use Rc/Arc::ptr_eq.
/// 
/// If there are different pointers with identical T content, it will still invalidate, so it performs best if there are no different pointers with identical content.
/// 
/// Do NOT use this with types with interior mutability, changes to the interior would NOT be detected (so it BREAKS caching)
/// 
#[repr(transparent)]
pub struct CachorPtrEqVRef<'a,T>(pub &'a T) where T: ?Sized;

impl<'a,T> Clone for CachorPtrEqVRef<'a,T> where T: ?Sized {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<'a,T> Copy for CachorPtrEqVRef<'a,T> where T: ?Sized {}

impl<T,E> AsCachorOf<&T,E> for CachorPtrEqRef where CachorPtrEq: AsCachorOf<T,E>, T: ?Sized {
    type Cachor = <CachorPtrEq as AsCachorOf<T,E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &&T) -> Self::Cachor {
        <CachorPtrEq as AsCachorOf<T,E>>::cachor(&CachorPtrEq, v)
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &&T, cachored: &Self::Cachor) -> bool {
        <CachorPtrEq as AsCachorOf<T,E>>::valid(&CachorPtrEq, v, cachored)
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorPtrEqV<T> where Self: AsCachor<E>, T: ?Sized {
    type Cachor = <Self as AsCachor<E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        <Self as AsCachor<E>>::cachor(self)
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        <Self as AsCachor<E>>::valid(self, cachored)
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorPtrEqVRef<'_,T> where Self: AsCachor<E>, T: ?Sized {
    type Cachor = <Self as AsCachor<E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        <Self as AsCachor<E>>::cachor(self)
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        <Self as AsCachor<E>>::valid(self, cachored)
    }
}

impl<T,E> AsCachorOf<&'static T,E> for CachorPtrEq where T: 'static + ?Sized {
    type Cachor = PtrEqCachor<&'static T>;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &&'static T) -> Self::Cachor {
        PtrEqCachor(*v)
    }
}

impl<T,E> AsCachorOf<Rc<T>,E> for CachorPtrEq where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Rc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &Rc<T>) -> Self::Cachor {
        PtrEqCachor(v.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &Rc<T>, cachored: &Self::Cachor) -> bool {
        Rc::ptr_eq(v, &cachored.0)
    }
}

impl<T,E> AsCachorOf<Arc<T>,E> for CachorPtrEq where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Arc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &Arc<T>) -> Self::Cachor {
        PtrEqCachor(v.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &Arc<T>, cachored: &Self::Cachor) -> bool {
        Arc::ptr_eq(v, &cachored.0)
    }
}

impl<T,E> AsCachor<E> for CachorPtrEqV<&'static T> where T: 'static + ?Sized{
    type Cachor = PtrEqCachor<&'static T>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(self.0)
    }
}
impl<T,E> AsCachor<E> for CachorPtrEqVRef<'_,&'static T> where T: 'static + ?Sized {
    type Cachor = PtrEqCachor<&'static T>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(*self.0)
    }
}

impl<T,E> AsCachor<E> for CachorPtrEqV<Rc<T>> where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Rc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(self.0.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        Rc::ptr_eq(&self.0, &cachored.0)
    }
}
impl<T,E> AsCachor<E> for CachorPtrEqVRef<'_,Rc<T>> where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Rc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(self.0.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        Rc::ptr_eq(self.0, &cachored.0)
    }
}

impl<T,E> AsCachor<E> for CachorPtrEqV<Arc<T>> where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Arc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(self.0.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        Arc::ptr_eq(&self.0, &cachored.0)
    }
}
impl<T,E> AsCachor<E> for CachorPtrEqVRef<'_,Arc<T>> where T: PartialEq + ?Sized + 'static {
    type Cachor = PtrEqCachor<Arc<T>>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        PtrEqCachor(self.0.clone())
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        Arc::ptr_eq(self.0, &cachored.0)
    }
}

#[doc(hidden)]
#[derive(Clone)]
#[repr(transparent)]
pub struct PtrEqCachor<T>(T) where T: Clone + 'static;

impl<T> Copy for PtrEqCachor<T> where T: Copy + ?Sized {}

impl<T> PartialEq for PtrEqCachor<&'static T> where T: ?Sized + 'static {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if TypeId::of::<T>() == TypeId::of::<()>() {
            return true;
        }
        std::ptr::eq(self.0, other.0)
    }
}

impl<T> PartialEq for PtrEqCachor<Rc<T>> where T: ?Sized + 'static {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> PartialEq for PtrEqCachor<Arc<T>> where T: ?Sized + 'static {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
