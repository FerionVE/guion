use crate::core::util::bounded_widget::IBoundedWidget;
use crate::core::ctx::Context;
use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;

pub trait Render<E>: Sized where E: Context<Renderer=Self> {
    #[inline]
    fn requires_render(&self, w: &E::DynWidget) -> bool {
        w.invalid() || self.force()
    }
    #[inline] 
    fn render_widgets<'a,W: IBoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, c: &mut E, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.widget_mut(&w.id()).expect("Lost Child");
                render |= self.requires_render(&ww);
                if render {
                    ww.handler().render(c,self.slice(w.bounds()));
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

    fn fill_rect_rgba(&mut self, c: [u8;4]);
    fn border_rect_rgba(&mut self, c: [u8;4], thickness: u32);
}