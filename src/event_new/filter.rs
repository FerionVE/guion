use std::marker::PhantomData;
use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::queron::query::Query;
use crate::util::bounds::Offset;

/// Widget event handler do need to query receive_self before querying event types, or they may get events not supposed to get
#[derive(Clone,Debug)]
pub struct StdEventMode<E> where E: Env {
    /// Whether the current widget receives the event
    /// 
    /// The event may still be routed to childs if [`route_to_childs`] is set
    pub receive_self: bool,
    /// Whether this event should be routed to child widgets
    /// 
    /// The event may still be received by the current widget if [`receive_self`] is set
    /// 
    /// child_filters are only relevant if [`route_to_childs`] is set
    /// 
    /// If no child_filters are set, the event will be routed to all childs
    pub route_to_childs: bool,
    //pub child_filter_visible: bool,
    /// If set, the event only needs be routed to childs which bounds overlap with the point
    /// 
    /// child_filters may be ignored by widgets and events routed to all childs
    /// 
    /// Only relevent if route_to_childs is enabled
    /// 
    /// If no child_filters are set, the event will be routed to all childs
    pub child_filter_point: Option<Offset>,
    // /// If set, the event only needs be routed to the widget and maybe also it's childs with the specified path
    // /// 
    // /// child_filters may be ignored by widgets and events routed to all childs
    // /// 
    // /// Only relevent if route_to_childs is enabled
    // /// 
    // /// If no child_filters are set, the event will be routed to all childs
    // pub child_filter_absolute_path: Option<Arc<dyn PathResolvusDyn<E>>>,
    _p: PhantomData<E>,
}

#[derive(Clone)]
pub struct QueryStdEventMode;

impl<E> Query<E> for QueryStdEventMode where E: Env {
    type Out<'b> = StdEventMode<E>;
    type Builder<'b> = Option<StdEventMode<E>>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}

/// Query for legacy variant
#[derive(Clone)]
pub struct QueryVariant<V>(pub PhantomData<V>) where V: Clone + 'static;

impl<V,E> Query<E> for QueryVariant<V> where V: Clone + 'static, E: Env {
    type Out<'b> = &'b V;
    type Builder<'b> = Option<&'b V>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}
