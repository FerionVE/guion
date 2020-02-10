use std::{slice::SliceIndex, ops::Deref};
use super::ctx::widgets::Widgets;
use qwutils::*;
use super::*;

pub mod sub;
pub use sub::*;

/// A WidgetPath contains information to resolve to a specific Widget in a widget tree
pub trait WidgetPath<E>: AsWPSlice<E> + Clone + PartialEq + Sized + Send + Sync + 'static where E: Env<WidgetPath=Self> {
    type SubPath: SubPath<E>;
    type RcPath: RefClonable + From<Self> + Into<Self> + Deref<Target=Self>;
    
    fn attach(&mut self, sub: Self::SubPath);
    fn attached(self, sub: Self::SubPath) -> Self;

    fn id(&self) -> &E::WidgetID; //TODO no ref id
    fn tip(&self) -> &Self::SubPath;

    fn parent(&self) -> Option<WPSlice<E>>;

    #[inline]
    fn eq<F: Env + 'static>(&self, o: &F::WidgetPath) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        self.id().id_eq(o.id())
    }

    #[inline]
    fn eq_of_slice<F: Env>(s: WPSlice<E>, o: WPSlice<F>) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        s.id().id_eq(o.id())
    }
    
    /*#[inline]
    fn childs_of_slice<'a>(s: WPSlice<E>, c: CtxRefR<'a,E>) -> Result<Vec<WidgetRef<'a,E>>,()> {
        c.0.widget(s).ok_or(()).map(|w| Widget::childs(&**w) )
    }
    #[inline]
    fn childs_of_slice_mut<'a>(s: WPSlice<E>, c: CtxRefM<'a,E>) -> Result<Vec<WidgetRefMut<'a,E>>,()> {
        c.0.widget_mut(s).ok_or(()).map(|w| Widget::childs_mut(&mut **w) )
    }*/
    #[inline]
    fn child_paths_of_slice<'a>(s: WPSlice<E>, c: CtxRefR<'a,E>) -> Result<Vec<E::WidgetPath>,()> {
        Ok( c.0.widget(s)?.child_paths() )
    }

    #[inline]
    fn render_of_slice(s: WPSlice<E>, c: CtxRef<E>, r: (&mut ERenderer<E>,&Bounds,&EStyle<E>)) -> Result<(),()> {
        Ok( c.1.render(c.0.widget(s)?,r) )
    }
    #[inline]
    fn event_of_slice(s: WPSlice<E>, c: CtxRef<E>, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        Ok( c.1.event(c.0.widget(s)?,e) )
    }
    #[inline]
    fn size_of_slice(s: WPSlice<E>, c: CtxRef<E>) -> Result<ESize<E>,()> {
        Ok( c.1.size(c.0.widget(s)?) )
    }

    fn id_of_slice(s: WPSlice<E>) -> &E::WidgetID;
    fn parent_of_slice(s: WPSlice<E>) -> Option<WPSlice<E>>;
    fn from_slice(s: WPSlice<E>) -> Self;

    #[inline]
    fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> Self where E::WidgetPath: WidgetPath<F> {
        self
    }
}

#[repr(transparent)]
pub struct WPSlice<'a,E> where E: Env {
    pub slice: EWPSlice<'a,E>,
}

impl<'a,E> WPSlice<'a,E> where E: Env {
    #[inline]
    pub fn id(&self) -> &E::WidgetID {
        E::WidgetPath::id_of_slice(*self)
    }
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        E::WidgetPath::parent_of_slice(*self)
    }

    #[inline]
    pub fn unslice(&self) -> E::WidgetPath {
        E::WidgetPath::from_slice(*self)
    }

    #[inline]
    pub fn path_eq<F: Env + 'static>(&self, o: &F::WidgetPath) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        Any::downcast_ref::<Self>(o)
            .map_or(false, |r| self == r )
    }

    /*#[inline]
    pub fn childs<'c>(&self, c: CtxRefR<'c,E>) -> Result<Vec<WidgetRef<'c,E>>,()> {
        E::WidgetPath::childs_of_slice(*self,c)
    }
    #[inline]
    pub fn childs_mut<'c>(&self, c: CtxRefM<'c,E>) -> Result<Vec<WidgetRefMut<'c,E>>,()> {
        E::WidgetPath::childs_of_slice_mut(*self,c)
    }*/

    #[inline]
    pub fn child_paths<'c>(&self, c: CtxRefR<'c,E>) -> Result<Vec<E::WidgetPath>,()> {
        E::WidgetPath::child_paths_of_slice(*self,c)
    }
    
    #[inline]
    pub fn render(&self, c: CtxRef<E>, r: (&mut ERenderer<E>,&Bounds,&EStyle<E>)) -> Result<(),()> {
        E::WidgetPath::render_of_slice(*self,c,r)
    }
    #[inline]
    pub fn event(&self, c: CtxRef<E>, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        E::WidgetPath::event_of_slice(*self,c,e)
    }
    #[inline]
    pub fn size(&self, c: CtxRef<E>) -> Result<ESize<E>,()> {
        E::WidgetPath::size_of_slice(*self,c)
    }

    #[inline]
    pub fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> WPSlice<'a,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F> {
        WPSlice{slice: self.slice}
    }

    #[inline]
    pub fn slice<S: SliceIndex<[EWPSub<E>],Output=[EWPSub<E>]>>(&self, s: S) -> WPSlice<'a,E> {
        WPSlice{slice: &self.slice[s]}
    }
    #[inline]
    pub fn index<S: SliceIndex<[EWPSub<E>],Output=EWPSub<E>>>(&self, s: S) -> &'a EWPSub<E> {
        &self.slice[s]
    }
}

impl<'a,E> PartialEq for WPSlice<'a,E> where E: Env {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        E::WidgetPath::eq_of_slice(*self,*other)
    }
}

impl<'a,E> Clone for WPSlice<'a,E> where E: Env {
    #[inline]
    fn clone(&self) -> Self {
        Self{slice: self.slice}
    }
}

impl<'a,E> Copy for WPSlice<'a,E> where E: Env {}

pub trait AsWPSlice<E> where E: Env {
    fn slice(&self) -> WPSlice<E>;
}

#[inline]
pub fn rc_path_with_env<E: Env, F: Env<WidgetPath=E::WidgetPath>>(e: EWPRc<E>) -> EWPRc<F> where E::WidgetPath: WidgetPath<F,RcPath=EWPRc<E>> {
    e
}