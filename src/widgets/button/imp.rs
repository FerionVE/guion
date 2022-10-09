use crate::view::mutor_trait::MutorEndBuilderExt;

use super::*;

pub trait IButton<E> where E: Env {
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
}

impl<'w,E,Text,Tr> IButton<E> for Button<'w,E,Text,Tr> where
    E: Env,
    Tr: Trigger<E>,
{
    #[inline]
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        self.trigger.trigger(root,ctx);
        if let Some(t) = self.trigger_mut {
            ctx.mutate_closure(t.build_box_mut_event(()));
        }
    }
}

traitcast_for_from_widget!(IButton<E>);
