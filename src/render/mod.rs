//! [`Render`] functions and a [Link struct](RenderLink) tracking [bounds](Bounds) and [style](Style)
use crate::style::selector::StyleSelectorAppend;

use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env, /*for<'r> ERenderer<'r,E>: AsRefMut<Self>*/ {
    /// If widgets should be rendered even if the don't require to
    fn force(&self) -> bool;
    /// Return false if rendered widgets should not be set rendered
    fn validate_widgets(&mut self) -> bool;

    fn lt<'r>(self) -> ERenderer<'r,E> where Self: 'r;

    /// Fork with [force](Self::force) set
    fn with_force<'r>(&'r mut self, force: bool) -> ERenderer<'r,E> where Self: 'r;

    /// Fork with [force](Self::force) set to true
    fn enforced<'r>(&'r mut self) -> ERenderer<'r,E> where Self: 'r;

    /// Fork with area inside the border
    fn inside_border_specific<'r>(&'r mut self, s: &Border) -> ERenderer<'r,E> where Self: 'r;
    /// Fork with area inside the border defined by the [style](Self::style)
    #[inline]
    fn inside_border<'r>(&'r mut self, c: &mut E::Context) -> ERenderer<'r,E> where Self: 'r {
        self.inside_border_specific(&self.style().border(&self.selector(),c))
    }
    /// Fork with area inside the border defined by the [style](Self::style)  
    /// Default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    fn inside_border_by<'r,S>(&'r mut self, selectags: S, c: &mut E::Context) -> ERenderer<'r,E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E>, Self: 'r { //ESVariant<E>: StyleVariantSupport<V>
        self.inside_border_specific(&self.style().border(&self.selector().with(selectags),c))
    }
    /// Fork with area inside the border defined by the [style](Self::style)  
    /// Default style border is determined by the attached tags which **won't** be present on the forked RenderLink
    #[inline]
    fn inside_border_by_mul<'r,S>(&'r mut self, selectags: S, multiplier: u32, c: &mut E::Context) -> ERenderer<'r,E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E>, Self: 'r { //ESVariant<E>: StyleVariantSupport<V>
        self.inside_border_specific(&(self.style().border(&self.selector().with(selectags),c)*multiplier))
    }

    /// Fork with area inside the [bounds](Self::bounds)
    fn slice<'r>(&'r mut self, s: &Bounds) -> ERenderer<'r,E> where Self: 'r;
    /// Fork with area inside the [bounds](Self::bounds)
    fn slice_abs<'r>(&'r mut self, s: &Bounds) -> ERenderer<'r,E> where Self: 'r;

    /// Fork with area inside the [bounds](Self::bounds)
    fn inner_centered<'r>(&'r mut self, size: Dims) -> ERenderer<'r,E> where Self: 'r;
    /// Fork with area inside the [bounds](Self::bounds)
    fn inner_aligned<'r>(&'r mut self, size: Dims, align: (f32,f32)) -> ERenderer<'r,E> where Self: 'r;

    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    #[inline]
    fn with<'r,S>(&'r mut self, selectags: S) -> ERenderer<'r,E> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E>, Self: 'r {
        self.with_style_selector(&self.selector().with(selectags))
    }
    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    fn with_style<'r>(&'r mut self, style: &EStyle<E>) -> ERenderer<'r,E>;
    /// Fork with attached [style](Self::style) variant [selectors](Self::selector)
    fn with_style_selector<'r>(&'r mut self, style_selector: &ESSelector<E>) -> ERenderer<'r,E>;

    fn with_bounds<'r>(&'r mut self, bounds: Bounds) -> ERenderer<'r,E>;
    fn with_viewport<'r>(&'r mut self, viewport: Bounds) -> ERenderer<'r,E>;

    fn bounds(&self) -> &Bounds;
    fn viewport(&self) -> &Bounds;
    fn style(&self) -> &EStyle<E>;
    fn selector(&self) -> &ESSelector<E>;

    #[deprecated]
    fn render_widget(&mut self, w: Link<E>);

    #[deprecated]
    fn fork_with<'r>(&'r mut self, bounds: Option<Bounds>, viewport: Option<Bounds>, style: Option<EStyle<E>>, selector: Option<ESSelector<E>>) -> ERenderer<'r,E>;
}
