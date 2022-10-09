use crate::error::ResolveResult;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEnd, MutorTo, MutorEndBuilderDyn, MutorToBuilderDyn, MutorEndBuilderExt, MutorToBuilder, MutorEndBuilder, MutorToBuilderExt};

use super::*;
use super::util::state::AtomStateMut;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
    pub childs: (L,R),
    pub state: V,
    updater: SplitPaneUpdater<'w,E>,
    pub orientation: Orientation,
    pub width: u32, //TODO with from style
    pub style: EStyle<E>,
    p: PhantomData<&'w (L,R,V)>,
}

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> Self {
        Self{
            id,
            childs,
            state,
            updater: SplitPaneUpdater::None,
            orientation,
            width: 8,
            style: Default::default(),
            p: PhantomData,
        }
    }
}

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V> where
    E: Env,
{   
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_update<T>(self, mutor: &'w T) -> SplitPane<'w,E,L,R,V> where T: MutorEndBuilder<f32,E> {
        SplitPane{
            id: self.id,
            childs: self.childs,
            state: self.state,
            updater: SplitPaneUpdater::Apply(mutor.erase()),
            orientation: self.orientation,
            width: self.width,
            style: self.style,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, mutor: &'w T) -> SplitPane<'w,E,L,R,V>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<f32>,E>,
    {
        SplitPane{
            id: self.id,
            childs: self.childs,
            state: self.state,
            updater: SplitPaneUpdater::Atomstate(mutor.erase()),
            orientation: self.orientation,
            width: self.width,
            style: self.style,
            p: PhantomData,
        }
    }
}

pub enum SplitPaneUpdater<'w,E> where E: Env {
    None,
    Apply(&'w (dyn MutorEndBuilderDyn<f32,E>+'w)),
    Atomstate(&'w (dyn MutorToBuilderDyn<(),DynAtomStateMutTarget<f32>,E>+'w)),
}

impl<'w,E> SplitPaneUpdater<'w,E> where E: Env {
    fn submit_update(&self, update: f32, ctx: &mut E::Context<'_>) -> bool {
        match self {
            SplitPaneUpdater::None => {return false;},
            &SplitPaneUpdater::Apply(x) => ctx.mutate_closure(x.build_box_mut_event(update)),
            &SplitPaneUpdater::Atomstate(x) => ctx.mutate_closure(
                x.mutor_end_if((), |state,_,value,ctx| {
                    //TODO ResolveResult handling
                    state.set(value,ctx);
                }).build_box_mut_event(update)
            ),
        }
        true
    }
}
