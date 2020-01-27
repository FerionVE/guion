use std::rc::Rc;
use super::*;

pub enum Resolvable<'a,E> where E: Env {
    Widget(Rc<WidgetRef<'a,E>>),
    Path(EWPRc<E>),
}

impl<'a,E> Resolvable<'a,E> where E: Env {
    #[inline]
    pub fn resolve(self, i: EWPSlice<E>) -> Result<Resolvable<'a,E>,()> {
        match self {
            Resolvable::Widget(w) => w.resolve(i),
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