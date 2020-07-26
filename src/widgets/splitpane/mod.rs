use super::*;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V,Stil> where
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

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V,()> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> SplitPane<'w,L,R,V,E,()> {
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

unsafe impl<'w,E,L,R,V,Stil> Statize<E> for SplitPane<'w,E,L,R,V,Stil> where 
    E: Env,
    L: StatizeSized<E>+'w,
    R: StatizeSized<E>+'w,
    V: StatizeSized<E>+'w,
    Stil: StatizeSized<E>+'w,
{
    type Statur = SplitPane<'static,E,L::StaturSized,R::StaturSized,V::StaturSized,Stil::StaturSized>;
}
