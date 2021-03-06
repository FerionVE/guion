//! [`Widget`] reference including it's [path](Env::WidgetPath) and a reference to the [root](Env::Storage)
use super::*;
use std::ops::{DerefMut, Deref};

/// A reference to a resolved [`Widget`]
pub struct Resolved<'a,E> where E: Env {
    pub wref: WidgetRef<'a,E>,
    pub path: E::WidgetPath,
    pub direct_path: E::WidgetPath,
    pub stor: &'a E::Storage,
}
/// A mutable reference to a resolved [`Widget`][WidgetMut]
pub struct ResolvedMut<'a,E> where E: Env {
    pub wref: WidgetRefMut<'a,E>,
    pub path: E::WidgetPath,
    pub direct_path: E::WidgetPath,
}

impl<'a,E> Resolved<'a,E> where E: Env {
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::render`]
    #[inline]
    pub fn render(&self, c: &mut E::Context, r: &mut RenderLink<E>) {
        c.render(self.clone(),r)
    }
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::event`](Link::event_direct)
    #[inline]
    pub fn event_direct(&self, c: &mut E::Context, e: &EventCompound<E>) -> EventResp {
        c.event_direct(self.clone(),e)
    }
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::event`](Link::send_event)
    #[inline]
    pub fn send_event(&self, c: &mut E::Context, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,E::Error> {
        c.send_event(self.clone(),e,child)
    }
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::size`]
    #[inline]
    pub fn size(&self, c: &mut E::Context, e: &EStyle<E>) -> ESize<E> {
        c.size(self.clone(),e)
    }

    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _render(&self, c: &mut E::Context, r: &mut RenderLink<E>) {
        (**self)._render(c.link(self.clone()),r)
    }
    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _event_direct(&self, c: &mut E::Context, e: &EventCompound<E>) -> EventResp {
        (**self)._event_direct(c.link(self.clone()),e)
    }
    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _size(&self, c: &mut E::Context, e: &EStyle<E>) -> ESize<E> {
        (**self)._size(c.link(self.clone()),e)
    }
    #[inline]
    pub fn link(&self, c: &'a mut E::Context) -> Link<'a,E> {
        c.link(self.clone())
    }

    #[inline]
    pub fn trace_bounds(&mut self, c: &mut E::Context, root_bounds: &Bounds, e: &EStyle<E>, force: bool) -> Bounds {
        self.stor.trace_bounds(c,self.path.refc(),root_bounds,e,force).unwrap()
    }

    #[inline]
    pub fn reference(&self) -> Resolved<E> {
        Resolved{
            wref: self.wref.reference(),
            path: self.path.clone(),
            direct_path: self.direct_path.clone(),
            stor: &self.stor,
        }
    }

    #[inline]
    pub fn ident(&self) -> WidgetIdent<E> {
        WidgetIdent{
            id: self.id().clone(),
            path: self.path.refc(),
        }
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

    #[inline]
    pub fn with_env<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self) -> Resolved<'a,F> where E::WidgetPath: WidgetPath<F>, E::Storage: Widgets<F> {
        let stor = self.stor.with_env::<F>();
        let path = rc_path_with_env::<E,F>(self.path.refc());
        stor.widget(path).unwrap()
    }
}

impl<'a,E> Deref for Resolved<'a,E> where E: Env {
    type Target = WidgetRef<'a,E>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.wref
    }
}

impl<'a,E> ResolvedMut<'a,E> where E: Env {
    #[inline]
    pub fn widget(&mut self) -> &mut (dyn WidgetMut<E>+'_) {
        &mut *self.wref
    }
}

impl<'a,E> Deref for ResolvedMut<'a,E> where E: Env {
    type Target = WidgetRefMut<'a,E>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.wref
    }
}
impl<'a,E> DerefMut for ResolvedMut<'a,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wref
    }
}

impl<'a,E> Clone for Resolved<'a,E> where E: Env {
    #[inline]
    fn clone(&self) -> Self {
        let mut s = self.stor.widget(self.direct_path.refc()).unwrap();
        s.path = self.path.refc();
        s
    }
}
