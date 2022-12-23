use crate::EventResp;
use crate::env::Env;
use crate::newpath::{PathResolvusDyn, PathStack};
use crate::queron::Queron;
use crate::widget::Widget;

/// The "EDM" EventDowncastMap is an experiment to eventually improve performance,
/// to downcast the most frequent events so that specific code can be generated for that event between the typed widget zone between the dyn borders.
/// This can reduce the amount of event downcasting inside the individual widgets and
/// optimize away the code for the other events in the code for that specific event.
pub trait EventDowncastMap<E> where E: Env {
    fn event_downcast_map<W,Ph,S,Evt>(
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: crate::event_new::Event<E> + ?Sized;
}

#[macro_export]
macro_rules! event_downcast_map_tryion {
    (
        $widget:ident,
        $path:ident,
        $stack:ident,
        $event:ident,
        $route_to_widget:ident,
        $cache:ident,
        $root:ident,
        $ctx:ident;
        $(
            $dest_type:ty
        );*
    ) => {
        let __try_downcast_event: &dyn std::any::Any = $crate::event_new::Event::_as_any($event);
        $(
            if let Some($event) = __try_downcast_event.downcast_ref::<$dest_type>() {
                //eprintln!("EVENT DOWNCAST MAP {}",std::any::type_name::<$dest_type>());
                return $crate::widget::Widget::event_direct($widget,$path,$stack,$event,$route_to_widget,$cache,$root,$ctx);
            }
        );*
    };
}

impl<E> EventDowncastMap<E> for () where E: Env {
    #[inline]
    fn event_downcast_map<W,Ph,S,Evt>(
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: crate::event_new::Event<E> + ?Sized
    {
        widget.event_direct(path, stack, event, route_to_widget, root, ctx)
    }
}
