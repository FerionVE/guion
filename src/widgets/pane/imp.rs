use crate::core::ctx::aliases::*;
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
        fn childs(&self) -> Vec<&dyn $crate::macro_prelude::WPProvider<E>> {
            $crate::macro_prelude::IPane::childs(self)
                .iter()
                .map(|p| p as &dyn $crate::macro_prelude::WPProvider<E> )
                .collect()
        }
        #[inline]
        fn childs_mut(&mut self) -> Vec<&mut dyn $crate::macro_prelude::WPProvider<E>> {
            $crate::macro_prelude::IPane::childs_mut(self)
                .iter_mut()
                .map(|p| p as &mut dyn $crate::macro_prelude::WPProvider<E> )
                .collect()
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
        fn style(&self) -> &$crate::macro_prelude::EStyle<E> {
            $crate::macro_prelude::IPane::style(self)
        }
    };
}

pub fn _render<W: IPane<E> + Widget<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: (&mut ERenderer<E>,&Bounds)) {
    let senf = l.me::<W>();
    let o = senf.orientation();
    
    let c = l.child_paths();
    
    let b = c.iter()
    .map(|c| 
        c
        .slice() //TODO remove constant slice() requirement
        .size(&mut l)
        .expect("Lost Widget")
    )
    .collect::<Vec<_>>();
    
    let b = calc_bounds(r.1.size.clone(), &b[..], o);
    
    for (cc,bb) in c.iter().zip(b.iter()) {
        cc
        .slice()
        .render( &mut *l, (r.0,&r.1.slice(bb)) )
        .expect("Pane contains lost Widget");
    }
    
}

pub fn _event<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds)) {
    todo!()
}

pub fn _size<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    let senf = l.me::<W>();
    let o = senf.orientation();
    
    let mut s = Size::empty();
    
    for c in l.child_paths() {
        let cs = c
        .slice()
        .size(&mut l)
        .expect("Lost Widget");
        
        s.add(&cs, o)
    }
    
    s
}