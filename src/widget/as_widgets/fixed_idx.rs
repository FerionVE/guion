use super::*;

#[repr(transparent)]
pub struct WidgetsFixedIdx<T>(pub T) where T: ?Sized;

#[inline]
fn bender<'a,'b,T>(v: &'a WidgetsFixedIdx<&'b T>) -> &'a WidgetsFixedIdx<T> where 'b: 'a, T: 'b + Sized {
    unsafe{std::mem::transmute(v)}
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<&'_ T> where T: Sized, WidgetsFixedIdx<T>: AsWidgets<E>, E: Env {
    type Widget<'v,'z> = <WidgetsFixedIdx<T> as AsWidgets<E>>::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = <WidgetsFixedIdx<T> as AsWidgets<E>>::WidgetCache;
    type ChildID = <WidgetsFixedIdx<T> as AsWidgets<E>>::ChildID;
    type IdIdxIter = <WidgetsFixedIdx<T> as AsWidgets<E>>::IdIdxIter;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).by_index(idx, &mut callback, root, ctx)
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).by_id(id, &mut callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (*bender(self)).iter_ids()
    }

    #[inline]
    fn len(&self) -> usize {
        (*bender(self)).len()
    }

    #[inline]
    fn idx_range<'w>(&self, range: Range<usize>, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (*bender(self)).idx_range(range, &mut callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (*bender(self)).idx_range_filtered(range, filter, &mut callback, root, ctx)
    }

    #[inline]
    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsResolveClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResolveResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).resolve(path, &mut callback, root, ctx)
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<[T]> where T: AsWidget<E>, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(&self.0)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }
}

impl<E,T,const N: usize> AsWidgets<E> for WidgetsFixedIdx<[T;N]> where T: AsWidget<E>, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<&'_ [T]> where T: AsWidget<E>, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(&self.0)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
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

        impl<E,$t,$($tt),+> AsWidgets<E> for WidgetsFixedIdx<($t,$($tt),+)> where
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ ,
            E: Env
        {
            type Widget<'v,'z> = dyn WidgetDyn<E> + 'v where 'z: 'v, Self: 'z;
            type WidgetCache = DynWidgetCache<E>;
            type ChildID = FixedIdx;
            type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;
        
            #[inline]
            fn by_index<'w,XR>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                Self: 'w
            {
                let $senf = &self.0;
                
                match idx {
                    $m => 
                        AsWidget::with_widget(
                            & $x,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ,
                    $($mm => 
                        AsWidget::with_widget(
                            & $xx,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ),+ ,
                    _ => callback.call(None, root, ctx),
                }
            }
        
            #[inline]
            fn by_id<'w,XR>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                Self: 'w
            {
                let $senf = &self.0;
                
                match id.0 {
                    $m => 
                        AsWidget::with_widget(
                            & $x,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ,
                    $($mm => 
                        AsWidget::with_widget(
                            & $xx,
                            &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ),+ ,
                    _ => callback.call(None, root, ctx),
                }
            }
        
            #[inline]
            fn iter_ids(&self) -> Self::IdIdxIter {
                (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
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
                let ($l,$($ll),*) = &self.0;

                let mut i = 0;

                {
                    let idx = i;
                    i += 1;

                    if idx >= range.start && idx < range.end {
                        if (filter)(idx,&FixedIdx(idx)) {
                            AsWidget::with_widget(
                                $l,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(idx, FixedIdx(idx), widget, root, ctx)
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
                        if (filter)(idx,&FixedIdx(idx)) {
                            AsWidget::with_widget(
                                $ll,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(idx, FixedIdx(idx), widget, root, ctx)
                                }),
                                root.fork(),ctx,
                            )
                        }
                    }
                })*
            }
        
            fn resolve<'w,XR>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,XR,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                Self: 'w
            {
                if let Some(v) = path.try_fragment::<Self::ChildID>() {
                    let idx = v.0;
                    
                    let $senf = &self.0;
                
                    match idx {
                        $m => 
                            AsWidget::with_widget(
                                & $x,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                                }),
                                root,ctx,
                            )
                        ,
                        $($mm => 
                            AsWidget::with_widget(
                                & $xx,
                                &mut AsWidgetClosureErased::new(#[inline] |widget,root,ctx| {
                                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                                }),
                                root,ctx,
                            )
                        ),+ ,
                        _ => callback.call(None, root, ctx),
                    }
                } else {
                    callback.call(None,root,ctx)
                }
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
