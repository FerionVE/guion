use std::marker::PhantomData;

use crate::env::Env;
use crate::widget::Widget;
use crate::widget::as_widget::{AsWidget, WCow};

use super::View;

pub struct ViewWidget<Wid,WFn,MFn,E>(WFn,MFn,PhantomData<(fn()->Wid,E)>) where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + Send + Sync + 'static,
    E: Env;

pub fn view_widget_adv<Wid,WFn,MFn,E>(w: WFn, f: MFn) -> ViewWidget<Wid,WFn,MFn,E> where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + Send + Sync + 'static,
    E: Env,
{
    ViewWidget(w,f,PhantomData)
}

impl<Wid,WFn,MFn,E> AsWidget<E> for ViewWidget<Wid,WFn,MFn,E> where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + Send + Sync + 'static,
    E: Env,
{
    type Widget = Wid::Viewed;
    type WidgetOwned = Wid::Viewed;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx) )
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx) )
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx).boxed() )
    }
    fn into_widget_dyn<'w,'s>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx).boxed() )
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        self.into_widget_dyn(root,ctx)
    }
}

//TODO impl AsWidget

#[macro_export]
macro_rules! view_widget {
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))?  =>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $($mutor)?                   |=>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
        )
    };
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor                       |=>? |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr
            }
        )
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            $crate::view::view_widget::view_widget_adv(
                $viewgen,
                move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr},
            )
        }
    };
}

/*#[macro_export]
macro_rules! view_widget { // calling view direct with mutor! works but this doesn't (closure lifetime error)
    (
        $viewgen:expr,
        $($mutor:tt)*
    ) => {
        $crate::view::view_widget::view_widget_adv(
            $viewgen,
            $crate::mutor!( $($mutor)* ),
        )
    };
}*/

#[macro_export]
macro_rules! mutor {
    (
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))?  =>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor                       |=>? |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr
            }
        )
    };
    (
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr}
        }
    };

    (
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $($mutor)?                   |=>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };

    (
        $mutor:ident $(($($extra_out:expr),*))?  =>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>| |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr;},
                $crate::error::ResolveResult::Err(_) => {/*TODO*/},
            }}
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor                       |=>| |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr;
            }
        )
    };
    (
        $($mutor:ident)?                        |=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr;}
        }
    };
}
