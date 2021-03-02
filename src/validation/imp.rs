use super::*;
use std::path::PathBuf;

impl<E,T> Validation<E> for &T where T: Validation<E> {
    //type Cached = T::Cached;

    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        (**self).valid(v)
    }
}

impl<E,T> Validation<E> for &mut T where T: Validation<E> {
    //type Cached = T::Cached;

    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        (**self).valid(v)
    }
}
impl<E,T> ValidationMut<E> for &mut T where T: ValidationMut<E> {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        (**self).validate()
    }
}

impl<E,T> Validation<E> for Box<T> where T: Validation<E> {
    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        (**self).valid(v)
    }
}
impl<E,T> ValidationMut<E> for Box<T> where T: ValidationMut<E> {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        (**self).validate()
    }
}

macro_rules! impl_validation_primitives {
    ($t:ty;$($tt:ty);+) => {
        impl_validation_primitives!($t);
        impl_validation_primitives!($($tt);*);
    };
    ($t:ty) => {
        impl<E> Validation<E> for $t {
            #[inline]
            fn valid(&self, v: &dyn Any) -> bool {
                if let Some(v) = v.downcast_ref::<Self>() {
                    self == v
                }else{
                    false
                }
            }
        }
        impl<E> ValidationMut<E> for $t {
            #[inline]
            fn validate(&mut self) -> Arc<dyn Any> {
                Arc::new((*self).clone())
            }
        }
    }
}

impl_validation_primitives!(
    bool;char;
    f32;f64;
    i8;i16;i32;i64;i128;isize;
    u8;u16;u32;u64;u128;usize;
    String;PathBuf
);

impl<E> Validation<E> for &str {
    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        if let Some(v) = v.downcast_ref::<String>() {
            *self == v
        }else{
            false
        }
    }
}
impl<E> ValidationMut<E> for &str {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        Arc::new((*self).to_owned())
    }
}

impl<E> Validation<E> for &mut str {
    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        if let Some(v) = v.downcast_ref::<String>() {
            *self == v
        }else{
            false
        }
    }
}
impl<E> ValidationMut<E> for &mut str {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        Arc::new((*self).to_owned())
    }
}

impl<E> Validation<E> for () {
    #[inline]
    fn valid(&self, _: &dyn Any) -> bool {
        true
    }
}
impl<E> ValidationMut<E> for () {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        Arc::new(())
    }
}
