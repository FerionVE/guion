use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{Deref, Add, DerefMut};

use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::{Query, QueryStack};
use crate::widget::stack::{QueryCurrentBounds, QueriedCurrentBounds, WithCurrentBounds};

use super::*;

pub mod widgets;

pub trait Render<E>: Sized where E: Env {
    //TODO brand new force and caching
}

pub struct StdRenderProps<'a,S,E> where S: ?Sized, E: Env {
    pub inner: &'a S,
    pub absolute_bounds: Bounds,
    pub absolute_viewport: Bounds,
    pub style: TestStyle<'a,E>,
    _p: PhantomData<E>,
}

impl<'a,S,E> StdRenderProps<'a,S,E> where E: Env, S: ?Sized {
    pub fn new(inner: &'a S) -> Self where S: Queron<E> {
        let current_bounds = QueryCurrentBounds.query_in(&*inner).unwrap();
        let style = QueryTestStyle.query_in(&*inner).unwrap();
        Self {
            inner,
            absolute_bounds: current_bounds.bounds.clone(),
            absolute_viewport: current_bounds.viewport.clone(),
            style: style.clone(),
            _p: PhantomData,
        }
    }

    //pub fn with_props<'b,SS>(f: impl FnOnce(&'a S) -> SS) -> StdRenderProps<'b,SS,E> where SS: Queron<E> + ?Sized

    pub fn inside_border(&self, border: impl Borrow<Border>) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.inside_border(border.borrow()),
            ..self.clone()
        }
    }

    pub fn inside_border_mul(&self, border: impl Borrow<Border>, multiplier: u32) -> Self {
        Self {
            absolute_bounds: self.absolute_bounds.inside_border(&(border.borrow() * multiplier)),
            ..self.clone()
        }
    }

    pub fn inside_spacing_border(&self) -> Self {
        self.inside_border_of_type(TestStyleBorderType::Spacing)
    }

    pub fn inside_current_border(&self) -> Self {
        self.inside_border_of_type(self.style.border_type)
    }

    pub fn inside_border_of_type(&self, border_type: TestStyleBorderType<E>) -> Self {
        self.inside_border(self.style.border_of_type(border_type))
    }

    pub fn inside_border_of_type_mul(&self, border_type: TestStyleBorderType<E>, multiplier: u32) -> Self {
        self.inside_border_mul(self.style.border_of_type(border_type), multiplier)
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

    pub fn fork_with(&self, f: impl FnOnce(&mut Self)) -> Self {
        let mut s = self.clone();
        (f)(&mut s);
        s
    }

    pub fn with_style_type(&self, variant_type: TestStyleType<'a,E>) -> Self {
        let mut s = self.clone();
        s.style.variant_type = variant_type;
        s
    }

    pub fn with_style_color_type(&self, color_type: TestStyleColorType<E>) -> Self {
        let mut s = self.clone();
        s.style.color_type = color_type;
        s
    }

    pub fn with_style_border_type(&self, border_type: TestStyleBorderType<E>) -> Self {
        let mut s = self.clone();
        s.style.border_type = border_type;
        s
    }

    //TODO the temporary test style is flawed. e.g. can be selected and hovered, e.g. current stupid style would directly regress as e.g. differenc for border and fg color
    pub fn with_vartype(&self, hovered: bool, selected: bool, activated: bool, disabled: bool) -> Self {
        let mut s = self.clone();
        s.style.variant_type = TestStyleType::Default;
        s.style.variant_type.toggle_id(1, hovered);
        s.style.variant_type.toggle_id(2, selected);
        s.style.variant_type.toggle_id(3, activated);
        s.style.variant_type.toggle_id(4, disabled);
        s
    }
}

impl<'a,S,E> Deref for StdRenderProps<'a,S,E> where E: Env, S: ?Sized {
    type Target = TestStyle<'a,E>;

    fn deref(&self) -> &Self::Target {
        &self.style
    }
}
impl<'a,S,E> DerefMut for StdRenderProps<'a,S,E> where E: Env, S: ?Sized {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

impl<'x,S,E> Queron<E> for StdRenderProps<'x,S,E> where S: Queron<E> + ?Sized, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryCurrentBounds>() {
            *builder = Some(QueriedCurrentBounds{
                bounds: &self.absolute_bounds,
                viewport: &self.absolute_viewport,
            })
        } else if let Some((_,builder)) = builder.downcast::<'_,QueryTestStyle>() {
            *builder = Some(&self.style)
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

impl<'a,S,E> Clone for StdRenderProps<'a,S,E> where S: ?Sized, E: Env {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            absolute_bounds: self.absolute_bounds.clone(),
            absolute_viewport: self.absolute_viewport.clone(),
            _p: PhantomData,
            style: self.style.clone(),
        }
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
    pub variant_type: TestStyleType<'a,E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub color_type: TestStyleColorType<E>,
    pub border_type: TestStyleBorderType<E>,
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

impl<'a,E> TestStyleType<'a,E> where E: Env {
    fn toggle_id(&mut self, idx: u8, v: bool) {
        assert!(idx <= 4);

        let mut current = match self {
            TestStyleType::Default => 0,
            TestStyleType::Hovered => 1,
            TestStyleType::Selected => 2,
            TestStyleType::Activated => 3,
            TestStyleType::Disabled => 4,
            TestStyleType::Custom(_) => 5,
        };

        if v {
            current = current.max(idx);
        } else {
            if current == idx {
                current -= 1;
            }
        }

        match current {
            0 => *self = TestStyleType::Default,
            1 => *self = TestStyleType::Hovered,
            2 => *self = TestStyleType::Selected,
            3 => *self = TestStyleType::Activated,
            4 => *self = TestStyleType::Disabled,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub enum TestStyleColorType<E> where E: Env {
    Bg,
    Fg,
    Border,
    Custom(ESColor<E>),
}

#[derive(Clone,Copy)]
pub enum TestStyleBorderType<E> where E: Env {
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
        test_style.color_type = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<'a,E,S> Add<&'a S> for TestStyleBorderType<E> where S: Queron<E> + 'a, E: Env {
    type Output = WithTestStyle<'a,&'a S,E>;

    fn add(self, rhs: &'a S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
        test_style.border_type = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<'a,E,S> Add<&'a S> for TestStyleType<'a,E> where S: Queron<E> + 'a, E: Env {
    type Output = WithTestStyle<'a,&'a S,E>;

    fn add(self, rhs: &'a S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(rhs).unwrap().clone();
        test_style.variant_type = self;
        WithTestStyle(rhs,test_style)
    }
}

pub fn with_inside_spacing_border<S,E>(stack: S) -> WithCurrentBounds<S> where S: Queron<E>, E: Env {
    with_inside_border_by_type::<S,E>(stack, TestStyleBorderType::Spacing)
}

pub fn with_inside_border_by_type<S,E>(stack: S, border_type: TestStyleBorderType<E>) -> WithCurrentBounds<S> where S: Queron<E>, E: Env {
    let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
    let style = QueryTestStyle.query_in(&stack).unwrap();

    WithCurrentBounds {
        bounds: bounds.bounds.inside_border(&style.border_of_type(border_type)),
        viewport: *bounds.viewport,
        inner: stack,
    }
}

/// For retrieving constraints inside a border. This adds the border to the bounds on the stack and to the returned constraints
pub fn widget_size_inside_border_type<S,F,E>(stack: S, border_type: TestStyleBorderType<E>, func: F) -> ESize<E>
where 
    S: Queron<E>,
    F: FnOnce(WithCurrentBounds<S>) -> ESize<E>,
    E: Env
{
    let style = QueryTestStyle.query_in(&stack).unwrap();
    let border = style.border_of_type(border_type);
    widget_size_inside_border(stack, border, func)
}

/// For retrieving constraints inside a border. This adds the border to the bounds on the stack and to the returned constraints
pub fn widget_size_inside_border<S,F,E>(stack: S, border: Border, func: F) -> ESize<E>
where 
    S: Queron<E>,
    F: FnOnce(WithCurrentBounds<S>) -> ESize<E>,
    E: Env
{
    let bounds = QueryCurrentBounds.query_in(&stack).unwrap();

    let stack = WithCurrentBounds {
        bounds: bounds.bounds.inside_border(&border),
        viewport: *bounds.viewport,
        inner: stack,
    };

    let mut size = (func)(stack);
    size.add_border(&border);

    size
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
    pub fn current_variant(&self) -> &TestStyleVariant<E> {
        self.variant_of_type(self.variant_type)
    }
    pub fn variant_of_type(&self, variant: TestStyleType<'a,E>) -> &'a TestStyleVariant<E> {
        match variant {
            TestStyleType::Default => self.default_variant,
            TestStyleType::Hovered => self.hovered_variant,
            TestStyleType::Selected => self.selected_variant,
            TestStyleType::Activated => self.activated_variant,
            TestStyleType::Disabled => self.disabled_variant,
            TestStyleType::Custom(variant) => variant,
        }
    }

    pub fn current_color(&self) -> ESColor<E> {
        self.color_of_type(self.color_type.clone())
    }
    pub fn color_of_type(&self, color_type: TestStyleColorType<E>) -> ESColor<E> {
        match color_type {
            TestStyleColorType::Bg => self.bg_color.clone(),
            TestStyleColorType::Fg => self.current_variant().fg_color.clone(),
            TestStyleColorType::Border => self.current_variant().border_color.clone(),
            TestStyleColorType::Custom(color) => color,
        }
    }

    pub fn current_border(&self) -> Border {
        self.border_of_type(self.border_type)
    }
    pub fn border_of_type(&self, border_type: TestStyleBorderType<E>) -> Border {
        match border_type {
            TestStyleBorderType::Component => self.component_border,
            TestStyleBorderType::Spacing => self.spacing,
            TestStyleBorderType::Custom(border) => border,
            TestStyleBorderType::PhantomData(_) => todo!(),
        }
    }

    pub fn current(&self) -> TestStyleCurrent<'a,E> {
        let variant = self.current_variant();

        TestStyleCurrent {
            fg_color: variant.fg_color.clone(),
            border_color: variant.border_color.clone(),
            variant: self.variant_type.clone(),
            bg_color: self.bg_color.clone(),
            text_color: self.text_color.clone(),
            component_border: self.component_border,
            spacing: self.spacing,
            cursor: self.cursor.clone(),
            current_color_type: self.color_type.clone(),
            current_color: self.current_color(),
            current_border_type: self.border_type.clone(),
            current_border: self.current_border(),
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
