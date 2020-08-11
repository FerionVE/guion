use super::*;
use std::ops::{DerefMut, Deref};

/// reference-compound of renderer, current bounds and style
pub struct RenderLink<'a,E> where E: Env {
    pub r: &'a mut ERenderer<E>,

    bounds: Bounds,
    prev_bounds: Option<&'a Bounds>,
    viewport: Bounds,
    prev_viewport: Option<&'a Bounds>,
    style: ESVariant<E>,
    prev_style: Option<&'a ESVariant<E>>,

    /// whether rendering is enforced (e.g. if invalidation from outside occured)
    pub force: bool,
}

impl<'a,E> RenderLink<'a,E> where E: Env {
    pub fn new(r: &'a mut ERenderer<E>, bounds: Bounds, viewport: Bounds, style: ESVariant<E>, force: bool) -> Self {
        Self{
            r,
            bounds: bounds,
            prev_bounds: None,
            viewport: viewport,
            prev_viewport: None,
            style: style,
            prev_style: None,
            force: force,
        }
            ._set_bounds()
            ._set_style()
            ._set_viewport()
    }
    pub fn simple(r: &'a mut ERenderer<E>, dim: (u32,u32), c: &E::Context) -> Self {
        Self::new(
            r,
            Bounds::from_xywh(0,0,dim.0,dim.1),
            Bounds::from_xywh(0,0,dim.0,dim.1),
            ESVariant::<E>::default(),
            false,
        )
    }

    #[inline]
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
    #[inline]
    pub fn viewport(&self) -> &Bounds {
        &self.viewport
    }
    #[inline]
    pub fn style(&self) -> &ESVariant<E> {
        &self.style
    }

    #[inline]
    pub fn force(&self) -> bool {
        self.force || self.r.force(&self.bounds)
    }
    /// fork with force set
    #[inline]
    pub fn with_force<'s>(&'s mut self, force: bool) -> RenderLink<'s,E> where 'a: 's {
        let mut f = self.forked(false,false,false);
        f.force = force;
        f
    }
    /// fork with force set to true
    #[inline]
    pub fn enforced<'s>(&'s mut self) -> RenderLink<'s,E> where 'a: 's {
        self.with_force(true)
    }

    /// fork with area inside the border
    #[inline]
    pub fn inside_border_specific<'s>(&'s mut self, s: &Border) -> RenderLink<'s,E> where 'a: 's {
        let bounds = self.bounds.inside_border(s);
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// fork with area inside the border defined by the style
    #[inline]
    pub fn inside_border<'s>(&'s mut self, c: &E::Context) -> RenderLink<'s,E> where 'a: 's {
        self.inside_border_specific(&c.style_provider().border(&self.style))
    }
    /// fork with area inside the border defined by the style  
    /// default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    pub fn inside_border_by<'s,V>(&'s mut self, tags: V, c: &E::Context) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Clone, 'a: 's {
        self.inside_border_specific(&c.style_provider().border(&self.style.with(tags)))
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn slice<'s>(&'s mut self, s: &Bounds) -> RenderLink<'s,E> where 'a: 's {
        let bounds = self.bounds.slice(s);
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn slice_abs<'s>(&'s mut self, s: &Bounds) -> RenderLink<'s,E> where 'a: 's {
        let bounds = self.bounds & s;
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn inner_centered<'s>(&'s mut self, size: Dims) -> RenderLink<'s,E> where 'a: 's {
        let bounds = self.bounds.inner_centered(size);
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// fork with area inside the bounds
    #[inline]
    pub fn inner_aligned<'s>(&'s mut self, size: Dims, align: (f32,f32)) -> RenderLink<'s,E> where 'a: 's {
        let bounds = self.bounds.inner_aligned(size,align);
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// fork with attached style variant tags
    #[inline]
    pub fn with<'s,V>(&'s mut self, tags: V) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Clone, 'a: 's {
        let style = self.style.with(tags);
        let mut f = self.forked(false,false,true);
        f.style = style;
        f._set_style()
    }
    /// fork with default style and attache tags
    #[inline]
    pub fn with_default_style<'s,V>(&'s mut self, tags: V) -> RenderLink<'s,E> where ESVariant<E>: StyleVariantSupport<V>, V: Clone, 'a: 's {
        let mut f = self.forked(false,false,true);
        f.style = ESVariant::<E>::default().with(tags);
        f._set_style()
    }
    /// get the current color defined by the style variant
    #[inline]
    pub fn color(&self, c: &E::Context) -> ESColor<E> {
        c.style_provider().color(&self.style)
    }

    #[inline]
    pub fn with_bounds<'s>(&'s mut self, bounds: Bounds) -> RenderLink<'s,E> where 'a: 's {
        let mut f = self.forked(true,false,false);
        f.bounds = bounds;
        f._set_bounds()
    }

    #[inline]
    pub fn with_viewport<'s>(&'s mut self, viewport: Bounds) -> RenderLink<'s,E> where 'a: 's {
        let mut f = self.forked(false,true,true);
        f.viewport = viewport;
        f._set_viewport()
    }

    /*#[inline]
    pub fn for_widget<'s,W>(&'s mut self, w: &W, mut border: Border) -> RenderLink<'s,E> where W: Widget<E>, 'a: 's {
        let mut b = self.v.clone();
        let mut v = self.v.clone();
        w.border(&mut b);
        w.style(&mut v);
        
    }*/

    #[inline]
    #[deprecated]
    pub fn render_widget(&mut self, mut w: Link<E>) {
        w.render(self)
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
    
    #[inline]
    pub fn fork_with<'s>(&'s mut self, bounds: Option<Bounds>, viewport: Option<Bounds>, style: Option<ESVariant<E>>) -> RenderLink<'s,E> where 'a: 's {
        let mut r = self.forked(bounds.is_some(),viewport.is_some(),style.is_some());
        if let Some(b) = bounds {
            r.bounds = b;
            r=r._set_bounds();
        }
        if let Some(b) = viewport {
            r.viewport = b;
            r=r._set_viewport();
        }
        if let Some(b) = style {
            r.style = b;
            r=r._set_style();
        }
        r
    }
    #[inline]
    fn forked<'s>(&'s mut self, prev_bounds: bool, prev_viewport: bool, prev_style: bool) -> RenderLink<'s,E> where 'a: 's {
        let mut r = RenderLink{
            r: self.r,
            bounds: self.bounds.clone(),
            prev_bounds: Some(&self.bounds),
            viewport: self.viewport.clone(),
            prev_viewport: Some(&self.viewport),
            style: self.style.clone(),
            prev_style: Some(&self.style),
            force: self.force,
        };
        if !prev_bounds { r.prev_bounds = None; }
        if !prev_viewport { r.prev_viewport = None; }
        if !prev_style { r.prev_style = None; }
        r
    }
    #[inline]
    fn _set_bounds(self) -> Self {
        self.r._set_bounds(&self.bounds);
        self
    }
    #[inline]
    fn _set_viewport(self) -> Self {
        self.r._set_viewport(&self.viewport);
        self
    }
    #[inline]
    fn _set_style(self) -> Self {
        self.r._set_style(&self.style);
        self
    }
}

impl<'a,E> Deref for RenderLink<'a,E> where E: Env {
    type Target = ERenderer<E>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.r
    }
}
impl<'a,E> DerefMut for RenderLink<'a,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r
    }
}

impl<'a,E> Drop for RenderLink<'a,E> where E: Env {
    #[inline(never)]
    fn drop(&mut self) {
        if let Some(v) = &self.prev_bounds {
            self.r._set_bounds(v);
        }
        if let Some(v) = &self.prev_viewport {
            self.r._set_viewport(v);
        }
        if let Some(v) = &self.prev_style {
            self.r._set_style(v);
        }
    }
}