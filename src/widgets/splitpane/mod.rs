use super::*;
use super::util::state::AtomStateMut;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
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
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> Self {
        Self{
            id,
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
    pub fn with_update<T>(self, fun: T) -> SplitPane<'w,E,L,R,V,T> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,f32) + Clone + Send + Sync + 'static {
        SplitPane{
            id: self.id,
            childs: self.childs,
            state: self.state,
            updater: fun,
            orientation: self.orientation,
            width: self.width,
            style: self.style,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, fun: T) -> SplitPane<'w,E,L,R,V,impl TriggerMut<E>> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>) -> &'r mut (dyn AtomStateMut<E,f32>) + Clone + Send + Sync + 'static {
        self.with_update(move |r,x,c,v| fun(r,x,c).set(v,c) )
    }
}

/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TriggerMut<E> where E: Env {
    fn boxed(&self, value: f32) -> Option<BoxMutEvent<E>>;
}

impl<E> TriggerMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self, _: f32) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TriggerMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,f32) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, value: f32) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,value) ))
    }
}
