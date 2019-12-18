use super::*;

pub trait RenderStdWidgets<E>: Render<E> where E::Env<Renderer=Self> {
    fn fill_rect_rgba(&mut self, c: [u8;4]);
    fn border_rect_rgba(&mut self, c: [u8;4], thickness: u32);
    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, text: &str, style: &E::Style) {
        let pp = style.preprocess_text(text);
        self.render_preprocessed_text(&pp);
    }
    fn render_preprocessed_text(&mut self, text: &<E::Style as Style>::PreprocessedText);

    fn set_cursor(&mut self, cursor: <E::Style as Style>::Cursor);

    fn draw_button(&mut self, pressed: bool, style: &E::Style);

    fn draw_selected(&mut self, s: &E::Style);
}