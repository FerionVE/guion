use std::marker::PhantomData;

use crate::aliases::EStyle;
use crate::env::Env;
use crate::layout::Orientation;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEndBuilder, MutorToBuilder, MutorToBuilderExt};
use crate::widget::as_widgets::fixed_idx::WidgetsFixedIdx;

pub mod widget;

pub struct SplitPane<E,L,R,V,TrMut> where
    E: Env,
{
    pub childs: WidgetsFixedIdx<(L,R)>,
    pub state: V,
    updater: TrMut,
    pub orientation: Orientation,
    pub width: u32, //TODO with from style
    pub style: EStyle<E>,
    p: PhantomData<()>,
}

impl<E,L,R,V> SplitPane<E,L,R,V,()> where
    E: Env,
{
    #[inline]
    pub fn new(orientation: Orientation, state: V, childs: (L,R)) -> Self {
        Self{
            childs: WidgetsFixedIdx(childs),
            state,
            updater: (),
            orientation,
            width: 8,
            style: Default::default(),
            p: PhantomData,
        }
    }
}

impl<E,L,R,V,TrMut> SplitPane<E,L,R,V,TrMut> where
    E: Env,
{   
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_update<T>(self, mutor: T) -> SplitPane<E,L,R,V,T> where T: MutorEndBuilder<f32,E> {
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
    pub fn with_atomstate<T>(self, mutor: T) -> SplitPane<E,L,R,V,impl MutorEndBuilder<f32,E>>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<f32>,E>,
    {
        self.with_update(
            mutor.mutor_end_if((), |state,_,value,ctx| {
                //TODO ResolveResult handling
                state.set(value,ctx);
            })
        )
    }
}
