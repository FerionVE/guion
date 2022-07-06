use crate::aliases::EStyle;
use crate::env::Env;
//use crate::widget::resolved::Resolved;
use crate::util::bounds::Bounds;
use crate::widget::dyn_tunnel::WidgetDyn;

pub trait RootRef<E> where E: Env {
    fn fork<'s,'w:'s>(&'s self) -> E::RootRef<'w> where Self: 'w;

    //TODO fix old resolve stack
    fn with_widget<'s,'l:'s>(
        &'s self,
        i: E::WidgetPath,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>),
        ctx: &mut E::Context<'_>,
    ) -> Result<(),E::Error> where Self: 'l;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath, ctx: &mut E::Context<'_>) -> bool {
        self.with_widget(i,&mut |_,_|{},ctx).is_ok()
    }

    #[deprecated] 
    fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error>;
}

pub trait RootMut<E> where E: Env {
    fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's;
}

// impl<'a,T,E> RootRef<E> for &'a T where for<'z> E: Env<RootRef<'z>=&'z T> {
//     #[inline]
//     fn fork<'s,'w:'s>(&'s self) -> E::RootRef<'w> where Self: 'w {
//         &**self
//     }
//     #[inline]
//     fn widget<'s,'w:'s>(&'s self, i: E::WidgetPath, ctx: &mut E::Context<'_>) -> Result<Resolved<'w,E>,E::Error> where Self: 'w {
//         todo!()
//     }
//     #[inline]
//     fn has_widget(&self, i: E::WidgetPath, ctx: &mut E::Context<'_>) -> bool {
//         todo!()
//     }
//     #[inline]
//     fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
//         todo!()
//     }
// }

// impl<'a,T,E> RootMut<E> for &'a mut T where for<'z> E: Env<RootMut<'z>=&'z mut T> {
//     #[inline]
//     fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's {
//         &mut **self
//     }
// }

// impl<'a,T,E> RootRef<E> for std::borrow::Cow<'a,T> where for<'z> E: Env<RootRef<'z>=std::borrow::Cow<'z,T>>, T: Clone {
//     fn fork<'s,'w:'s>(&'s self) -> E::RootRef<'w> where Self: 'w {
//         std::borrow::Cow::Borrowed(self.as_ref())
//     }

//     fn widget<'s,'w:'s>(&'s self, i: E::WidgetPath) -> Result<Resolved<'w,E>,E::Error> where Self: 'w {
//         todo!()
//     }

//     fn has_widget(&self, i: E::WidgetPath) -> bool {
//         todo!()
//     }

//     fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
//         todo!()
//     }
// }
