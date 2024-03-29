//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state

use std::marker::PhantomData;
use std::sync::Arc;

use crate::EventResp;
use crate::aliases::EEvent;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::standard::variants::{Unfocus, Focus};
use crate::event_new::variants::StdVariant;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::root::RootRef;
use crate::state::CtxStdState;
use crate::state::standard::StdStdState;
use crate::widget::Widget;

use super::{HandlerBuilder, HandlerStateResolve};

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
    pub fn unfocus<W,Ph,S>(&self, root_widget: &W, root_path: &Ph, stack: &S, ts: u64, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> EventResp where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized {
        if let Some(widget) = (self.access)(ctx).state.kbd.focused.take() {
            let event = StdVariant {
                variant: Unfocus{},
                ts,
                //filter_path: Some(widget.clone().path),
                filter_point: None,
                direct_only: false,
                //filter_path_strict: true,
                _p: PhantomData,
            };
            root_widget.event_direct(root_path,stack,&event,Some(&*widget),cache,root,ctx)
        } else {
            false
        }
    }

    pub fn focus<W,Ph,S>(&self, root_widget: &W, root_path: &Ph, path_to_focus: Arc<dyn PathResolvusDyn<E>>, stack: &S, ts: u64, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<EventResp,E::Error> where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized {
        self.unfocus(root_widget,root_path,stack,ts,cache,root.fork(),ctx);
        (self.access)(ctx).state.kbd.focused = Some(path_to_focus.clone());
        
        let event = StdVariant {
            variant: Focus{},
            ts,
            //filter_path: Some(p),
            filter_point: None,
            direct_only: false,
            //filter_path_strict: true,
            _p: PhantomData
        };
        Ok(root_widget.event_direct(root_path,stack,&event,Some(&*path_to_focus),cache,root,ctx))
    }
}

impl<S,E> HandlerBuilder<E> for StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, for<'a> E::Context<'a>: CtxStdState<'a,E>, EEvent<E>: StdVarSup<E> {
    type Built = StdHandlerLive<S,E>;

    fn build<Acc>(ctx: &mut E::Context<'_>) -> Self::Built where Acc: HandlerStateResolve<Self,E> {
        StdHandlerLive {
            access: Acc::resolve_handler_state,
            sup: S::build::<StdHandlerSubAccess<Acc,S>>(ctx),
            _c: PhantomData,
        }
    }
}

pub struct StdHandlerLive<SB,E> where SB: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: SB::Built,
    pub access: for<'c,'cc> fn(&'c mut E::Context<'cc>)->&'c mut StdHandler<SB,E>,
    _c: PhantomData<E>,
}

pub struct StdHandlerSubAccess<SuperAccess,Dest>(PhantomData<(SuperAccess,Dest)>);

impl<SuperAccess,Dest,E> HandlerStateResolve<Dest,E> for StdHandlerSubAccess<SuperAccess,Dest>
where
    E: Env,
    SuperAccess: HandlerStateResolve<StdHandler<Dest,E>,E>,
    for<'a> E::Context<'a>: CtxStdState<'a, E>,
    EEvent<E>: StdVarSup<E>,
    Dest: HandlerBuilder<E>,
{
    #[inline]
    fn resolve_handler_state<'a>(ctx_root: &'a mut <E as Env>::Context<'_>) -> &'a mut Dest {
        &mut SuperAccess::resolve_handler_state(ctx_root).sup
    }
}
