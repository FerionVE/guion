use super::*;

#[deprecated]
pub trait SubPath<E>:
    Clone +
    PartialEq<Self> +
    Sized +
where E: Env {
    fn from_id(id: E::WidgetID) -> Self;
    fn _eq_id(&self, id: E::WidgetID) -> bool;
    fn into_id(self) -> E::WidgetID;

    fn resolve_to_same_widget(&self, o: &Self) -> bool;

    fn is<T: Any>(&self) -> bool;
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
    fn downcast_into<T: Any>(self) -> Result<T,Self> where Self: Sized + 'static;
    #[inline]
    fn eq<I: SubPath<F> + 'static, F: Env>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, #[inline] |r| self == r )
    }
}
