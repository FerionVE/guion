use crate::ctx::Context;
use crate::env::Env;
use crate::traitcast_for_from_widget;
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widgets::util::state::AtomState;

use super::CheckBox;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
    fn set(&self, v: bool, ctx: &mut E::Context<'_>);
}

impl<'w,E,State,Text,TrMut> ICheckBox<E> for CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    State: AtomState<E,bool>,
    TrMut: MutorEndBuilder<bool,E>,
{
    #[inline]
    fn state(&self) -> &dyn AtomState<E,bool> {
        &self.state
    }
    fn set(&self, v: bool, ctx: &mut E::Context<'_>) {
        if let Some(t) = self.updater.build_box_mut_event(v) {
            ctx.mutate_closure(t)
        }
    }
}

traitcast_for_from_widget!(ICheckBox<E>);
