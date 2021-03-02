use selector::{StyleSelector, StyleSelectorAppend};

use super::*;
use std::ops::{DerefMut, Deref};

/// Reference-compound of renderer, current bounds and style
pub struct RenderLink<'a,E> where E: Env {
    pub r: &'a mut ERenderer<E>,

    bounds: Bounds,
    prev_bounds: Option<&'a Bounds>,
    viewport: Bounds,
    prev_viewport: Option<&'a Bounds>,
    style: EStyle<E>,
    prev_style: Option<&'a EStyle<E>>,
    selector: ESSelector<E>,
    prev_selector: Option<&'a ESSelector<E>>,

    /// Whether rendering is enforced (e.g. if invalidation from outside occurred)
    pub force: bool,
}

impl<'a,E> RenderLink<'a,E> where E: Env {
    pub fn new(r: &'a mut ERenderer<E>, bounds: Bounds, viewport: Bounds, style: EStyle<E>, selector: ESSelector<E>, force: bool) -> Self {
        Self{
            r,
            bounds,
            prev_bounds: None,
            viewport,
            prev_viewport: None,
            style,
            prev_style: None,
            selector,
            prev_selector: None,
            force,
        }
            ._set_bounds()
            ._set_style()
            ._set_selector()
            ._set_viewport()
    }
    pub fn simple(r: &'a mut ERenderer<E>, dim: (u32,u32), c: &E::Context) -> Self {
        Self::new(
            r,
            Bounds::from_xywh(0,0,dim.0,dim.1),
            Bounds::from_xywh(0,0,dim.0,dim.1),
            Default::default(),
            Default::default(),
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
    pub fn style(&self) -> &EStyle<E> {
        &self.style
    }
    #[inline]
    pub fn selector(&self) -> &ESSelector<E> {
        &self.selector
    }

    #[inline]
    pub fn force(&self) -> bool {
        self.force || self.r.force(&self.bounds)
    }
    /// Fork with [force](Self::force) set
    #[inline]
    pub fn with_force(&mut self, force: bool) -> RenderLink<E> {
        let mut f = self.forked(false,false,false, false);
        f.force = force;
        f
    }
    /// Fork with [force](Self::force) set to true
    #[inline]
    pub fn enforced(&mut self) -> RenderLink<E> {
        self.with_force(true)
    }

    /// Fork with area inside the border
    #[inline]
    pub fn inside_border_specific(&mut self, s: &Border) -> RenderLink<E> {
        let bounds = self.bounds.inside_border(s);
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// Fork with area inside the border defined by the [style](Self::style)
    #[inline]
    pub fn inside_border(&mut self, c: &mut E::Context) -> RenderLink<E> {
        self.inside_border_specific(&self.style.border(&self.selector,c))
    }
    /// Fork with area inside the border defined by the [style](Self::style)  
    /// Default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    pub fn inside_border_by<S>(&mut self, selectags: S, c: &mut E::Context) -> RenderLink<E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E> { //ESVariant<E>: StyleVariantSupport<V>
        self.inside_border_specific(&self.style.border(&self.selector.with(selectags),c))
    }
    /// Fork with area inside the border defined by the [style](Self::style)  
    /// Default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    pub fn inside_border_by_mul<S>(&mut self, selectags: S, multiplier: u32, c: &mut E::Context) -> RenderLink<E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E> { //ESVariant<E>: StyleVariantSupport<V>
        self.inside_border_specific(&(self.style.border(&self.selector.with(selectags),c)*multiplier))
    }
    /// Fork with area inside the [bounds](Self::bounds)
    #[inline]
    pub fn slice(&mut self, s: &Bounds) -> RenderLink<E> {
        let bounds = self.bounds.slice(s);
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// Fork with area inside the bounds
    #[inline]
    pub fn slice_abs(&mut self, s: &Bounds) -> RenderLink<E> {
        let bounds = self.bounds & s;
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// Fork with area inside the bounds
    #[inline]
    pub fn inner_centered(&mut self, size: Dims) -> RenderLink<E> {
        let bounds = self.bounds.inner_centered(size);
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// Fork with area inside the bounds
    #[inline]
    pub fn inner_aligned(&mut self, size: Dims, align: (f32,f32)) -> RenderLink<E> {
        let bounds = self.bounds.inner_aligned(size,align);
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }
    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    #[inline]
    pub fn with<S>(&mut self, selectags: S) -> RenderLink<E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E> {
        self.with_style_selector(&self.selector.with(selectags))
    }
    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    #[inline]
    pub fn with_style(&mut self, style: &EStyle<E>) -> RenderLink<E> {
        let style = self.style.and(style);
        let mut f = self.forked(false,false,true, false);
        f.style = style;
        f._set_style()
    }
    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    #[inline]
    pub fn with_style_selector(&mut self, style_selector: &ESSelector<E>) -> RenderLink<E> {
        let selector = self.selector.and(style_selector);
        let mut f = self.forked(false,false,false, true);
        f.selector = selector;
        f._set_selector()
    }

    #[inline]
    pub fn with_bounds(&mut self, bounds: Bounds) -> RenderLink<E> {
        let mut f = self.forked(true,false,false, false);
        f.bounds = bounds;
        f._set_bounds()
    }

    #[inline]
    pub fn with_viewport(&mut self, viewport: Bounds) -> RenderLink<E> {
        let mut f = self.forked(false,true,true, false);
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
    pub fn fork_with(&mut self, bounds: Option<Bounds>, viewport: Option<Bounds>, style: Option<EStyle<E>>, selector: Option<ESSelector<E>>) -> RenderLink<E> {
        let mut r = self.forked(bounds.is_some(),viewport.is_some(),style.is_some(), selector.is_some());
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
        if let Some(b) = selector {
            r.selector = b;
            r=r._set_selector();
        }
        r
    }
    #[inline]
    fn forked(&mut self, prev_bounds: bool, prev_viewport: bool, prev_style: bool, prev_selector: bool) -> RenderLink<E> {
        let mut r = RenderLink{
            r: self.r,
            bounds: self.bounds.clone(),
            prev_bounds: Some(&self.bounds),
            viewport: self.viewport.clone(),
            prev_viewport: Some(&self.viewport),
            style: self.style.clone(),
            prev_style: Some(&self.style),
            selector: self.selector.clone(),
            prev_selector: Some(&self.selector),
            force: self.force,
        };
        if !prev_bounds { r.prev_bounds = None; }
        if !prev_viewport { r.prev_viewport = None; }
        if !prev_style { r.prev_style = None; }
        if !prev_selector { r.prev_selector = None; }
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
    #[inline]
    fn _set_selector(self) -> Self {
        self.r._set_selector(&self.selector);
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
