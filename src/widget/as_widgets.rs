use std::ops::{Range, Mul, Div};

use crate::dispatchor::{AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsIndexedWrap, AsWidgetClosure, AsWidgetsAllClosure, AsWidgetsClosure};
use crate::env::Env;
use crate::root::RootRef;

use super::Widget;
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
    type Widget<'v>: Widget<E> + ?Sized + 'v where 'z: 'v;
    type Bound: Rangor<E> + Clone + 'static; //must be range
    type ChildID: ChildIDSerialize<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    fn by_index<'w,F>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>;

    fn by_id<'w,F>(&'w self, id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>;

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

    fn resolve<'w,F>(&'w self, path: &[DynIDFragment], f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>;
}

impl<'z,E,T> AsWidgets<'z,E> for &T where T: AsWidgets<'z,E> + ?Sized, E: Env, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type Bound = T::Bound;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<'w,F>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        let dis = AsWidgetsClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).by_index(idx, dis, root, ctx)
    }

    #[inline]
    fn by_id<'w,F>(&'w self, id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        let dis = AsWidgetsClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).by_id(id, dis, root, ctx)
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
    fn all<'w,F>(&'w self, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all(dis, root, ctx)
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_filtered(filter, dis, root, ctx)
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_in_bounds(bound, dis, root, ctx)
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).all_in_bounds_filtered(bound, filter, dis, root, ctx)
    }

    #[inline]
    fn resolve<'w,F>(&'w self, path: &[DynIDFragment], mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound,child_id,widget,root,ctx| {
            f.call(idx, bound, child_id, widget, root, ctx)
        });
        (**self).resolve(path, dis, root, ctx)
    }
}

impl<'z,E,T> AsWidgets<'z,E> for [T] where T: AsWidget<'z,E>, E: Env, Self: 'z {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;
    type Bound = Range<usize>;
    type ChildID = usize;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        self.get(idx).map(#[inline] |v| {
            let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                f.call(idx, idx..idx+1, idx, widget, root, ctx)
            });
            v.with_widget(dis,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,F>(&'w self, &id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        self.get(id).map(#[inline] |v| {
            let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                f.call(id, id..id+1, id, widget, root, ctx)
            });
            v.with_widget(dis,root,ctx)
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
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,v) in self.iter().enumerate() {
            if (filter)(i,&(i..i+1),&i) {
                let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    f.call(i, i..i+1, i, widget, root, ctx)
                });
                v.with_widget(dis,root.fork(),ctx)
            }
        }
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.get(bound.start .. bound.end.min(self.len())) {
            s.all_filtered(filter,f,root,ctx)
        }
    }

    fn resolve<'w,F>(&'w self, path: &[DynIDFragment], mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(idx) = path[0].downcast_ref::<usize>() {
            // let dis = AsWidgetsClosure::new(#[inline] move |idx,bound,child_id,widget,root,ctx| {
            //    // f.call(idx, bound, child_id, widget, root, ctx)
            // });
            // //None
            self.by_index(*idx, AsWidgetsIndexedWrap(f), root, ctx)
            // self.get(*idx).map(#[inline] |v| {
            //     let dis = CallbackClosure::<'z,_,T,_,E>::for_as_widget(#[inline] |widget,root,ctx| {
            //         f.call(*idx, (*idx)..(*idx)+1, *idx, widget, root, ctx)
            //     });
            //     v.with_widget(dis,root,ctx);
            // })//::<'z,_,Self,E>
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
    type Bound = T::Bound;
    type ChildID = T::ChildID;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        let dis = AsWidgetsClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.by_index(idx, dis, root, ctx)
    }

    #[inline]
    fn by_id<'w,F>(&'w self, id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        let dis = AsWidgetsClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.by_id(id, dis, root, ctx)
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
    fn all<'w,F>(&'w self, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all(dis, root, ctx)
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_filtered(
            #[inline] move |i,b,id|
                (filter)(i,&b.scaled_mul(self.0.clone()),id),
            dis,
            root,
            ctx
        )
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_in_bounds(&bound.scaled_div(self.0.clone()), dis, root, ctx)
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.all_in_bounds_filtered(
            &bound.scaled_div(self.0.clone()),
            #[inline] move |i,b,id|
                (filter)(i,&b.scaled_mul(self.0.clone()),id),
            dis,
            root,
            ctx
        )
    }

    fn resolve<'w,F>(&'w self, path: &[DynIDFragment], mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        let dis = AsWidgetsAllClosure::new(#[inline] |idx,bound:Self::Bound,child_id,widget,root,ctx| {
            f.call(idx, bound.scaled_mul(self.0.clone()), child_id, widget, root, ctx)
        });
        self.1.resolve(path, dis, root, ctx)
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
    type Bound = Range<usize>;
    type ChildID = I;
    type IdIdxIter = impl Iterator<Item=(usize,Self::Bound,Self::ChildID)>;

    #[inline]
    fn by_index<'w,F>(&'w self, idx: usize, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        self.0.get(idx).map(#[inline] |(id,v)| {
            let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                f.call(idx, idx..idx+1, id.clone(), widget, root, ctx)
            });
            v.with_widget(dis,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,F>(&'w self, id: &Self::ChildID, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsDispatch<'z,Self,E>
    {
        self.0.iter().enumerate()
            .find(#[inline] |(_,(i,_))| *i == *id)
            .map(#[inline] |(i,(id,v))| {
                let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    f.call(i, i..i+1, id.clone(), widget, root, ctx)
                });
                v.with_widget(dis,root,ctx)
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
    fn all<'w,F>(&'w self, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                f.call(i, i..i+1, id.clone(), widget, root, ctx)
            });
            v.with_widget(dis,root.fork(),ctx)
        }
    }

    #[inline]
    fn all_filtered<'w,F>(&'w self, mut filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, mut f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
            F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            if (filter)(i,&(i..i+1),id) {
                let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    f.call(i, i..i+1, id.clone(), widget, root, ctx)
                });
                v.with_widget(dis,root.fork(),ctx)
            }
        }
    }

    #[inline]
    fn all_in_bounds<'w,F>(&'w self, bound: &Self::Bound, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.0.get(bound.start .. bound.end.min(self.len())) {
            Tupled(s).all(f,root,ctx)
        }
    }

    #[inline]
    fn all_in_bounds_filtered<'w,F>(&'w self, bound: &Self::Bound, filter: impl for<'a> FnMut(usize,&'a Self::Bound,&'a Self::ChildID) -> bool, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
            F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        if let Some(s) = self.0.get(bound.start .. bound.end.min(self.len())) {
            Tupled(s).all_filtered(filter,f,root,ctx)
        }
    }

    fn resolve<'w,F>(&'w self, path: &[DynIDFragment], f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<()>
    where
        F: AsWidgetsIndexedDispatch<'z,Self,E>
    {
        todo!()
    }
}

macro_rules! impl_tuple {
    {
        $t:ident $($tt:ident)+;
        $l:ident $($ll:ident)+;
        $enumt:ident $($enumtt:ident)+;
        $enumv:ident $($enumvv:ident)+;
    } => {
        impl_tuple!($($tt)+;$($ll)+;$($enumtt)+;$($enumvv)+;);

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
    };
    {
        $t:ident;$l:ident;$enumt:ident;$enumv:ident;
    } => {}
}

impl_tuple!(
    A B C D F G H I J K L M N O P R S T U V W X Y Z AA AB AC AD AE AF AG AH;
    a b c d f g h i j k l m n o p r s t u v w x y z aa ab ac ad ae af ag ah;
    Widgets32 Widgets31 Widgets30 
    Widgets29 Widgets28 Widgets27 Widgets26 Widgets25 Widgets24 Widgets23 Widgets22 Widgets21 Widgets20 
    Widgets19 Widgets18 Widgets17 Widgets16 Widgets15 Widgets14 Widgets13 Widgets12 Widgets11 Widgets10 
    Widgets9 Widgets8 Widgets7 Widgets6 Widgets5 Widgets4 Widgets3 Widgets2 Widgets1;
    V32 V31 V30 
    V29 V28 V27 V26 V25 V24 V23 V22 V21 V20 
    V19 V18 V17 V16 V15 V14 V13 V12 V11 V10 
    V9 V8 V7 V6 V5 V4 V3 V2 V1;
);
