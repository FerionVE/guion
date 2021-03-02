//! A simple incremental usize-based ID
use super::*;
use std::{any::TypeId, fmt::Debug, sync::atomic::{AtomicUsize,Ordering}};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// A simple incremental usize-based ID
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub enum StdID {
    Dyn(usize),
    Const(TypeId),
}

impl StdID {
    #[inline]
    pub fn new() -> Self {
        StdID::Dyn(NEXT_ID.fetch_add(1,Ordering::Relaxed))
    }
}

/// Macro for defining const [`StdIDs`](StdID)
// 
// This defines a dummy type and uses it's TypeID for `StdID::Const` variant
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
    fn resolve_to_same_widget(&self, o: &Self) -> bool {
        self == o
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

impl Debug for StdID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dyn(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f), //TODO what if const int is same than dyn int?
        }
    }
}
