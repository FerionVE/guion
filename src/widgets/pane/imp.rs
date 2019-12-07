use crate::core::ctx::Context;
use super::*;

#[macro_export]
macro_rules! impl_pane {
    ($t:ty) => {
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::IPane<E>, E: $crate::macro_prelude::Context + 'static {
            $crate::impl_pane_inner!($t,E);
        }
    };
}

#[macro_export]
macro_rules! impl_pane_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::widgets::pane::IPane::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$c> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::pane::_render::<$s,$c>,
                event: $crate::widgets::pane::_event::<$s,$c>,
                size: $crate::widgets::pane::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::IPane::invalid(self)
        }
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::IPane::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::IPane::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::IPane::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$c>::WidgetID> + 'a> {
            Box::new(
                $crate::macro_prelude::IPane::childs(self)
                .iter()
                .cloned()
            )
        }
        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<<$c>::WidgetID> {
            $crate::macro_prelude::IPane::childs(self).to_owned()
        }
        #[inline]
        fn selectable(&self) -> bool {
            false
        }
        #[inline]
        fn has_childs(&self) -> bool {
            true
        }
        #[inline]
        fn style(&self) -> E::Style {
            $crate::macro_prelude::IPane::style(self)
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IPane<E> + Widget<E> + 'static, E: Context + 'static>(mut l: Link<E>, r: &mut E::Renderer) {
    let o = l.me::<W>().orientation();
    
    let c = childs::<W,E>(&l);
    
    let b = c.iter()
    .map(|c| 
        l.widget(c)
        .expect("Lost Widget")
        .handler()
        .size(&mut l)
    )
    .collect::<Vec<_>>();
    
    let b = calc_bounds(r.bounds_abs().size, &b[..], o);
    
    for (cc,bb) in c.iter().zip(b.iter()) {
        l.widget(cc)
        .expect("Pane contains lost Widget")
        .handler()
        .render( &mut *l, &mut r.slice(bb) );
    }
    
}

pub fn _event<W: IPane<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: IPane<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    let o = l.me::<W>().orientation();
    
    let mut s = Size::empty();
    
    for c in childs::<W,E>(&l) {
        let cs = l.widget(&c)
        .expect("Lost Widget")
        .handler()
        .size(&mut l);
        
        s.add(&cs, o)
    }
    
    s
}
#[inline]
fn childs<W: IPane<E> + 'static, E: Context + 'static>(l: &Link<E>) -> Vec<E::WidgetID> {
    IPane::childs(l.me::<W>()).to_owned()
}