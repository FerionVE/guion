use crate::core::*;
use ctx::*;
use super::*;

#[macro_export]
macro_rules! impl_pane {
    ($t:ty) => {
        #[doc(hidden)]
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::IPane<E>, E: $crate::macro_prelude::Env + 'static {
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
        fn _fns(&self) -> $crate::macro_prelude::WidgetFns<$c> {
            $crate::macro_prelude::WidgetFns{
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
        fn style(&self) -> &E::Style {
            $crate::macro_prelude::IPane::style(self)
        }
    };
}

pub fn _render<W: IPane<E> + Widget<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: (&mut E::Renderer,&Bounds)) {
    let o = l.me::<W>().orientation();
    
    let c = childs::<W,E>(&l);
    
    let b = c.iter()
    .map(|c| 
        c
        .size::<E>(&mut l)
        .expect("Lost Widget")
    )
    .collect::<Vec<_>>();
    
    let b = calc_bounds(r.1.size.clone(), &b[..], o);
    
    for (cc,bb) in c.iter().zip(b.iter()) {
        cc
        .render::<E>( &mut *l, (r.0,&r.1.slice(bb)) )
        .expect("Pane contains lost Widget");
    }
    
}

pub fn _event<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    let o = l.me::<W>().orientation();
    
    let mut s = Size::empty();
    
    for c in childs::<W,E>(&l) {
        let cs = c
        .size::<E>(&mut l)
        .expect("Lost Widget");
        
        s.add(&cs, o)
    }
    
    s
}
#[inline]
fn childs<W: IPane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<E::WidgetID> {
    IPane::childs(l.me::<W>()).to_owned()
}