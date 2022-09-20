use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::{Query, QueryStack};
use crate::widget::stack::{QueryCurrentBounds, QueriedCurrentBounds};

use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env {
    //TODO brand new force and caching
}

pub struct StdRenderProps<'a,S,E> {
    pub inner: &'a S,
    pub absolute_bounds: Bounds,
    pub absolute_viewport: Bounds,
    _p: PhantomData<E>,
}

impl<'a,S,E> StdRenderProps<'a,S,E> where E: Env {
    pub fn new(inner: &'a S) -> Self where S: Queron<E> {
        let current_bounds = QueryCurrentBounds.query_in(&*inner).unwrap();
        Self {
            inner,
            absolute_bounds: current_bounds.bounds.clone(),
            absolute_viewport: current_bounds.viewport.clone(),
            _p: PhantomData,
        }
    }

    //pub fn with_props<'b,SS>(f: impl FnOnce(&'a S) -> SS) -> StdRenderProps<'b,SS,E> where SS: Queron<E>

    pub fn inside_border(&self, border: impl Borrow<Border>) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.inside_border(border.borrow()),
            ..self.clone()
        }
    }

    pub fn slice(&self, slice_relative: impl Borrow<Bounds>) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.slice(slice_relative.borrow()),
            ..self.clone()
        }
    }

    pub fn slice_absolute(&self, slice_absolute: impl Borrow<Bounds>) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds & slice_absolute.borrow(),
            ..self.clone()
        }
    }

    pub fn inner_centered(&self, size_of_inner: impl Borrow<Dims>) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.inner_centered(size_of_inner.borrow().clone()),
            ..self.clone()
        }
    }

    pub fn inner_aligned(&self, size_of_inner: impl Borrow<Dims>, align: (f32,f32)) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.inner_aligned(size_of_inner.borrow().clone(),align),
            ..self.clone()
        }
    }
}

impl<'x,S,E> Queron<E> for StdRenderProps<'x,S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryCurrentBounds>() {
            *builder = Some(QueriedCurrentBounds{
                bounds: &self.absolute_bounds,
                viewport: &self.absolute_viewport,
            })
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

impl<'a,S,E> Clone for StdRenderProps<'a,S,E> {
    fn clone(&self) -> Self {
        Self { inner: self.inner, absolute_bounds: self.absolute_bounds.clone(), absolute_viewport: self.absolute_viewport.clone(), _p: PhantomData }
    }
}

#[derive(Clone)]
pub struct TestStyle<'a,S,E> where E: Env {
    pub inner: &'a S,
    pub default_variant: &'a TestStyleVariant<E>,
    pub hovered_variant: &'a TestStyleVariant<E>,
    pub selected_variant: &'a TestStyleVariant<E>,
    pub disabled_variant: &'a TestStyleVariant<E>,
    pub variant: TestStyleType,
    pub bg_color: ESColor<E>,
    pub border: Border,
    pub cursor: ESCursor<E>,
}

#[derive(Clone)]
pub struct TestStyleCurrent<E> where E: Env {
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
    pub variant: TestStyleType,
    pub bg_color: ESColor<E>,
    pub border: Border,
    pub cursor: ESCursor<E>,
}

#[derive(Clone,Copy)]
pub enum TestStyleType {
    Default,
    Hovered,
    Selected,
    Disabled,
}

pub struct TestStyleVariant<E> where E: Env {
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
}

impl<'a,S,E> TestStyle<'a,S,E> where E: Env {
    pub fn new(inner: &'a S) -> Self where S: Queron<E> {
        let test_style = QueryTestStyle.query_in(&*inner).unwrap();
        Self {
            inner,
            default_variant: test_style.default_variant,
            hovered_variant: test_style.hovered_variant,
            selected_variant: test_style.selected_variant,
            disabled_variant: test_style.disabled_variant,
            variant: test_style.variant,
            bg_color: test_style.bg_color,
            border: test_style.border,
            cursor: test_style.cursor,
        }
    }

    pub fn current(&self) -> TestStyleCurrent<E> {
        let variant = match self.variant {
            TestStyleType::Default => self.default_variant,
            TestStyleType::Hovered => self.hovered_variant,
            TestStyleType::Selected => self.selected_variant,
            TestStyleType::Disabled => self.disabled_variant,
        };

        TestStyleCurrent {
            fg_color: variant.fg_color.clone(),
            border_color: variant.border_color.clone(),
            variant: self.variant,
            bg_color: self.bg_color.clone(),
            border: self.border,
            cursor: self.cursor.clone(),
        }
    }
}

impl<'x,S,E> Queron<E> for TestStyle<'x,S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryTestStyleCurrent>() {
            *builder = Some(self.current())
        } else if let Some((_,builder)) = builder.downcast::<'_,QueryTestStyle>() {
            *builder = Some(TestStyle{
                inner: &(),
                default_variant: self.default_variant,
                hovered_variant: self.hovered_variant,
                selected_variant: self.selected_variant,
                disabled_variant: self.disabled_variant,
                variant: self.variant,
                bg_color: self.bg_color.clone(),
                border: self.border,
                cursor: self.cursor.clone(),
            })
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[derive(Clone)]
pub struct QueryTestStyle;

impl<E> Query<E> for QueryTestStyle where E: Env {
    type Out<'b> = TestStyle<'b,(),E>;
    type Builder<'b> = Option<TestStyle<'b,(),E>>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}

#[derive(Clone)]
pub struct QueryTestStyleCurrent;

impl<E> Query<E> for QueryTestStyleCurrent where E: Env {
    type Out<'b> = TestStyleCurrent<E>;
    type Builder<'b> = Option<TestStyleCurrent<E>>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}
