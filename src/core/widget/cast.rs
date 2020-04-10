use super::*;

//TODO simplify Statize and downcast impls into AnyLt struct
/// Trait for retrieving the TypeId of a non-'static type by providing the 'static variant of the type  
/// [RFC 1849](https://github.com/rust-lang/rust/issues/41875)
pub unsafe trait Statize {
    /// Must be `Self`, but with all lifetimes 'static
    type Statur: ?Sized + 'static;
    
    fn _typeid() -> TypeId {
        TypeId::of::<Self::Statur>()
    }
}

impl<'w,E> dyn Widget<'w,E> where E: Env {
    pub fn is_type<T>(&self) -> bool where T: Statize {
        self.typeid() == T::_typeid()
    }

    pub fn _downcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Statize+'d, 'w: 's, 'w: 'd, 'd: 's {
        if self.is_type::<T>() {
            unsafe { Some(&*(self as *const dyn Widget<'w,E> as *const T)) }
        } else {
            None
        }
    }
    /// downcast the current widget to a specific implementation
    pub fn downcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Statize+'d, 'w: 's, 'w: 'd, 'd: 's {
        if let Some(v) = Self::_downcast_ref::<T>(self) {
            Some(v)
        }else if let Some(senf) = self.inner() {
            senf.downcast_ref::<T>()
        }else{
            None
        }
    }
    pub fn _traitcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Statize+?Sized+'d, 'w: 's, 'w: 'd, 'd: 's {
        let t = unsafe{self._as_trait_ref(T::_typeid())};
        if let Some(v) = t {
            unsafe { Some(std::mem::transmute_copy::<TraitObject,&'s T>(&v)) }
        } else {
            None
        }
    }
    /// this will definetly cause UB and delet ur computer
    pub fn traitcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Statize+?Sized+'d, 'w: 's, 'w: 'd, 'd: 's {
        if let Some(v) = Self::_traitcast_ref::<T>(self) {
            Some(v)
        }else if let Some(senf) = self.inner() {
            senf.traitcast_ref::<T>()
        }else{
            None
        }
    }
}
impl<'w,E> dyn WidgetMut<'w,E> where E: Env {
    pub fn is_type<T>(&self) -> bool where T: Statize {
        self.typeid() == T::_typeid()
    }
    
    pub fn _downcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: Statize+'d, 'w: 's, 'd: 's {
        if self.is_type::<T>() {
            unsafe { Some(&mut *(self as *mut dyn WidgetMut<'w,E> as *mut T)) }
        } else {
            None
        }
    }
    /// downcast the current widget to a specific implementation
    pub fn downcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: Statize+'d, 'w: 's, 'd: 's {
        if self.is_type::<T>() {
            self._downcast_mut::<T>()
        }else if let Some(senf) = self.inner_mut() {
            senf.downcast_mut::<T>()
        }else{
            None
        }
    }
    pub fn _traitcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: Statize+?Sized+'d, 'w: 's, 'd: 's {
        let t = unsafe{self._as_trait_mut(T::_typeid())};
        if let Some(v) = t {
            unsafe { Some(std::mem::transmute_copy::<TraitObject,&'s mut T>(&v)) }
        } else {
            None
        }
    }
    /// this will definetly cause UB and delet ur computer
    pub fn traitcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: Statize+?Sized+'d, 'w: 's, 'd: 's {
        let t = unsafe{WidgetMut::_as_trait_ref(self,T::_typeid()).is_some()};
        if t {
            self._traitcast_mut::<T>()
        }else if let Some(senf) = self.inner_mut() {
            senf.traitcast_mut::<T>()
        }else{
            None
        }
    }
}

unsafe impl<'w,E> Statize for dyn Widget<'w,E> where E: Env {
    type Statur = dyn Widget<'static,E>;
}
unsafe impl<'w,E> Statize for dyn WidgetMut<'w,E> where E: Env {
    type Statur = dyn WidgetMut<'static,E>;
}

unsafe impl<'l,'s,E> Statize for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    type Statur = &'static dyn Widget<'static,E>;
}
unsafe impl<'l,'s,E> Statize for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    type Statur = &'static mut dyn WidgetMut<'static,E>;
}

unsafe impl<'w,E> Statize for WidgetRef<'w,E> where E: Env {
    type Statur = WidgetRef<'static,E>;
}
unsafe impl<'w,E> Statize for WidgetRefMut<'w,E> where E: Env {
    type Statur = WidgetRefMut<'static,E>;
}

mod imp {
    use super::*;
    use std::{borrow::Cow, path::{Path,PathBuf}};

    unsafe impl<'w,T> Statize for Box<T> where T: Statize {
        type Statur = Box<T::Statur>;
    }
    unsafe impl<'w,T> Statize for Vec<T> where T: Statize, T::Statur: Sized {
        type Statur = Vec<T::Statur>;
    }
    unsafe impl<'w,T> Statize for Option<T> where T: Statize, T::Statur: Sized {
        type Statur = Option<T::Statur>;
    }
    unsafe impl<'w,T,U> Statize for Result<T,U> where T: Statize, T::Statur: Sized, U: Statize, U::Statur: Sized {
        type Statur = Result<T::Statur,U::Statur>;
    }
    unsafe impl<'w,T> Statize for Cow<'w,T> where T: Statize+Clone, T::Statur: Sized {
        type Statur = Option<T::Statur>;
    }
    /*unsafe impl<'w,T> Statize for &'w T where T: Statize {
        type Statur = &'static T::Statur;
    }
    unsafe impl<'w,T> Statize for &'w mut T where T: Statize {
        type Statur = &'static mut T::Statur;
    }*/
    unsafe impl<'w,T> Statize for &'w [T] where T: Statize, T::Statur: Sized {
        type Statur = &'static [T::Statur];
    }
    unsafe impl<'w,T> Statize for &'w mut [T] where T: Statize, T::Statur: Sized {
        type Statur = &'static mut [T::Statur];
    }
    unsafe impl<'w,T> Statize for Box<[T]> where T: Statize, T::Statur: Sized {
        type Statur = Box<[T::Statur]>;
    }

    macro_rules! impl_statize_static {
        ($t:ty;$($tt:ty);+) => {
            impl_statize_static!($t);
            impl_statize_static!($($tt);*);
        };
        ($t:ty) => {
            unsafe impl Statize for $t {
                type Statur = Self;
            }
            unsafe impl Statize for &$t {
                type Statur = &'static $t;
            }
            unsafe impl Statize for &mut $t {
                type Statur = &'static $t;
            }
        }
    }

    impl_statize_static!(
        bool;char;();
        f32;f64;
        i8;i16;i32;i64;i128;isize;
        u8;u16;u32;u64;u128;usize;
        str;String;//&'static str;
        Path;PathBuf
    );

    macro_rules! impl_statize_tuple {
        ($t:ident $($tt:ident)+) => {
            impl_statize_tuple!($($tt)+);

            unsafe impl<$t,$($tt),+> Statize for ($t,$($tt),+) where
                $t: Statize, $t::Statur: Sized,
                $($tt: Statize, $tt::Statur: Sized),+ {
                type Statur = ($t::Statur,$($tt::Statur),+);
            }
        };
        ($t:ident) => {}
    }

    impl_statize_tuple!(A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG);
}