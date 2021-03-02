//! Traits for state types
use super::*;
use std::borrow::Cow;

/// Simple atomic type state
pub trait AtomState<E,T> where E: Env {
    #[inline]
    fn get(&self, _: &mut E::Context) -> T {
        self.get_direct().unwrap()
    }
    fn get_direct(&self) -> Result<T,()>;

    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>>;
    fn try_set(&mut self, v: T, _: &mut E::Context) -> Result<(),GuionError<E>>;
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>>;

    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's;
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's;
}
/// Simple atomic type state
pub trait AtomStateMut<E,T>: AtomState<E,T> where E: Env {
    #[inline]
    fn set(&mut self, v: T, _: &mut E::Context) {
        self.set_direct(v).unwrap()
    }
    fn set_direct(&mut self, v: T) -> Result<(),()>;
}

impl<E,T> AtomState<E,T> for T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Ok(self)
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set(self,v,c);
        Ok(())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set_direct(self,v)?;
        Ok(())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's{
        Box::new(self)
    }
}
impl<E,T> AtomState<E,T> for &T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((**self).clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set(&mut self, v: T, _: &mut E::Context) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
}
impl<E,T> AtomState<E,T> for &mut T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((**self).clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Ok(self)
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set(self,v,c);
        Ok(())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set_direct(self,v)?;
        Ok(())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
}
impl<E,T> AtomStateMut<E,T> for &mut T where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        **self = v;
        Ok(())
    }
}
impl<E,T> AtomStateMut<E,T> for T where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((*self.as_ref()).clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Ok(self)
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set(self,v,c);
        Ok(())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set_direct(self,v)?;
        Ok(())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(self)
    }
}
impl<E,T> AtomState<E,T> for &Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((*self.as_ref()).clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
}
impl<E,T> AtomState<E,T> for &mut Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((*self.as_ref()).clone())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Ok(self)
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set(self,v,c);
        Ok(())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set_direct(self,v)?;
        Ok(())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
}
impl<E,T> AtomStateMut<E,T> for Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self.to_mut() = v;
        Ok(())
    }
}
impl<E,T> AtomStateMut<E,T> for &mut Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self.to_mut() = v;
        Ok(())
    }
}

unsafe impl<T,E> Statize<E> for dyn AtomState<E,T> where T: 'static, E: Env {
    type Statur = dyn AtomState<E,T>;
}
unsafe impl<T,E> Statize<E> for dyn AtomStateMut<E,T> where T: 'static, E: Env {
    type Statur = dyn AtomStateMut<E,T>;
}

unsafe impl<'z,'s:'z,'w:'s,T,E> Traitcast<'s,dyn AtomState<E,T>+'z,E> for dyn Widget<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomState<E,T>;
}
/*unsafe impl<'w,T,E> TraitcastMut<'w,Box<dyn AtomState<E,T>+'w>,E> for dyn WidgetMut<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomState<E,T>;
}*/
/*unsafe impl<'w,T,E> TraitcastMut<dyn AtomStateMut<E,T>+'w,E> for dyn WidgetMut<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomStateMut<E,T>;
}*/

impl<E,T> AtomState<E,T> for &dyn AtomState<E,T> where E: Env {
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        (**self).get(c)
    }
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set(&mut self, v: T, _: &mut E::Context) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        Err(todo!())
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }

}
impl<E,T> AtomState<E,T> for &mut dyn AtomState<E,T> where E: Env {
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        (**self).get(c)
    }#[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        (**self).mutate()
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        (**self).try_set(v,c)
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        (**self).try_set_direct(v)
    }
    #[inline]
    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
    #[inline]
    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,T>+'_> where Self: 's {
        Box::new(*self)
    }
}