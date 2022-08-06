use std::marker::PhantomData;

use crate::env::Env;
use crate::path::WidgetPath;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::widget::stack::{QueryCurrentWidget, QueryCurrentBounds};

use self::filter::{QueryEventFilterPath, QueryEventFilterPos};

pub mod filter;

#[derive(Clone)]
pub struct QueryEvent<V>(pub PhantomData<V>) where V: Clone + 'static;

pub struct WithEvent<S,V> where V: Clone + 'static {
    pub inner: S,
    pub event: V,
    pub ts: u64,
}

#[derive(Clone)]
pub struct QueriedEvent<'a,V> where V: Clone + 'static {
    pub event: &'a V,
    pub ts: u64,
}

impl<S,V,E> Queron<E> for WithEvent<S,V> where S: Queron<E>, E: Env, V: Clone + 'static {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: crate::queron::query::QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryEvent<V>>() {
            *builder = Some(QueriedEvent{
                event: &self.event,
                ts: self.ts,
            })
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn crate::queron::dyn_tunnel::QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

impl<E,V> Query<E> for QueryEvent<V> where E: Env, V: Clone + 'static {
    type Out<'b> = QueriedEvent<'b,V>;
    type Builder<'b> = Option<QueriedEvent<'b,V>>;

    

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }

    fn query_in<'b,S>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
        if let Some(filter) = QueryEventFilterPath.query_in(stack) {
            if let Some(current_widget) = QueryCurrentWidget.query_in(stack) {
                if !filter.dest_eq(&current_widget.path) { //TODO: incorrect path compare, new path system
                    return None;
                }
            } else {
                return None;
            }
        }

        if let Some(filter_pos) = QueryEventFilterPos.query_in(stack) {
            if let Some(current_pos) = QueryCurrentBounds.query_in(stack) {
                if filter_pos.is_inside(current_pos.bounds) {
                    return None;
                }
            } else {
                return None;
            }
        }

        self._query_direct(stack)
    }
}