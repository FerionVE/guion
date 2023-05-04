use std::marker::PhantomData;

use crate::aliases::{EStyle, ESize, ERenderer, EEvent};
use crate::ctx::Context;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::invalidation::Invalidation;
use crate::layout::{Gonstraints, Orientation};
use crate::newpath::{PathStackDyn, PathStack, PathResolvusDyn, SimpleId, PathFragment, PathResolvus};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::render::widgets::RenderStdWidgets;
use crate::root::RootRef;
use crate::state::CtxStdState;
use crate::traitcast::WQuery;
use crate::util::bounds::Dims;
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::pane_childs::PaneChildsDyn;
use crate::widget_decl::WidgetDecl;
use crate::widget_decl::mutor_trait::MutorEndBuilder;
use crate::widget_decl::pane_childs::PaneChildsDecl;
use crate::widget_decl::route::UpdateRoute;

pub struct Pane<E,T> where
    E: Env,
{
    pub(super) style: Option<EStyle<E>>,
    pub(super) childs: T,
    pub(super) orientation: Orientation,
}

impl<E,T> WidgetDecl<E> for Pane<E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: PaneChildsDecl<E>,
{
    type Widget = super::widget::Pane<E,T::Retained>;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.childs.send_mutation(path, resolve, args, root, ctx)
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        super::widget::Pane {
            id: ctx.retained_id(),
            childs: self.childs.instantiate(path, root, ctx),
            orientation: self.orientation,
            style: self.style.clone().unwrap_or_default(),
            layouted_dims: None,
            layouted_constraints: None,
            rerender_childs: true,
            rerender_full: true,
        }
    }

    fn update(
        &self,
        w: &mut Self::Widget,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation {
        let mut vali = Invalidation::valid();

        if self.orientation != w.orientation {
            w.orientation = self.orientation;
            w.rerender_childs = true; w.rerender_full = true; w.layouted_dims = None; vali = vali.relayout();
        }

        vali |= self.childs.update(&mut w.childs, path, route, root, ctx);

        if vali.render {
            w.rerender_childs = true;
        }
        if vali.layout {
            w.rerender_full = true; w.layouted_constraints = None; w.layouted_dims = None;
        }

        vali
    }

    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        let (inner,_) = if let Some(prev_inner) = prev.query_mut::<WQueryPaneRestore<<T::Retained as PaneChildsDyn<E>>::ChildID>>() {
            self.childs.update_restore(prev_inner, path, root, ctx)
        } else {
            prev.end(path, root.fork(), ctx);
            (self.childs.instantiate(path, root, ctx),Invalidation::new())
        };

        (
            super::widget::Pane {
                id: prev.id(),
                childs: inner,
                orientation: self.orientation,
                style: self.style.clone().unwrap_or_default(),
                layouted_dims: None,
                layouted_constraints: None,
                rerender_childs: true,
                rerender_full: true,
            },
            Invalidation::new()
        )
    }
}

pub struct WQueryPaneRestore<CID>(PhantomData<CID>) where CID: 'static;

impl<E,CID> WQuery<E> for WQueryPaneRestore<CID> where E: Env, CID: 'static {
    type Result<'a> = &'a mut dyn PaneChildsDyn<E,ChildID=CID>;
}
