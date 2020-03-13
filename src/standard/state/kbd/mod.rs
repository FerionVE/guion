use crate::core::ctx::aliases::*;
use crate::core::*;
use crate::core::ctx::widgets::Widgets;
use event::variants::LostFocus;
use std::fmt::Debug;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub focused: Option<E::WidgetPath>,
}

impl<E> KbdState<E> where E: Env {
    /*#[inline]
    pub fn unfocus(ctx: &mut E::Context, deref_to_self: impl Fn(&mut E::Context) -> &mut Self, root: &E::Storage, root_bounds: &Bounds, ts: u64) 
        where EEvent<E>: StdVarSup<E>,
    {
        if let Some(p) = deref_to_self(ctx).focused.take() {
            if let Ok(w) = root.widget(p) {
                let bounds = root.trace_bounds(p).unwrap();
                ctx.link(w)._event_root((Event::from(LostFocus{}),&bounds,ts));
            }
        }
    }
    #[inline]
    pub fn focus(&mut self, l: &mut Link<E>, bounds: &Bounds, ts: u64) {
        
    }*/

    pub fn new() -> Self {
        Self{
            focused: None,
        }
    }
}
