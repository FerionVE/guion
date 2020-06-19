use super::*;
use std::{marker::PhantomData};

pub mod imp;

pub struct SplitPane<'w,L,R,V,E> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    id: E::WidgetID,
    pub border: Option<Border>,
    pub childs: (L,R),
    pub state: V,
    pub orientation: Orientation,
    pub width: u32,
    p: PhantomData<&'w mut ()>,
}

impl<'w,L,R,V,E> SplitPane<'w,L,R,V,E> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> SplitPane<'w,L,R,V,E> {
        SplitPane{
            id,
            border: None,
            childs,
            state,
            orientation,
            width: 8,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,L,R,V,E> Statize<E> for SplitPane<'w,L,R,V,E> where 
    E: Env,
    L: Statize<E>+'w, L::Statur: Sized,
    R: Statize<E>+'w, R::Statur: Sized,
    V: Statize<E>+'w, V::Statur: Sized,
{
    type Statur = SplitPane<'static,L::Statur,R::Statur,V::Statur,E>;
}
