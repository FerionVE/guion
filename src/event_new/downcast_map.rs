use crate::EventResp;
use crate::env::Env;
use crate::queron::Queron;
use crate::widget::Widget;

pub trait EventDowncastMap<E> where E: Env {
    fn event_downcast_map<W,S,Evt>(
        widget: &W,
        stack: &S,
        event: &Evt,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, S: Queron<E> + ?Sized, Evt: crate::event_new::Event<E> + ?Sized;
}

#[macro_export]
macro_rules! event_downcast_map_tryion {
    (
        $widget:ident,
        $stack:ident,
        $event:ident,
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
                return $crate::widget::Widget::event_direct($widget,$stack,$event,$cache,$root,$ctx);
            }
        );*
    };
}

impl<E> EventDowncastMap<E> for () where E: Env {
    #[inline]
    fn event_downcast_map<W,S,Evt>(
        widget: &W,
        stack: &S,
        event: &Evt,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, S: Queron<E> + ?Sized, Evt: crate::event_new::Event<E> + ?Sized
    {
        widget.event_direct(stack, event, cache, root, ctx)
    }
}
