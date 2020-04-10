use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
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