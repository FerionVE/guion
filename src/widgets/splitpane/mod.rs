use super::*;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,L,R,V,E,Stil> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    id: E::WidgetID,
    pub childs: (L,R),
    pub state: V,
    pub orientation: Orientation,
    pub width: u32,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,L,R,V,E,Stil> SplitPane<'w,L,R,V,E,Stil> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> SplitPane<'w,L,R,V,E> {
        SplitPane{
            id,
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
    L: StatizeSized<E>+'w,
    R: StatizeSized<E>+'w,
    V: StatizeSized<E>+'w,
{
    type Statur = SplitPane<'static,L::StaturSized,R::StaturSized,V::StaturSized,E>;
}
