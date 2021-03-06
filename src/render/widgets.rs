use crate::text::layout::*;

use super::*;

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env, /*ERenderer<E>: AsRefMut<Self>,*/ {
    /// Fill the current bounds with the color derived from style
    fn fill_rect(&mut self, c: &mut E::Context);

    /// Fill the current bounds with the color and thickness derived from style
    fn fill_border_inner(&mut self, c: &mut E::Context);

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, text: &str, align: (f32,f32), c: &mut E::Context) {
        let g: ETextLayout<E> = TxtLayoutFromStor::<E,&str>::from(&text,c);
        let oldb = self._bounds().clone();
        let newb = oldb.inner_aligned(g.size(),align);
        self._set_bounds(&newb);
        self.render_preprocessed_text(&g,Offset::default(),c);
        self._set_bounds(&oldb);
    }
    fn render_preprocessed_text(&mut self, text: &ETextLayout<E>, inner_offset: Offset, c: &mut E::Context);

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, c: &mut E::Context);

    /// Set the cursor to the cursor derived from style
    #[inline]
    fn set_cursor(&mut self, c: &mut E::Context) {
        self.set_cursor_specific(&self._style().cursor(self._selector(),c),c);
    }

    //fn draw_text_button(&mut self, c: &mut E::Context, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context);
}
