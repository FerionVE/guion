use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::Env;
use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;

pub trait Render {
    #[inline]
    fn requires_render<W: Widget<E>, E: Env>(&self, w: &W) -> bool {
        w.render() || self.force()
    }

    fn render_widgets<'a,E: Env,W: BoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {

            }
        }else{

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

//fn a(e: Box<dyn Render<Sliced=(dyn Render)>>) {}