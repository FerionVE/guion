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
                    let senf: &'impl_traitcast_lt_a $trait = $access;
                    let senf = std::mem::transmute::<&'impl_traitcast_lt_a $trait,$crate::util::traitcast::TraitObject>(senf);
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
                if t == std::any::TypeId::of::<<(dyn $crate::widget::WidgetMut<_>) as $crate::util::traitcast::TraitcastMut::<$trait,_>>::DestTypeID>() {
                    let $id = self;
                    let senf: &'impl_traitcast_lt_a mut $trait = $access;
                    let senf = std::mem::transmute::<&'impl_traitcast_lt_a mut $trait,$crate::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

impl<E> dyn Widget<E>+'_ where E: Env {
    #[inline]
    pub fn traitcast_ref<'s,T>(&'s self) -> Result<&'s T,E::Error> where Self: Traitcast<T,E>, T: ?Sized {
        unsafe{Self::_traitcast_ref(self)}
    }
    #[inline]
    pub fn try_traitcast_ref<'s,T>(&'s self) -> Result<&'s T,()> where Self: Traitcast<T,E>, T: ?Sized {
        unsafe{Self::_try_traitcast_ref(self)}
    }
}
impl<E> dyn WidgetMut<E>+'_ where E: Env {
    #[inline]
    pub fn traitcast_ref<'s,T>(&'s self) -> Result<&'s T,E::Error> where Self: Traitcast<T,E>, T: ?Sized {
        unsafe{Self::_traitcast_ref(self.erase())}
    }
    #[inline]
    pub fn try_traitcast_ref<'s,T>(&'s self) -> Result<&'s T,()> where Self: Traitcast<T,E>, T: ?Sized {
        unsafe{Self::_try_traitcast_ref(self.erase())}
    }
    #[inline]
    pub fn traitcast_mut<'s,T>(&'s mut self) -> Result<&'s mut T,E::Error> where Self: TraitcastMut<T,E>, T: ?Sized {
        unsafe{Self::_traitcast_mut(self)}
    }
    #[inline]
    pub fn try_traitcast_mut<'s,T>(&'s mut self) -> Result<&'s mut T,()> where Self: TraitcastMut<T,E>, T: ?Sized {
        unsafe{Self::_try_traitcast_mut(self)}
    }
}

/// Trait to secure Traitcasting, generally implemented by [macro](traitcast_for)  
/// - Always implemented on `dyn Widget<E>`
/// - `T` is the destination `dyn Trait` to which should be traitcasted
/// - `DestTypeID` must be the same type as `T`, but with 'static lifetimes. Used to retrieve TypeID
pub unsafe trait Traitcast<T,E>: Widget<E> where T: ?Sized, E: Env {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _traitcast_ref<'s>(senf: &'s dyn Widget<E>) -> Result<&'s T,E::Error> {
        Self::_try_traitcast_ref(senf)
            .map_err(|_| traitcast_error_info::<E,Self::DestTypeID>(senf,"traitcast") )
    }
    #[inline]
    unsafe fn _try_traitcast_ref<'s>(senf: &'s dyn Widget<E>) -> Result<&'s T,()> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_ref(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s T>(&v))
        } else if let Some(senf) = senf.inner() {
            Self::_try_traitcast_ref(senf)
        } else {
            Err(())
        }
    }
}

/// Trait to secure Traitcasting, generally implemented by [macro](traitcast_for)  
/// - Always implemented on `dyn WidgetMut<E>`
/// - `T` is the destination `dyn Trait` to which should be traitcasted
/// - `DestTypeID` must be the same type as `T`, but with 'static lifetimes. Used to retrieve TypeID
pub unsafe trait TraitcastMut<T,E>: WidgetMut<E> where T: ?Sized, E: Env {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _traitcast_mut<'s>(senf: &'s mut dyn WidgetMut<E>) -> Result<&'s mut T,E::Error> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let e = traitcast_error_info_mut::<E,Self::DestTypeID>(senf,"traitcast_mut");
        Self::_try_traitcast_mut(senf)
            .map_err(|_| e )
    }
    #[inline]
    unsafe fn _try_traitcast_mut<'s>(senf: &'s mut dyn WidgetMut<E>) -> Result<&'s mut T,()> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_mut(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s mut T>(&v))
        } else if let Some(senf) = senf.inner_mut() {
            Self::_try_traitcast_mut(senf)
        } else {
            Err(())
        }
    }
}

/// Syntax:  
/// `traitcast_for_immu!([<...>] Trait[<...>] [where ...]);`  
#[macro_export]
macro_rules! traitcast_for_immu {
    (
        $( < $($args:ident),* $(,)* > )?
        $trait:path
        $(where $($preds:tt)+)?
    ) => {
        unsafe impl<'w,E,$( $($args),* )?> $crate::util::traitcast::Traitcast<dyn $trait+'w,E> for dyn $crate::widget::Widget<E>+'w where E: $crate::env::Env, $( $($preds)+ )? {
            type DestTypeID = dyn $trait+'static;
        }
    }
}

/// Syntax:  
/// `traitcast_for_mut!([<...>] Trait[<...>] [where ...]);`  
#[macro_export]
macro_rules! traitcast_for_mut {
    (
        $( < $($args:ident),* $(,)* > )?
        $trait:path
        $(where $($preds:tt)+)?
    ) => {
        unsafe impl<'w,E,$( $($args),* )?> $crate::util::traitcast::TraitcastMut<dyn $trait+'w,E> for dyn $crate::widget::WidgetMut<E>+'w where E: $crate::env::Env, $( $($preds)+ )? {
            type DestTypeID = dyn $trait+'static;
        }
    }
}

/// Implement [`Traitcast`] and [`TraitcastMut`] for traits to be traitcasted from [`Widget`]
/// 
/// Syntax:  
/// `traitcast_for!([<...>] Trait[<...>] [;Trait[<...>]] [where ...]);`  
/// `traitcast_for!(trait_path; mut_trait_path);` implements Traitcast & TraitcastMut for trait_path, and only TraitcastMut for mut_trait_path  
/// `traitcast_for!(trait_path);` implements Traitcast and TraitcastMut for trait_path
/// 
/// Example:  
/// `traitcast_for!(ICheckBox<E>;ICheckBoxMut<E>);`  
/// `traitcast_for!(<T> NuclearGet<T> where T: Clone);`
#[macro_export]
macro_rules! traitcast_for {
    (
        $( < $($args:ident),* $(,)* > )?
        $trait_a:path
        ;
        $trait_b:path
        $(where $($preds:tt)+)?
    ) => {
        $crate::traitcast_for_immu!( $(<$($args),*>)? $trait_a $(where $($preds)+)? );
        $crate::traitcast_for_mut!( $(<$($args),*>)? $trait_a $(where $($preds)+)? );
        $crate::traitcast_for_mut!( $(<$($args),*>)? $trait_b $(where $($preds)+)? );
    };
    (
        $( < $($args:ident),* $(,)* > )?
        $trait:path
        $(where $($preds:tt)+)?
    ) => {
        $crate::traitcast_for_immu!( $(<$($args),*>)? $trait $(where $($preds)+)? );
        $crate::traitcast_for_mut!( $(<$($args),*>)? $trait $(where $($preds)+)? );
    };
}

fn traitcast_error_info<E,DestTypeID>(senf: &(dyn Widget<E>+'_), op: &'static str) -> E::Error where E: Env, DestTypeID: ?Sized + 'static {
    GuionError::TraitcastError(Box::new(TraitcastError{
        op,
        src_type: senf.debugged_type_name(),
        dest_trait_type: type_name::<DestTypeID>(),
    })).into()
}
fn traitcast_error_info_mut<E,DestTypeID>(senf: &mut (dyn WidgetMut<E>+'_), op: &'static str) -> E::Error where E: Env, DestTypeID: ?Sized + 'static {
    GuionError::TraitcastError(Box::new(TraitcastError{
        op,
        src_type: senf.debugged_type_name_mut(),
        dest_trait_type: type_name::<DestTypeID>(),
    })).into()
}
