use super::ctx::widgets::Widgets;
use qwutils::*;
use super::*;

pub mod sub;
pub use sub::*;

pub trait WidgetPath: Clone + PartialEq + Sized + 'static {
    type SubPath: SubPath;
    
    fn attach(&mut self, sub: Self::SubPath);
    fn attached(&self, sub: Self::SubPath) -> Self;

    fn id<E: Env<WidgetPath=Self>>(&self) -> &E::WidgetID;

    fn slice<E: Env<WidgetPath=Self>>(&self) -> EWPSlice<E>;

    #[inline]
    fn path_eq<I: WidgetPath + 'static>(&self, o: &I) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        Any::downcast_ref::<Self>(o)
            .map_or(false, |r| self.eq(r) )
    }
    
    #[inline]
    fn render<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._render::<E>(c,r) )
    }
    #[inline]
    fn event<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._event::<E>(c,e) )
    }
    #[inline]
    fn size<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context) -> Result<Size,()> {
        c.has_widget(self).result()
            .map(|_| self._size::<E>(c) )
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    fn _render<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        c._render(self,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _event<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) {
        c._event(self,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _size<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context) -> Size {
        c._size(self)
    }
}

pub trait WidgetPathSlice<'a>: Sized {
    fn parent(self) -> Option<Self>;

    fn from_ref<E: Env>(r: &'a E::WidgetPath) where &'a E: EnvLt<PathSlice=Self>;

    #[inline]
    fn render<E: Env>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._render::<E>(c,r) )
    }
    #[inline]
    fn event<E: Env>(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._event::<E>(c,e) )
    }
    #[inline]
    fn size<E: Env>(&self, c: &mut E::Context) -> Result<Size,()> where E: WidgetPath<Slice=Self>/*, for<'a> Self: WidgetPathSlice<'a>*/ {
        c.has_widget(self).result()
            .map(|_| self._size::<E>(c) )
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    fn _render<E: Env>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        c._render(self,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _event<E: Env>(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds)) {
        c._event(self,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _size<E: Env<WidgetPath=Self>>(&self, c: &mut E::Context) -> Size {
        c._size(self)
    }
}