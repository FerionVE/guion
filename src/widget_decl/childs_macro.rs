#[macro_export]
macro_rules! widget_childs_macro {
    () => {
        fn childs(&self) -> ::std::ops::Range<isize> {
            0 .. 0
        }

        fn child_dyn(&self, _: isize) -> Option<$crate::widget::WidgetChildDynResult<'_,E>> {
            None
        }

        fn child_dyn_mut(&mut self, _: isize) -> Option<$crate::widget::WidgetChildDynResultMut<'_,E>> {
            None
        }

        fn childs_dyn<'__childs_macros_a,ChildsMacrosF>(&'__childs_macros_a self, _: ::std::ops::Range<isize>, mut callback: ChildsMacrosF)
        where
            ChildsMacrosF: ::std::ops::FnMut($crate::widget::WidgetChildDynResult<'__childs_macros_a,E>)
        {}

        fn childs_dyn_mut<'__childs_macros_a,ChildsMacrosF>(&'__childs_macros_a mut self, _: ::std::ops::Range<isize>, mut callback: ChildsMacrosF)
        where
            ChildsMacrosF: ::std::ops::FnMut($crate::widget::WidgetChildDynResultMut<'__childs_macros_a,E>)
        {}

        fn resolve_child_dyn<'__childs_macros_a,'__childs_macros_b>(&'__childs_macros_a self, _: $crate::pathslice::PathSliceRef<'__childs_macros_b>) -> Option<$crate::widget::WidgetChildResolveDynResult<'__childs_macros_a,'__childs_macros_b,E>> {
            None
        }

        fn resolve_child_dyn_mut<'__childs_macros_a,'__childs_macros_b>(&'__childs_macros_a mut self, _: $crate::pathslice::PathSliceRef<'__childs_macros_b>) -> Option<$crate::widget::WidgetChildResolveDynResultMut<'__childs_macros_a,'__childs_macros_b,E>> {
            None
        }

        fn _call_tabulate_on_child_idx(
            &self,
            _: isize,
            _: &mut $crate::pathslice::NewPathStack,
            _: &(dyn $crate::queron::dyn_tunnel::QueronDyn<E>+'_),
            _: $crate::util::tabulate::TabulateOrigin,
            _: $crate::util::tabulate::TabulateDirection,
            _: <E as $crate::env::Env>::RootRef<'_>,
            _: &mut <E as $crate::env::Env>::Context<'_>
        ) -> Result<$crate::util::tabulate::TabulateResponse,E::Error> {
            Err(todo!())
        }
    };
    (
       $accty:ty |$crei:ident| $creder:expr;
       |$senf:ident|
        $($n:expr => $identi:expr);+
        $(;)*
    ) => {
        fn childs(&self) -> ::std::ops::Range<isize> {
            0 .. (0 $( + 1 + (0 * $n) )* )
        }

        fn child_dyn(&self, idx: isize) -> Option<$crate::widget::WidgetChildDynResult<'_,E>> {
            let $senf = self;

            match idx {
                $(
                    $n => Some($crate::widget::WidgetChildDynResult {
                        idx: $n,
                        widget_id: $crate::widget::Widget::id(& $identi),
                        widget: & $identi,
                    })
                ),* ,
                _ => None,
            }
        }

        fn child_dyn_mut(&mut self, idx: isize) -> Option<$crate::widget::WidgetChildDynResultMut<'_,E>> {
            let $senf = self;

            match idx {
                $(
                    $n => Some($crate::widget::WidgetChildDynResultMut {
                        idx: $n,
                        widget_id: $crate::widget::Widget::id(& $identi),
                        widget: &mut $identi,
                    })
                ),* ,
                _ => None,
            }
        }

        fn childs_dyn<'__childs_macros_a,ChildsMacrosF>(&'__childs_macros_a self, range: ::std::ops::Range<isize>, mut callback: ChildsMacrosF)
        where
            ChildsMacrosF: ::std::ops::FnMut($crate::widget::WidgetChildDynResult<'__childs_macros_a,E>)
        {
            let $senf = self;

            $({
                if range.start <= $n && range.end > $n {
                    (callback)($crate::widget::WidgetChildDynResult {
                        idx: $n,
                        widget_id: $crate::widget::Widget::id(& $identi),
                        widget: & $identi,
                    })
                }
            });*
        }

        fn childs_dyn_mut<'__childs_macros_a,ChildsMacrosF>(&'__childs_macros_a mut self, range: ::std::ops::Range<isize>, mut callback: ChildsMacrosF)
        where
            ChildsMacrosF: ::std::ops::FnMut($crate::widget::WidgetChildDynResultMut<'__childs_macros_a,E>)
        {
            let $senf = self;

            $({
                if range.start <= $n && range.end > $n {
                    (callback)($crate::widget::WidgetChildDynResultMut {
                        idx: $n,
                        widget_id: $crate::widget::Widget::id(& $identi),
                        widget: &mut $identi,
                    })
                }
            });*
        }

        fn resolve_child_dyn<'__childs_macros_a,'__childs_macros_b>(&'__childs_macros_a self, path: $crate::pathslice::PathSliceRef<'__childs_macros_b>) -> Option<$crate::widget::WidgetChildResolveDynResult<'__childs_macros_a,'__childs_macros_b,E>> {
            let $senf = self;
            let child_range = self.childs();
            
            match path.fetch().slice_forward::<$accty>() {
                PathSliceMatch::Match(value, inner) => {
                    $(
                        let $crei = $n;
                        if $n >= child_range.start && $n < child_range.end && value == & $creder {
                            return Some($crate::widget::WidgetChildResolveDynResult {
                                idx: $n,
                                widget_id: $crate::widget::Widget::id(& $identi),
                                widget: & $identi,
                                sub_path: inner,
                            });
                        }
                    )*
                    None
                },
                PathSliceMatch::Mismatch => None,
                PathSliceMatch::End => None,
            }
        }

        fn resolve_child_dyn_mut<'__childs_macros_a,'__childs_macros_b>(&'__childs_macros_a mut self, path: $crate::pathslice::PathSliceRef<'__childs_macros_b>) -> Option<$crate::widget::WidgetChildResolveDynResultMut<'__childs_macros_a,'__childs_macros_b,E>> {
            let child_range = self.childs();
            let $senf = self;
            
            match path.fetch().slice_forward::<$accty>() {
                PathSliceMatch::Match(value, inner) => {
                    $(
                        let $crei = $n;
                        if $n >= child_range.start && $n < child_range.end && value == & $creder {
                            return Some($crate::widget::WidgetChildResolveDynResultMut {
                                idx: $n,
                                widget_id: $crate::widget::Widget::id(& $identi),
                                widget: &mut $identi,
                                sub_path: inner,
                            });
                        }
                    )*
                    None
                },
                PathSliceMatch::Mismatch => None,
                PathSliceMatch::End => None,
            }
        }

        fn _call_tabulate_on_child_idx(
            &self,
            idx: isize,
            path: &mut $crate::pathslice::NewPathStack,
            stack: &(dyn $crate::queron::dyn_tunnel::QueronDyn<E>+'_),
            op: $crate::util::tabulate::TabulateOrigin,
            dir: $crate::util::tabulate::TabulateDirection,
            root: <E as $crate::env::Env>::RootRef<'_>,
            ctx: &mut <E as $crate::env::Env>::Context<'_>
        ) -> Result<$crate::util::tabulate::TabulateResponse,E::Error> {
            let $senf = self;

            let $crei = idx;

            match idx {
                $(
                    $n => $identi ._tabulate(&mut path.with($creder), stack, op.clone(), dir, root, ctx)
                ),* ,
                _ => Err(todo!()),
            }
        }
    };
}
