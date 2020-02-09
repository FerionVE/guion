use crate::core::ctx::widgets::Widgets;
use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env, E::Backend: Backend<E,Renderer=Self> {
    #[inline]
    fn requires_render(&mut self, b: &Bounds, w: &E::DynWidget) -> bool {
        w.invalid() || self.force(b)
    }
    #[inline] 
    fn render_widgets<'a>(&mut self, b: &Bounds, i: impl Iterator<Item=WPSlice<'a,E>>+'a, c: CtxRef<E>, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.0.widget(w).expect("Lost Widget");
                render |= self.requires_render(b,&ww);
                if render {
                    let mut border = c.1.default_border().clone();
                    ww.border(&mut border);
                    let sliced = b.inside(&border);
                    ww.render(c.1,(self,&sliced));
                }
                render &= overlap;
            }
        }
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

