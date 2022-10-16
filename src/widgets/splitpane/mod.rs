use crate::error::ResolveResult;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEndBuilder, MutorToBuilder, MutorToBuilderExt};

use super::*;
use super::util::state::AtomStateMut;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    Self: 'w,
{
    pub childs: (L,R),
    pub state: V,
    updater: TrMut,
    pub orientation: Orientation,
    pub width: u32, //TODO with from style
    pub style: EStyle<E>,
    p: PhantomData<&'w (L,R,V,TrMut)>,
}

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V,()> where
    E: Env,
{
    #[inline]
    pub fn new(orientation: Orientation, state: V, childs: (L,R)) -> Self {
        Self{
            childs,
            state,
            updater: (),
            orientation,
            width: 8,
            style: Default::default(),
            p: PhantomData,
        }
    }
}

impl<'w,E,L,R,V,TrMut> SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
{   
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_update<T>(self, mutor: T) -> SplitPane<'w,E,L,R,V,T> where T: MutorEndBuilder<f32,E> {
        SplitPane{
            childs: self.childs,
            state: self.state,
            updater: mutor,
            orientation: self.orientation,
            width: self.width,
            style: self.style,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, mutor: T) -> SplitPane<'w,E,L,R,V,impl MutorEndBuilder<f32,E>>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<f32>,E> + 'w,
    {
        self.with_update(
            mutor.mutor_end_if((), |state,_,value,ctx| {
                //TODO ResolveResult handling
                state.set(value,ctx);
            })
        )
    }
}
