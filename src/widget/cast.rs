//! functionality for downcast or traitcast widget references
use super::*;

//TODO simplify Statize and downcast impls into AnyLt struct
/// Trait for retrieving the TypeId of a non-'static type by providing the 'static variant of the type
/// 
/// See [RFC 1849](https://github.com/rust-lang/rust/issues/41875)
#[deprecated]
pub unsafe trait Statize<E> {
    /// Must be `Self`, but with all lifetimes 'static
    type Statur: ?Sized + 'static;
    
    #[inline(always)]
    fn _typeid() -> TypeId {
        TypeId::of::<Self::Statur>()
    }
}

/// StatizeSized is Statize but with Statur: Sized
///
/// StatizeSized is implemented on all Statize where Statur: Sized
#[deprecated]
pub unsafe trait StatizeSized<E> {
    type StaturSized: Sized + 'static; //TODO rename to Statur

    #[inline(always)]
    fn _typeid() -> TypeId {
        TypeId::of::<Self::StaturSized>()
    }
}

#[allow(deprecated)]
mod imp {
    use super::*;
    use std::{borrow::Cow, path::{Path,PathBuf}, sync::Arc, rc::Rc};

    unsafe impl<T,E> StatizeSized<E> for T where T: Statize<E>, T::Statur: Sized {
        type StaturSized = T::Statur;
    }

    unsafe impl<E> Statize<E> for dyn Widget<E>+'_ where E: Env {
        type Statur = dyn Widget<E>+'static;
    }
    unsafe impl<'w,E> Statize<E> for dyn WidgetMut<E>+'_ where E: Env {
        type Statur = dyn WidgetMut<E>+'static;
    }
    unsafe impl<E> Statize<E> for dyn Any {
        type Statur = dyn Any;
    }

    unsafe impl<'w,T,E> Statize<E> for Box<T> where T: Statize<E>+?Sized {
        type Statur = Box<T::Statur>;
    }
    unsafe impl<'w,T,E> Statize<E> for Rc<T> where T: Statize<E>+?Sized {
        type Statur = Rc<T::Statur>;
    }
    unsafe impl<'w,T,E> Statize<E> for Arc<T> where T: Statize<E>+?Sized {
        type Statur = Arc<T::Statur>;
    }
    unsafe impl<'w,T,E> Statize<E> for Vec<T> where T: StatizeSized<E> {
        type Statur = Vec<T::StaturSized>;
    }
    unsafe impl<'w,T,E> Statize<E> for Option<T> where T: StatizeSized<E> {
        type Statur = Option<T::StaturSized>;
    }
    unsafe impl<'w,T,U,E> Statize<E> for Result<T,U> where T: StatizeSized<E>, U: StatizeSized<E> {
        type Statur = Result<T::StaturSized,U::StaturSized>;
    }
    unsafe impl<'w,T,E> Statize<E> for Cow<'w,T> where T: Statize<E>+Clone+?Sized, T::Statur: Clone {
        type Statur = Cow<'static,T::Statur>;
    }
    unsafe impl<'w,T,E> Statize<E> for &'w T where T: Statize<E>+?Sized {
        type Statur = &'static T::Statur;
    }
    unsafe impl<'w,T,E> Statize<E> for &'w mut T where T: Statize<E>+?Sized {
        type Statur = &'static mut T::Statur;
    }
    unsafe impl<'w,T,E> Statize<E> for [T] where T: StatizeSized<E> {
        type Statur = [T::StaturSized];
    }

    macro_rules! impl_statize_static {
        ($t:ty;$($tt:ty);+) => {
            impl_statize_static!($t);
            impl_statize_static!($($tt);*);
        };
        ($t:ty) => {
            unsafe impl<E> Statize<E> for $t {
                type Statur = Self;
            }
        }
    }

    impl_statize_static!(
        bool;char;();
        f32;f64;
        i8;i16;i32;i64;i128;isize;
        u8;u16;u32;u64;u128;usize;
        str;String;//&'static str;
        Path;PathBuf;
        crate::widgets::textbox::state::Cursor;
        StdID;
        StdGonstraints;StdGonstraintAxis
    );

    macro_rules! impl_statize_tuple {
        ($t:ident $($tt:ident)+) => {
            impl_statize_tuple!($($tt)+);

            unsafe impl<E,$t,$($tt),+> Statize<E> for ($t,$($tt),+) where
                $t: StatizeSized<E>,
                $($tt: StatizeSized<E>),+ {
                type Statur = ($t::StaturSized,$($tt::StaturSized),+);
            }
        };
        ($t:ident) => {}
    }

    impl_statize_tuple!(A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG);
}
