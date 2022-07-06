use crate::ctx::Context;
use crate::ctx::queue::Queue;
use crate::env::Env;
use crate::error::ResolveResult;
use crate::handler::Handler;
use crate::{impl_view, view_widget, mutor};
use crate::widget::Widget;

use super::View;
use super::view_widget::DummyWidget;

pub struct TestRoot {
    a: A,
}

pub struct A {
    b: B,
}

pub struct B {
    c: C,
}

pub struct C {
    d: u32,
}

pub struct ViewC<'a>(&'a C);
pub struct ViewCMut<'a>(&'a mut C);

impl<'z,E> View<'z,E> for TestRoot where
    E: Env,
{
    type Viewed<'v,MutFn> = dyn WidgetDyn<E>+'v where MutFn: 'static, 'z: 'v;
    type Mutable<'k> = &'k mut TestRoot;

    fn view<'d,MutFn,DispatchFn>(&'d self, dispatch: DispatchFn, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>)
    where
        MutFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Ctx<'cc>) -> ResolveResult<Self::Mutable<'s>> + Clone + 'static,
        DispatchFn: ViewDispatch<'z,Self,MutFn,E>,
    {
        let w = DummyWidget(view_widget!(
            || &self.a,
            remut => |root,todo_omittable_field| &mut root.a
        ));

        dispatch.call(&w,root,ctx)
    }
    
    // fn view(self, remut: MutFn, _: E::RootRef<'_>, _: &mut E::Ctx<'_>) -> Self::Viewed {
    //     DummyWidget(view_widget!(
    //         || &self.a,
    //         remut => |root,todo_omittable_field| &mut root.a
    //     ))
    // }

    
}

// impl_view!(
//     for &A : <'a> &'a mut A {
//         fn view(self, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>) -> Self::Viewed {
//             // self.b.view(
//             //     mutor!(remut => |root,todo_omittable_field| &mut root.b),
//             //     ctx
//             // ) // direct nested view
//             DummyWidget(view_widget!(
//                 || &self.b,
//                 remut => |root,todo_omittable_field| &mut root.b
//             )) // lazy nested view
//         }
//     }
// );

impl<'z,E> View<'z,E> for A where
    E: Env,
{
    type Viewed<'v,MutFn> = dyn WidgetDyn<E>+'v where MutFn: 'static, 'z: 'v;
    type Mutable<'k> = &'k mut A;

    fn view<'d,MutFn,DispatchFn>(&'d self, dispatch: DispatchFn, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>)
    where
        MutFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Ctx<'cc>) -> ResolveResult<Self::Mutable<'s>> + Clone + 'static,
        DispatchFn: ViewDispatch<'z,Self,MutFn,E>,
    {
        let w = DummyWidget(view_widget!(
            || &self.b,
            remut => |root,todo_omittable_field| &mut root.b
        ));

        let m = messaged!(E;remut |root,ctx|a:()| 25i32);

        dispatch.call(&w,root,ctx)
    }
}

impl<'z,E> Messagable<E> for &'z mut A where E: Env {
    fn message(&mut self, m: &dyn std::any::Any, ctx: &mut <E as Env>::Ctx<'_>) {
        todo!()
    }
}

// impl<E,MutFn> View<E,MutFn> for &B where
//     MutFn: for<'a> Fn(E::RootMut<'a>,&'a (),&mut E::Ctx<'_>)->ResolveResult<&'a mut B> + Clone + 'static,
//     E: Env,
// {
//     type Viewed = impl Widget<E>;

//     fn view(self, remut: MutFn, root: E::RootRef<'_>, _: &mut E::Ctx<'_>) -> Self::Viewed {
//         DummyWidget(view_widget!(
//             || ViewC(&self.c),
//             remut => |root,todo_omittable_field| ViewCMut(&mut root.c)
//         ))
//     }
// }

impl<'z,E> View<'z,E> for B where
    E: Env,
{
    type Viewed<'v,MutFn> = dyn WidgetDyn<E>+'v where MutFn: 'static, 'z: 'v;
    type Mutable<'k> = &'k mut B;

    fn view<'d,MutFn,DispatchFn>(&'d self, dispatch: DispatchFn, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>)
    where
        MutFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Ctx<'cc>) -> ResolveResult<Self::Mutable<'s>> + Clone + 'static,
        DispatchFn: ViewDispatch<'z,Self,MutFn,E>,
        Self: 'z,
    {
        let c = ViewC(&self.c);
        let w = DummyWidget(view_widget!(
            || &c,
            remut => |root,todo_omittable_field| ViewCMut(&mut root.c)
        ));

        dispatch.call(&w,root,ctx)
    }
}

// impl<E,MutFn> View<E,MutFn> for ViewC<'_> where
//     MutFn: for<'a> Fn(E::RootMut<'a>,&'a (),&mut E::Ctx<'_>)->ResolveResult<ViewCMut<'a>> + Clone + 'static,
//     E: Env,
// {
//     type Viewed = impl Widget<E>;

//     fn view(self, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>) -> Self::Viewed {
//         ctx.enqueue(
//             mutor!(remut =>| |root,todo_omittable_field| root.0.d = 42 )
//         )
//     }
// }

impl<'z,E> View<'z,E> for ViewC<'z> where
    E: Env,
{
    type Viewed<'v,MutFn> = dyn WidgetDyn<E>+'v where MutFn: 'static, Self: 'v;
    type Mutable<'k> = ViewCMut<'k>;

    fn view<'d,MutFn,DispatchFn>(&'d self, dispatch: DispatchFn, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Ctx<'_>)
    where
        MutFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Ctx<'cc>) -> ResolveResult<Self::Mutable<'s>> + Clone + 'static,
        DispatchFn: ViewDispatch<'z,Self,MutFn,E>,
    {
        ctx.enqueue(
            mutor!(remut =>| |root,todo_omittable_field| root.0.d = 42 )
        );
    }
}

// TestEnv/TestCtx separation required here, else Queue closure would do funny closure infer lifetime reduction bug
// pub struct TestEnv;

// pub struct TestCtx<E> where E: Env {
//     v: Vec<Box<dyn for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>) + 'static>>,
// }

// impl Env for TestEnv {
//     type RootRef<'a> = &'a TestRoot;
//     type RootMut<'a> = &'a mut TestRoot;
//     type Ctx<'a> = TestCtx<Self>;
// }

// impl<E> Context<E> for TestCtx<E> where E: Env {
//     type HandlerStack where Self: Sized = impl Handler<E>;

//     fn make_handler_stack_adhoc() -> Self::HandlerStack where Self: Sized {
//         ()
//     }
// }

// impl<E> Queue<E> for TestCtx<E> where E: Env {
//     fn enqueue(&mut self, f: impl for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>) + Clone + 'static ) {
//         self.v.push(Box::new(f));
//     }
// }

// #[test]
// fn views() {
//     let mut ctx = TestCtx{v: Vec::new()};
    
//     {
//         let mut dom = TestRoot{a:A{b:B{c:C{d:23}}}};

//         {
//             View::<TestEnv,_>::view(
//                 &dom,
//                 |mut a,_,_| {
//                     let _ = crate::root::RootMut::<TestEnv>::fork(&mut a).a.b.c.d; //test
//                     Ok(a)
//                 },
//                 &dom,
//                 &mut ctx,
//             ).run(&dom,&mut ctx);
//         }

//         assert_eq!(dom.a.b.c.d, 23);

//         for i in std::mem::replace(&mut ctx.v, Vec::new()) {
//             i(&mut dom,&(),&mut ctx);
//         }

//         assert_eq!(dom.a.b.c.d, 42);
//     }
// }
