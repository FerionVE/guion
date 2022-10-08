use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::QueryStack;

use super::*;

#[non_exhaustive]
#[derive(Clone)]
pub enum SelectStdColorType<E> where E: Env {
    Bg,
    Fg,
    Border,
    Custom(ESColor<E>),
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum SelectStdBorderType<E> where E: Env {
    Component,
    Spacing,
    Custom(Border),
    PhantomData(E),
}

pub struct WithSelectStdColorType<S,E> where S: Queron<E>, E: Env {
    inner: S,
    select: SelectStdColorType<E>,
    _p: PhantomData<E>,
}

impl<'x,S,E> Queron<E> for WithSelectStdColorType<S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        self.inner._query(builder.fork());
        if let Some((_,builder)) = builder.downcast::<'_,QueryStdStyle>() {
            builder.selected_color = match self.select.clone() {
                SelectStdColorType::Bg => todo!(),
                SelectStdColorType::Fg => todo!(),
                SelectStdColorType::Border => todo!(),
                SelectStdColorType::Custom(c) => c,
            };
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}
