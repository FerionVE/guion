use std::ops::{Deref,DerefMut};
use super::*;

pub struct Resolved<'a,E> where E: Env {
    pub wref: WidgetRef<'a,E>,
    pub path: WPSlice<'a,E>,
    pub stor: &'a E::Storage,
}
pub struct ResolvedMut<'a,E> where E: Env {
    pub wref: WidgetRefMut<'a,E>,
    pub path: WPSlice<'a,E>,
}

impl<'a,E> Resolved<'a,E> where E: Env {
    #[inline]
    pub fn render(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        unimplemented!()
    }
    #[inline]
    pub fn event(&self, c: &mut E::Context, r: (EEvent<E>,&Bounds)) {
        unimplemented!()
    }
    #[inline]
    pub fn size(&self, c: &mut E::Context) -> Size {
        unimplemented!()
    }
    #[inline]
    pub fn link(&self, c: &'a mut E::Context) -> Link<'a,E> {
        Link{
            ctx: c,
            stor: self.stor,
            path: self.path,
        }
    }
}

impl<'a,E> Deref for Resolved<'a,E> where E: Env {
    type Target = E::DynWidget;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &**self.wref
    }
}
impl<'a,E> Deref for ResolvedMut<'a,E> where E: Env {
    type Target = E::DynWidget;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &**self.wref
    }
}
impl<'a,E> DerefMut for ResolvedMut<'a,E> where E: Env {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut **self.wref
    }
}