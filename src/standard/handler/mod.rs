use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use super::state::StdState;
use event::variants::{GainedFocus, LostFocus};

pub mod imp;

pub struct StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub s: StdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            s: StdState::new(),
            _c: PhantomData,
        }
    }

    pub fn focus(l: Link<E>, ts: u64) {
        if let Some(p) = l.as_mut().s.kbd.focused.take() {
            l.with_widget(p.slice())
                .expect("TODO")
                ._event_root(Event::from(LostFocus{ts}));
            }
        l.as_mut().s.kbd.focused = Some(l.widget.path.clone());
        l._event_root(Event::from(GainedFocus{ts}));
    }
}