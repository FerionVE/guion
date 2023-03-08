use super::*;

macro_rules! impl_tuple {
    {
        $n:expr;
        $senf:ident;

        $($tt:ident)+;
        $($ll:ident)+;
        $($ll2:ident)+;

        $($mm:pat => $mmm:expr => $xx:expr),+;
    } => {
        impl<$($tt),+> Default for DefaultHack<($($tt),+,)>
        where
            $($tt: Default),+
        {
            #[inline]
            fn default() -> Self {
                Self(($(<$tt as Default>::default()),+,))
            }
        }

        impl<$($tt),+> Clone for DefaultHack<($($tt),+,)>
        where
            $($tt: Clone),+
        {
            #[inline]
            fn clone(&self) -> Self {
                // let ($($ll),+,) = &self.0;
                // Self(($(Clone::clone($ll)),+,))
                Self(self.0.clone())
            }
        }

        impl<E,$($tt),+> PaneChilds<E> for WidgetsFixedIdx<($(PaneChildWidget<$tt,E>),+,)> where
            $($tt: Widget<E>),+ ,
            E: Env
        {
            type Caches = DefaultHack<($($tt::Cache),+,)>;
        
            // #[inline]
            // fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsDispatch<Self::ChildID,R,E>
            // {
            //     let ($($ll),+,) = &self.0;
                
            //     match idx {
            //         $($mm =>
            //             callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx as isize),$ll), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // #[inline]
            // fn by_index_mut<F,R>(&mut self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsDispatchMut<Self::ChildID,R,E>
            // {
            //     let ($($ll),+,) = &mut self.0;
                
            //     match idx {
            //         $($mm =>
            //             callback.call(AsWidgetsResultMut::from_some(idx,FixedIdx(idx as isize),$ll), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // #[inline]
            // fn by_index_c<F,R>(&self, idx: usize, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsCDispatch<Self::ChildID,R,E>
            // {
            //     let ($($ll),+,) = &self.0;
            //     let ($($ll2),+,) = &mut caches.0;
                
            //     match idx {
            //         $($mm => {
            //             callback.call(AsWidgetsCResult::from_some(idx,FixedIdx(idx as isize),$ll,$ll2), root, ctx)
            //         }),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // #[inline]
            // fn by_index_c_mut<F,R>(&mut self, idx: usize, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
            // {
            //     let ($($ll),+,) = &mut self.0;
            //     let ($($ll2),+,) = &mut caches.0;
                
            //     match idx {
            //         $($mm => {
            //             callback.call(AsWidgetsCResultMut::from_some(idx,FixedIdx(idx as isize),$ll,$ll2), root, ctx)
            //         }),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }
        
            // #[inline]
            // fn iter_ids(&self) -> Self::IdIdxIter {
            //     (0..$n).map(#[inline] |i| (i,FixedIdx(i)))
            // }
        
            // #[inline]
            // fn idx_range<F>(&self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            // where
            //     F: AsWidgetsIndexedDispatch<Self::ChildID,E>
            // {
            //     let ($($ll),+,) = &self.0;

            //     $({
            //         if $mmm >= range.start && $mmm < range.end {
            //             callback.call($mmm, FixedIdx($mmm), $ll, root.fork(), ctx)
            //         }
            //     })+
            // }

            // #[inline]
            // fn idx_range_mut<F>(&mut self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            // where
            //     F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
            // {
            //     let ($($ll),+,) = &mut self.0;

            //     $({
            //         if $mmm >= range.start && $mmm < range.end {
            //             callback.call($mmm, FixedIdx($mmm), $ll, root.fork(), ctx)
            //         }
            //     })+
            // }

            // #[inline]
            // fn idx_range_c<F>(&self, range: Range<usize>, mut callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            // where
            //     F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
            // {
            //     let ($($ll),+,) = &self.0;
            //     let ($($ll2),+,) = &mut caches.0;

            //     $({
            //         if $mmm >= range.start && $mmm < range.end {
            //             callback.call($mmm, FixedIdx($mmm), $ll, $ll2, root.fork(), ctx)
            //         }
            //     })*
            // }

            // #[inline]
            // fn idx_range_c_mut<F>(&mut self, range: Range<usize>, mut callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
            // where
            //     F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
            // {
            //     let ($($ll),+,) = &mut self.0;
            //     let ($($ll2),+,) = &mut caches.0;

            //     $({
            //         if $mmm >= range.start && $mmm < range.end {
            //             callback.call($mmm, FixedIdx($mmm), $ll, $ll2, root.fork(), ctx)
            //         }
            //     })*
            // }
        
            // fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
            // {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return callback.call_none(root,ctx) };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &self.0;
            
            //     match idx {
            //         $($mm => 
            //             callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),$ll), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
            // {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return callback.call_none(root,ctx) };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &mut self.0;
            
            //     match idx {
            //         $($mm => 
            //             callback.call(AsWidgetsResolveResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),$ll), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
            // {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return callback.call_none(root,ctx) };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &self.0;
            //     let ($($ll2),+,) = &mut caches.0;
            
            //     match idx {
            //         $($mm => 
            //             callback.call(AsWidgetsResolveCResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),$ll,$ll2), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }

            // fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
            // where
            //     F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
            // {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return callback.call_none(root,ctx) };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &mut self.0;
            //     let ($($ll2),+,) = &mut caches.0;
            
            //     match idx {
            //         $($mm => 
            //             callback.call(AsWidgetsResolveCResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),$ll,$ll2), root, ctx)
            //         ),+ ,
            //         _ => callback.call_none(root, ctx),
            //     }
            // }
        }

        impl<E,$($tt),+> PaneChildsDyn<E> for WidgetsFixedIdx<($(PaneChildWidget<$tt,E>),+,)> where
            $($tt: Widget<E>),+ ,
            E: Env
        {
            type ChildID = FixedIdx;

            #[inline]
            fn len(&self) -> usize {
                $n
            }

            fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>>{
                let ($($ll),+,) = &self.0;
                
                let (widget_id,widget) = match idx {
                    $($mm =>
                        ($ll.widget.id(), (&$ll.widget) as &(dyn WidgetDyn<E> + '_))
                    ),+ ,
                    _ => return None,
                };

                Some(ChildWidgetDynResult {
                    widget,
                    widget_id,
                    child_id: FixedIdx(idx as isize),
                    idx,
                })
            }
        
            fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
                let ($($ll),+,) = &mut self.0;
                
                let (widget_id,widget) = match idx {
                    $($mm =>
                        ($ll.widget.id(), (&mut $ll.widget) as &mut (dyn WidgetDyn<E> + '_))
                    ),+ ,
                    _ => return None,
                };

                Some(ChildWidgetDynResultMut {
                    widget,
                    widget_id,
                    child_id: FixedIdx(idx as isize),
                    idx,
                })
            }
        
            fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
                let ($($ll),+,) = &self.0;

                $({
                    if $mmm >= range.start && $mmm < range.end {
                        callback(ChildWidgetDynResult {
                            widget: &$ll.widget,
                            widget_id: $ll.widget.id(),
                            child_id: FixedIdx($mmm),
                            idx: $mmm,
                        });
                    }
                })+
            }
        
            fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
                let ($($ll),+,) = &mut self.0;

                $({
                    if $mmm >= range.start && $mmm < range.end {
                        callback(ChildWidgetDynResultMut {
                            widget_id: $ll.widget.id(),
                            widget: &mut $ll.widget,
                            child_id: FixedIdx($mmm),
                            idx: $mmm,
                        });
                    }
                })+
            }
        
            // fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return None };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &self.0;
            
            //     let (widget_id,widget) = match idx {
            //         $($mm =>
            //             ($ll.id(), $ll as &(dyn WidgetDyn<E> + '_))
            //         ),+ ,
            //         _ => return None,
            //     };

            //     Some(ChildWidgetDynResolveResult {
            //         widget,
            //         widget_id,
            //         child_id: FixedIdx(idx as isize),
            //         idx,
            //         resolvus: path.inner().unwrap(),
            //     })
            // }
        
            // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
            //     let Some(v) = path.try_fragment::<Self::ChildID>() else { return None };

            //     let idx = v.0;
                
            //     let ($($ll),+,) = &mut self.0;
            
            //     let (widget_id,widget) = match idx {
            //         $($mm =>
            //             ($ll.id(), $ll as &mut (dyn WidgetDyn<E> + '_))
            //         ),+ ,
            //         _ => return None,
            //     };

            //     Some(ChildWidgetDynResolveResultMut {
            //         widget,
            //         widget_id,
            //         child_id: FixedIdx(idx as isize),
            //         idx,
            //         resolvus: path.inner().unwrap(),
            //     })
            // }
        }
    };
}

impl_tuple!(
    1;this;
   T1;
   t1;
   tt1;
    0 =>  0 => this. 0;
);
impl_tuple!(
    2;this;
   T1 T2;
   t1 t2;
   tt1 tt2;
    0 =>  0 => this. 0, 1 =>  1 => this. 1;
);
impl_tuple!(
    3;this;
   T1 T2 T3;
   t1 t2 t3;
   tt1 tt2 tt3;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2;
);
impl_tuple!(
    4;this;
   T1 T2 T3 T4;
   t1 t2 t3 t4;
   tt1 tt2 tt3 tt4;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3;
);
impl_tuple!(
    5;this;
   T1 T2 T3 T4 T5;
   t1 t2 t3 t4 t5;
   tt1 tt2 tt3 tt4 tt5;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4;
);
impl_tuple!(
    6;this;
   T1 T2 T3 T4 T5 T6;
   t1 t2 t3 t4 t5 t6;
   tt1 tt2 tt3 tt4 tt5 tt6;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5;
);
impl_tuple!(
    7;this;
   T1 T2 T3 T4 T5 T6 T7;
   t1 t2 t3 t4 t5 t6 t7;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6;
);
impl_tuple!(
    8;this;
   T1 T2 T3 T4 T5 T6 T7 T8;
   t1 t2 t3 t4 t5 t6 t7 t8;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7;
);
impl_tuple!(
    9;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9;
   t1 t2 t3 t4 t5 t6 t7 t8 t9;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8;
);
impl_tuple!(
   10;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9;
);
impl_tuple!(
   11;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10;
);
impl_tuple!(
   12;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11;
);
impl_tuple!(
   13;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
    12 => 12 => this.12;
);
impl_tuple!(
   14;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13;
);
impl_tuple!(
   15;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14;
);
impl_tuple!(
   16;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15;
);
impl_tuple!(
   17;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16;
);
impl_tuple!(
   18;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17;
);
impl_tuple!(
   19;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18;
);
impl_tuple!(
   20;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19;
);
impl_tuple!(
   21;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20;
);
impl_tuple!(
   22;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21;
);
impl_tuple!(
   23;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22;
);
impl_tuple!(
   24;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23;
);
impl_tuple!(
   25;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24;
);
impl_tuple!(
   26;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25;
);
impl_tuple!(
   27;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26;
);
impl_tuple!(
   28;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27 t28;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27 tt28;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26,27 => 27 => this.27;
);
impl_tuple!(
   29;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27 t28 t29;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27 tt28 tt29;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26,27 => 27 => this.27,
   28 => 28 => this.28;
);
impl_tuple!(
   30;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27 t28 t29 t30;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27 tt28 tt29 tt30;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26,27 => 27 => this.27,
   28 => 28 => this.28,29 => 29 => this.29;
);
impl_tuple!(
   31;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27 t28 t29 t30 t31;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27 tt28 tt29 tt30 tt31;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26,27 => 27 => this.27,
   28 => 28 => this.28,29 => 29 => this.29,30 => 30 => this.30;
);
impl_tuple!(
   32;this;
   T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31 T32;
   t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17 t18 t19 t20 t21 t22 t23 t24 t25 t26 t27 t28 t29 t30 t31 t32;
   tt1 tt2 tt3 tt4 tt5 tt6 tt7 tt8 tt9 tt10 tt11 tt12 tt13 tt14 tt15 tt16 tt17 tt18 tt19 tt20 tt21 tt22 tt23 tt24 tt25 tt26 tt27 tt28 tt29 tt30 tt31 tt32;
    0 =>  0 => this. 0, 1 =>  1 => this. 1, 2 =>  2 => this. 2, 3 =>  3 => this. 3,
    4 =>  4 => this. 4, 5 =>  5 => this. 5, 6 =>  6 => this. 6, 7 =>  7 => this. 7,
    8 =>  8 => this. 8, 9 =>  9 => this. 9,10 => 10 => this.10,11 => 11 => this.11,
   12 => 12 => this.12,13 => 13 => this.13,14 => 14 => this.14,15 => 15 => this.15,
   16 => 16 => this.16,17 => 17 => this.17,18 => 18 => this.18,19 => 19 => this.19,
   20 => 20 => this.20,21 => 21 => this.21,22 => 22 => this.22,23 => 23 => this.23,
   24 => 24 => this.24,25 => 25 => this.25,26 => 26 => this.26,27 => 27 => this.27,
   28 => 28 => this.28,29 => 29 => this.29,30 => 30 => this.30,31 => 31 => this.31;
);
