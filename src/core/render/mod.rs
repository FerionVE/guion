use crate::core::ctx::widgets::Widgets;
use super::*;

pub mod widgets;
pub mod link;

pub trait Render<E>: Sized where E: Env, E::Backend: Backend<E,Renderer=Self> {
    #[inline]
    fn requires_render(&mut self, b: &Bounds, w: &E::DynWidget) -> bool {
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

