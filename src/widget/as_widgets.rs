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

pub trait AsWidgets<E> where E: Env {
    type Widget<'v,'z>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where 'z: 'v, Self: 'z;
    type WidgetCache: WidgetCache<E>;
    type ChildID: ChildIDSerialize<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::ChildID)>;

    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w;

    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w;

    fn iter_ids(&self) -> Self::IdIdxIter;

    //fn sliced

    fn len(&self) -> usize;

    fn idx_range<'w>(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        self.idx_range_filtered(range, #[inline] |_, _| true, callback, root, ctx)
    }

    fn idx_range_filtered<'w>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w;

    fn resolve<'w,R>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w;
}

impl<E,T> AsWidgets<E> for &'_ T where T: AsWidgets<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).by_index(idx, &mut callback, root, ctx)
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).by_id(id, &mut callback, root, ctx)
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
    fn idx_range<'w>(&self, range: Range<usize>, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).idx_range(range, &mut callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).idx_range_filtered(range, filter, &mut callback, root, ctx)
    }

    #[inline]
    fn resolve<'w,R>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).resolve(path, &mut callback, root, ctx)
    }
}

impl<E,T> AsWidgets<E> for [T] where T: AsWidget<E>, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = usize;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.get(idx).map(#[inline] |v| {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, idx, widget, root, ctx)
            });
            v.with_widget(&mut callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,R>(&self, &id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.get(id).map(#[inline] |v| {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(id, id, widget, root, ctx)
            });
            v.with_widget(&mut callback,root,ctx)
        })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,i))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,v) in self[idx_range].iter().enumerate() {
            if (filter)(i,&i) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i, widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        if let Some(idx) = path[0].downcast_ref::<usize>() {
            self.by_index(*idx, callback, root, ctx)
        } else {
            None
        }
    }
}

impl<E,T,const N: usize> AsWidgets<E> for [T;N] where T: AsWidget<E>, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = usize;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.get(idx).map(#[inline] |v| {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, idx, widget, root, ctx)
            });
            v.with_widget(&mut callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,R>(&self, &id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.get(id).map(#[inline] |v| {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(id, id, widget, root, ctx)
            });
            v.with_widget(&mut callback,root,ctx)
        })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,i))
    }

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,v) in self.iter().enumerate() {
            if (filter)(i,&i) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, i, widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        if let Some(idx) = path[0].downcast_ref::<usize>() {
            self.by_index(*idx, callback, root, ctx)
        } else {
            None
        }
    }
}

#[repr(transparent)]
pub struct Tupled<T>(pub T) where T: ?Sized;

impl<'s,E,I,T> AsWidgets<E> for Tupled<&'s [(I,T)]> where T: AsWidget<E> + 's, E: Env, I: Clone + PartialEq + 'static {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = I;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.0.get(idx).map(#[inline] |(id,v)| {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(idx, id.clone(), widget, root, ctx)
            });
            v.with_widget(&mut callback,root,ctx)
        })
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
    {
        self.0.iter().enumerate()
            .find(#[inline] |(_,(i,_))| *i == *id)
            .map(#[inline] |(i,(id,v))| {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, id.clone(), widget, root, ctx)
                });
                v.with_widget(&mut callback,root,ctx)
            })
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i,id.clone()) )
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn idx_range<'w>(&self, range: Range<usize>, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(i, id.clone(), widget, root, ctx)
            });
            v.with_widget(&mut callback,root.fork(),ctx)
        }
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,(id,v)) in self.0.iter().enumerate() {
            if (filter)(i,id) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, id.clone(), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<R>
    where
        Self: 'w
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

        impl<E,$t,$($tt),+> AsWidgets<E> for ($t,$($tt),+) where
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ ,
            E: Env
        {
            type Widget<'v,'z> = dyn WidgetDyn<E> + 'v where 'z: 'v, Self: 'z;
            type WidgetCache = DynWidgetCache<E>;
            type ChildID = usize;
            type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;
        
            #[inline]
            fn by_index<'w,XR>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                Self: 'w
            {
                let $senf = self;
                
                match idx {
                    $m => 
                        Some(AsWidget::with_widget(
                            & $x,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx, widget, root, ctx)
                            }),
                            root,ctx,
                        ))
                    ,
                    $($mm => 
                        Some(AsWidget::with_widget(
                            & $xx,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(idx, idx, widget, root, ctx)
                            }),
                            root,ctx,
                        ))
                    ),+ ,
                    _ => None,
                }
            }
        
            #[inline]
            fn by_id<'w,XR>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                Self: 'w
            {
                todo!()
            }
        
            #[inline]
            fn iter_ids(&self) -> Self::IdIdxIter {
                (0..self.len()).map(#[inline] |i| (i,i))
            }
        
            #[inline]
            fn len(&self) -> usize {
                $n
            }
        
            #[inline]
            fn idx_range_filtered<'w>(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            where
                Self: 'w
            {
                let ($l,$($ll),*) = self;

                let mut i = 0;

                {
                    let idx = i;
                    i += 1;

                    if idx >= range.start && idx < range.end {
                        if (filter)(idx,&idx) {
                            AsWidget::with_widget(
                                $l,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(idx, idx, widget, root, ctx)
                                }),
                                root.fork(),ctx,
                            )
                        }
                    }
                }
                $({
                    let idx = i;
                    i += 1;

                    if idx >= range.start && idx < range.end {
                        if (filter)(idx,&idx) {
                            AsWidget::with_widget(
                                $ll,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(idx, idx, widget, root, ctx)
                                }),
                                root.fork(),ctx,
                            )
                        }
                    }
                })*
            }
        
            fn resolve<'w,XR>(&self, path: &[DynIDFragment], callback: &mut (dyn AsWidgetsDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<XR>
            where
                Self: 'w
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
