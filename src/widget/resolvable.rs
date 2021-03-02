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

    #[inline]
    pub fn as_widget(self) -> Result<WidgetRef<'w,E>,E::WidgetPath> {
        match self {
            Self::Widget(w) => Ok(w),
            Self::Path(p) => Err(p),
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
