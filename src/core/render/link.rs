use super::*;
use std::ops::Deref;

/// reference-compound of renderer, current bounds and style
pub struct RenderLink<'a,E> where E: Env {
    pub r: &'a mut ERenderer<E>,
    pub b: Bounds,
    pub v: ESVariant<E>,
    pub s: EStyle<E>,
}

impl<'a,E> RenderLink<'a,E> where E: Env {
    /// fork with area inside the border
    #[inline]
    pub fn inside<'s>(&'s mut self, s: &'s Border) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.inside(s),
            v: self.v.clone(),
            s: self.s.clone(),
        }
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn slice<'s>(&'s mut self, s: &'s Bounds) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.slice(s),
            v: self.v.clone(),
            s: self.s.clone(),
        }
    }
    // fork with attached style variant verbs
    #[inline]
    pub fn with<'s,V>(&'s mut self, verbs: impl IntoIterator<Item=impl Deref<Target=V>>) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Copy, 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.clone(),
            v: self.v.with(verbs),
            s: self.s.clone(),
        }
    }

    #[inline] 
    pub fn render_widgets<'b>(&mut self, i: impl Iterator<Item=WPSlice<'b,E>>+'b, c: CtxRef<E>, overlap: bool) {
        todo!()
        /*
        if overlap {
            let mut render = false;
            for w in i {
                let ww = c.0.widget(w).expect("Lost Widget");
                render |= self.requires_render(b,&ww);
                if render {
                    let mut border = c.1.default_border().clone();
                    ww.border(&mut border);
                    let sliced = b.inside(&border);

                    let mut style = s.clone();
                    ww.style(&mut style);

                    ww.render(c.1,(self,&sliced,&style));
                }
                render &= overlap;
            }
        }
        */
    }
}

impl<'a,E> RenderLink<'a,E> where E: Env, ERenderer<E>: RenderStdWidgets<E> {
    #[inline]
    pub fn fill_rect(&mut self) {
        todo!()
    }
    #[inline]
    pub fn border_rect(&mut self, thickness: u32) {
        todo!()
    }
    #[deprecated = "avoid this because stuff is not cached"]
    #[inline]
    pub fn render_text(&mut self, text: &str, c: &mut E::Context) {
        todo!()
    }
    #[inline]
    pub fn render_preprocessed_text(&mut self, text: &ESPPText<E>) {
        todo!()
    }
    #[inline]
    pub fn set_cursor(&mut self, cursor: ESCursor<E>) {
        todo!()
    }
    #[inline]
    pub fn draw_text_button(&mut self, pressed: bool, caption: &str) {
        todo!()
    }
    #[inline]
    pub fn draw_selected(&mut self, s: &EStyle<E>) {
        todo!()
    }
}