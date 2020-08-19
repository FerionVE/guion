//! An enum over a widget reference of a path
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
    pub fn resolve_child(self, sub: E::WidgetPath) -> Result<Resolvable<'a,E>,()> {
        match self {
            Resolvable::Widget(w) => w.into_resolve(sub),
            Resolvable::Path(p) => Ok(Resolvable::Path(p.attached_path(&sub))),
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
    /// if the path particle would resolve to this widget
    #[deprecated]
    #[inline]
    pub fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        match self {
            Resolvable::Widget(w) => w.resolves_by(p),
            Resolvable::Path(w) => p.resolves_to_path(w.refc()), //TODO WRONG use widget's fns
        }
    }
    /// extend the path representing the parent of this widget to resolve to this widget
    #[deprecated]
    #[inline]
    pub fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        match self {
            Resolvable::Widget(w) => w.in_parent_path(parent),
            Resolvable::Path(w) => w.refc().into(), //TODO WRONG use widget's fns
        }
    }
}

pub enum ResolvableMut<'a,E> where E: Env {
    Widget(WidgetRefMut<'a,E>),
    Path(E::WidgetPath),
}

impl<'a,E> ResolvableMut<'a,E> where E: Env {
    /// unwrap widget
    #[inline]
    pub fn as_widget(self) -> Result<WidgetRefMut<'a,E>,E::WidgetPath> {
        match self {
            ResolvableMut::Widget(w) => Ok(w),
            ResolvableMut::Path(p) => Err(p),
        }
    }
    /// resolve further with the subpath if not a path
    /// meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_child_mut(self, i: E::WidgetPath) -> Result<ResolvableMut<'a,E>,()> {
        match self {
            ResolvableMut::Widget(w) => w.into_resolve_mut(i),
            ResolvableMut::Path(p) => Ok(ResolvableMut::Path(p.attached_path(&i))),
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
    pub fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        match self {
            ResolvableMut::Widget(w) => w.resolves_by(p),
            ResolvableMut::Path(w) => p.resolves_to_path(w.refc()), //TODO WRONG use widget's fns
        }
    }
}
