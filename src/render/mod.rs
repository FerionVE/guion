//! [`Render`] functions and a [Link struct](RenderLink) tracking [bounds](Bounds) and [style](Style)
use super::*;

pub mod widgets;
pub mod link;

pub trait Render<E>: Sized where E: Env, /*ERenderer<E>: AsRefMut<Self>*/ {
    /// If widgets should be rendered even if the don't require to
    #[inline]
    fn force(&self, _b: &Bounds) -> bool {
        false
    }
    /// Return false if rendered widgets should not be set rendered
    #[inline]
    fn validate_widgets(&mut self, _b: &Bounds) -> bool {
        true
    }

    fn _style(&self) -> &EStyle<E>;
    fn _bounds(&self) -> &Bounds;
    fn _viewport(&self) -> &Bounds;
    fn _selector(&self) -> &ESSelector<E>;

    fn _set_style(&mut self, v: &EStyle<E>);
    fn _set_bounds(&mut self, v: &Bounds);
    fn _set_viewport(&mut self, v: &Bounds);
    fn _set_selector(&mut self, v: &ESSelector<E>);
}
