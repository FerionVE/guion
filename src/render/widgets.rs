use crate::aliases::{ETextLayout, ERenderer, ESCursor};
use crate::env::Env;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::text::layout::{TxtLayoutFromStor, TxtLayout};
use crate::util::bounds::Offset;

use super::{StdRenderProps, QueryTestStyleCurrent, Render};

//TODO refine standard render functions
pub trait RenderStdWidgets<E>: Render<E> where E: Env {
    /// Fill the current bounds with the color derived from style
    fn fill_rect<Q>(&mut self, props: &Q, ctx: &mut E::Context<'_>) where Q: Queron<E> + ?Sized;

    /// Fill the current bounds with the color and thickness derived from style
    fn fill_border_inner<Q>(&mut self, props: &Q, ctx: &mut E::Context<'_>) where Q: Queron<E> + ?Sized;

    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    fn render_text<Q>(&mut self, text: &str, align: (f32,f32), props: &Q, ctx: &mut E::Context<'_>) where for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r, Q: Queron<E> + ?Sized {
        let text_layout: ETextLayout<E> = TxtLayoutFromStor::<str,E>::from(text,ctx);
        let props = StdRenderProps::new(props).inner_aligned(text_layout.display_size(),align);
        self.render_preprocessed_text(&text_layout,Offset::default(),&props,ctx);
    }
    fn render_preprocessed_text<Q>(&mut self, text: &ETextLayout<E>, inner_offset: Offset, props: &Q, ctx: &mut E::Context<'_>) where Q: Queron<E> + ?Sized;

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, ctx: &mut E::Context<'_>);

    /// Set the cursor to the cursor derived from style
    #[inline]
    fn set_cursor<Q>(&mut self, props: &Q, ctx: &mut E::Context<'_>) where Q: Queron<E> + ?Sized {
        let style = QueryTestStyleCurrent.query_in(props).unwrap();
        self.set_cursor_specific(&style.cursor,ctx);
    }

    //fn draw_text_button(&mut self, c: &mut E::Context<'_>, pressed: bool, caption: &str);

    //fn draw_selected(&mut self, c: &mut E::Context<'_>);

    type RenderPreprocessedTextStyleCachors: PartialEq + 'static;
    
    fn render_preprocessed_text_cachors<Q>(&self, props: &Q) where Q: Queron<E> + ?Sized;
}
