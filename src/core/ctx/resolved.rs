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
        (**self).render(c.link(self.clone()),r)
    }
    #[inline]
    pub fn event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) {
        (**self).event(c.link(self.clone()),e)
    }
    #[inline]
    pub fn size(&self, c: &mut E::Context) -> ESize<E> {
        (**self).size(c.link(self.clone()))
    }
    #[inline]
    pub fn link(&self, c: &'a mut E::Context) -> Link<'a,E> {
        c.link(self.clone())
    }
    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        (**self).child_paths(self.path)
    }

    pub fn with_env<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self) -> Resolved<'a,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F>, E::Storage: Widgets<F> {
        let stor = self.stor.with_env::<F>();
        let path = self.path.with_env::<F>();
        stor.widget(path).unwrap()
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

impl<'a,E> Clone for Resolved<'a,E> where E: Env {
    fn clone(&self) -> Self {
        self.stor.widget(self.path).unwrap()
    }
}