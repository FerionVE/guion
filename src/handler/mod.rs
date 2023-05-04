//! Handlers can be chained and dispatch events and other stuff

use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::traitcast::{WQueryResponder, WQueryResponderGeneric, WQueryGeneric};
use crate::{event_new, EventResp};
use crate::newpath::{PathResolvusDyn, PathStack};
use crate::queron::Queron;
use crate::widget::Widget;

pub mod standard;

//TODO SUPER DIFFICULT support non-'static handlers
pub trait HandlerBuilder<E>: 'static where E: Env {
    type Built: Handler<E>;

    //TODO arc slow
    fn build<Acc>(ctx: &mut E::Context<'_>) -> Self::Built where Acc: HandlerStateResolve<Self,E>;
}

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: 'static where E: Env {
    //TODO move into feature traits
    fn _render<W,Ph,S>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut W::Cache,
        //handler_root: &ECHandlerBuilt<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized;

    fn _event_direct<W,Ph,S,Evt>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized;

    fn _event_root<W,Ph,S,Evt>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized;

    fn _size<W,Ph,S>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized;

    //fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's;

    #[inline]
    fn is_tail(&self) -> bool {
        false
    }

    //TODO separate from WQuery, as this definitely doesn't query the widget behind the handler but the handler itself
    fn respond_query<'a>(&'a self, t: WQueryResponder<'_,'a,E>);

    fn respond_query_generic<'a,Q,G>(&'a self, t: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render<W,Ph,S>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    )
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized
    {
        widget._render(path, stack, renderer, force_render, cache, root, ctx)
    }
    #[inline] 
    fn _event_direct<W,Ph,S,Evt>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized
    {
        widget._event_direct(path, stack, event, route_to_widget, cache, root, ctx)
    }
    #[inline] 
    fn _event_root<W,Ph,S,Evt>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized
    {
        if !event._root_only() {//TODO warn eprint??
            //TODO everything wrong here with event root propagation and tail
            widget._event_direct(path, stack, event, route_to_widget, cache, root, ctx)
            //l.ctx.event_direct(l.widget,e)
        }else{
            false
        }
    }
    #[inline] 
    fn _size<W,Ph,S>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized
    {
        widget._size(path, stack, cache, root, ctx)
    }

    // fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's {
    //     todo!()
    // }
    #[inline]
    fn is_tail(&self) -> bool {
        true
    }

    #[inline]
    fn respond_query<'a>(&'a self, _: WQueryResponder<'_,'a,E>) {}
    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, _: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized {}
}

impl<E> HandlerBuilder<E> for () where E: Env {
    type Built = ();

    fn build<Acc>(_: &mut E::Context<'_>) -> Self::Built where Acc: HandlerStateResolve<Self,E> {}
}

pub trait HandlerStateResolve<Dest,E> where E: Env, Dest: HandlerBuilder<E> + ?Sized + 'static {
    fn resolve_handler_state<'a>(ctx_root: &'a mut E::Context<'_>) -> &'a mut Dest;
}

// impl<E> HandlerStateResolve<<E as Env>::Context<'_>,E> for () where E: Env, Dest: 'static {
//     fn resolve_handler_state<'a>(ctx_root: &'a mut <E as Env>::Context<'_>) -> &'a mut Dest {
//         ctx_root
//     }
// }
