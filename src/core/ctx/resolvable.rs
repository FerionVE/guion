use std::rc::Rc;
use super::*;

/// This enum is returned by widget's resolve function
pub enum Resolvable<'a,E> where E: Env {
    Widget(Rc<WidgetRef<'a,E>>),
    Path(EWPRc<E>),
}

impl<'a,E> Resolvable<'a,E> where E: Env {
    #[inline]
    pub fn resolve(self, i: WPSlice<E>) -> Result<Resolvable<'a,E>,()> {
        match self {
            Resolvable::Widget(w) => w.resolve_ref(i),
            Resolvable::Path(p) => Ok(Resolvable::Path(p)),
        }
    }
    #[inline]
    pub fn resolve_widget(self, stor: &'a E::Storage) -> Result<Rc<WidgetRef<'a,E>>,()> {
        match self {
            Resolvable::Widget(w) => Ok(w),
            Resolvable::Path(p) => Ok(stor.widget(p.slice())?.wref),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut EWPRc<E>) {
        if let Resolvable::Path(p) = self {
            *dest = p.refc();
        }
    }
    #[inline]
    pub fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        match self {
            Resolvable::Widget(w) => w.widget().is_subpath(p),
            Resolvable::Path(w) => w.tip() == p,
        }
    }
    #[inline]
    pub fn self_in_parent(&self, parent: WPSlice<E>) -> E::WidgetPath {
        match self {
            Resolvable::Widget(w) => w.widget().self_in_parent(parent),
            Resolvable::Path(w) => w.refc().into(),
        }
    }
}

impl<'a,E> RefClonable for Resolvable<'a,E> where E: Env {
    #[inline]
    fn refc(&self) -> Self {
        match self {
            Resolvable::Widget(w) => Resolvable::Widget(w.refc()),
            Resolvable::Path(p) => Resolvable::Path(p.refc()),
        }
    }
}

pub enum ResolvableMut<'a,E> where E: Env {
    Widget(WidgetRefMut<'a,E>),
    Path(EWPRc<E>),
}

impl<'a,E> ResolvableMut<'a,E> where E: Env {
    pub fn as_widget(self) -> Result<WidgetRefMut<'a,E>,EWPRc<E>> {
        match self {
            ResolvableMut::Widget(w) => Ok(w),
            ResolvableMut::Path(p) => Err(p),
        }
    }

    #[inline]
    pub fn resolve_mut(self, i: WPSlice<E>, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> {
        match self {
            ResolvableMut::Widget(w) => w.resolve_box(i,invalidate),
            ResolvableMut::Path(p) => Ok(ResolvableMut::Path(p)),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut EWPRc<E>) {
        if let ResolvableMut::Path(p) = self {
            *dest = p.refc();
        }
    }
    #[inline]
    pub fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        match self {
            ResolvableMut::Widget(w) => w.widget().is_subpath(p),
            ResolvableMut::Path(w) => w.tip() == p,
        }
    }
}