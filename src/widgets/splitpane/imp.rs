use super::*;
use util::state::*;

impl<'w,L,R,V,E> Widget<'w,E> for SplitPane<'w,L,R,V,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    L: AsWidget<'w,E>+Statize+Sized+'w, L::Statur: Sized,
    R: AsWidget<'w,E>+Statize+Sized+'w, R::Statur: Sized,
    V: AtomState<f32>+Statize+Sized+'w, V::Statur: Sized,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) -> bool {
        let bounds = self.calc_bounds(&r.b); 
        
        let mut validate = true;

        {
            let left = l.for_child(0).expect("Dead Path inside Pane");
            let mut r = r.slice(&bounds[0]);
            validate &= r.render_widget(left);
        }
        {
            let right = l.for_child(1).expect("Dead Path inside Pane");
            let mut r = r.slice(&bounds[2]);
            validate &= r.render_widget(right);
        }
        {
            //TODO render center
        }

        false
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        let bounds = self.calc_bounds(e.1); 

        {
            let sliced = e.1.slice(&bounds[1]);
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                //event hits the center
                //TODO receive mousemove here and let update f32
                //mousemove should also be sent to mousedown initiating widget with flag receive_outer_mousemove or such
                //get x/y of curser
                //subtract min constraints of left/right widget from left/right of widget's bounds (including self.handle_width/2)
                //limit (min/max) curser x to the shrinked bounds
                //into relative f32
                //enqueue mutate setting the AtomStateMut
            }
        }
        {
            let mut left = l.for_child(0).expect("Dead Path inside Pane");
            let sliced = e.1.slice(&bounds[0]);
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                left.event((ee,&sliced,e.2));
            }
        }
        {
            let mut right = l.for_child(0).expect("Dead Path inside Pane");
            let sliced = e.1.slice(&bounds[2]);
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                right.event((ee,&sliced,e.2));
            }
        }
    }
    fn _size(&self, mut l: Link<E>) -> ESize<E> {
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(), self.orientation) ).expect("Dead Path inside Pane");
        s.add_space(self.width,self.orientation);
        s
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        if i > 2 {return Err(());}
        Ok(self.calc_bounds(b)[i])
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
        let _ = b;
    }

    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        self.childs.child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        self.childs.into_child(i)
    }
}
impl<'w,L,R,V,E> WidgetMut<'w,E> for SplitPane<'w,L,R,V,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    L: AsWidgetMut<'w,E>+Statize+Sized+'w, L::Statur: Sized,
    R: AsWidgetMut<'w,E>+Statize+Sized+'w, R::Statur: Sized,
    V: AtomStateMut<f32>+Statize+Sized+'w, V::Statur: Sized,
{
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

impl<'w,L,R,V,E> SplitPane<'w,L,R,V,E> where
    E: Env,
    V: AtomState<f32>+Statize+Sized+'w, V::Statur: Sized,
{
    fn calc_bounds(&self, b: &Bounds) -> [Bounds;3] {
        let o = self.orientation;
        let (x,w) = b.par(o);
        let (y,h) = b.unpar(o);
        let v = self.state.get();
        let w0 = ((w as f32 - self.width as f32)*v.max(0.0).min(1.0)) as u32;
        let w2 = w - w0 - self.width;
        let x1 = x + w0 as i32;
        let x2 = x1 + self.width as i32;
        let left = Bounds::from_ori(x, y, w0, h, o);
        let center = Bounds::from_ori(x1, y, self.width, h, o);
        let right = Bounds::from_ori(x2, y, w2, h, o);
        [left,center,right]
    }
}