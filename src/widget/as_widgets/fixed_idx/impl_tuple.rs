use super::*;

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

        impl<E,$t,$($tt),+> AsWidgets<E> for WidgetsFixedIdx<($t,$($tt),+)> where
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ ,
            E: Env
        {
            type WidgetCache = DynWidgetCache<E>;
            type ChildID = FixedIdx;
            type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;
        
            #[inline]
            fn by_index<XF,XR>(&self, idx: usize, mut callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                XF: AsWidgetsDispatch<Self::ChildID,XR,E>
            {
                let $senf = &self.0;
                
                match idx {
                    $m => 
                        AsWidget::with_widget(
                            & $x,
                            &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ,
                    $($mm => 
                        AsWidget::with_widget(
                            & $xx,
                            &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ),+ ,
                    _ => callback.call_none(root, ctx),
                }
            }
        
            #[inline]
            fn by_id<XF,XR>(&self, id: &Self::ChildID, callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                XF: AsWidgetsDispatch<Self::ChildID,XR,E>
            {
                self.by_index(id.0, callback, root, ctx)
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
            fn idx_range_filtered<XF>(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            where
                XF: AsWidgetsIndexedDispatch<Self::ChildID,E>
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
                                &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
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
                                &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                                    callback.call(idx, FixedIdx(idx), widget, root, ctx)
                                }),
                                root.fork(),ctx,
                            )
                        }
                    }
                })*

                let _ = i;
            }
        
            fn resolve<XF,XR>(&self, path: &(dyn PathResolvusDyn<E>+'_), mut callback: XF, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> XR
            where
                XF: AsWidgetsResolveDispatch<Self::ChildID,XR,E>
            {
                let Some(v) = path.try_fragment::<Self::ChildID>() else { return callback.call_none(root,ctx) };

                let idx = v.0;
                
                let $senf = &self.0;
            
                match idx {
                    $m => 
                        AsWidget::with_widget(
                            & $x,
                            &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ,
                    $($mm => 
                        AsWidget::with_widget(
                            & $xx,
                            &mut AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                                callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                            }),
                            root,ctx,
                        )
                    ),+ ,
                    _ => callback.call_none(root, ctx),
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
    11 => senf.11,10 => senf.10, 9 => senf. 9, 8 => senf. 8,
     7 => senf. 7, 6 => senf. 6, 5 => senf. 5, 4 => senf. 4,
     3 => senf. 3, 2 => senf. 2, 1 => senf. 1, 0 => senf. 0;
    Widgets32 Widgets31 Widgets30 
    Widgets29 Widgets28 Widgets27 Widgets26 Widgets25 Widgets24 Widgets23 Widgets22 Widgets21 Widgets20 
    Widgets19 Widgets18 Widgets17 Widgets16 Widgets15 Widgets14 Widgets13 Widgets12 Widgets11 Widgets10 
    Widgets9 Widgets8 Widgets7 Widgets6 Widgets5 Widgets4 Widgets3 Widgets2 Widgets1;
    V32 V31 V30 
    V29 V28 V27 V26 V25 V24 V23 V22 V21 V20 
    V19 V18 V17 V16 V15 V14 V13 V12 V11 V10 
    V9 V8 V7 V6 V5 V4 V3 V2 V1;
);
