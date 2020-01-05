use super::ctx::widgets::Widgets;
use qwutils::*;
use super::*;

pub mod sub;
pub use sub::*;

pub mod provider;
pub use provider::*;

pub mod result;
pub use result::*;

pub trait WidgetPath<E>: WPProvider<E> + AsWPSlice<E> + Clone + PartialEq + Sized + 'static where E: Env<WidgetPath=Self> {
    type SubPath: SubPath<E>;
    
    fn attach(&mut self, sub: Self::SubPath);
    fn attached(self, sub: Self::SubPath) -> Self;

    fn id(&self) -> &E::WidgetID;

    fn parent(&self) -> Option<WPSlice<E>>;

    #[inline]
    fn eq<F: Env + 'static>(&self, o: &F::WidgetPath) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        self.id().id_eq(o.id())
    }

    #[inline]
    fn eq_of_slice<F: Env>(s: WPSlice<E>, o: WPSlice<F>) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        s.id().id_eq(o.id())
    }
    
    #[inline]
    fn render_of_slice(s: WPSlice<E>, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(s).result()
            .map(|_| Self::_render_of_slice(s,c,r) )
    }
    #[inline]
    fn event_of_slice(s: WPSlice<E>, c: &mut E::Context, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(s).result()
            .map(|_| Self::_event_of_slice(s,c,e) )
    }
    #[inline]
    fn size_of_slice(s: WPSlice<E>, c: &mut E::Context) -> Result<Size,()> {
        c.has_widget(s).result()
            .map(|_| Self::_size_of_slice(s,c) )
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    fn _render_of_slice(s: WPSlice<E>, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        c._render(s,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _event_of_slice(s: WPSlice<E>, c: &mut E::Context, e: (EEvent<E>,&Bounds)) {
        c._event(s,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _size_of_slice(s: WPSlice<E>, c: &mut E::Context) -> Size {
        c._size(s)
    }

    fn id_of_slice(s: WPSlice<E>) -> &E::WidgetID;
    fn parent_of_slice(s: WPSlice<E>) -> Option<WPSlice<E>>;
    fn from_slice(s: WPSlice<E>) -> Self;
}

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
    
    #[inline]
    pub fn render(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) -> Result<(),()> {
        E::WidgetPath::render_of_slice(*self,c,r)
    }
    #[inline]
    pub fn event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        E::WidgetPath::event_of_slice(*self,c,e)
    }
    #[inline]
    pub fn size(&self, c: &mut E::Context) -> Result<Size,()> {
        E::WidgetPath::size_of_slice(*self,c)
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    pub fn _render(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        E::WidgetPath::_render_of_slice(*self,c,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    pub fn _event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) {
        E::WidgetPath::_event_of_slice(*self,c,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    pub fn _size(&self, c: &mut E::Context) -> Size {
        E::WidgetPath::_size_of_slice(*self,c)
    }

    #[inline]
    pub fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> WPSlice<'a,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F> {
        WPSlice{slice: self.slice}
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