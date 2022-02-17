use std::marker::PhantomData;

use crate::env::Env;
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::as_widget::{AsWidget, WCow};

use super::View;

pub struct ViewWidget<Wid,WFn,MFn,E>(WFn,MFn,PhantomData<(Wid,E)>) where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + 'static,
    E: Env;

pub fn view_widget_adv<Wid,WFn,MFn,E>(w: WFn, f: MFn) -> ViewWidget<Wid,WFn,MFn,E> where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + 'static,
    E: Env,
{
    ViewWidget(w,f,PhantomData)
}
pub fn view_widget_dummy_adv<Wid,WFn,MFn,E>(w: WFn, f: MFn) -> DummyWidget<ViewWidget<Wid,WFn,MFn,E>> where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + 'static,
    E: Env,
{
    DummyWidget(ViewWidget(w,f,PhantomData))
}

impl<Wid,WFn,MFn,E> AsWidget<E> for ViewWidget<Wid,WFn,MFn,E> where
    Wid: View<E,MFn>,
    WFn: Fn()->Wid, MFn: Clone + 'static,
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

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx).boxed() )
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned( (self.0)().view(self.1.clone(), root,ctx).boxed() )
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        self.into_widget_dyn(root,ctx)
    }
}

pub struct DummyWidget<T>(pub T);

impl<T,E> Widget<E> for DummyWidget<T> where T: AsWidget<E>, E: Env {
    fn id(&self) -> <E as Env>::WidgetID {
        todo!()
    }

    fn _render(&self, l: crate::widget::link::Link<E>, r: &mut crate::aliases::ERenderer<'_,E>) {
        todo!()
    }

    fn _event_direct(&self, l: crate::widget::link::Link<E>, e: &crate::event::compound::EventCompound<E>) -> crate::EventResp {
        todo!()
    }

    fn _size(&self, l: crate::widget::link::Link<E>, e: &crate::aliases::EStyle<E>) -> crate::aliases::ESize<E> {
        todo!()
    }

    fn childs(&self) -> usize {
        todo!()
    }

    fn child<'s>(&'s self, i: usize, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Result<crate::aliases::WidgetRef<'s,E>,()> {
        todo!()
    }

    fn into_child<'s>(self: Box<Self>, i: usize, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Result<crate::aliases::WidgetRef<'s,E>,()> where Self: 's {
        todo!()
    }

    fn into_childs<'w>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Vec<crate::aliases::WidgetRef<'w,E>> where Self: 'w {
        todo!()
    }

    fn child_bounds(&self, l: crate::widget::link::Link<E>, b: &crate::util::translate::bounds::Bounds, e: &crate::aliases::EStyle<E>, force: bool) -> Result<Vec<crate::util::translate::bounds::Bounds>,()> {
        todo!()
    }

    fn focusable(&self) -> bool {
        todo!()
    }
    // fn run(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
    //     self.0.as_widget(root.fork(),ctx).run(root,ctx)
    // }
}

impl<T,E> AsWidget<E> for DummyWidget<T> where T: AsWidget<E>, E: Env  {
    type Widget = Self;
    type WidgetOwned = Self;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> crate::widget::as_widget::DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
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
