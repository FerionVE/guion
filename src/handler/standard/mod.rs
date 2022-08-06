//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state
use crate::*;
use crate::event_new::variants::StdVariant;
use crate::widget::stack::{WithCurrentBounds, WithCurrentWidget};
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
    pub fn unfocus<W>(&self, root: &W, root_bounds: Bounds, ts: u64) -> EventResp where W: Widget<E> + ?Sized {
        if let Some(p) = (self.access)(root.ctx).s.kbd.focused.take() {
            let event = StdVariant {
                variant: Unfocus{},
                ts,
                filter_path: Some(p.refc().path),
                filter_point: None,
            };
            // TODO style where?
            let stack = WithCurrentBounds {
                inner: WithCurrentWidget {
                    inner: (),
                    path: Default::default(), //TODO real root path
                    id: root.id(), //wat
                },
                bounds: root_bounds,
                viewport: root_bounds,
            };
            root.event(&stack,&event)
        }else{
            false
        }
    }

    pub fn focus<W>(&self, root: &W, p: E::WidgetPath, root_bounds: Bounds, ts: u64) -> Result<EventResp,E::Error> where W: Widget<E> + ?Sized {
        self.unfocus(root.reference(),root_bounds,ts);
        (self.access)(root.ctx).s.kbd.focused = Some(WidgetIdent::from_path(p.refc(),&root.widget.root,root.ctx)?);
        
        let event = StdVariant {
            variant: Focus{},
            ts,
            filter_path: Some(p),
            filter_point: None,
        };
        // TODO style where?
        let stack = WithCurrentBounds {
            inner: WithCurrentWidget {
                inner: (),
                path: Default::default(), //TODO real root path
                id: root.id(), //wat
            },
            bounds: root_bounds,
            viewport: root_bounds,
        };
        root.event(&stack,&event)
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
