use crate::core::util::bounded_widget::IBoundedWidget;
use crate::core::env::Env;
use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;
use crate::core::env::Context;
use crate::core::env::WidgetStore;
use std::any::TypeId;

pub trait Render {
    #[inline]
    fn requires_render<W: Widget<E>, E: Env>(&self, w: &W) -> bool {
        w.render() || self.force()
    }

    fn render_widgets<'a,E: Env,W: IBoundedWidget<E> + 'a>(&mut self, i: impl Iterator<Item=&'a W>, c: &mut E::Ctx, overlap: bool) {
        let senf: &mut E::Renderer = hackcast_mut(self).expect("Differenting Renderer");
        
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.widgets_mut().get_mut(&w.id()).expect("Lost Child");
                render |= senf.requires_render(&ww);
                if render {
                    ww.handler().render(c,senf.slice(w.bounds()));
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

//fn a(e: Box<dyn Render<Sliced=(dyn Render)>>) {}

fn hackcast_mut<'a,T,U>(t: &'a mut T) -> Option<&'a mut U> {
    if TypeId::of::<T>() == TypeId::of::<U>() {
        Some( unsafe { *(t as *mut T as *mut U) } )
    }else{
        None
    }
}