use super::*;

//TODO simplify WDC and downcast impls into AnyLt struct
pub trait WDC<E> {
    type Statur: ?Sized + 'static;
    
    fn _typeid() -> TypeId {
        TypeId::of::<Self::Statur>()
    }
}
pub trait WDCSized<E>: Sized {
    type StaturSized: Sized + 'static;
}

impl<'w,E> dyn Widget<'w,E> where E: Env {
    pub fn is_type<'s,'d,T>(&'s self) -> bool where T: Widget<'d,E>+WDC<E>+'d, 'w: 's, 'w: 'd, 'd: 's {
        self.typeid() == T::_typeid()
    }

    pub fn w_downcast_ref<'s,'d,T>(&'s self) -> Option<&'s T> where T: Widget<'d,E>+WDC<E>+'d, 'w: 's, 'w: 'd, 'd: 's {
        if self.is_type::<T>() {
            unsafe { Some(&*(self as *const dyn Widget<'w,E> as *const T)) }
        } else {
            None
        }
    }
}
impl<'w,E> dyn WidgetMut<'w,E> where E: Env {
    pub fn is_type<'s,T>(&self) -> bool where T: WidgetMut<'s,E>+WDC<E> {
        self.typeid() == T::_typeid()
    }
    
    pub fn w_downcast_mut<'s,T>(&'s mut self) -> Option<&'s mut T> where T: WidgetMut<'s,E>+WDC<E>, 'w: 's {
        if self.is_type::<T>() {
            unsafe { Some(&mut *(self as *mut dyn WidgetMut<'w,E> as *mut T)) }
        } else {
            None
        }
    }
}

impl<'w,E> WDC<E> for dyn Widget<'w,E> where E: Env {
    type Statur = dyn Widget<'static,E>;
}
impl<'w,E> WDC<E> for dyn WidgetMut<'w,E> where E: Env {
    type Statur = dyn WidgetMut<'static,E>;
}

/*impl<'s,T,E> WDC<E> for &'s T where T: WDC<E>, E: Env {
    type Statur = &'static T::Statur;
}
impl<'s,T,E> WDC<E> for &'s mut T where T: WDC<E>, E: Env {
    type Statur = &'static mut T::Statur;
}
impl<T,E> WDC<E> for Box<T> where T: WDC<E>+?Sized, E: Env {
    type Statur = Box<T::Statur>;
}

impl<'s,T,E> WDCSized<E> for &'s T where T: WDC<E>, E: Env {
    type StaturSized = &'static T::Statur;
}
impl<'s,T,E> WDCSized<E> for &'s mut T where T: WDC<E>, E: Env {
    type StaturSized = &'static mut T::Statur;
}
impl<T,E> WDCSized<E> for Box<T> where T: WDC<E>+?Sized, E: Env {
    type StaturSized = Box<T::Statur>;
}*/

impl<'l,'s,E> WDC<E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    type Statur = &'static dyn Widget<'static,E>;
}
impl<'l,'s,E> WDC<E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    type Statur = &'static mut dyn WidgetMut<'static,E>;
}

impl<'l,'s,E> WDCSized<E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    type StaturSized = &'static dyn Widget<'static,E>;
}
impl<'l,'s,E> WDCSized<E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    type StaturSized = &'static mut dyn WidgetMut<'static,E>;
}

impl<'w,E> WDC<E> for Box<dyn Widget<'w,E>> where E: Env {
    type Statur = Box<dyn Widget<'static,E>>;
}
impl<'w,E> WDCSized<E> for Box<dyn Widget<'w,E>> where E: Env {
    type StaturSized = Box<dyn Widget<'static,E>>;
}

impl<'w,E> WDC<E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    type Statur = Box<dyn WidgetMut<'static,E>>;
}
impl<'w,E> WDCSized<E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    type StaturSized = Box<dyn WidgetMut<'static,E>>;
}



/*impl<T,E> WDC<E> for T where T: 'static, E: Env {
    type Statur = Self;
}*/

/*impl<'s,'l,T,E> WDC<E> for &'s T where T: Widget<'l,E>+WDC<E>+'l + ?Sized, E: Env, 'l: 's {
    type Statur = &'static T::Statur;
}
impl<'s,'l,T,E> WDC<E> for &'s mut T where T: Widget<'l,E>+WDC<E>+'l + ?Sized, E: Env, 'l: 's {
    type Statur = &'static mut T::Statur;
}
impl<'s,T,E> WDC<E> for Box<T> where T: Widget<'s,E>+WDC<E> + ?Sized, E: Env {
    type Statur = Box<T::Statur>;
}*/