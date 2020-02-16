use super::*;

pub trait RenderStdWidgets<E>: Render<E> where E: Env, E::Backend: Backend<E,Renderer=Self>, EStyle<E>: Style<E> {
    fn fill_rect(&mut self, b: &Bounds, c: ESColor<E>);
    fn border_rect(&mut self, b: &Bounds, c: ESColor<E>, thickness: u32);
    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, b: &Bounds, text: &str, style: &EStyle<E>, variant: &ESVariant<E>, c: &mut E::Context) {
        let pp = style.preprocess_text(text,c);
        self.render_preprocessed_text(b,&pp,style,variant,c);
    }
    fn render_preprocessed_text(&mut self, b: &Bounds, text: &ESPPText<E>, style: &EStyle<E>, variant: &ESVariant<E>, c: &mut E::Context);

    fn set_cursor(&mut self, b: &Bounds, cursor: ESCursor<E>);

    fn draw_text_button(&mut self, b: &Bounds, pressed: bool, caption: &str, style: &EStyle<E>, variant: &ESVariant<E>);

    fn draw_selected(&mut self, b: &Bounds, s: &EStyle<E>, variant: &ESVariant<E>);
}