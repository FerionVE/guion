use crate::aliases::{EStyle, ESize};
use crate::ctx::Context;
use crate::env::Env;
use crate::newpath::{PathStackDyn, PathStack};
use crate::util::bounds::Dims;

pub struct Button<E,Text,Tr,TrIm,TrMut> where
    E: Env,
{
    pub(super) trigger: Tr,
    pub(super) trigger_im: TrIm,
    pub(super) trigger_mut: TrMut,
    pub(super) size: Option<ESize<E>>,
    pub(super) style: Option<EStyle<E>>,
    pub(super) locked: bool,
    //pressed: Option<EEKey<E>>,
    pub(super) text: Text,
}

struct Trigon;

pub(super) fn send_mutation_trigger<E>(path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) where E: Env {
    ctx.queue_send_mutation(path.to_resolvus(), Box::new(Trigon));
}
