use crate::aliases::{EStyle, ESize, ERenderer, EEvent};
use crate::ctx::Context;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::newpath::{PathStackDyn, PathStack, PathResolvusDyn, SimpleId, PathFragment, PathResolvus};
use crate::render::widgets::RenderStdWidgets;
use crate::root::RootRef;
use crate::state::CtxStdState;
use crate::traitcast::WQuery;
use crate::util::bounds::Dims;
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget_decl::WidgetDecl;
use crate::widget_decl::mutor_trait::MutorEndBuilder;
use crate::widget_decl::route::UpdateRoute;

use super::Trigger;
use super::widget::ButtonChild;

pub struct Button<E,Text,Tr,TrIm,TrMut> where
    E: Env,
{
    pub(super) trigger: Tr,
    pub(super) trigger_im: TrIm,
    pub(super) trigger_mut: TrMut,
    pub(super) size: Option<ESize<E>>,
    pub(super) style: Option<EStyle<E>>,
    pub(super) locked: bool,
    //pressed: Option<EEKey<E>>,
    pub(super) text: Text,
}

impl<E,Text,Tr,TrIm,TrMut> WidgetDecl<E> for Button<E,Text,Tr,TrIm,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    Text: WidgetDecl<E>,
    Tr: Trigger<E> + Clone + 'static,
    TrIm: Trigger<E>,
    TrMut: MutorEndBuilder<(),E>,
{
    type Widget = super::widget::Button<E,Text::Widget,Tr>;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn std::any::Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        if args.downcast_ref::<Trigon>().is_none() {return;}

        self.trigger_im.trigger(path._erase(), root, ctx);
        
        if let Some(t) = self.trigger_mut.build_box_mut_event(()) {
            ctx.mutate_closure(t);
        }
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        super::widget::Button {
            id: ctx.retained_id(),
            size: self.size.clone().unwrap_or(ESize::<E>::zero()),
            style: self.style.clone().unwrap_or_default(),
            text: self.text.instantiate(&SimpleId(ButtonChild).push_on_stack(path), root, ctx),
            rendered_dims: None,
            trigger: self.trigger.clone(),
            locked: self.locked,
        }
    }

    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        // if let Some(resolve) = route.resolving() {
        //     if let Some(r2) = resolve.try_fragment::<SimpleId<ButtonChild>>() {
        //         return self.text.update(&mut w.text, &r2.push_on_stack(path), route.for_child_1(), root, ctx);
        //     }
        //     return Invalidation::new();
        // }

        let mut vali = Invalidation::valid();

        w.trigger = self.trigger.clone();

        if self.size.as_ref().map_or(false, |v| v != &w.size ) {
            w.size = self.size.clone().unwrap();
            w.rendered_dims = None; vali = vali.relayout();
        }

        vali |= self.text.update(&mut w.text, &SimpleId(ButtonChild).push_on_stack(path), route.for_child_1(), root, ctx);

        vali
    }

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) where Ph: PathStack<E> + ?Sized {
        let (inner,_) = if let Some(prev_inner) = prev.query_mut::<WQueryButtonRestore>() {
            self.text.update_restore(prev_inner, &SimpleId(ButtonChild).push_on_stack(path), root, ctx)
        } else {
            prev.end(path, root.fork(), ctx);
            (self.text.instantiate(path, root, ctx),Invalidation::new())
        };

        (
            super::widget::Button {
                id: prev.id(),
                size: self.size.clone().unwrap_or(ESize::<E>::zero()),
                style: self.style.clone().unwrap_or_default(),
                text: inner,
                rendered_dims: None,
                trigger: self.trigger.clone(),
                locked: self.locked,
            },
            Invalidation::new()
        )
    }
}

struct Trigon;

pub(super) type send_mutation_trigger_ty<E> = fn(&(dyn PathStackDyn<E>+'_),<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>);

pub(super) fn send_mutation_trigger<E>(path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) where E: Env {
    ctx.queue_send_mutation(path.to_resolvus(), Box::new(Trigon));
}

pub struct WQueryButtonRestore;

impl<E> WQuery<E> for WQueryButtonRestore where E: Env {
    type Result<'a> = &'a mut dyn WidgetDyn<E>;
}
