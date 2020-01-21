use crate::core::ctx::widgets::Widgets;
use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env, E::Backend: Backend<E,Renderer=Self> {
    #[inline]
    fn requires_render(&mut self, b: &Bounds, w: &E::DynWidget) -> bool {
        w.invalid() || self.force(b)
    }
    #[inline] 
    fn render_widgets<'a>(&mut self, b: &Bounds, i: impl Iterator<Item=&'a E::DynWidget>, c: CtxRef<E>, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                render |= self.requires_render(b,w);
                if render {
                    let border = w.border().clone();
                    let sliced = b.inside(&border);

                    w.render((c.0,c.1),(self,&sliced)).expect("Lost Widget");
                }
                render &= overlap;
            }
        }
    }
    /// if widgets should be rendered even if the don't require to
    #[inline]
    fn force(&mut self, b: &Bounds) -> bool {
        false
    }
    /// return false if rendered widgets should not be set rendered
    #[inline]
    fn validate_widgets(&mut self, b: &Bounds) -> bool {
        true
    }
}

