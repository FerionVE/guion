use crate::core::util::bounded_widget::IBoundedWidget;
use crate::core::env::Env;
use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;
use crate::core::env::Context;
use crate::core::env::WidgetStore;
use std::any::TypeId;

pub trait Render {
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

pub trait RenderExt<E> where E: Env {
    fn requires_render(&self, w: &E::DynWidget) -> bool;

    fn render_widgets<'a,W: IBoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, c: &mut E::Ctx, overlap: bool);
}

impl<E> RenderExt<E> for E::Renderer where E: Env {
    #[inline]
    fn requires_render(&self, w: &E::DynWidget) -> bool {
        w.render() || self.force()
    }

    fn render_widgets<'a,W: IBoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, c: &mut E::Ctx, overlap: bool) {
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.widgets_mut().get_mut(&w.id()).expect("Lost Child");
                render |= RenderExt::<E>::requires_render(self,&ww);
                if render {
                    ww.handler().render(c,self.slice(w.bounds()));
                }
                render &= overlap;
            }
        }
    }
}