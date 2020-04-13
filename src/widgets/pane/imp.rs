use super::*;

impl<'w,T,E> Widget<'w,E> for Pane<'w,T,E> where T: WidgetArray<'w,E>+Statize, T::Statur: Statize+Sized, E: Env {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn _event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        _event(l,e,self.orientation)
    }
    fn _size(&self, l: Link<E>) -> ESize<E> {
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
        self.childs.childs()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        self.childs.into_childs()
    }

    fn focusable(&self) -> bool {
        false
    }

    fn border(&self, b: &mut Border) {
        if let Some(senf) = &self.border {
            *b = *senf;
        }
    }

    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        self.childs.child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        self.childs.into_child(i)
    }
}
impl<'w,T,E> WidgetMut<'w,E> for Pane<'w,T,E> where T: WidgetArrayMut<'w,E>+Statize, T::Statur: Statize+Sized, E: Env {
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.childs.childs_mut()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        self.childs.into_childs_mut()
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        self.childs.child_mut(i)
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        self.childs.into_child_mut(i)
    }
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