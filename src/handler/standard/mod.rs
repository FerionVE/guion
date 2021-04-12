//! Standard Handler featuring hovering/focusing of widgets and tracking of keyboard/mouse state
use crate::*;
use std::marker::PhantomData;
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
                &EventCompound{
                    event: Event::from(Unfocus{}),
                    bounds: root_bounds,
                    ts,
                    filter: Default::default(),
                    style: Default::default(),
                    flag: false,
                }, //TODO check if default stylevariant here is correct
                p.refc(),
            ).unwrap_or(false)
        }else{
            false
        }
    }

    pub fn focus(mut root: Link<E>, p: E::WidgetPath, root_bounds: Bounds, ts: u64) -> Result<EventResp,E::Error> {
        Self::unfocus(root.reference(),root_bounds,ts);
        root.as_mut().s.kbd.focused = Some(p.refc());
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
