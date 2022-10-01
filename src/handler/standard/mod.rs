//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state
use crate::*;
use crate::event_new::variants::StdVariant;
use crate::queron::Queron;
use crate::root::RootRef;
use crate::widget::stack::{WithCurrentBounds, WithCurrentWidget};
use std::marker::PhantomData;
use std::sync::Arc;
use state::standard::StdStdState;

pub mod imp;
pub mod imps;

pub struct StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub state: StdStdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            state: StdStdState::new(),
            _c: PhantomData,
        }
    }
}

impl<SB,E> StdHandlerLive<SB,E> where SB: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn unfocus<W,S>(&self, root_widget: &W, stack: &S, ts: u64, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> EventResp where W: Widget<E> + ?Sized, S: Queron<E> + ?Sized {
        if let Some(widget) = (self.access)(ctx).state.kbd.focused.take() {
            let event = StdVariant {
                variant: Unfocus{},
                ts,
                filter_path: Some(widget.refc().path),
                filter_point: None,
                direct_only: false,
                filter_path_strict: true,
            };
            root_widget.event_direct(stack,&event,cache,root,ctx)
        }else{
            false
        }
    }

    pub fn focus<W,S>(&self, root_widget: &W, p: E::WidgetPath, stack: &S, ts: u64, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<EventResp,E::Error> where W: Widget<E> + ?Sized, S: Queron<E> + ?Sized {
        self.unfocus(root_widget,stack,ts,cache,root.fork(),ctx);
        (self.access)(ctx).state.kbd.focused = Some(WidgetIdent::from_path(p.refc(),&root,ctx)?);
        
        let event = StdVariant {
            variant: Focus{},
            ts,
            filter_path: Some(p),
            filter_point: None,
            direct_only: false,
            filter_path_strict: true,
        };
        Ok(root_widget.event_direct(stack,&event,cache,root,ctx))
    }
}

impl<S,E> HandlerBuilder<E> for StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, for<'a> E::Context<'a>: CtxStdState<'a,E>, EEvent<E>: StdVarSup<E> {
    type Built = StdHandlerLive<S,E>;

    fn build(access: Arc<dyn for<'c,'cc> Fn(&'c mut E::Context<'cc>)->&'c mut Self>, ctx: &mut E::Context<'_>) -> Self::Built {
        let f2 = access.clone();
        StdHandlerLive {
            sup: S::build(Arc::new(move |c| &mut f2(c).sup ),ctx),
            access,
            _c: PhantomData,
        }
    }
}

pub struct StdHandlerLive<SB,E> where SB: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: SB::Built,
    pub access: Arc<dyn for<'c,'cc> Fn(&'c mut E::Context<'cc>)->&'c mut StdHandler<SB,E>>,
    _c: PhantomData<E>,
}
