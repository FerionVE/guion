use super::*;

pub trait DynWidget<E>: Widget<E> where E: Env + 'static {
    #[inline]
    fn is<T: Any>(&self) -> bool {
        self.as_any().is::<T>() || self.as_any_inner().is::<T>()
    }
    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
        .or(
            self.as_any_inner().downcast_ref()
        )
    }
    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.as_any().is::<T>() {
            self.as_any_mut().downcast_mut()
        }else{
            self.as_any_inner_mut().downcast_mut()
        }
    }
}

impl<T,E> DynWidget<E> for T where T: Widget<E>, E: Env + 'static {

}

impl<E> DynWidget<E> for dyn Widget<E> where E: Env + 'static {

}
/// is implemented on any Widget, shouldn't be used from external
pub trait WidgetAsAny<E>: 'static where E: Env {
    fn _as_any(&self) -> &dyn Any;
    fn _as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T,E> WidgetAsAny<E> for T where T: Widget<E>, E: Env {
    fn _as_any(&self) -> &dyn Any {self}
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}
}