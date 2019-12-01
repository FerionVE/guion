use crate::core::util::bounds::Bounds;

pub trait Render {
    fn bounds_abs(&self) -> Bounds;
    fn slice(&mut self, b: &Bounds) -> Self;

    fn fill_rect_rgba(&mut self, c: [u8;4]);
    fn border_rect_rgba(&mut self, c: [u8;4], thickness: u32);
}

//fn a(e: Box<dyn Render<Sliced=(dyn Render)>>) {}