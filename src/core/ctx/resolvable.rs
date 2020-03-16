use std::rc::Rc;
use super::*;

/// This enum is returned by widget's resolve function
pub enum Resolvable<'a,E> where E: Env {
    Widget(WidgetRef<'a,E>),
    Path(E::WidgetPath),
}

impl<'a,E> Resolvable<'a,E> where E: Env {
    /// resolve further with the subpath if not a path
    /// meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve(self, sub: E::WidgetPath) -> Result<Resolvable<'a,E>,()> {
        match self {
            Resolvable::Widget(w) => w.resolve_box(sub),
            Resolvable::Path(p) => Ok(Resolvable::Path(p)),
        }
    }
    /// completely resolve using the storage
    #[inline]
    pub fn resolve_widget(self, stor: &'a E::Storage) -> Result<WidgetRef<'a,E>,()> {
        match self {
            Resolvable::Widget(w) => Ok(w),
            Resolvable::Path(p) => Ok(stor.widget(p)?.wref),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut E::WidgetPath) {
        if let Resolvable::Path(p) = self {
            *dest = p.refc();
        }
    }
    /// is_subpath on the targeted widget
    #[deprecated]
    #[inline]
    pub fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        match self {
            Resolvable::Widget(w) => w.is_subpath(p),
            Resolvable::Path(w) => w.tip() == p, //TODO WRONG use widget's fns
        }
    }
    /// self_in_parent on the targeted widget
    #[deprecated]
    #[inline]
    pub fn self_in_parent(&self, parent: E::WidgetPath) -> E::WidgetPath {
        match self {
            Resolvable::Widget(w) => w.self_in_parent(parent),
            Resolvable::Path(w) => w.refc().into(), //TODO WRONG use widget's fns
        }
    }
}

/*impl<'a,E> RefClonable for Resolvable<'a,E> where E: Env {
    #[inline]
    fn refc(&self) -> Self {
        match self {
            Resolvable::Widget(w) => Resolvable::Widget(w.refc()),
            Resolvable::Path(p) => Resolvable::Path(p.refc()),
        }
    }
}*/

pub enum ResolvableMut<'a,E> where E: Env {
    Widget(WidgetRefMut<'a,E>),
    Path(E::WidgetPath),
}

impl<'a,E> ResolvableMut<'a,E> where E: Env {
    /// unwrap widget
    pub fn as_widget(self) -> Result<WidgetRefMut<'a,E>,E::WidgetPath> {
        match self {
            ResolvableMut::Widget(w) => Ok(w),
            ResolvableMut::Path(p) => Err(p),
        }
    }
    /// resolve further with the subpath if not a path
    /// meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_mut(self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> {
        match self {
            ResolvableMut::Widget(w) => w.resolve_box_mut(i,invalidate),
            ResolvableMut::Path(p) => Ok(ResolvableMut::Path(p)),
        }
    }
    #[deprecated]
    #[inline]
    pub fn resolve_widget(self, stor: &'a mut E::Storage) -> Result<WidgetRefMut<'a,E>,()> {
        match self {
            ResolvableMut::Widget(w) => Ok(w),
            ResolvableMut::Path(p) => Ok(stor.widget_mut(p)?.wref),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut E::WidgetPath) {
        if let ResolvableMut::Path(p) = self {
            *dest = p.refc();
        }
    }
    /// is_subpath on the targeted widget
    #[deprecated]
    #[inline]
    pub fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        match self {
            ResolvableMut::Widget(w) => w.is_subpath(p),
            ResolvableMut::Path(w) => w.tip() == p, //TODO WRONG use widget's fns
        }
    }
}

pub fn short_resolvable<'l: 's,'s,E: Env>(i: Resolvable<'l,E>) -> Resolvable<'s,E> {
    match i {
        Resolvable::Widget(w) => Resolvable::Widget(short_wref(w)),
        Resolvable::Path(p) => Resolvable::Path(p),
    }
}
pub fn short_resolvable_vec<'l: 's,'s,E: Env>(i: Vec<Resolvable<'l,E>>) -> Vec<Resolvable<'s,E>> {
    unsafe{
        std::mem::transmute::<Vec<Resolvable<'l,E>>,Vec<Resolvable<'s,E>>>(i) //roast me
    }
}