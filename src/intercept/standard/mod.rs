//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state

use std::marker::PhantomData;
use std::sync::Arc;

use crate::EventResp;
use crate::aliases::EEvent;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::standard::variants::{Unfocus, Focus};
use crate::event_new::variants::StdVariant;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::root::RootRef;
use crate::state::CtxStdState;
use crate::state::standard::StdStdState;
use crate::widget::Widget;

use super::{InterceptBuilder, InterceptStateResolve};

pub mod imp;
pub mod imps;

pub struct StdIntercept<S,E> where S: InterceptBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub state: StdStdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdIntercept<S,E> where S: InterceptBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            state: StdStdState::new(),
            _c: PhantomData,
        }
    }
}

impl<SB,E> StdInterceptLive<SB,E> where SB: InterceptBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn unfocus<W,Ph,S>(&self, root_widget: &mut W, root_path: &Ph, stack: &S, ts: u64, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Invalidation where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized {
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
            root_widget.event_direct(root_path,stack,&event,Some(&*widget),root,ctx)
        } else {
            Invalidation::valid()
        }
    }

    pub fn focus<W,Ph,S>(&self, root_widget: &mut W, root_path: &Ph, path_to_focus: Arc<dyn PathResolvusDyn<E>>, stack: &S, ts: u64, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Invalidation,E::Error> where W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized {
        self.unfocus(root_widget,root_path,stack,ts,root.fork(),ctx);
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
        Ok(root_widget.event_direct(root_path,stack,&event,Some(&*path_to_focus),root,ctx))
    }
}

impl<S,E> InterceptBuilder<E> for StdIntercept<S,E> where S: InterceptBuilder<E>, E: Env, for<'a> E::Context<'a>: CtxStdState<'a,E>, EEvent<E>: StdVarSup<E> {
    type Built = StdInterceptLive<S,E>;

    fn build<Acc>(ctx: &mut E::Context<'_>) -> Self::Built where Acc: InterceptStateResolve<Self,E> {
        StdInterceptLive {
            access: Acc::resolve_intercept_state,
            sup: S::build::<StdInterceptSubAccess<Acc,S>>(ctx),
            _c: PhantomData,
        }
    }
}

pub struct StdInterceptLive<SB,E> where SB: InterceptBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: SB::Built,
    pub access: for<'c,'cc> fn(&'c mut E::Context<'cc>)->&'c mut StdIntercept<SB,E>,
    _c: PhantomData<E>,
}

pub struct StdInterceptSubAccess<SuperAccess,Dest>(PhantomData<(SuperAccess,Dest)>);

impl<SuperAccess,Dest,E> InterceptStateResolve<Dest,E> for StdInterceptSubAccess<SuperAccess,Dest>
where
    E: Env,
    SuperAccess: InterceptStateResolve<StdIntercept<Dest,E>,E>,
    for<'a> E::Context<'a>: CtxStdState<'a, E>,
    EEvent<E>: StdVarSup<E>,
    Dest: InterceptBuilder<E>,
{
    #[inline]
    fn resolve_intercept_state<'a>(ctx_root: &'a mut E::Context<'_>) -> &'a mut Dest {
        &mut SuperAccess::resolve_intercept_state(ctx_root).sup
    }
}
