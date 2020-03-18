use super::*;
use std::marker::PhantomData;

use calc::calc_bounds;

pub struct Pane<'w,T,E> where E: Env, T: Statize<E>+Sized+'w {
    id: E::WidgetID,
    childs: Vec<T>,
    orientation: Orientation,
    p: PhantomData<&'w mut ()>,
}

impl<'w,T,E> Pane<'w,T,E> where E: Env, T: AsWidget<'w,E>+Statize<E> {
    pub fn new(id: E::WidgetID, childs: Vec<T>, orientation: Orientation) -> Pane<'w,T,E> {
        Pane{
            id,
            childs,
            orientation,
            p: PhantomData,
        }
    }
}

impl<'w,T,E> Widget<'w,E> for Pane<'w,T,E> where T: AsWidget<'w,E>+Statize<E>, T::Statur: Statize<E>+Sized, E: Env {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        _trace_bounds(l,i,b,force,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
        //self.invalid
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.childs.iter()
            .map(|c| c.as_ref() )
            .collect::<Vec<_>>()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        self.childs.into_iter()
            .map(|c: T| c.consume_ref() )
            .collect::<Vec<_>>()
    }

    fn focusable(&self) -> bool {
        false
    }

    fn border(&self, mut b: &mut Border) {
        //*b = Border::empty();
        //b/=2;
    }
}
impl<'w,T,E> WidgetMut<'w,E> for Pane<'w,T,E> where T: AsWidgetMut<'w,E>+Statize<E>, T::Statur: Statize<E>+Sized, E: Env {
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.childs.iter_mut()
            .map(|c| c.as_mut() )
            .collect::<Vec<_>>()
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        self.childs.into_iter()
            .map(|c| c.consume_mut() )
            .collect::<Vec<_>>()
    }
}
unsafe impl<'w,T,E> Statize<E> for Pane<'w,T,E> where T: Statize<E>, T::Statur: Statize<E>+Sized, E: Env {
    type Statur = Pane<'static,T::Statur,E>;
}

pub fn _render<E>(mut l: Link<E>, r: &mut RenderLink<E>, o: Orientation) -> bool where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&r.b.size,&sizes,o); 
    
    let mut validate = true;
    let mut i = 0usize;

    l.for_childs(|c| {
        let mut r = r.slice(&bounds[i]);
        validate &= r.render_widget(c);
        i+=1;
    }).expect("Dead Path inside Pane");

    false
}

pub fn _event<E>(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64), o: Orientation) where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&e.1.size,&sizes,o); 

    let mut i = 0usize;

    l.for_childs(|mut c| {
        let sliced = e.1.slice(&bounds[i]);
        if let Some(ee) = e.0.filter_cloned(&sliced) {
            c.event((ee,&sliced,e.2));
        }
        i+=1;
    }).expect("Dead Path inside Pane");
}

pub fn _size<E>(mut l: Link<E>, o: Orientation) -> ESize<E> where
    E: Env,
{
    let mut s = ESize::<E>::empty();
    l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(), o) ).expect("Dead Path inside Pane");
    s
}

pub fn _trace_bounds<E>(mut l: Link<E>, i: usize, b: &Bounds, force: bool, o: Orientation) -> Result<Bounds,()> where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&b.size,&sizes,o); 

    bounds.get(i).map(|w| *w).ok_or(())
}

/*pub fn brokion<'a,W,E>(id: E::WidgetID, e: W) -> Pane<'a,W,E> where W: WidgetImmediate<'a,E>, E: Env {
    Pane::immediate(
        id,
        vec![e],
        Orientation::Horizontal,
    )
}
pub fn bockion<'a,W,E>(id: E::WidgetID, e: W) -> bool where W: WidgetImmediate<'a,E>, E: Env {
    let pane = brokion::<'a,W,E>(id, e);
    pane.invalid()
}*/