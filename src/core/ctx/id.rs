use crate::core::lazout::size::Size;
use crate::core::ctx::Context;
use qwutils::*;

pub trait WidgetID<E>: Clone + PartialEq + Sized where E: Context<WidgetID=Self> {
    #[inline]
    fn render(&self, c: &mut E, r: E::Renderer) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._render(c,r) )
    }
    #[inline]
    fn event(&self, c: &mut E, e: E::Event) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._event(c,e) )
    }
    #[inline]
    fn size(&self, c: &mut E) -> Result<Size,()> {
        c.has_widget(self).result()
            .map(|_| self._size(c) )
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    fn _render(&self, c: &mut E, r: E::Renderer) {
        c._render(self,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _event(&self, c: &mut E, e: E::Event) {
        c._event(self,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _size(&self, c: &mut E) -> Size {
        c._size(self)
    }

    #[inline]
    fn is_hovered(&self, c: &E) -> bool {
        c.is_hovered(self)
    }
    #[inline]
    fn is_selected(&self, c: &E) -> bool {
        c.is_selected(self)
    }
}