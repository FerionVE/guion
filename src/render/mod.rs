use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{Deref, Add};

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

pub struct WithTestStyle<'a,S,E>(pub S,pub TestStyle<'a,E>) where E: Env;

impl<'a,S,E> Deref for WithTestStyle<'a,S,E> where E: Env {
    type Target = TestStyle<'a,E>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

#[derive(Clone)]
pub struct TestStyle<'a,E> where E: Env {
    pub default_variant: &'a TestStyleVariant<E>,
    pub hovered_variant: &'a TestStyleVariant<E>,
    pub selected_variant: &'a TestStyleVariant<E>,
    pub activated_variant: &'a TestStyleVariant<E>,
    pub disabled_variant: &'a TestStyleVariant<E>,
    pub variant: TestStyleType<'a,E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub current_color: TestStyleColorType<E>,
    pub current_border: TestStyleBorderType<E>,
}

#[derive(Clone)]
pub struct TestStyleCurrent<'a,E> where E: Env {
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
    pub variant: TestStyleType<'a,E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub current_color_type: TestStyleColorType<E>,
    pub current_color: ESColor<E>,
    pub current_border_type: TestStyleBorderType<E>,
    pub current_border: Border,
}

#[derive(Clone,Copy)]
pub enum TestStyleType<'a,E> where E: Env {
    Default,
    Hovered,
    Selected,
    Activated,
    Disabled,
    Custom(&'a TestStyleVariant<E>),
}

#[derive(Clone)]
pub enum TestStyleColorType<E> where E: Env {
    Bg,
    Fg,
    Border,
    Custom(ESColor<E>),
}

#[derive(Clone)]
pub enum TestStyleBorderType<E> {
    Component,
    Spacing,
    Custom(Border),
    PhantomData(E),
}

impl<'a,E,S> Add<S> for TestStyle<'a,E> where S: Queron<E>, E: Env {
    type Output = WithTestStyle<'a,S,E>;

    fn add(self, rhs: S) -> Self::Output {
        WithTestStyle(rhs,self)
    }
}

impl<'a,E,S> Add<&'a S> for TestStyleColorType<E> where S: Queron<E> + 'a, E: Env {
    type Output = WithTestStyle<'a,&'a S,E>;

    fn add(self, rhs: &'a S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
        test_style.current_color = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<'a,E,S> Add<&'a S> for TestStyleBorderType<E> where S: Queron<E> + 'a, E: Env {
    type Output = WithTestStyle<'a,&'a S,E>;

    fn add(self, rhs: &'a S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
        test_style.current_border = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<'a,E,S> Add<&'a S> for TestStyleType<'a,E> where S: Queron<E> + 'a, E: Env {
    type Output = WithTestStyle<'a,&'a S,E>;

    fn add(self, rhs: &'a S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
        test_style.variant = self;
        WithTestStyle(rhs,test_style)
    }
}

// impl<'a,E,S> Add<&'a S> for TestStyleColorType<E> where S: Queron<E> + 'a, E: Env {
//     type Output = WithTestStyle<'a,&'a S,E>;

//     fn add(self, rhs: &'a S) -> Self::Output {
//         let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
//         test_style.current_color = self;
//         WithTestStyle(rhs,test_style)
//     }
// }

// impl<E> TestStyleColorType<E> where E: Env {
//     pub fn with<'a,S>(&self, stack: S) -> WithTestStyle<'a,S,E> where S: Queron<E> + 'a {
//         let mut test_style = QueryTestStyle.query_in(&stack).unwrap().clone();
//         test_style.current_color = self.clone();
//         WithTestStyle(stack,test_style)
//     }
// }

pub struct TestStyleVariant<E> where E: Env {
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
}

impl<'a,E> TestStyle<'a,E> where E: Env {
    pub fn current(&self) -> TestStyleCurrent<'a,E> {
        let variant = match self.variant {
            TestStyleType::Default => self.default_variant,
            TestStyleType::Hovered => self.hovered_variant,
            TestStyleType::Selected => self.selected_variant,
            TestStyleType::Activated => self.activated_variant,
            TestStyleType::Disabled => self.disabled_variant,
            TestStyleType::Custom(variant) => variant,
        };

        let current_color = match &self.current_color {
            TestStyleColorType::Bg => &self.bg_color,
            TestStyleColorType::Fg => &variant.fg_color,
            TestStyleColorType::Border => &variant.border_color,
            TestStyleColorType::Custom(color) => color,
        }.clone();

        let current_border = match &self.current_border {
            TestStyleBorderType::Component => &self.component_border,
            TestStyleBorderType::Spacing => &self.spacing,
            TestStyleBorderType::Custom(border) => border,
            TestStyleBorderType::PhantomData(_) => todo!(),
        }.clone();

        TestStyleCurrent {
            fg_color: variant.fg_color.clone(),
            border_color: variant.border_color.clone(),
            variant: self.variant.clone(),
            bg_color: self.bg_color.clone(),
            text_color: self.text_color.clone(),
            component_border: self.component_border,
            spacing: self.spacing,
            cursor: self.cursor.clone(),
            current_color_type: self.current_color.clone(),
            current_color,
            current_border_type: self.current_border.clone(),
            current_border,
        }
    }
}

impl<'x,S,E> Queron<E> for WithTestStyle<'x,S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryTestStyle>() {
            *builder = Some(&self.1)
        } else {
            self.0._query(builder)
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
    type Out<'b> = &'b TestStyle<'b,E>;
    type Builder<'b> = Option<&'b TestStyle<'b,E>>;

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
    type Out<'b> = TestStyleCurrent<'b,E>;
    type Builder<'b> = Option<TestStyleCurrent<'b,E>>;

    fn query_in<'b,S>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
        QueryTestStyle.query_in(stack).map(|test_style| test_style.current() )
    }

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        panic!()
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        panic!()
    }
}
