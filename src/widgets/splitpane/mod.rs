use super::*;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V,Stil> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub childs: (L,R),
    pub state: V,
    pub orientation: Orientation,
    pub width: u32, //TODO with from style
    pub style: Stil,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V,()> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    #[inline]
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> SplitPane<'w,E,L,R,V,()> {
        SplitPane{
            id,
            childs,
            state,
            orientation,
            width: 8,
            style: (),
            p: PhantomData,
        }
    }
}

impl<'w,E,L,R,V,Stil> SplitPane<'w,E,L,R,V,Stil> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
    Stil: 'w,
{
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> SplitPane<'w,E,L,R,V,SStil> where SStil: 'w {
        SplitPane{
            id: self.id,
            childs: self.childs,
            orientation: self.orientation,
            width: self.width,
            style,
            state: self.state,
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
