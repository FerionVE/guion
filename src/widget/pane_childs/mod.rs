use std::ops::Range;

use crate::aliases::{ESize, ERenderer};
use crate::env::Env;
use crate::event_new;
use crate::invalidation::Invalidation;
use crate::layout::Orientation;
use crate::newpath::{PathFragment, PathResolvusDyn, PathStack};
use crate::queron::Queron;
use crate::render::StdRenderProps;
use crate::util::bounds::{Bounds, Dims};
use crate::util::tabulate::{TabulateOrigin, TabulateDirection, TabulateResponse};

use super::{Widget, WidgetChildDynResult, WidgetChildDynResultMut, WidgetChildResolveDynResult, WidgetChildResolveDynResultMut};
use super::dyn_tunnel::WidgetDyn;
use super::id::WidgetID;
use super::stack::QueriedCurrentBounds;

pub mod fixed_idx;
pub mod tupled;

pub trait PaneChilds<E>: PaneChildsDyn<E> where E: Env {
    type Caches: Default + Sized + 'static;

    fn render<P,Ph>(
        &mut self,
        path: &Ph,
        render_props: &StdRenderProps<'_,P,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Caches,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;

    fn event<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        bounds: &QueriedCurrentBounds,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized;

    fn constraints<P,Ph>(
        &mut self,
        relayout: Option<Dims>,
        orientation: Orientation,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: usize,
        path: &Ph,
        stack: &P,
        op: TabulateOrigin<E>,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;

    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized;

    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized;

    fn send_mutation<Ph>(
        &mut self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized;

    fn invalidate_recursive(&mut self, vali: Invalidation);
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
    vali: Invalidation,
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

    pub fn invalidate(&mut self, v: Invalidation) {
        self.vali |= v;
        if self.vali.layout {
            self.constraints = None;
        }
    }
}

pub struct ChildWidgetDynResult<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub idx: usize,
    pub child_id: CID,
    pub widget_id: WidgetID,
    pub widget: &'a (dyn WidgetDyn<E>+'a),
}
pub struct ChildWidgetDynResolveResult<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub idx: usize,
    pub child_id: CID,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
    pub widget_id: WidgetID,
    pub widget: &'a (dyn WidgetDyn<E>+'a),
}

pub struct ChildWidgetDynResultMut<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub idx: usize,
    pub child_id: CID,
    pub widget_id: WidgetID,
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
}
pub struct ChildWidgetDynResolveResultMut<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub idx: usize,
    pub child_id: CID,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
    pub widget_id: WidgetID,
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
}

impl<'a,CID,E> From<ChildWidgetDynResult<'a,CID,E>> for WidgetChildDynResult<'a,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResult<'a,CID,E>) -> Self {
        Self {
            idx: v.idx as isize,
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}
impl<'a,CID,E> From<ChildWidgetDynResultMut<'a,CID,E>> for WidgetChildDynResultMut<'a,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResultMut<'a,CID,E>) -> Self {
        Self {
            idx: v.idx as isize,
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}

impl<'a,'b,CID,E> From<ChildWidgetDynResolveResult<'a,'b,CID,E>> for WidgetChildResolveDynResult<'a,'b,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResolveResult<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx as isize,
            widget: v.widget,
            widget_id: v.widget_id,
            sub_path: v.resolvus,
        }
    }
}
impl<'a,'b,CID,E> From<ChildWidgetDynResolveResultMut<'a,'b,CID,E>> for WidgetChildResolveDynResultMut<'a,'b,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: ChildWidgetDynResolveResultMut<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx as isize,
            widget: v.widget,
            widget_id: v.widget_id,
            sub_path: v.resolvus,
        }
    }
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
