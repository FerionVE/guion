use std::{slice::SliceIndex, ops::{RangeBounds, Deref}};
use super::ctx::widgets::Widgets;
use qwutils::*;
use super::*;

pub mod sub;
pub use sub::*;

/// A WidgetPath contains information to resolve to a specific Widget in a widget tree
pub trait WidgetPath<E>: AsWidget<E> + AsWidgetImmediate<'static,E> + AsWidgetImmediateMut<'static,E> + RefClonable + Clone + PartialEq + Sized + Send + Sync + 'static where E: Env<WidgetPath=Self> {
    type SubPath: SubPath<E>;
    
    fn attach(&mut self, sub: Self::SubPath);
    fn attached(mut self, sub: Self::SubPath) -> Self {
        self.attach(sub);
        self
    }

    fn attach_subpath(&mut self, sub: &Self);
    fn attached_subpath(mut self, sub: &Self) -> Self {
        self.attach_subpath(sub);
        self
    }

    fn id(&self) -> &E::WidgetID; //TODO no ref id
    fn tip(&self) -> &Self::SubPath;

    fn parent(&self) -> Option<Self>;

    fn is_empty(&self) -> bool;

    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize>;
    fn index<T>(&self, i: T) -> &Self::SubPath where T: SliceIndex<[Self::SubPath],Output=Self::SubPath>;

    #[inline]
    fn eq<F: Env + 'static>(&self, o: &F::WidgetPath) -> bool where Self: 'static/*, for<'a> &'a I: AsPathSlice<'a>*/ {
        self.id().id_eq(o.id())
    }
    
    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    fn child_paths_of_slice<'a>(&self, c: CtxRefR<'a,E>) -> Result<Vec<E::WidgetPath>,()> {
        Ok( c.0.widget(self.refc())?.child_paths() )
    }

    #[inline]
    fn render(&self, c: CtxRef<E>, r: &mut RenderLink<E>) -> Result<bool,()> {
        Ok( c.1.render(c.0.widget(self.refc())?,r) )
    }
    #[inline]
    fn event(&self, c: CtxRef<E>, e: (EEvent<E>,&Bounds,u64)) -> Result<(),()> {
        Ok( c.1.event(c.0.widget(self.refc())?,e) )
    }
    #[inline]
    fn size(&self, c: CtxRef<E>) -> Result<ESize<E>,()> {
        Ok( c.1.size(c.0.widget(self.refc())?) )
    }

    #[inline]
    fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> Self where E::WidgetPath: WidgetPath<F> {
        self
    }
}

#[inline]
pub fn rc_path_with_env<E: Env, F: Env<WidgetPath=E::WidgetPath>>(e: E::WidgetPath) -> F::WidgetPath where E::WidgetPath: WidgetPath<F> {
    e
}