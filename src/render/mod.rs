pub trait Render {
    fn bounds_abs(&self) -> (u32,u32,u32,u32);
    fn slice(&mut self, b: (u32,u32,u32,u32)) -> Self;

    fn fill_rect_rgba(&mut self, c: [u8;4]);
    fn border_rect_rgba(&mut self, c: [u8;4], thickness: u32);
}

//fn a(e: Box<dyn Render<Sliced=(dyn Render)>>) {}