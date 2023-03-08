use std::ops::Range;

use crate::aliases::ESize;
use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathFragment, PathResolvusDyn};
use crate::util::bounds::Bounds;

use super::Widget;
use super::dyn_tunnel::WidgetDyn;
use super::id::WidgetID;

pub mod fixed_idx;
pub mod tupled;

pub trait PaneChilds<E>: PaneChildsDyn<E> where E: Env {
    type Caches: Default + Sized + 'static;

    
}

pub trait PaneChildsDyn<E> where E: Env {
    type ChildID: PathFragment<E> + Clone + 'static;

    fn len(&self) -> usize;

    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>>;

    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>>;

    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) );

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) );
}

pub struct PaneChildWidget<W,E> where W: Widget<E>, E: Env {
    pub(crate) widget: W,
    pub(crate) vali: Invalidation,
    pub(crate) constraints: Option<ESize<E>>,
    pub(crate) relative_bounds: Option<Bounds>,
}

impl<W,E> PaneChildWidget<W,E> where W: Widget<E>, E: Env {
    pub fn new(widget: W) -> Self {
        Self {
            widget,
            vali: Invalidation::new(),
            constraints: None,
            relative_bounds: None,
        }
    }

    pub fn from_update_restore<O>(prev: &PaneChildWidget<O,E>, (new,vali): (W,Invalidation)) -> Self where O: Widget<E> {
        Self {
            widget: new,
            vali: prev.vali | vali,
            constraints: prev.constraints.clone(),
            relative_bounds: prev.relative_bounds,
        }
    }

    // pub(crate) fn apply_validation(&mut self, v: Invalidation) {
    //     self.vali |= v;
    //     if self.vali
    // }
}

pub struct ChildWidgetDynResult<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: usize,
}
pub struct ChildWidgetDynResolveResult<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: usize,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
}

pub struct ChildWidgetDynResultMut<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: usize,
}
pub struct ChildWidgetDynResolveResultMut<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: usize,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
}

impl<'a,'b,CID,E> From<ChildWidgetDynResolveResult<'a,'b,CID,E>> for ChildWidgetDynResult<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResolveResult<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
            child_id: v.child_id,
        }
    }
}
impl<'a,'b,CID,E> From<ChildWidgetDynResolveResultMut<'a,'b,CID,E>> for ChildWidgetDynResultMut<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResolveResultMut<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
            child_id: v.child_id,
        }
    }
}
