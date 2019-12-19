use crate::core::util::bounded_widget::IBoundedWidget;
use crate::core::ctx::*;
use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;
use crate::core::style::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env<Renderer=Self> {
    #[inline]
    fn requires_render(&self, w: &E::DynWidget) -> bool {
        w.invalid() || self.force()
    }
    #[inline] 
    fn render_widgets<'a,W: IBoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, c: &mut E::Context, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.widget_mut(&w.id()).expect("Lost Child");
                render |= self.requires_render(&ww);
                if render {
                    let border = ww.border().clone();
                    let sliced = self.slice( &w.bounds().inside(&border) );

                    w.id().render::<E>(c,sliced).expect("Lost Widget");
                }
                render &= overlap;
            }
        }
    }

    fn bounds_abs(&self) -> Bounds;
    fn slice(&mut self, b: &Bounds) -> Self;

    ///if widgets should be rendered even if the don't require to
    fn force(&self) -> bool {
        false
    }
    ///return false if rendered widgets should not be set rendered
    fn validate_widgets(&self) -> bool {
        true
    }
}