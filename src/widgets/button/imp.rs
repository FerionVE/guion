use crate::ctx::Context;
use crate::env::Env;
use crate::newpath::PathStackDyn;
use crate::pathslice::NewPathStack;
use crate::traitcast::WQuery;
use crate::widget_decl::mutor_trait::MutorEndBuilder;

use super::{widget::Button, Trigger};

pub trait IButton<E> where E: Env {
    fn trigger(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
}

impl<E,Text,Tr> IButton<E> for Button<E,Text,Tr> where
    E: Env,
    Tr: Trigger<E>,
{
    #[inline]
    fn trigger(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        self.trigger.trigger(path, root, ctx);
        // if let Some(t) = self.trigger_mut.build_box_mut_event(()) {
        //     ctx.mutate_closure(t);
        // }
    }
}

impl<E> WQuery<E> for dyn IButton<E> where E: Env {
    type Result<'a> = &'a (dyn IButton<E> + 'a);
}
