use super::*;

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r {
    /// Fill the current bounds with the color derived from style
    fn fill_rect(&mut self, c: &mut E::Context);

    /// Fill the current bounds with the color and thickness derived from style
    fn fill_border_inner(&mut self, c: &mut E::Context);

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, text: &str, align: (f32,f32), c: &mut E::Context) {
        let pp = ESGlyphs::<E>::generate(text,(20.0,20.0),c); //style.preprocess_text(text,c);
        self.inner_aligned(pp.size(),align)
            .render_preprocessed_text(&pp,Offset::default(),c);
    }
    fn render_preprocessed_text(&mut self, text: &ESGlyphs<E>, inner_offset: Offset, c: &mut E::Context);

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, c: &mut E::Context);

    /// Set the cursor to the cursor derived from style
    #[inline]
    fn set_cursor(&mut self, c: &mut E::Context) {
        self.set_cursor_specific(&self.style().cursor(self.selector(),c),c);
    }

    //fn draw_text_button(&mut self, c: &mut E::Context, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context);
}
