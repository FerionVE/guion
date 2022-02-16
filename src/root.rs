use crate::env::Env;
use crate::widget::resolved::Resolved;
use crate::util::bounds::Bounds;

pub trait RootRef<E> where E: Env {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's;

    //TODO fix old resolve stack
    fn widget(&self, i: E::WidgetPath) -> Result<Resolved<E>,E::Error>;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath) -> bool {
        self.widget(i).is_ok()
    }

    #[deprecated] 
    fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error>;
}

pub trait RootMut<E> where E: Env {
    fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's;
}

impl<'a,T,E> RootRef<E> for &'a T where for<'z> E: Env<RootRef<'z>=&'z T> {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's {
        &**self
    }

    fn widget(&self, i: <E as Env>::WidgetPath) -> Result<Resolved<E>,<E as Env>::Error> {
        todo!()
    }

    fn has_widget(&self, i: E::WidgetPath) -> bool {
        todo!()
    }

    fn trace_bounds(&self, ctx: &mut <E as Env>::Context<'_>, i: <E as Env>::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,<E as Env>::Error> {
        todo!()
    }
}

impl<'a,T,E> RootMut<E> for &'a mut T where for<'z> E: Env<RootMut<'z>=&'z mut T> {
    fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's {
        &mut **self
    }
}

impl<'a,T,E> RootRef<E> for std::borrow::Cow<'a,T> where for<'z> E: Env<RootRef<'z>=std::borrow::Cow<'z,T>>, T: Clone {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's {
        std::borrow::Cow::Borrowed(self.as_ref())
    }

    fn widget(&self, i: <E as Env>::WidgetPath) -> Result<Resolved<E>,<E as Env>::Error> {
        todo!()
    }

    fn has_widget(&self, i: E::WidgetPath) -> bool {
        todo!()
    }

    fn trace_bounds(&self, ctx: &mut <E as Env>::Context<'_>, i: <E as Env>::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,<E as Env>::Error> {
        todo!()
    }
}
