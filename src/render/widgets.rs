use crate::text::layout::*;

use super::*;

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env {
    /// Fill the current bounds with the color derived from style
    fn fill_rect(&mut self, c: &mut E::Context<'_>);

    /// Fill the current bounds with the color and thickness derived from style
    fn fill_border_inner(&mut self, c: &mut E::Context<'_>);

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text(&mut self, text: &str, align: (f32,f32), c: &mut E::Context<'_>) where for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r {
        let g: ETextLayout<E> = TxtLayoutFromStor::<str,E>::from(text,c);
        self.inner_aligned(g.size(),align)
            .render_preprocessed_text(&g,Offset::default(),c);
    }
    fn render_preprocessed_text(&mut self, text: &ETextLayout<E>, inner_offset: Offset, c: &mut E::Context<'_>);

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, c: &mut E::Context<'_>);

    /// Set the cursor to the cursor derived from style
    #[inline]
    fn set_cursor(&mut self, c: &mut E::Context<'_>) {
        self.set_cursor_specific(&self.style().cursor(self.selector(),c),c);
    }

    //fn draw_text_button(&mut self, c: &mut E::Context<'_>, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context<'_>);
}
