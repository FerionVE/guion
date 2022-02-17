use std::any::TypeId;

use crate::env::Env;
use crate::widget::Widget;

use super::{TraitcastImpl, TraitcastImplBase, TraitObject};

pub trait TraitcastWidget<E>: Widget<E> where E: Env {
    #[inline]
    fn try_traitcast_ref<'a,'b,T>(&'a self) -> Result<&'a T,()> where T: ?Sized + 'b, dyn Widget<E>+'b: TraitcastImpl<'b,T> + 'b, Self: 'b, 'b: 'a {
        if let Ok(e) = _try_traitcast_ref(self.erase()) {
            Ok(e)
        } else {
            if let Some(s) = self.inner() {
                s.try_traitcast_ref()
            } else {
                Err(())
            }
        }
    }
    #[inline]
    fn try_traitcast_ref_nonrecursive<'a,'b,T>(&'a self) -> Result<&'a T,()> where T: ?Sized + 'b, dyn Widget<E>+'b: TraitcastImpl<'b,T> + 'b, Self: 'b, 'b: 'a {
        _try_traitcast_ref(self.erase())
    }
}

impl<T,E> TraitcastWidget<E> for T where T: Widget<E> + ?Sized, E: Env {

}

#[inline]
fn _try_traitcast_ref<'a,'b,T,E>(s: &'a (dyn Widget<E>+'b)) -> Result<&'a T,()> where T: ?Sized + 'b, dyn Widget<E>+'b: TraitcastImpl<'b,T> + 'b, 'b: 'a, E: Env {
    unsafe{<dyn Widget<E>+'b as TraitcastImpl<'b,T>>::_try_traitcast_ref(s)}
}

impl<'a,E> TraitcastImplBase<'a> for dyn Widget<E>+'a where E: Env {
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        Widget::_as_trait_ref(self, t)
    }
}

/// Syntax:  
/// `traitcast_for_from_widget!([<...>] Trait[<...>] [where ...]);`  
#[macro_export]
macro_rules! traitcast_for_from_widget {
    (
        $( < $($args:ident),* $(,)* > )?
        $trait:path
        $(where $($preds:tt)+)?
    ) => {
        $crate::traitcast_for!(
            < E $(, $($args),* )? >
            $trait
            where E: $crate::env::Env $(, $($preds)+ )?
        );
    }
}
