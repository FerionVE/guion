use std::rc::Rc;
use std::ops::{Deref,DerefMut};
use super::*;

/// A reference to a resolved Widget
pub struct Resolved<'a,E> where E: Env {
    pub wref: Rc<WidgetRef<'a,E>>,
    pub path: E::WidgetPath,
    pub stor: &'a E::Storage,
}
/// A mutable reference to a resolved Widget
pub struct ResolvedMut<'a,E> where E: Env {
    pub wref: WidgetRefMut<'a,E>,
    pub path: E::WidgetPath,
}

impl<'a,E> Resolved<'a,E> where E: Env {
    #[inline]
    pub fn render(&self, c: &mut E::Context, r: &mut RenderLink<E>) -> bool {
        (**self).render(c.link(self.clone()),r)
    }
    #[inline]
    pub fn event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds,u64)) {
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

    /*#[inline]
    pub fn childs(&self) -> Vec<Resolved<E>> {
        (**self)._childs(self.path)
    }
    #[inline]
    pub fn childs_mut(&self) -> Vec<Resolved<E>> {
        (**self)._childs_mut(self.path)
    }*/
    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        (**self).child_paths(self.path.refc())
    }

    pub fn with_env<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self) -> Resolved<'a,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F>, E::Storage: Widgets<F> {
        let stor = self.stor.with_env::<F>();
        let path = rc_path_with_env::<E,F>(self.path.refc());
        stor.widget(path).unwrap()
    }
}

impl<'a,E> Deref for Resolved<'a,E> where E: Env {
    type Target = E::DynWidget;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.wref.widget().erase()
    }
}
impl<'a,E> Deref for ResolvedMut<'a,E> where E: Env {
    type Target = E::DynWidget;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.wref.widget().erase()
    }
}
impl<'a,E> DerefMut for ResolvedMut<'a,E> where E: Env {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.wref.widget_mut().erase_mut()
    }
}

impl<'a,E> Clone for Resolved<'a,E> where E: Env {
    fn clone(&self) -> Self {
        self.stor.widget(self.path.refc()).unwrap()
    }
}
/// shrink the lifetime
pub fn short_resolved<'l: 's,'s,E: Env>(i: Resolved<'l,E>) -> Resolved<'s,E> {
    Resolved{
        wref: short_wref(i.wref),
        path: i.path,
        stor: i.stor,
    }
}
/// shrink the lifetime
pub fn short_wref<'l: 's,'s,E: Env>(i: Rc<WidgetRef<'l,E>>) -> Rc<WidgetRef<'s,E>> {
    unsafe{
        std::mem::transmute::<Rc<WidgetRef<'l,E>>,Rc<WidgetRef<'s,E>>>(i) //roast me
    }
}