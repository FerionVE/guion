use super::*;

/// should match the non-stabilized std::raw::TraitObject and represents a erased fat pointer
#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

/// This macro is used inside Widget/WidgetMut impls
/// 
/// Example:
/// impl_traitcast!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &s.state;
/// );
#[macro_export]
macro_rules! impl_traitcast {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        unsafe fn _as_trait_ref(&self, t: std::any::TypeId) -> Option<$crate::core::util::traitcast::TraitObject> {
            $(
                if t == <$trait as $crate::core::widget::cast::Statize>::_typeid() {
                    let $id = self;
                    let senf: &$trait = $access;
                    let senf = std::mem::transmute::<&$trait,$crate::core::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

/// This macro is used inside WidgetMut impls
/// 
/// Example:
/// impl_traitcast_mut!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &mut s.state;
/// );
#[macro_export]
macro_rules! impl_traitcast_mut {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        unsafe fn _as_trait_mut(&mut self, t: std::any::TypeId) -> Option<$crate::core::util::traitcast::TraitObject> {
            $(
                if t == <$trait as $crate::core::widget::cast::Statize>::_typeid() {
                    let $id = self;
                    let senf: &mut $trait = $access;
                    let senf = std::mem::transmute::<&mut $trait,$crate::core::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

macro_rules! impl_statize_lte {
    ($trait:ident) => {
        unsafe impl<'w,E> Statize for dyn $trait<'w,E> where E: Env {
            type Statur = dyn $trait<'static,E>;
        }
        unsafe impl<'l,'s,E> Statize for &'s dyn $trait<'l,E> where E: Env, 'l: 's {
            type Statur = &'static dyn $trait<'static,E>;
        }
        unsafe impl<'l,'s,E> Statize for &'s mut dyn $trait<'l,E> where E: Env, 'l: 's {
            type Statur = &'static mut dyn $trait<'static,E>;
        }
    };
}