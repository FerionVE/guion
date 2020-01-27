use std::ops::DerefMut;
use std::ops::Deref;
use super::*;

pub trait DynWidget<E>: Widget<E> where E: Env + 'static {
    type Owned: Deref<Target=Self> + DerefMut<Target=Self>;
    //type Ref: Widget<E>;
    //type RefMut: Widget<E>;

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
    fn erase<T: Widget<E>>(w: &T) -> &Self;
    fn erase_mut<T: Widget<E>>(w: &mut T) -> &mut Self;
    fn erase_move<T: Widget<E>>(w: T) -> Self::Owned;
    //fn downcast_into<T: Widget<E>>(e: Self::Owned) -> Result<T,Self::Owned>;
}

impl<E> DynWidget<E> for dyn Widget<E> where E: Env + 'static {
    type Owned = Box<dyn Widget<E>>;

    fn erase<T: Widget<E>>(w: &T) -> &Self {w}
    fn erase_mut<T: Widget<E>>(w: &mut T) -> &mut Self {w}
    fn erase_move<T: Widget<E>>(w: T) -> Self::Owned {Box::new(w)}
    //fn downcast_into<T: Widget<E>>(e: Self::Owned) -> Result<T,Self::Owned> {Box::downcast(e)}
}
/// is implemented on any Widget, shouldn't be used from external
pub trait WidgetAsAny<E>: 'static where E: Env {
    fn _as_any(&self) -> &dyn Any;
    fn _as_any_mut(&mut self) -> &mut dyn Any;
    fn _erase(&self) -> &E::DynWidget;
    fn _erase_mut(&mut self) -> &mut E::DynWidget;
    fn _as_immediate(&self) -> WidgetRef<E>;
    fn _as_immediate_mut(&mut self) -> WidgetRefMut<E>;
}

impl<T,E> WidgetAsAny<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn _as_any(&self) -> &dyn Any {self}
    #[inline]
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}
    #[inline]
    fn _erase(&self) -> &E::DynWidget {
        DynWidget::erase(self)
    }
    #[inline]
    fn _erase_mut(&mut self) -> &mut E::DynWidget {
        DynWidget::erase_mut(self)
    }
    #[inline]
    fn _as_immediate<'a>(&'a self) -> WidgetRef<'a,E> {
        Box::new(self)
    }
    #[inline]
    fn _as_immediate_mut<'a>(&'a mut self) -> WidgetRefMut<'a,E> {
        Box::new(self)
    }
}