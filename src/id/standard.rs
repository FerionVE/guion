//! A simple incremental usize-based ID
use super::*;
use std::{any::TypeId, sync::atomic::{AtomicUsize,Ordering}};

static ID_ITER: AtomicUsize = AtomicUsize::new(0);

/// A simple incremental usize-based ID
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum StdID {
    Dyn(usize),
    Const(TypeId),
}

impl StdID {
    #[inline]
    pub fn new() -> Self {
        StdID::Dyn(ID_ITER.fetch_add(1,Ordering::Relaxed))
    }
}

/// Macro for defining const StdIDs
#[macro_export]
macro_rules! const_std_id {
    ($n:ident) => {
        #[inline]
        pub fn $n() -> $crate::id::standard::StdID {
            struct Ident;
            $crate::id::standard::StdID::Const(std::any::TypeId::of::<Ident>())
        }
    };
    ($n:ident $($nn:ident)+) => {
        $crate::const_std_id!($n);
        $crate::const_std_id!($($nn)+);
    };
}

impl WidgetID for StdID {
    
}

impl WidgetIDAlloc for StdID {
    #[inline]
    fn new_id() -> Self where Self: Sized {
        Self::new()
    }
}

impl<E> SubPath<E> for StdID where E: Env, E::WidgetID: Into<Self> + From<Self> {
    #[inline]
    fn from_id(id: E::WidgetID) -> Self {
        id.into()
    }
    #[inline]
    fn _eq_id(&self, id: E::WidgetID) -> bool {
        self == &id.into()
    }
    #[inline]
    fn into_id(self) -> E::WidgetID {
        self.into()
    }

    #[inline]
    fn resolves_to_id(&self, id: E::WidgetID) -> bool {
        self == &id.into()
    }
    #[inline]
    fn resolves_to_path(&self, p: E::WidgetPath) -> bool {
        p.tip().map_or(false,|tip| { //TODO verify correctness of None => false
            *self == tip.clone().into_id().into()
        })
    }

    fn is<T: Any>(&self) -> bool { //TODO default underlying-trait impl hack
        Any::is::<T>(self)
    }
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        Any::downcast_ref::<T>(self)
    }
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        Any::downcast_mut::<T>(self)
    }
    fn downcast_into<T: Any>(self) -> Result<T,Self> where Self: Sized + 'static {
        todo!()
    }
    
}

#[allow(unused)]
mod const_id_test {
    const_std_id!(foo bar);
}