//! An enum over a widget reference of a path
use super::*;

/// This enum is returned by widget's resolve function
/// A widget or path which can be resolved further to widget
pub enum Resolvable<'w,E> where E: Env {
    Widget(WidgetRef<'w,E>),
    Path(E::WidgetPath),
}

impl<'w,E> Resolvable<'w,E> where E: Env + 'static {
    pub fn from_widget<W>(w: W) -> Self where W: Widget<E>+'w {
        Self::Widget(w.boxed())
    }
    /// resolve further with the subpath if not a path
    /// meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_child(self, sub: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        match self {
            Resolvable::Widget(w) => w.into_resolve(sub),
            Resolvable::Path(p) => Ok(Resolvable::Path(p.attached_path(&sub))),
        }
    }
    /// completely resolve using the storage
    #[inline]
    pub fn resolve_widget<'a>(self, stor: &'a E::Storage) -> Result<WidgetRef<'w,E>,()> where 'a: 'w {
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
    pub fn resolved_by_path(&self, p: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match self {
            Resolvable::Widget(w) => w.resolved_by_path(p),
            Resolvable::Path(w) => 
                p.index(0).unwrap().resolves_to_path(w.refc())
                    .then(|| ResolvesThruResult{ sub_path: p.slice(1..) } ) //TODO this is wrong, as the WidgetID isn't in the WidgetPath, so the current hack relies on the StdPath indeed having the last destination WidgetID. Resolving this requires architecturial modifications, either to enable resolving in this function, which requies the resolve fns to carry &E::Storage for resolving, which is ony possible in the immutable space. Alternatively path needs to somehow carry the last(dest) ID, which doesn't seem to be possible.
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

pub enum ResolvableMut<'w,E> where E: Env {
    Widget(WidgetRefMut<'w,E>),
    Path(E::WidgetPath),
}

impl<'w,E> ResolvableMut<'w,E> where E: Env {
    pub fn from_widget<W>(w: W) -> Self where W: WidgetMut<E>+'w {
        Self::Widget(w.boxed_mut())
    }
    /// unwrap widget
    #[inline]
    pub fn as_widget(self) -> Result<WidgetRefMut<'w,E>,E::WidgetPath> {
        match self {
            ResolvableMut::Widget(w) => Ok(w),
            ResolvableMut::Path(p) => Err(p),
        }
    }
    /// resolve further with the subpath if not a path
    /// meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_child_mut(self, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,()> {
        match self {
            ResolvableMut::Widget(w) => w.into_resolve_mut(i),
            ResolvableMut::Path(p) => Ok(ResolvableMut::Path(p.attached_path(&i))),
        }
    }
    #[deprecated]
    #[inline]
    pub fn resolve_widget<'a>(self, stor: &'a mut E::Storage) -> Result<WidgetRefMut<'w,E>,()> where 'a: 'w {
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
    pub fn resolved_by_path(&self, p: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match self {
            ResolvableMut::Widget(w) => w.resolved_by_path(p),
            ResolvableMut::Path(w) => 
                p.index(0).unwrap().resolves_to_path(w.refc())
                    .then(|| ResolvesThruResult{ sub_path: p.slice(1..) } ) //TODO this is wrong, as the WidgetID isn't in the WidgetPath, so the current hack relies on the StdPath indeed having the last destination WidgetID. Resolving this requires architecturial modifications, either to enable resolving in this function, which requies the resolve fns to carry &E::Storage for resolving, which is ony possible in the immutable space. Alternatively path needs to somehow carry the last(dest) ID, which doesn't seem to be possible.
        }
    }
}
