use std::ops::{Range, Mul, Div};

use crate::dispatchor::{AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsIndexedWrap, AsWidgetClosure, AsWidgetsAllClosure, AsWidgetsClosure, AsWidgetClosureErased};
use crate::env::Env;
use crate::root::RootRef;
use crate::widget::cache::DynWidgetCache;

use super::*;
use super::as_widget::AsWidget;

pub trait ChildIDSerialize<E> {
    fn ser_into(&self, v: &mut Vec<DynIDFragment>);
}

pub type DynIDFragment = std::sync::Arc<dyn std::any::Any>;

impl<E,T> ChildIDSerialize<E> for T where T: Clone + Sized + 'static {
    fn ser_into(&self, v: &mut Vec<DynIDFragment>) {
        v.push(std::sync::Arc::new(self.clone()));
    }
}

pub trait AsWidgets<'z,E> where E: Env, Self: 'z {
    type Widget<'v>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where 'z: 'v;
    type WidgetCache: WidgetCache<E>;
    type Bound: Rangor<E> + Clone + 'static; //must be range
    type ChildID: ChildIDSerialize<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    fn by_index<'w,F,R>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>;

    fn by_id<'w,F,R>(&'w self, id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>;

    fn iter_ids(&self) -> Self::IdIdxIter;

    //fn sliced

    fn len(&self) -> usize;

    fn full_bounds(&self) -> Self::Bound;

    fn all<'w,F>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        self.all_filtered(#[inline] |_,_,_| true, f, root, ctx)
    }

    fn all_filtered<'w,F>(&'w self, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>;

    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        self.all_in_bounds_filtered(bound, #[inline] |_,_,_| true, f, root, ctx)
    }

    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>;

    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>;
}

impl<'z,E,T> AsWidgets<'z,E> for &T where T: AsWidgets<'z,E> + ?Sized, E: Env, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type WidgetCache = T::WidgetCache;
    type Bound = T::Bound;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<'w,F,R>(&'w self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).by_index(idx, callback, root, ctx)
    }

    #[inline]
    fn by_id<'w,F,R>(&'w self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).by_id(id, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (**self).iter_ids()
    }

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn full_bounds(&self) -> Self::Bound {
        (**self).full_bounds()
    }

    #[inline]
    fn all<'w,F>(&'w self, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all(callback, root, ctx)
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_filtered(filter, callback, root, ctx)
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_in_bounds(bound, callback, root, ctx)
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_in_bounds_filtered(bound, filter, callback, root, ctx)
    }

    #[inline]
    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            callback.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).resolve(path, callback, root, ctx)
    }
}

impl<'z,E,T> AsWidgets<'z,E> for [T] where T: AsWidget<'z,E>, E: Env, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type WidgetCache = T::WidgetCache;
    type Bound = Range<usize>;
    type ChildID = usize;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F,R>(&'w self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.get(idx).map(#[inline] |v| {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
            });
            v.with_widget(callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,F,R>(&'w self, &id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.get(id).map(#[inline] |v| {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(id, id..id+1, id, widget, root, ctx)
            });
            v.with_widget(callback,root,ctx)
        })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,i..i+1,i))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    #[inline]
    fn full_bounds(&self) -> Self::Bound {
        0..self.len()
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,v) in self.iter().enumerate() {
            if (filter)(i,&(i..i+1),&i) {
                let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i..i+1, i, widget, root, ctx)
                });
                v.with_widget(callback,root.fork(),ctx)
            }
        }
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.get(bound.start .. bound.end.min(self.len())) {
            s.all_filtered(filter,callback,root,ctx)
        }
    }

    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        if let Some(idx) = path[0].downcast_ref::<usize>() {
            self.by_index(*idx, callback, root, ctx)
        } else {
            None
        }
    }
}

impl<'z,E,T,const N: usize> AsWidgets<'z,E> for [T;N] where T: AsWidget<'z,E>, E: Env, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type WidgetCache = T::WidgetCache;
    type Bound = Range<usize>;
    type ChildID = usize;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F,R>(&'w self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.get(idx).map(#[inline] |v| {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
            });
            v.with_widget(callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,F,R>(&'w self, &id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.get(id).map(#[inline] |v| {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(id, id..id+1, id, widget, root, ctx)
            });
            v.with_widget(callback,root,ctx)
        })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,i..i+1,i))
    }

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn full_bounds(&self) -> Self::Bound {
        0..self.len()
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,v) in self.iter().enumerate() {
            if (filter)(i,&(i..i+1),&i) {
                let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i..i+1, i, widget, root, ctx)
                });
                v.with_widget(callback,root.fork(),ctx)
            }
        }
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.get(bound.start .. bound.end.min(self.len())) {
            for (i,v) in self.iter().enumerate() {
                if (filter)(i,&(i..i+1),&i) {
                    let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                        callback.call(i, i..i+1, i, widget, root, ctx)
                    });
                    v.with_widget(callback,root.fork(),ctx)
                }
            }
        }
    }

    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        if let Some(idx) = path[0].downcast_ref::<usize>() {
            self.by_index(*idx, callback, root, ctx)
        } else {
            None
        }
    }
}

pub struct ScaleBoundsBy<T,V>(pub V,pub T) where T: ?Sized, V: Clone + 'static;

impl<'z,E,V,T> AsWidgets<'z,E> for ScaleBoundsBy<T,V>
where
    T: AsWidgets<'z,E> + ?Sized,
    V: Clone + 'static,
    T::Bound: RangorScale<V,E>,
    E: Env,
    Self: 'z,
{
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type WidgetCache = T::WidgetCache;
    type Bound = T::Bound;
    type ChildID = T::ChildID;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F,R>(&'w self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.by_index(idx, callback, root, ctx)
    }

    #[inline]
    fn by_id<'w,F,R>(&'w self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.by_id(id, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        let scale = self.0.clone();
        self.1.iter_ids().map(#[inline] move |(i,b,id)| (i,b.scaled_mul(scale.clone()),id))
    }

    #[inline]
    fn len(&self) -> usize {
        self.1.len()
    }

    #[inline]
    fn full_bounds(&self) -> Self::Bound {
        self.1.full_bounds().scaled_mul(self.0.clone())
    }

    #[inline]
    fn all<'w,F>(&'w self, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all(callback, root, ctx)
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_filtered(
            #[inline] move |i,b,id|
                (filter)(i,&b.scaled_mul(self.0.clone()),id),
            callback,
            root,
            ctx
        )
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_in_bounds(&bound.scaled_div(self.0.clone()), callback, root, ctx)
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let callback = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_in_bounds_filtered(
            &bound.scaled_div(self.0.clone()),
            #[inline] move |i,b,id|
                (filter)(i,&b.scaled_mul(self.0.clone()),id),
            callback,
            root,
            ctx
        )
    }

    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        let callback = AsWidgetsClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            callback.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.resolve(path, callback, root, ctx)
    }
}

pub trait Rangor<E>: Sized
    // Sized + Clone + 'static +
    // BitAnd<Self,Output=Option<Self>> +
    // for<'a> BitAnd<&'a Self,Output=Option<Self>> +
    // BitOr<Self,Output=Self> +
    // for<'a> BitOr<&'a Self,Output=Self> +
    // BitOrAssign<Self> +
    // for<'a> BitOrAssign<&'a Self> +
{
    fn any(&self, o: &Self) -> Self;
    fn all(&self, o: &Self) -> Option<Self>;
}

pub trait RangorScale<S,E>: Rangor<E> where S: Clone {
    fn scaled_mul(&self, s: S) -> Self;
    fn scaled_div(&self, s: S) -> Self;
}

impl<E,V> Rangor<E> for Range<V> {
    #[inline]
    fn any(&self, o: &Self) -> Self {
        todo!()
    }
    #[inline]
    fn all(&self, o: &Self) -> Option<Self> {
        todo!()
    }
}

impl<E,S,V> RangorScale<S,E> for Range<V> where S: Clone, for<'a> &'a V: Mul<S,Output=V> + Div<S,Output=V> {
    #[inline]
    fn scaled_mul(&self, s: S) -> Self {
        &self.start * s.clone() .. &self.end * s
    }
    #[inline]
    fn scaled_div(&self, s: S) -> Self {
        &self.start / s.clone() .. &self.end / s
    }
}

#[repr(transparent)]
pub struct Tupled<T>(pub T) where T: ?Sized;

impl<'z,E,I,T> AsWidgets<'z,E> for Tupled<&'z [(I,T)]> where T: AsWidget<'z,E>, E: Env, I: Clone + PartialEq + 'static, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type WidgetCache = T::WidgetCache;
    type Bound = Range<usize>;
    type ChildID = I;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F,R>(&'w self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.0.get(idx).map(#[inline] |(id,v)| {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, idx..idx+1, id.clone(), widget, root, ctx)
            });
            v.with_widget(callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,F,R>(&'w self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        self.0.iter().enumerate()
            .find(#[inline] |(_,(i,_))| *i == *id)
            .map(#[inline] |(i,(id,v))| {
                let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i..i+1, id.clone(), widget, root, ctx)
                });
                v.with_widget(callback,root,ctx)
            })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i,i..i+1,id.clone()) )
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn full_bounds(&self) -> Self::Bound {
        0..self.len()
    }

    #[inline]
    fn all<'w,F>(&'w self, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(i, i..i+1, id.clone(), widget, root, ctx)
            });
            v.with_widget(callback,root.fork(),ctx)
        }
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
            F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            if (filter)(i,&(i..i+1),id) {
                let callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i..i+1, id.clone(), widget, root, ctx)
                });
                v.with_widget(callback,root.fork(),ctx)
            }
        }
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.0.get(bound.start .. bound.end.min(self.len())) {
            Tupled(s).all(callback,root,ctx)
        }
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
            F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.0.get(bound.start .. bound.end.min(self.len())) {
            Tupled(s).all_filtered(filter,callback,root,ctx)
        }
    }

    fn resolve<'w,F,R>(&'w self, path: &[DynIDFragment], callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        F: AsWidgetsDispatch<'z,Self,R,E>
    {
        todo!()
    }
}

macro_rules! impl_tuple {
    {
        $n:expr;
        $senf:ident;

        $t:ident $($tt:ident)+;
        $l:ident $($ll:ident)+;

        $m:pat => $x:expr, $($mm:pat => $xx:expr),+;

        $enumt:ident $($enumtt:ident)+;
        $enumv:ident $($enumvv:ident)+;
    } => {
        impl_tuple!(($n-1);$senf;$($tt)+;$($ll)+;$($mm => $xx),+;$($enumtt)+;$($enumvv)+;);

        pub enum $enumt <$t,$($tt),+> {
            $enumv ($t),
            $(
                $enumvv ($tt)
            ),+
        }

        // impl<$t,$($tt),+> QueronSequential for ($t,$($tt),+) where
        //     $t: Queron,
        //     $($tt: Queron),+ 
        // {
        //     #[inline]
        //     fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q>, rev: bool, bounce: bool) where Self: 'a {
        //         let ($l,$($ll),+) = self;
        //         if rev {
        //             let ($l,$($ll),+) = reverse_idents!([$l $($ll)+]);
        //             $l._query(builder.fork());
        //             $(
        //                 $ll._query(builder.fork());
        //             )+
        //             if bounce {
        //                 let (_,$($ll),+) = reverse_idents!([$l $($ll)+]);
        //                 $(
        //                     $ll._query(builder.fork());
        //                 )+
        //             }
        //         } else {
        //             $l._query(builder.fork());
        //             $(
        //                 $ll._query(builder.fork());
        //             )+
        //             if bounce {
        //                 let (_,$($ll),+) = reverse_idents!([$l $($ll)+]);
        //                 $(
        //                     $ll._query(builder.fork());
        //                 )+
        //             }
        //         }
        //     }
        // }

        impl<'z,E,$t,$($tt),+> AsWidgets<'z,E> for ($t,$($tt),+) where
            $t: AsWidget<'z,E>,
            $($tt: AsWidget<'z,E>),+ ,
            E: Env, Self: 'z
        {
            type Widget<'v> = dyn WidgetDyn<E> + 'v where 'z: 'v;
            type WidgetCache = DynWidgetCache<E>;
            type Bound = Range<usize>;
            type ChildID = usize;
            type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;
        
            #[inline]
            fn by_index<'w,XF,XR>(&'w self, idx: usize, callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                XF: AsWidgetsDispatch<'z,Self,XR,E>
            {
                let $senf = self;
                
                match idx {
                    $m => 
                        Some(AsWidget::with_widget(
                            & $x,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root,ctx,
                        ))
                    ,
                    $($mm => 
                        Some(AsWidget::with_widget(
                            & $xx,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root,ctx,
                        ))
                    ),+ ,
                    _ => None,
                }
            }
        
            #[inline]
            fn by_id<'w,XF,XR>(&'w self, &id: &Self::ChildID, callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                XF: AsWidgetsDispatch<'z,Self,XR,E>
            {
                todo!()
            }
        
            #[inline]
            fn iter_ids(&self) -> Self::IdIdxIter {
                (0..self.len()).map(#[inline] |i| (i,i..i+1,i))
            }
        
            #[inline]
            fn len(&self) -> usize {
                $n
            }
        
            #[inline]
            fn full_bounds(&self) -> Self::Bound {
                0..self.len()
            }
        
            #[inline]
            fn all_filtered<'w,XF>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            where
                XF: AsWidgetsIndexedDispatch<'z,Self,E>
            {
                let ($l,$($ll),*) = self;

                let mut i = 0;

                {
                    let idx = i;
                    i += 1;

                    if (filter)(idx,&(idx..idx+1),&idx) {
                        AsWidget::with_widget(
                            $l,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root.fork(),ctx,
                        )
                    }
                }
                $({
                    let idx = i;
                    i += 1;

                    if (filter)(idx,&(idx..idx+1),&idx) {
                        AsWidget::with_widget(
                            $ll,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root.fork(),ctx,
                        )
                    }
                })*
            }
        
            #[inline]
            fn all_in_bounds_filtered<'w,XF>(&'w self, bound: &Self::Bound, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut fun: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            where
                XF: AsWidgetsIndexedDispatch<'z,Self,E>
            {
                let ($l,$($ll),*) = self;

                let mut idx = 0;

                {
                    let current_idx = idx;
                    idx += 1;

                    if idx >= bound.start && idx < bound.end && (filter)(current_idx,&(current_idx..current_idx+1),&current_idx) {
                        AsWidget::with_widget(
                            $l,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                fun.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root.fork(),ctx,
                        )
                    }
                }
                $({
                    let current_idx = idx;
                    idx += 1;

                    if idx >= bound.start && idx < bound.end && (filter)(current_idx,&(current_idx..current_idx+1),&current_idx) {
                        AsWidget::with_widget(
                            $ll,
                            AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                fun.call(idx, idx..idx+1, idx, widget, root, ctx)
                            }),
                            root.fork(),ctx,
                        )
                    }
                })*
            }
        
            fn resolve<'w,XF,XR>(&'w self, path: &[DynIDFragment], mut callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                XF: AsWidgetsDispatch<'z,Self,XR,E>
            {
                todo!()
            }
        }
    };
    {
        $n:expr;
        $senf:ident;
        
        $t:ident;$l:ident;
        $m:pat => $x:expr;
        
        $enumt:ident;
        $enumv:ident;
    } => {}
}

impl_tuple!(
    32;senf;
    A B C D F G H I J K L M N O P R S T U V W X Y Z AA AB AC AD AE AF AG AH;
    a b c d f g h i j k l m n o p r s t u v w x y z aa ab ac ad ae af ag ah;
    31 => senf.31,30 => senf.30,29 => senf.29,28 => senf.28,
    27 => senf.27,26 => senf.26,25 => senf.25,24 => senf.24,
    23 => senf.23,22 => senf.22,21 => senf.21,20 => senf.20,
    19 => senf.19,18 => senf.18,17 => senf.17,16 => senf.16,
    15 => senf.15,14 => senf.14,13 => senf.13,12 => senf.12,
    11 => senf.11,10 => senf.10,09 => senf. 9,08 => senf. 8,
    07 => senf. 7,06 => senf. 6,05 => senf. 5,04 => senf. 4,
    03 => senf. 3,02 => senf. 2,01 => senf. 1,00 => senf. 0;
    Widgets32 Widgets31 Widgets30 
    Widgets29 Widgets28 Widgets27 Widgets26 Widgets25 Widgets24 Widgets23 Widgets22 Widgets21 Widgets20 
    Widgets19 Widgets18 Widgets17 Widgets16 Widgets15 Widgets14 Widgets13 Widgets12 Widgets11 Widgets10 
    Widgets9 Widgets8 Widgets7 Widgets6 Widgets5 Widgets4 Widgets3 Widgets2 Widgets1;
    V32 V31 V30 
    V29 V28 V27 V26 V25 V24 V23 V22 V21 V20 
    V19 V18 V17 V16 V15 V14 V13 V12 V11 V10 
    V9 V8 V7 V6 V5 V4 V3 V2 V1;
);
