use crate::core::ctx::Context;
use super::*;
#[macro_export]
macro_rules! impl_pane {
    ($senf:ty,$Context:ty) => {
        #[inline]
        fn id(&self) -> <$Context>::WidgetID {
            $crate::widgets::pane::IPane::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$Context> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::pane::_render::<$senf,$Context>,
                event: $crate::widgets::pane::_event::<$senf,$Context>,
                size: $crate::widgets::pane::_size::<$senf,$Context>,
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
        fn parent(&self) -> Option<&<$Context>::WidgetID> {
            $crate::macro_prelude::IPane::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$Context>::WidgetID>) {
            $crate::macro_prelude::IPane::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$Context>::WidgetID> + 'a> {
            Box::new(
                $crate::macro_prelude::IPane::childs(self)
                .iter()
                .cloned()
            )
        }
        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<<$Context>::WidgetID> {
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
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IPane<E> + Widget<E> + 'static, E: Context + 'static>(mut l: Link<E>, mut r: E::Renderer) {
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
        .render( &mut *l, r.slice(bb) );
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