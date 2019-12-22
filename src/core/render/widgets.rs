use crate::core::ctx::aliases::*;
use super::*;

pub trait RenderStdWidgets<E>: Render<E> where E: Env<Renderer=Self>, E::Style: Style<E> {
    fn fill_rect(&mut self, b: &Bounds, c: ESColor<E>);
    fn border_rect(&mut self, b: &Bounds, c: ESColor<E>, thickness: u32);
    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, b: &Bounds, text: &str, style: &E::Style, c: &mut E::Context) {
        let pp = style.preprocess_text(text,c);
        self.render_preprocessed_text(b,&pp);
    }
    fn render_preprocessed_text(&mut self, b: &Bounds, text: &ESPPText<E>);

    fn set_cursor(&mut self, b: &Bounds, cursor: ESCursor<E>);

    fn draw_text_button(&mut self, b: &Bounds, pressed: bool, caption: &str, style: &E::Style);

    fn draw_selected(&mut self, b: &Bounds, s: &E::Style);
}