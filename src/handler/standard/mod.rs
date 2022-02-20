//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state
use crate::*;
use std::marker::PhantomData;
use std::sync::Arc;
use state::standard::StdStdState;

pub mod imp;
pub mod imps;

pub struct StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub s: StdStdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            s: StdStdState::new(),
            _c: PhantomData,
        }
    }
}

impl<SB,E> StdHandlerLive<SB,E> where SB: HandlerBuilder<E>, E: Env, EEvent<E>: StdVarSup<E> {
    pub fn unfocus(&self, mut root: Link<E>, root_bounds: Bounds, ts: u64) -> EventResp {
        if let Some(p) = (self.access)(root.ctx).s.kbd.focused.take() {
            root.send_event(
                &EventCompound{
                    event: Event::from(Unfocus{}),
                    bounds: root_bounds,
                    ts,
                    filter: Default::default(),
                    style: Default::default(),
                    flag: false,
                }, //TODO check if default stylevariant here is correct
                p.refc().path,
            ).unwrap_or(false)
        }else{
            false
        }
    }

    pub fn focus(&self, mut root: Link<E>, p: E::WidgetPath, root_bounds: Bounds, ts: u64) -> Result<EventResp,E::Error> {
        self.unfocus(root.reference(),root_bounds,ts);
        (self.access)(root.ctx).s.kbd.focused = Some(WidgetIdent::from_path(p.refc(),&root.widget.root,root.ctx)?);
        root.send_event(
            &EventCompound{
                event: Event::from(Focus{}),
                bounds: root_bounds,
                ts,
                filter: Default::default(),
                style: Default::default(),
                flag: false,
            },
            p,
        )
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
