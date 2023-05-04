use crate::ctx::Context;
use crate::env::Env;
use crate::traitcast::WQuery;
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widgets::util::state::AtomState;

use super::CheckBox;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
    fn set(&self, v: bool, ctx: &mut E::Context<'_>);
}

impl<E,State,Text,TrMut> ICheckBox<E> for CheckBox<E,State,Text,TrMut> where
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

impl<E> WQuery<E> for dyn ICheckBox<E> where E: Env {
    type Result<'a> = &'a (dyn ICheckBox<E> + 'a);
}
