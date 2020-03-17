use super::*;

//TODO simplify Statize and downcast impls into AnyLt struct
/// Trait for retrieving the TypeId of a non-'static type by providing the 'static variant of the type  
/// [RFC 1849](https://github.com/rust-lang/rust/issues/41875)
pub unsafe trait Statize<E> {
    /// Should be `Self`, but with static lifetimes.  
    /// CAUTION: As this type is used as TypeId for downcasting, it __must__ be as unique as the implementor, else __undefined behaviour__ can eventally occur at downcasting
    type Statur: ?Sized + 'static;
    
    fn _typeid() -> TypeId {
        TypeId::of::<Self::Statur>()
    }
}

impl<'w,E> dyn Widget<'w,E> where E: Env {
    pub fn is_type<'s,T>(&self) -> bool where T: Widget<'s,E>+Statize<E> {
        self.typeid() == T::_typeid()
    }

    pub fn _downcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Widget<'d,E>+Statize<E>, 'w: 's, 'w: 'd, 'd: 's {
        if self.is_type::<T>() {
            unsafe { Some(&*(self as *const dyn Widget<'w,E> as *const T)) }
        } else {
            None
        }
    }
    /// downcast the current widget to a specific implementation
    pub fn downcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Widget<'d,E>+Statize<E>, 'w: 's, 'w: 'd, 'd: 's {
        if let Some(v) = Self::_downcast_ref::<T>(self) {
            Some(v)
        }else if let Some(senf) = self.inner() {
            senf.downcast_ref::<T>()
        }else{
            None
        }
    }
}
impl<'w,E> dyn WidgetMut<'w,E> where E: Env {
    pub fn is_type<'s,T>(&self) -> bool where T: WidgetMut<'s,E>+Statize<E> {
        self.typeid() == T::_typeid()
    }
    
    pub fn _downcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: WidgetMut<'d,E>+Statize<E>, 'w: 's, 'w: 'd, 'd: 's {
        if self.is_type::<T>() {
            unsafe { Some(&mut *(self as *mut dyn WidgetMut<'w,E> as *mut T)) }
        } else {
            None
        }
    }
    /// downcast the current widget to a specific implementation
    pub fn downcast_mut<'s,'d,T>(&'s mut self) -> Option<&'s mut T> where T: WidgetMut<'d,E>+Statize<E>, 'w: 's, 'w: 'd, 'd: 's {
        if self.is_type::<T>() {
            self._downcast_mut::<T>()
        }else if let Some(senf) = self.inner_mut() {
            senf.downcast_mut::<T>()
        }else{
            None
        }
    }
}

unsafe impl<'w,E> Statize<E> for dyn Widget<'w,E> where E: Env {
    type Statur = dyn Widget<'static,E>;
}
unsafe impl<'w,E> Statize<E> for dyn WidgetMut<'w,E> where E: Env {
    type Statur = dyn WidgetMut<'static,E>;
}

unsafe impl<'l,'s,E> Statize<E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    type Statur = &'static dyn Widget<'static,E>;
}
unsafe impl<'l,'s,E> Statize<E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    type Statur = &'static mut dyn WidgetMut<'static,E>;
}

unsafe impl<'w,E> Statize<E> for WidgetRef<'w,E> where E: Env {
    type Statur = WidgetRef<'static,E>;
}
unsafe impl<'w,E> Statize<E> for WidgetRefMut<'w,E> where E: Env {
    type Statur = WidgetRefMut<'static,E>;
}

/*pub trait ProtectedSelf<T> where T: ?Sized {

}

impl<T> ProtectedSelf<Self> for T where T: ?Sized {

}*/

/*macro_rules! impl_statize_static {
    ($t:ident $($tt:ident)+) => {
        impl_statize_static!($t);
        impl_statize_static!($($tt)*);
    };
    ($t:ident) => {
        impl<E> Statize<E> for $t where E: Env {
            type Statur = Self;
        }
    }
}*/

//impl_statize_static!(bool char f32 f64 i8 i16 i32 i64);