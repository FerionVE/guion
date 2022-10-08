use std::any::TypeId;
use std::marker::PhantomData;

use super::*;
use super::variants::VariantDesc;

use crate::queron::Queron;
use crate::queron::query::Query;

pub mod select;

#[non_exhaustive]
#[derive(Default)]
pub struct SetStdProps<E> where E: Env {
    font_family: Option<ESFont<E>>,
    accent_color: Option<ESColor<E>>,
}

pub struct WithStdPropsForVariant<S,V,E> where S: Queron<E>, E: Env, V: VariantDesc {
    inner: S,
    set_std_props: SetStdProps<E>,
    _p: PhantomData<V>,
}

#[non_exhaustive]
#[derive(Clone)]
pub struct QueriedStdProps<E> where E: Env {
    font_family: ESFont<E>,
    selected_color: ESColor<E>,
    selected_border: Border,
}

// pub trait StdColorSelector<E> where E: Env {
//     fn select<Q>(&self, q: &Q) -> ESColor<E> where Q: Queron<E> + ?Sized;
// }

// pub trait StdBorderSelector<E> where E: Env {
//     fn select<Q>(&self, q: &Q) -> ESColor<E> where Q: Queron<E> + ?Sized;
// }

// impl<E> StdColorSelector<E> for SelectStdColorType<E> where E: Env {
//     fn select<Q>(&self, q: &Q) -> ESColor<E> where Q: Queron<E> + ?Sized {
//         let s = QueryStdStyle.query_in(q).unwrap();

//         match self {
//             SelectStdColorType::Bg => s.bg,
//             SelectStdColorType::Fg => s.fg,
//             SelectStdColorType::Border => todo!(),
//             SelectStdColorType::Custom(_) => todo!(),
//         }
//     }
// }

#[derive(Clone)]
pub struct QueryStdStyle;

impl<E> Query<E> for QueryStdStyle where E: Env {
    type Out<'b> = QueriedStdProps<E>;
    type Builder<'b> = QueriedStdProps<E>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        todo!()
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        Some(b)
    }
}
