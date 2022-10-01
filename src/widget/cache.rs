use crate::cachialize::cache::Cache;

use super::*;

pub trait WidgetCache<E>: Default + Clone + Sized + 'static {
    /// Reset current of current and child caches
    fn reset_current(&mut self);
}

pub trait WidgetCacheDyn<E>: 'static {
    fn reset_current(&mut self);
    fn _clone_to_box(&self) -> Box<dyn WidgetCacheDyn<E>>;
}

impl<T,E> WidgetCacheDyn<E> for T where T: WidgetCache<E> {
    #[inline]
    fn reset_current(&mut self) {
        WidgetCache::reset_current(self)
    }
    #[inline]
    fn _clone_to_box(&self) -> Box<dyn WidgetCacheDyn<E>> {
        Box::new(self.clone())
    }
}

pub struct DynWidgetCache<E> {
    inner_type_id: TypeId,
    inner: Box<dyn WidgetCacheDyn<E>>,
}

impl<E> DynWidgetCache<E> where E: 'static {
    #[inline]
    pub fn from_cache<T>(inner: T) -> Self where T: WidgetCache<E> {
        if TypeId::of::<Self>() == TypeId::of::<T>() {
            // avoid nesting
            let senf: Self = unsafe{ std::mem::transmute_copy(&inner) };
            std::mem::forget(inner);
            return senf;
        }
        Self {
            inner_type_id: TypeId::of::<T>(),
            inner: Box::new(inner),
        }
    }

    #[inline]
    pub fn downcast_mut_or_reset<T>(&mut self) -> &mut T where T: WidgetCache<E> {
        if TypeId::of::<Self>() == TypeId::of::<T>() {
            // return self in nested downcast
            return unsafe { &mut *(self as *mut Self as *mut T) }
        }
        if self.inner_type_id != TypeId::of::<T>() {
            self.inner = Box::new(T::default());
            self.inner_type_id = TypeId::of::<T>();
        }
        let inner = &mut *self.inner;
        unsafe { &mut *(inner as *mut dyn WidgetCacheDyn<E> as *mut T) }
    }
}

impl<E> Default for DynWidgetCache<E> where E: 'static {
    #[inline]
    fn default() -> Self {
        Self::from_cache(CacheDefault)
    }
}

impl<E> Clone for DynWidgetCache<E> where E: 'static {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner_type_id: self.inner_type_id,
            inner: self.inner._clone_to_box(),
        }
    }
}

impl<E> WidgetCache<E> for DynWidgetCache<E> where E: 'static {
    #[inline]
    fn reset_current(&mut self) {
        self.inner.reset_current()
    }
}

#[derive(Clone,Default)]
struct CacheDefault;

impl<E> WidgetCache<E> for CacheDefault {
    #[inline]
    fn reset_current(&mut self) {}
}

impl<E> WidgetCache<E> for () {
    #[inline]
    fn reset_current(&mut self) {}
}
