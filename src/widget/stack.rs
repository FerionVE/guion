use crate::env::Env;
use crate::queron::Queron;
use crate::queron::query::Query;

use super::Widget;

pub struct WithCurrentWidget<S,E> where E: Env {
    pub inner: S,
    pub path: E::WidgetPath,
    pub id: E::WidgetID,
}

impl<S,E> Queron<E> for WithCurrentWidget<S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: crate::queron::query::QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((query,builder)) = builder.downcast::<'_,QueryCurrentWidget>() {
            *builder = Some(QueriedCurrentWidget{
                path: &self.path,
                id: self.id.clone(),
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

#[derive(Clone)]
pub struct QueriedCurrentWidget<'a,E> where E: Env {
    pub path: &'a E::WidgetPath,
    pub id: E::WidgetID,
}

#[derive(Clone)]
pub struct QueryCurrentWidget;

impl<E> Query<E> for QueryCurrentWidget where E: Env {
    type Out<'b> = QueriedCurrentWidget<'b,E>;
    type Builder<'b> = Option<QueriedCurrentWidget<'b,E>>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}

#[inline]
pub fn for_child_widget<Q,W,E>(query: Q, child_widget: &W) -> WithCurrentWidget<Q,E> where Q: Queron<E>, W: Widget<E> + ?Sized, E: Env {
    let parent = QueryCurrentWidget.query_in(&query).unwrap();

    let new_path = child_widget.in_parent_path(parent.path.clone());
    let new_id = child_widget.id();

    WithCurrentWidget {
        inner: query,
        path: new_path,
        id: new_id,
    }
}
