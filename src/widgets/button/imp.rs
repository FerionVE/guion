use super::*;

pub trait IButton<E> where E: Env {
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
}

impl<'w,E,Text,Tr,TrMut> IButton<E> for Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Tr: Trigger<E>,
    TrMut: MutorEnd<(),E>,
{
    #[inline]
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        self.trigger.trigger(root,ctx);
        if let Some(t) = self.trigger_mut.box_mut_event(()) {
            ctx.mutate_closure(t);
        }
    }
}

traitcast_for_from_widget!(IButton<E>);
