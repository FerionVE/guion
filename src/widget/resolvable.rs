//! An enum over a [`Widget`] reference or a [`Path`](WidgetPath)
use super::*;

/// This enum is returned by widget's resolve function
/// 
/// A widget or path which can be resolved further to widget
pub enum Resolvable<'w,E> where E: Env {
    Widget(WidgetRef<'w,E>),
    Path(E::WidgetPath),
}

impl<'w,E> Resolvable<'w,E> where E: Env + 'static {
    pub fn from_widget<W>(w: W) -> Self where W: Widget<E>+'w {
        Self::Widget(w.boxed())
    }
    /// Resolve further with the subpath if not a path
    /// 
    /// Meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_child(self, sub: E::WidgetPath) -> Result<Resolvable<'w,E>,GuionError<E>> {
        match self {
            Self::Widget(w) => w.into_resolve(sub),
            Self::Path(p) => Ok(Self::Path(p.attached_subpath(&sub))),
        }
    }
    /// Completely resolve using the storage
    #[inline]
    pub fn resolve_widget<'a>(self, stor: &'a E::Storage) -> Result<WidgetRef<'w,E>,GuionError<E>> where 'a: 'w {
        match self {
            Self::Widget(w) => Ok(w),
            Self::Path(p) => Ok(stor.widget(p)?.wref),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut E::WidgetPath) {
        if let Self::Path(p) = self {
            *dest = p.refc();
        }
    }
    /// If the path particle would resolve to this widget
    #[deprecated]
    #[inline]
    pub fn resolved_by_path(&self, p: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match self {
            Self::Widget(w) => w.resolved_by_path(p),
            Self::Path(w) => E::WidgetPath::resolves_thru_child_path(w,p) //TODO this is wrong, as the WidgetID isn't in the WidgetPath, so the current hack relies on the StdPath indeed having the last destination WidgetID. Resolving this requires architectural modifications, either to enable resolving in this function, which requires the resolve fns to carry &E::Storage for resolving, which is ony possible in the immutable space. Alternatively path needs to somehow carry the last(dest) ID, which doesn't seem to be possible.
        }
    }
    /// Extend the path representing the parent of this widget to resolve to this widget
    #[deprecated]
    #[inline]
    pub fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        match self {
            Self::Widget(w) => w.in_parent_path(parent),
            Self::Path(w) => w.refc().into(), //TODO WRONG use widget's fns
        }
    }

    pub fn guion_resolve_error_child_info(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        match self {
            Self::Widget(w) => GuionResolveErrorChildInfo {
                child_idx,
                widget_type: w.debugged_type_name(),
                widget_path_if_path: None,
                widget_id: Some(w.id()),
            },
            Self::Path(w) => GuionResolveErrorChildInfo {
                child_idx,
                widget_type: vec![type_name::<E::WidgetPath>()],
                widget_path_if_path: Some(w.clone()),
                widget_id: w._dest_widget(),
            },
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
    /// Unwrap widget
    #[inline]
    pub fn as_widget(self) -> Result<WidgetRefMut<'w,E>,E::WidgetPath> {
        match self {
            Self::Widget(w) => Ok(w),
            Self::Path(p) => Err(p),
        }
    }
    /// Resolve further with the subpath if not a path
    /// 
    /// Meant to be used inside widget's resolve fn
    #[inline]
    pub fn resolve_child_mut(self, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,GuionError<E>> {
        match self {
            Self::Widget(w) => w.into_resolve_mut(i),
            Self::Path(p) => Ok(Self::Path(p.attached_subpath(&i))),
        }
    }
    #[deprecated]
    #[inline]
    pub fn resolve_widget<'a>(self, stor: &'a mut E::Storage) -> Result<WidgetRefMut<'w,E>,GuionError<E>> where 'a: 'w {
        match self {
            Self::Widget(w) => Ok(w),
            Self::Path(p) => Ok(stor.widget_mut(p)?.wref),
        }
    }
    #[inline]
    pub fn extract_path(&self, dest: &mut E::WidgetPath) {
        if let Self::Path(p) = self {
            *dest = p.refc();
        }
    }
    /// is_subpath on the targeted widget
    #[deprecated]
    #[inline]
    pub fn resolved_by_path(&self, p: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match self {
            Self::Widget(w) => w.resolved_by_path(p),
            Self::Path(w) => E::WidgetPath::resolves_thru_child_path(w,p)
        }
    }

    pub fn guion_resolve_error_child_info(&mut self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        match self {
            Self::Widget(w) => GuionResolveErrorChildInfo {
                child_idx,
                widget_type: w.debugged_type_name_mut(),
                widget_path_if_path: None,
                widget_id: Some(w.id()),
            },
            Self::Path(w) => GuionResolveErrorChildInfo {
                child_idx,
                widget_type: vec![type_name::<E::WidgetPath>()],
                widget_path_if_path: Some(w.clone()),
                widget_id: w._dest_widget(),
            },
        }
    }
}
