use std::any::TypeId;

use crate::env::Env;
use crate::handler::Handler;

use super::{TraitcastImpl, TraitcastImplBase, TraitObject};

/// Traitcast a handler
/// 
/// This is a macro to properly handle handler tail
/// 
/// `traitcast_handler!(self.inner() as dyn XYZHandler<E>)`
#[macro_export]
macro_rules! traitcast_handler {
    ($src:expr => $trait:ty) => {
        {
            let mut s: &dyn $crate::handler::Handler<E> = $src;
            loop {
                if $crate::handler::Handler::is_tail(s) {
                    // special case for tail as we can't traitcast from it
                    break &() as &$trait;
                } else if let Ok(ss) = $crate::traitcast::handler::_try_traitcast_ref::<$trait,E>(s) {
                    break ss;
                } else {
                    // go recursively through handler stack
                    s = $crate::handler::Handler::inner(s);
                }
            }
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn _try_traitcast_ref<'a,'b,T,E>(s: &'a (dyn Handler<E>+'b)) -> Result<&'a T,()> where T: ?Sized + 'b, dyn Handler<E>+'b: TraitcastImpl<'b,T> + 'b, 'b: 'a, E: Env {
    unsafe{<dyn Handler<E>+'b as TraitcastImpl<'b,T>>::_try_traitcast_ref(s)}
}

impl<'a,E> TraitcastImplBase<'a> for dyn Handler<E>+'a where E: Env {
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        Handler::_as_trait_ref(self, t)
    }
}
