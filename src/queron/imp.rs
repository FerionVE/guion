use std::borrow::Cow;
use std::rc::Rc;
use std::sync::{Arc, MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use super::*;

impl<E> Queron<E> for () {
    #[inline]
    fn query<'a,Q>(&'a self, _: &Q) -> Option<Q::Out<'a>> where Q: Query<E> + ?Sized, Self: 'a {
        None
    }
    #[inline]
    fn _query<'a,Q>(&'a self, _: QueryStack<'_,'a,Q,E>) where Self: 'a {}
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[macro_export]
macro_rules! impl_queron_transparent { // next-gen matching
    (
        ($($args:tt)*)
        $typ:ty => $subtyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident| $tosub:expr
    ) => {
        impl<$($args)*> Queron<E> for $typ $(where $($preds)*)? {
            #[inline]
            fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
                let $senf = self;
                <$subtyp as Queron<E>>::_query($senf,builder)
            }
            #[inline]
            fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
                self
            }
        }
    }
}

#[macro_export]
macro_rules! impl_queronseq_transparent { // next-gen matching
    (
        ($($args:tt)*)
        $typ:ty => $subtyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident| $tosub:expr
    ) => {
        impl<$($args)*> QueronSequential<E> for $typ $(where $($preds)*)? {
            #[inline]
            fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a {
                let $senf = self;
                <$subtyp as QueronSequential<E>>::_query($senf,builder,rev,bounce)
            }
        }
    }
}

impl_queron_transparent!((T,E) &T => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) &mut T => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) Box<T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) Rc<T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) Arc<T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) Cow<'_,T> => T where (T: Queron<E> + Clone + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) std::cell::Ref<'_,T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) std::cell::RefMut<'_,T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) MutexGuard<'_,T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) RwLockReadGuard<'_,T> => T where (T: Queron<E> + ?Sized) |s| &**s);
impl_queron_transparent!((T,E) RwLockWriteGuard<'_,T> => T where (T: Queron<E> + ?Sized) |s| &**s);

impl<T,E> Queron<E> for Option<T> where T: Queron<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some(v) = self {
            v._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

/// Query arrays/slices/tuples sequential
/// queries array from start to end, so the end has higher priority as it overrides previous responses
#[repr(transparent)]
pub struct QuerySequentialRPrio<T>(pub T) where T: ?Sized;

impl<T,E> Queron<E> for QuerySequentialRPrio<T> where T: QueronSequential<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        self.0._query(builder,false,false)
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

/// Query arrays/slices/tuples sequential
/// queries array from end to start, so the start has higher priority as it overrides previous responses
#[repr(transparent)]
pub struct QuerySequentialLPrio<T>(pub T) where T: ?Sized;

impl<T,E> Queron<E> for QuerySequentialLPrio<T> where T: QueronSequential<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        self.0._query(builder,true,false)
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[deprecated]
#[repr(transparent)]
pub struct QuerySequentialLPrioBounce<T>(pub T) where T: ?Sized;

#[allow(deprecated)]
impl<T,E> Queron<E> for QuerySequentialLPrioBounce<T> where T: QueronSequential<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        self.0._query(builder,false,true)
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[deprecated]
#[repr(transparent)]
pub struct QuerySequentialRPrioBounce<T>(pub T) where T: ?Sized;

#[allow(deprecated)]
impl<T,E> Queron<E> for QuerySequentialRPrioBounce<T> where T: QueronSequential<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a {
        self.0._query(builder,true,true)
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

pub trait QueronSequential<E> {
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a;
}

impl_queronseq_transparent!((T,E) &T => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) &mut T => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) Box<T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) Rc<T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) Arc<T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) Cow<'_,T> => T where (T: QueronSequential<E> + Clone + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) std::cell::Ref<'_,T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) std::cell::RefMut<'_,T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) MutexGuard<'_,T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) RwLockReadGuard<'_,T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);
impl_queronseq_transparent!((T,E) RwLockWriteGuard<'_,T> => T where (T: QueronSequential<E> + ?Sized) |s| &**s);

impl<T,E> QueronSequential<E> for [T] where T: Queron<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a {
        if rev {
            for v in self.iter().rev() {
                v._query(builder.fork())
            }
            if bounce {
                for v in self.iter().skip(1) {
                    v._query(builder.fork())
                }
            }
        } else {
            for v in self {
                v._query(builder.fork())
            }
            if bounce {
                for v in self.iter().rev().skip(1) {
                    v._query(builder.fork())
                }
            }
        }
    }
}

impl<T,E> QueronSequential<E> for Vec<T> where T: Queron<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a {
        <[T] as QueronSequential<E>>::_query(self, builder, rev, bounce)
    }
}

impl<T,E,const N: usize> QueronSequential<E> for [T;N] where T: Queron<E> {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a {
        <[T] as QueronSequential<E>>::_query(self, builder, rev, bounce)
    }
}

macro_rules! reverse_idents {
    ([] $($reversed:ident)*) => { 
        ($($reversed),*)
    };
    ([$first:ident $($rest:ident)*] $($reversed:ident)*) => { 
        reverse_idents!([$($rest)*] $first $($reversed)*)
    };
}

macro_rules! impl_tuple {
    {
        $t:ident $($tt:ident)+;
        $l:ident $($ll:ident)+;
    } => {
        impl_tuple!($($tt)+;$($ll)+;);

        impl<E,$t,$($tt),+> QueronSequential<E> for ($t,$($tt),+) where
            $t: Queron<E>,
            $($tt: Queron<E>),+ 
        {
            #[inline]
            fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, rev: bool, bounce: bool) where Self: 'a {
                let ($l,$($ll),+) = self;
                if rev {
                    let ($l,$($ll),+) = reverse_idents!([$l $($ll)+]);
                    $l._query(builder.fork());
                    $(
                        $ll._query(builder.fork());
                    )+
                    if bounce {
                        let (_,$($ll),+) = reverse_idents!([$l $($ll)+]);
                        $(
                            $ll._query(builder.fork());
                        )+
                    }
                } else {
                    $l._query(builder.fork());
                    $(
                        $ll._query(builder.fork());
                    )+
                    if bounce {
                        let (_,$($ll),+) = reverse_idents!([$l $($ll)+]);
                        $(
                            $ll._query(builder.fork());
                        )+
                    }
                }
            }
        }
    };
    {
        $t:ident;$l:ident;
    } => {}
}

impl_tuple!(
    A B C D F G H I J K L M N O P R S T U V W X Y Z AA AB AC AD AE AF AG AH;
    a b c d f g h i j k l m n o p r s t u v w x y z aa ab ac ad ae af ag ah;
);
