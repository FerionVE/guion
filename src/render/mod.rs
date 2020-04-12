//! Render functions and a Link struct tracking bounds and style
use super::*;

pub mod widgets;
pub mod link;

pub trait Render<E>: Sized where E: Env, E::Backend: Backend<E,Renderer=Self> {
    #[inline]
    fn requires_render<'l,'s>(&mut self, b: &Bounds, w: &'s dyn Widget<'l,E>) -> bool where 'l: 's {
        w.invalid() || self.force(b)
    }
    /// if widgets should be rendered even if the don't require to
    #[inline]
    fn force(&mut self, _b: &Bounds) -> bool {
        false
    }
    /// return false if rendered widgets should not be set rendered
    #[inline]
    fn validate_widgets(&mut self, _b: &Bounds) -> bool {
        true
    }
}

