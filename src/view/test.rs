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

impl<E,MutFn> View<E,MutFn> for &TestRoot where
    MutFn: for<'a> Fn(E::RootMut<'a>,&'a (),&mut E::Context<'_>)->ResolveResult<&'a mut TestRoot> + Clone + 'static,
    E: Env,
{
    type Viewed = impl Widget<E>;

    fn view(self, remut: MutFn, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Self::Viewed {
        DummyWidget(view_widget!(
            || &self.a,
            remut => |root,todo_omittable_field| &mut root.a
        ))
    }
}

/*impl<E,MutFn> View<E,MutFn> for &A where MutFn: for<'a> Fn(&'a mut Root,&mut E::Context<'_>)->ResolveResult<&'a mut A> + Clone + 'static, Ctx: Queue<Root> {
    type Viewed = impl Widget<E>;

    fn view(self, remut: MutFn, ctx: &mut E::Context<'_>) -> Self::Viewed {
        view_widget(
            || &self.b,
            move |E| &mut remut(E).b,
        )
    }
}*/
impl_view!(
    for &A : <'a> &'a mut A {
        fn view(self, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Viewed {
            // self.b.view(
            //     mutor!(remut => |root,todo_omittable_field| &mut root.b),
            //     ctx
            // ) // direct nested view
            DummyWidget(view_widget!(
                || &self.b,
                remut => |root,todo_omittable_field| &mut root.b
            )) // lazy nested view
        }
    }
);

impl<E,MutFn> View<E,MutFn> for &B where
    MutFn: for<'a> Fn(E::RootMut<'a>,&'a (),&mut E::Context<'_>)->ResolveResult<&'a mut B> + Clone + 'static,
    E: Env,
{
    type Viewed = impl Widget<E>;

    fn view(self, remut: MutFn, root: E::RootRef<'_>, _: &mut E::Context<'_>) -> Self::Viewed {
        DummyWidget(view_widget!(
            || ViewC(&self.c),
            remut => |root,todo_omittable_field| ViewCMut(&mut root.c)
        ))
    }
}

impl<E,MutFn> View<E,MutFn> for ViewC<'_> where
    MutFn: for<'a> Fn(E::RootMut<'a>,&'a (),&mut E::Context<'_>)->ResolveResult<ViewCMut<'a>> + Clone + 'static,
    E: Env,
{
    type Viewed = impl Widget<E>;

    fn view(self, remut: MutFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Viewed {
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
// fn suse() {
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
