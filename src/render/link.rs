use super::*;
use std::ops::Deref;

/// reference-compound of renderer, current bounds and style
pub struct RenderLink<'a,E> where E: Env {
    /// the underlying renderer
    pub r: &'a mut ERenderer<E>,
    /// current slice
    pub b: Bounds,
    /// current slice, but including last border
    pub br: Bounds,
    pub v: ESVariant<E>,
    pub s: EStyle<E>,
    /// whether rendering is enforced (e.g. if invalidation from outside occured)
    pub force: bool,
}

impl<'a,E> RenderLink<'a,E> where E: Env {
    pub fn new(r: &'a mut ERenderer<E>, b: Bounds, v: ESVariant<E>, s: EStyle<E>, force: bool) -> Self {
        Self{
            r,
            br: b.clone(),
            b,
            v,
            s,
            force,
        }
    }
    pub fn simple(r: &'a mut ERenderer<E>, dim: (u32,u32), c: &E::Context) -> Self {
        Self::new(
            r,
            Bounds::from_xywh(0,0,dim.0,dim.1),
            ESVariant::<E>::default(),
            c.style_provider().clone(),
            false,
        )
    }
    #[inline]
    pub fn force(&self) -> bool {
        self.force || self.r.force(&self.b)
    }
    /// fork with force set
    #[inline]
    pub fn with_force<'s>(&'s mut self, force: bool) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.clone(),
            br: self.br.clone(),
            v: self.v.clone(),
            s: self.s.clone(),
            force,
        }
    }
    /// fork with force set to true
    #[inline]
    pub fn enforced<'s>(&'s mut self) -> RenderLink<'s,E> where 'a: 's {
        self.with_force(true)
    }

    /// fork with area inside the border
    #[inline]
    pub fn inside_border_specific<'s>(&'s mut self, s: &Border) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.inside_border(s),
            br: self.b.clone(),
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        }
    }
    /// fork with area inside the border defined by the style
    #[inline]
    pub fn inside_border<'s>(&'s mut self) -> RenderLink<'s,E> where 'a: 's {
        self.inside_border_specific(&self.s.border(&self.v))
    }
    /// fork with area inside the border defined by the style  
    /// default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    pub fn inside_border_by<'s,V>(&'s mut self, tags: V) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Copy, 'a: 's {
        self.inside_border_specific(&self.s.border(&self.v.with(tags)))
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn slice<'s>(&'s mut self, s: &Bounds) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.slice(s),
            br: self.b.slice(s),
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        }
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn slice_abs<'s>(&'s mut self, s: &Bounds) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b & s,
            br: self.b & s,
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        }
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn inner_centered<'s>(&'s mut self, size: Dims) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.inner_centered(size),
            br: self.b.inner_centered(size),
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        }
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn inner_aligned<'s>(&'s mut self, size: Dims, align: (f32,f32)) -> RenderLink<'s,E> where 'a: 's {
        RenderLink{
            r: self.r,
            b: self.b.inner_aligned(size,align),
            br: self.b.inner_aligned(size,align),
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        }
    }
    /// fork with attached style variant tags
    #[inline]
    pub fn with<'s,V>(&'s mut self, tags: V) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Clone, 'a: 's {
        RenderLink{
            force: self.force(),
            r: self.r,
            b: self.b.clone(),
            br: self.br.clone(),
            v: self.v.with(tags),
            s: self.s.clone(),
        }
    }
    /// fork with default style and attache tags
    #[inline]
    pub fn with_default_style<'s,V>(&'s mut self, tags: V) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Clone, 'a: 's {
        RenderLink{
            force: self.force(),
            r: self.r,
            b: self.b.clone(),
            br: self.br.clone(),
            v: ESVariant::<E>::default().with(tags),
            s: self.s.clone(),
        }
    }
    /// get the current color defined by the style variant
    #[inline]
    pub fn color(&self) -> ESColor<E> {
        self.s.color(&self.v)
    }

    /*#[inline]
    pub fn for_widget<'s,W>(&'s mut self, w: &W, mut border: Border) -> RenderLink<'s,E> where W: Widget<E>, 'a: 's {
        let mut b = self.v.clone();
        let mut v = self.v.clone();
        w.border(&mut b);
        w.style(&mut v);
        
    }*/

    #[inline]
    pub fn render_widget(&mut self, mut w: Link<E>) {
        let mut fork = RenderLink{
            r: self.r,
            b: self.b,
            br: self.b.clone(),
            v: self.v.clone(),
            s: self.s.clone(),
            force: self.force,
        };

        w.render(&mut fork)
    }

    /*#[deprecated]
    #[inline] 
    pub fn render_widgets<'b>(&mut self, i: impl Iterator<Item=E::WidgetPath>+'b, c: CtxRef<E>, overlap: bool) {
        /*if overlap {
            let mut render = false;
            for w in i {
                let ww = c.0.widget(w).expect("Lost Widget");
                render |= self.r.requires_render(b,&ww);
                if render {
                    let mut border = c.1.default_border().clone();
                    ww.border(&mut border);

                    let mut style = s.clone();
                    ww.style(&mut style);

                    ww.render(c.1,senf);
                }
                render &= overlap;
            }
        }*/
        todo!()
    }*/
}

impl<'a,E> RenderLink<'a,E> where E: Env, ERenderer<E>: RenderStdWidgets<E> {
    #[inline]
    pub fn fill_rect(&mut self) {
        self.r.fill_rect(&self.b,self.color())
    }
    #[inline]
    pub fn border_rect(&mut self, thickness: u32) {
        self.r.border_rect(&self.b,self.color(),thickness)
    }
    #[inline]
    pub fn  fill_border_inner_specific(&mut self, b: &Border) {
        self.border_rect(b.top) //TODO IMPL real impl
    }
    #[inline]
    pub fn fill_border_inner(&mut self) {
        self.fill_border_inner_specific(&self.s.border(&self.v))
    }
    #[deprecated = "avoid this because stuff is not cached"]
    #[allow(deprecated)]
    #[inline]
    pub fn render_text(&mut self, text: &str, c: &mut E::Context) {
        self.render_text_aligned(text,(0.5,0.5),c)
    }
    #[deprecated = "avoid this because stuff is not cached"]
    #[allow(deprecated)]
    #[inline]
    pub fn render_text_aligned(&mut self, text: &str, align: (f32,f32), c: &mut E::Context) {
        self.r.render_text(&self.b,text,align,&self.s,&self.v,c)
    }
    #[inline]
    pub fn render_preprocessed_text(&mut self, text: &ESGlyphs<E>, inner_offset: Offset, c: &mut E::Context) {
        self.r.render_preprocessed_text(&self.b,text,inner_offset,&self.s,&self.v,c) //TODO we should not always give ctx through the render, for example the text/font can be inside the render head
    }
    #[inline]
    pub fn set_cursor(&mut self, cursor: ESCursor<E>) {
        self.r.set_cursor(&self.b,cursor)
    }
    #[inline]
    pub fn draw_text_button(&mut self, pressed: bool, caption: &str) {
        self.r.draw_text_button(&self.b,pressed,caption,&self.s,&self.v)
    }
    #[inline]
    pub fn draw_selected(&mut self) {
        self.r.draw_selected(&self.b,&self.s,&self.v)
    }
}