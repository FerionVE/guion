//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state
use crate::*;
use std::marker::PhantomData;
use event::variants::{Focus, Unfocus};
use state::standard::StdStdState;

pub mod imp;
pub mod imps;

pub struct StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub s: StdStdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            s: StdStdState::new(),
            _c: PhantomData,
        }
    }

    pub fn unfocus(mut root: Link<E>, root_bounds: Bounds, ts: u64) -> EventResp {
        if let Some(p) = root.ctx.as_mut().s.kbd.focused.take() {
            root.send_event(
                &EventCompound(Event::from(Unfocus{}),root_bounds,ts,Default::default(),Default::default(),false), //TODO check if default stylevariant here is correct
                p.refc().path,
            ).unwrap_or(false)
        }else{
            false
        }
    }

    pub fn focus(mut root: Link<E>, p: E::WidgetPath, root_bounds: Bounds, ts: u64) -> Result<EventResp,()> {
        Self::unfocus(root.reference(),root_bounds,ts);
        root.as_mut().s.kbd.focused = Some(WidgetIdent::from_path(p.refc(),root.widget.stor)?);
        root.send_event(
            &EventCompound(Event::from(Focus{}),root_bounds,ts,Default::default(),Default::default(),false),
            p,
        )
    }
}