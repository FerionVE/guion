use super::*;

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env, /*ERenderer<E>: AsRefMut<Self>,*/ {
    /// fill the current bounds with the color derived from style
    fn fill_rect(&mut self, c: &mut E::Context);

    /// fill the current bounds with the color and thickness derived from style
    fn fill_border_inner(&mut self, c: &mut E::Context);

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, text: &str, align: (f32,f32), c: &mut E::Context) {
        let pp = ESGlyphs::<E>::generate(text,(20.0,20.0),c); //style.preprocess_text(text,c);
        let oldb = self._bounds().clone();
        let newb = oldb.inner_aligned(pp.size(),align);
        self._set_bounds(&newb);
        self.render_preprocessed_text(&pp,Offset::default(),c);
        self._set_bounds(&oldb);
    }
    fn render_preprocessed_text(&mut self, text: &ESGlyphs<E>, inner_offset: Offset, c: &mut E::Context);

    /// set the cursor to the cursor derived from style
    fn set_cursor(&mut self, c: &mut E::Context);

    //fn draw_text_button(&mut self, c: &mut E::Context, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context);
}
