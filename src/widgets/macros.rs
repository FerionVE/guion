#[macro_export]
macro_rules! create_widget_as_widget_module {
    ($trait:tt,$as_struct_name:ident) => {
        use std::ops::DerefMut;
        use std::ops::Deref;
        use std::marker::PhantomData;
        use $crate::core::util::ScopedMut;
        use $crate::core::ctx::Context;
        //use crate::core::util::qwutils::impl_scoped_mut_inner;
        use super::*;

        /// put a type or mutable reference implementing ITemplate inside this to enforce view as Template
        pub struct $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            pub inner: T,
            _e: PhantomData<E>,
        }

        impl<T,U,E> $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            #[inline]
            pub fn new(inner: T) -> Self {
                Self{
                    inner,
                    _e: PhantomData,
                }
            }
        }

        impl<T,U,E> From<T> for $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            #[inline]
            fn from(inner: T) -> Self {
                Self::new(inner)
            }
        }

        impl<T,U,E> Deref for $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            type Target=T;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<T,U,E> DerefMut for $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        /*impl<T,U,E> ScopedMut for $as_struct_name<T,U,E> where T: ScopedMut<T=U> + 'static, U: $trait<E>, E: Context + 'static {
            impl_scoped_mut_inner!(T);
        }*/
    };
}