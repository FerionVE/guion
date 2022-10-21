use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Add};

use crate::aliases::{ESColor, ESCursor, ESize};
use crate::env::Env;
use crate::layout::Gonstraints;
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::{QueryStack, Query};
use crate::style::color::Color;
use crate::util::border::Border;
use crate::util::bounds::{Bounds, Dims};
use crate::widget::cache::StdRenderCachors;
use crate::widget::stack::{QueryCurrentBounds, QueriedCurrentBounds, WithCurrentBounds};

pub mod widgets;

pub trait Render<E>: Sized where E: Env {
    //TODO brand new force and caching
}

#[non_exhaustive]
pub struct StdRenderProps<'a,S,E,C> where S: ?Sized, E: Env, C: PartialEq + Clone + 'static {
    pub inner: &'a S,
    pub absolute_bounds: Bounds,
    pub absolute_viewport: Bounds,
    pub style: TestStyle<E>,
    just_cachors: C,
    _p: PhantomData<E>,
}

impl<'a,S,E> StdRenderProps<'a,S,E,()> where E: Env, S: ?Sized {
    pub fn new(inner: &'a S) -> Self where S: Queron<E> {
        let current_bounds = QueryCurrentBounds.query_in(&*inner).unwrap();
        let style = QueryTestStyle.query_in(&*inner).unwrap();
        Self {
            inner,
            absolute_bounds: current_bounds.bounds.clone(),
            absolute_viewport: current_bounds.viewport.clone(),
            style: style.clone(),
            just_cachors: (),
            _p: PhantomData,
        }
    }
}

impl<'a,S,E,C> StdRenderProps<'a,S,E,C> where E: Env, S: ?Sized, C: PartialEq + Clone + 'static {
    pub fn current_std_render_cachors(&self) -> StdRenderCachors<E> {
        let current_style = self.style.current();
        StdRenderCachors {
            dims: self.absolute_bounds.size,
            fg_color: current_style.fg_color,
            border_color: current_style.border_color,
            bg_color: current_style.bg_color,
            text_color: current_style.text_color,
            component_border: current_style.component_border,
            spacing: current_style.spacing,
            current_color: current_style.current_color,
            current_border: current_style.current_border,
        }
    }

    fn just_cachors(&self) -> (Bounds,C) {
        (self.absolute_bounds,self.just_cachors.clone()) // viewport isn't in child cachors
    }

    //pub fn with_props<'b,SS>(f: impl FnOnce(&'a S) -> SS) -> StdRenderProps<'b,SS,E> where SS: Queron<E> + ?Sized

    fn _and_cachor<CC>(self, c: CC) -> StdRenderProps<'a,S,E,(C,CC)> where CC: PartialEq + Clone + 'static {
        StdRenderProps {
            inner: self.inner,
            absolute_bounds: self.absolute_bounds,
            absolute_viewport: self.absolute_viewport,
            style: self.style,
            just_cachors: (self.just_cachors,c),
            _p: PhantomData,
        }
    }

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

    pub fn with_style_type(&self, variant_type: TestStyleVariant<E>) -> Self {
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
        s.style.variant_type = TestStyleVariant {
            disabled,
            hovered,
            selected,
            activated,
            ..Default::default()
        };
        s
    }
}

impl<'a,S,E,C> Deref for StdRenderProps<'a,S,E,C> where E: Env, S: ?Sized, C: PartialEq + Clone + 'static {
    type Target = TestStyle<E>;

    fn deref(&self) -> &Self::Target {
        &self.style
    }
}
impl<'a,S,E,C> DerefMut for StdRenderProps<'a,S,E,C> where E: Env, S: ?Sized, C: PartialEq + Clone + 'static {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

impl<'x,S,E,C> Queron<E> for StdRenderProps<'x,S,E,C> where S: Queron<E> + ?Sized, E: Env, C: PartialEq + Clone + 'static {
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

impl<'a,S,E,C> Clone for StdRenderProps<'a,S,E,C> where S: ?Sized, E: Env, C: PartialEq + Clone + 'static {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            absolute_bounds: self.absolute_bounds.clone(),
            absolute_viewport: self.absolute_viewport.clone(),
            _p: PhantomData,
            style: self.style.clone(),
            just_cachors: self.just_cachors.clone()
        }
    }
}

pub struct WithTestStyle<S,E>(pub S,pub TestStyle<E>) where E: Env;

impl<'a,S,E> Deref for WithTestStyle<S,E> where E: Env {
    type Target = TestStyle<E>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

#[non_exhaustive]
#[derive(Clone)]
pub struct TestStyle<E> where E: Env {
    pub default_border_color: ESColor<E>,
    pub default_fg_color: ESColor<E>,
    pub disabled_border_color: ESColor<E>,
    pub disabled_fg_color: ESColor<E>,
    pub hovered_border_color: Option<ESColor<E>>,
    pub hovered_fg_color: Option<ESColor<E>>,
    pub selected_border_color: Option<ESColor<E>>,
    pub selected_fg_color: Option<ESColor<E>>,
    pub activated_border_color: Option<ESColor<E>>,
    pub activated_fg_color: Option<ESColor<E>>,
    pub variant_type: TestStyleVariant<E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub color_type: TestStyleColorType<E>,
    pub border_type: TestStyleBorderType<E>,
}

#[non_exhaustive]
#[derive(Clone)]
pub struct TestStyleCurrent<E> where E: Env {
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub current_variant: TestStyleVariant<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub current_color_type: TestStyleColorType<E>,
    pub current_color: ESColor<E>,
    pub current_border_type: TestStyleBorderType<E>,
    pub current_border: Border,
}

#[non_exhaustive]
#[derive(Clone,Copy,Default)]
pub struct TestStyleVariant<E> {
    pub disabled: bool,
    pub hovered: bool,
    pub selected: bool,
    pub activated: bool,
    _p: PhantomData<E>,
}

#[non_exhaustive]
#[derive(Clone)]
pub enum TestStyleColorType<E> where E: Env {
    Bg,
    Fg,
    Border,
    Custom(ESColor<E>),
}

impl<E> From<[u8;4]> for TestStyleColorType<E> where E: Env {
    #[inline]
    fn from(v: [u8;4]) -> Self {
        Self::Custom(Color::from_rgba8(v))
    }
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum TestStyleBorderType<E> where E: Env {
    Component,
    Spacing,
    Custom(Border),
    PhantomData(E),
}

impl<E> From<Border> for TestStyleBorderType<E> where E: Env {
    #[inline]
    fn from(v: Border) -> Self {
        Self::Custom(v)
    }
}

impl<E,S> Add<S> for TestStyle<E> where S: Queron<E>, E: Env {
    type Output = WithTestStyle<S,E>;

    fn add(self, rhs: S) -> Self::Output {
        WithTestStyle(rhs,self)
    }
}

impl<E,S> Add<S> for TestStyleColorType<E> where S: Queron<E>, E: Env {
    type Output = WithTestStyle<S,E>;

    fn add(self, rhs: S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(&rhs).unwrap().clone();
        test_style.color_type = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<E,S> Add<S> for TestStyleBorderType<E> where S: Queron<E>, E: Env {
    type Output = WithTestStyle<S,E>;

    fn add(self, rhs: S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(&rhs).unwrap().clone();
        test_style.border_type = self;
        WithTestStyle(rhs,test_style)
    }
}

impl<E,S> Add<S> for TestStyleVariant<E> where S: Queron<E>, E: Env {
    type Output = WithTestStyle<S,E>;

    fn add(self, rhs: S) -> Self::Output {
        let mut test_style = QueryTestStyle.query_in(&rhs).unwrap().clone();
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

impl<E> TestStyle<E> where E: Env {
    pub fn current_variant(&self) -> (ESColor<E>,ESColor<E>) {
        self.variant_of_type(&self.variant_type)
    }
    pub fn variant_of_type(&self, variant: &TestStyleVariant<E>) -> (ESColor<E>,ESColor<E>) {
        let (mut fg,mut border) = (self.default_fg_color.clone(),self.default_border_color.clone());
        if variant.disabled {
            return (self.disabled_fg_color.clone(),self.disabled_border_color.clone())
        }
        let mut set = |fg_ref: &Option<ESColor<E>>,border_ref: &Option<ESColor<E>>| {
            if let Some(v) = fg_ref.clone() {
                fg = v;
            }
            if let Some(v) = border_ref.clone() {
                border = v;
            }
        };
        if variant.hovered {
            set(&self.hovered_fg_color,&self.hovered_border_color);
        }
        if variant.selected {
            set(&self.selected_fg_color,&self.selected_border_color);
        }
        if variant.activated {
            set(&self.activated_fg_color,&self.activated_border_color);
        }
        (fg,border)
    }

    pub fn current_color(&self) -> ESColor<E> {
        self.color_of_type(self.color_type.clone())
    }
    pub fn color_of_type(&self, color_type: TestStyleColorType<E>) -> ESColor<E> {
        match color_type {
            TestStyleColorType::Bg => self.bg_color.clone(),
            TestStyleColorType::Fg => self.current_variant().0.clone(),
            TestStyleColorType::Border => self.current_variant().1.clone(),
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

    pub fn current(&self) -> TestStyleCurrent<E> {
        let (fg_color,border_color) = self.current_variant();

        TestStyleCurrent {
            fg_color,
            border_color,
            current_variant: self.variant_type.clone(),
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

impl<S,E> Queron<E> for WithTestStyle<S,E> where S: Queron<E>, E: Env {
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
    type Out<'b> = &'b TestStyle<E>;
    type Builder<'b> = Option<&'b TestStyle<E>>;

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

    fn query_in<'b,S>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
        QueryTestStyle.query_in(stack).map(|test_style| test_style.current() )
    }

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        panic!()
    }
    #[inline]
    fn end_builder<'b>(&self, _: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        panic!()
    }
}

pub struct TestStyleV1<E> where E: Env {
    pub default_border_color: ESColor<E>,
    pub default_fg_color: ESColor<E>,
    pub disabled_border_color: ESColor<E>,
    pub disabled_fg_color: ESColor<E>,
    pub hovered_border_color: Option<ESColor<E>>,
    pub hovered_fg_color: Option<ESColor<E>>,
    pub selected_border_color: Option<ESColor<E>>,
    pub selected_fg_color: Option<ESColor<E>>,
    pub activated_border_color: Option<ESColor<E>>,
    pub activated_fg_color: Option<ESColor<E>>,
    pub current_variant: TestStyleVariant<E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub cursor: ESCursor<E>,
    pub color_type: TestStyleColorType<E>,
    pub border_type: TestStyleBorderType<E>,
}

impl<E> From<TestStyleV1<E>> for TestStyle<E> where E: Env {
    #[inline]
    fn from(value: TestStyleV1<E>) -> Self {
        Self {
            bg_color: value.bg_color,
            text_color: value.text_color,
            component_border: value.component_border,
            spacing: value.spacing,
            cursor: value.cursor,
            color_type: value.color_type,
            border_type: value.border_type,
            default_border_color: value.default_border_color,
            default_fg_color: value.default_fg_color,
            disabled_border_color: value.disabled_border_color,
            disabled_fg_color: value.disabled_fg_color,
            hovered_border_color: value.hovered_border_color,
            hovered_fg_color: value.hovered_fg_color,
            selected_border_color: value.selected_border_color,
            selected_fg_color: value.selected_fg_color,
            activated_border_color: value.activated_border_color,
            activated_fg_color: value.activated_fg_color,
            variant_type: value.current_variant,
        }
    }
}
