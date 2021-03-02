//! Macros for implementing traitcast for widgets
use std::any::{TypeId, type_name};

use super::*;

/// Should match the non-stabilized std::raw::TraitObject and represents an erased fat pointer
#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    data: *mut (),
    vtable: *mut (),
}

/// This macro is used inside [`Widget/WidgetMut`](Widget) impls
/// 
/// Example:
/// ```ignore
/// impl_traitcast!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &s.state;
/// );
///
#[macro_export]
macro_rules! impl_traitcast {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        #[inline]
        unsafe fn _as_trait_ref<'impl_traitcast_lt_a>(&'impl_traitcast_lt_a self, t: std::any::TypeId) -> Option<$crate::util::traitcast::TraitObject> {
            $(
                if t == std::any::TypeId::of::<<(dyn $crate::widget::Widget<_>) as $crate::util::traitcast::Traitcast::<$trait,_>>::DestTypeID>() {
                    let $id = self;
                    let senf: Box<$trait> = $access;
                    let senf = std::mem::transmute::<Box<$trait>,$crate::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

/// This macro is used inside [`WidgetMut`](WidgetMut) impls
/// 
/// Example:
/// ```ignore
/// impl_traitcast_mut!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &mut s.state;
/// );
///
#[macro_export]
macro_rules! impl_traitcast_mut {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        #[inline]
        unsafe fn _as_trait_mut<'impl_traitcast_lt_a>(&'impl_traitcast_lt_a mut self, t: std::any::TypeId) -> Option<$crate::util::traitcast::TraitObject> {
            $(
                if t == std::any::TypeId::of::<<(dyn $crate::widget::Widget<_>) as $crate::util::traitcast::Traitcast::<$trait,_>>::DestTypeID>() {
                    let $id = self;
                    let senf: Box<$trait> = $access;
                    let senf = std::mem::transmute::<Box<$trait>,$crate::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

pub trait SafeTraitcase<'s,'w:'s,E> where E: Env {
    fn traitcast_ref<T>(&'s self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized;
    fn try_traitcast_ref<T>(&'s self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized;
    fn traitcast_mut<T>(&'s mut self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized;
    fn try_traitcast_mut<T>(&'s mut self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized;
}

impl<'s,'w:'s,E> SafeTraitcase<'s,'w,E> for dyn Widget<E>+'w where E: Env {
    #[inline]
    fn traitcast_ref<T>(&'s self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        unsafe{Self::_traitcast_ref(self)}
    }
    #[inline]
    fn try_traitcast_ref<T>(&'s self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        unsafe{Self::_try_traitcast_ref(self)}
    }
    #[inline]
    fn traitcast_mut<T>(&'s mut self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        unsafe{Self::_traitcast_mut(self)}
    }
    #[inline]
    fn try_traitcast_mut<T>(&'s mut self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        unsafe{Self::_try_traitcast_mut(self)}
    }
}
impl<'s,'w:'s,E> SafeTraitcase<'s,'w,E> for &'_ (dyn Widget<E>+'w) where E: Env {
    #[inline]
    fn traitcast_ref<T>(&'s self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_ref()
    }
    #[inline]
    fn try_traitcast_ref<T>(&'s self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_ref()
    }
    #[inline]
    fn traitcast_mut<T>(&'s mut self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_ref()
    }
    #[inline]
    fn try_traitcast_mut<T>(&'s mut self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_ref()
    }
}
impl<'s,'w:'s,E> SafeTraitcase<'s,'w,E> for &'_ mut (dyn Widget<E>+'w) where E: Env {
    #[inline]
    fn traitcast_ref<T>(&'s self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_ref()
    }
    #[inline]
    fn try_traitcast_ref<T>(&'s self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_ref()
    }
    #[inline]
    fn traitcast_mut<T>(&'s mut self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_mut()
    }
    #[inline]
    fn try_traitcast_mut<T>(&'s mut self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_mut()
    }
}
impl<'s,'w:'s,E> SafeTraitcase<'s,'w,E> for Box<dyn Widget<E>+'w> where E: Env {
    #[inline]
    fn traitcast_ref<T>(&'s self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_ref()
    }
    #[inline]
    fn try_traitcast_ref<T>(&'s self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_ref()
    }
    #[inline]
    fn traitcast_mut<T>(&'s mut self) -> Result<Box<T>,GuionError<E>> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).traitcast_mut()
    }
    #[inline]
    fn try_traitcast_mut<T>(&'s mut self) -> Result<Box<T>,()> where dyn Widget<E>+'w: Traitcast<'s,T,E>, T: ?Sized {
        (**self).try_traitcast_mut()
    }
}

/// Trait to secure Traitcasting, generally implemented by [macro](traitcast_for)  
/// - Must receive lifetime 'w
/// - Must be implemented onto implemented on `dyn Widget<E>+'w`  
/// - `T` is the destination `Box<dyn Trait+'w>` to which should be traitcasted  
/// - `DestTypeID` must be `dyn Widget+'static`
pub unsafe trait Traitcast<'s,T,E>: Widget<E> where T: ?Sized, E: Env {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _traitcast_ref(senf: &'s dyn Widget<E>) -> Result<Box<T>,GuionError<E>> {
        Self::_try_traitcast_ref(senf)
            .map_err(|_| traitcast_error_info::<E,Self::DestTypeID>(senf,"traitcast") )
    }
    #[inline]
    unsafe fn _try_traitcast_ref(senf: &'s dyn Widget<E>) -> Result<Box<T>,()> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_ref(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,Box<T>>(&v))
        } else if let Some(senf) = senf.inner() {
            Self::_try_traitcast_ref(senf)
        } else {
            Err(())
        }
    }
    #[inline]
    unsafe fn _traitcast_mut(senf: &'s mut dyn Widget<E>) -> Result<Box<T>,GuionError<E>> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let e = traitcast_error_info::<E,Self::DestTypeID>(senf,"traitcast_mut");
        Self::_try_traitcast_mut(senf)
            .map_err(|_| e )
    }
    #[inline]
    unsafe fn _try_traitcast_mut(senf: &'s mut dyn Widget<E>) -> Result<Box<T>,()> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_mut(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,Box<T>>(&v))
        } else if let Some(senf) = Widget::inner_mut(senf) {
            Self::_try_traitcast_mut(senf)
        } else {
            Err(())
        }
    }
}

/// Implement [`Traitcast`] for traits to be traitcasted from [`Widget`]
/// 
/// Syntax:  
/// `traitcast_for!(trait_path;mut_trait_path);`
/// 
/// 
/// Implements for: Widget -> Trait, WidgetMut -> Trait, WidgetMut -> TraitMut
/// 
/// Example:  
/// `traitcast_for!(ICheckBox<E>;ICheckBoxMut<E>);`
#[macro_export]
macro_rules! traitcast_for {
    ($trait:path) => {
        unsafe impl<'z,'s:'z,'w:'s,E> $crate::util::traitcast::Traitcast<'s,dyn $trait+'z,E> for dyn $crate::widget::Widget<E>+'w where E: $crate::env::Env {
            type DestTypeID = dyn $trait+'static;
        }
    };
}

fn traitcast_error_info<E,DestTypeID>(senf: &(dyn Widget<E>+'_), op: &'static str) -> GuionError<E> where E: Env, DestTypeID: ?Sized + 'static {
    GuionError::TraitcastError(Box::new(TraitcastError{
        op,
        src_type: senf.debugged_type_name(),
        dest_trait_type: type_name::<DestTypeID>(),
    }))
}
