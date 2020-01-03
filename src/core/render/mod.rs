use crate::core::ctx::widgets::Widgets;
use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env, E::Backend: Backend<E,Renderer=Self> {
    #[inline]
    fn requires_render(&mut self, b: &Bounds, w: &E::DynWidget) -> bool {
        w.invalid() || self.force(b)
    }
    #[inline] 
    fn render_widgets<'a>(&mut self, b: &Bounds, i: impl Iterator<Item=WPSlice<'a,E>>, c: &mut E::Context, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.widget_mut(w).expect("Lost Child");
                render |= self.requires_render(b,&ww);
                if render {
                    let border = ww.border().clone();
                    let sliced = b.inside(&border);

                    w.render(c,(self,&sliced)).expect("Lost Widget");
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

