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
        impl<E,$($tt),+> PaneChildsDecl<E> for WidgetsFixedIdx<($($tt),+,)> where
            $($tt: WidgetDecl<E>),+ ,
            E: Env
        {
            type Retained = WidgetsFixedIdx<($(PaneChildWidget<$tt::Widget,E>),+,)>;

            fn send_mutation<Ph>(
                &self,
                path: &Ph,
                resolve: &(dyn PathResolvusDyn<E>+'_),
                args: &dyn Any,
                root: E::RootRef<'_>,
                ctx: &mut E::Context<'_>,
            ) where Ph: PathStack<E> + ?Sized {
                let ($($ll),+,) = &self.0;

                if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
                    match r2.0 {
                        $($mm =>
                            $ll.send_mutation(&r2.push_on_stack(path), resolve.inner().unwrap(), args, root, ctx)
                        ),+ ,
                        _ => {},
                    }
                } else {
                    //TODO what happens if the mutor is lost
                }
            }

            fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized, Ph: PathStack<E> + ?Sized {
                let ($($ll),+,) = self.0;

                WidgetsFixedIdx(
                    ($({
                        PaneChildWidget::new( $ll.build(&FixedIdx($mmm).push_on_stack(path), root.fork(), ctx) )
                    }),+,)
                )
            }
        
            fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
                let ($($ll),+,) = &self.0;

                WidgetsFixedIdx(
                    ($({
                        PaneChildWidget::new( $ll.instantiate(&FixedIdx($mmm).push_on_stack(path), root.fork(), ctx) )
                    }),+,)
                )
            }
        
            fn update<Ph>(
                &self,
                w: &mut Self::Retained,
                path: &Ph,
                route: UpdateRoute<'_,E>,
                root: E::RootRef<'_>,
                ctx: &mut E::Context<'_>
            ) -> Invalidation where Ph: PathStack<E> + ?Sized {
                let ($($ll),+,) = &self.0;
                let ($($ll2),+,) = &mut w.0;
                
                // If resolve, try only update resolve scope
                if let Some(resolve) = route.resolving() {
                    if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
                        match r2.0 {
                            $($mm => {
                                let v = $ll.update(&mut $ll2.widget, &r2.push_on_stack(path), route.for_child_1(), root, ctx);
                                $ll2.invalidate(v);
                                return v;
                            }),+ ,
                            _ => {},
                        }
                    } else {
                        //TODO what to do on invalid scope resolves in update?
                    }
                    return Invalidation::new();
                }
                
                let mut vali = Invalidation::valid();

                // Update persisted exising area
                $({
                    let v = $ll.update(&mut $ll2.widget, &FixedIdx($mmm).push_on_stack(path), route.for_child_1(), root.fork(), ctx);
                    $ll2.invalidate(v);
                    vali |= v;
                })+

                vali
            }
        
            fn update_restore<Ph>(
                &self,
                prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
                path: &Ph,
                root: E::RootRef<'_>,
                ctx: &mut E::Context<'_>
            ) -> (Self::Retained,Invalidation) where Ph: PathStack<E> + ?Sized {
                let ($($ll),+,) = &self.0;

                let prev_len = prev.len();

                let mut vali = Invalidation::valid();
        
                // Remove old tail
                if prev_len > $n {
                    end_range_dyn(prev, $n .. prev_len, path, root.fork(), ctx);
                    vali = Invalidation::new();
                }
                
                let restorable = prev_len.min($n);
        
                let new = ($({
                    let path = FixedIdx($mmm).push_on_stack(path);
                    if $mmm < restorable {
                        let result = prev.by_index_dyn_mut($mmm);
                        let result = result.unwrap();
                        debug_assert_eq!(result.idx, $mmm);
                        let (w,v) = $ll.update_restore(result.widget, &path, root.fork(), ctx);
                        let mut w = PaneChildWidget::new(w);
                        w.invalidate(v);
                        vali |= v;
                        w
                    } else {
                        vali = Invalidation::new();
                        PaneChildWidget::new( $ll.instantiate(&path, root.fork(), ctx) )
                    }
                }),+,);
        
                (WidgetsFixedIdx(new), vali)
            }
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
