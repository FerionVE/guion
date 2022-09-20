use crate::text::layout::*;

use super::*;

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env {
    /// Fill the current bounds with the color derived from style
    fn fill_rect<Q>(&mut self, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E>;

    /// Fill the current bounds with the color and thickness derived from style
    fn fill_border_inner<Q>(&mut self, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E>;

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text<Q>(&mut self, text: &str, align: (f32,f32), props: &Q, c: &mut E::Context<'_>) where for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r, Q: Queron<E> {
        let g: ETextLayout<E> = TxtLayoutFromStor::<str,E>::from(text,c);
        let props = StdRenderProps::new(props).inner_aligned(g.display_size(),align);
        self.render_preprocessed_text(&g,Offset::default(),&props,c);
    }
    fn render_preprocessed_text<Q>(&mut self, text: &ETextLayout<E>, inner_offset: Offset, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E>;

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, c: &mut E::Context<'_>);

    /// Set the cursor to the cursor derived from style
    #[inline]
    fn set_cursor<Q>(&mut self, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E> {
        let style = QueryTestStyleCurrent.query_in(props).unwrap();
        self.set_cursor_specific(&style.cursor,c);
    }

    //fn draw_text_button(&mut self, c: &mut E::Context<'_>, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context<'_>);
}
