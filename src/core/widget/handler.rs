use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;
use crate::core::util::bounds::BoundedWidget;
use crate::core::widget::link::Link;
use crate::core::env::Env;
use crate::core::env::Context;
use crate::core::env::WidgetStore;

pub struct Handler<E> where E: Env {
    pub(crate) own_id: E::WidgetID,
    pub(crate) fns: HandlerFns<E>,
}

pub struct HandlerFns<E> where E: Env {
    pub render: fn(Link<E>, E::Renderer),
    pub event: fn(Link<E>, E::Event),
}

impl<E> Handler<E> where E: Env {
    pub fn render(&self, c: &mut E::Ctx, r: E::Renderer) {
        (self.fns.render)(self.link(c),r)
    }

    pub fn event(&self, c: &mut E::Ctx, e: E::Event) {
        (self.fns.event)(self.link(c),e)
    }

    pub fn iter<'a>(&self, c: &'a E::Ctx, predicate: impl FnMut(&BoundedWidget<E>)->bool ) -> impl Iterator<Item=(Bounds,&'a E::DynWidget)> {
        c.widgets().get(&self.own_id).unwrap()
            .childs()
            .filter(predicate)
            .map(move |e| {
                (
                    e.bounds,
                    c.widgets().get(&e.id).expect("Lost Child")
                )
            })
    }

    /*pub fn iter_mut<'a>(&self, c: &'a mut E::Ctx, predicate: impl FnMut(&BoundedWidget<E>)->bool ) -> impl Iterator<Item=(Bounds,&'a mut E::DynWidget)> {
        let childs: Vec<BoundedWidget<E>> = c.widgets().get(&self.own_id).unwrap().childs().collect();

        /*childs.into_iter()
            .filter(predicate)
            .map(move |e| {
                (
                    e.bounds,
                    c.widgets_mut().get_mut(&e.id).expect("Lost Child")
                )
            })*/

        FknIter {
            ctx: c,
            childs,
            idx: 0,
        }
    }*/

    pub fn iter_mut<'a>(&self, c: &'a mut E::Ctx, mut f: impl FnMut((Bounds,&mut E::DynWidget)) ) {
        let childs: Vec<BoundedWidget<E>> = c.widgets().get(&self.own_id).unwrap().childs().collect();

        for e in childs {
            let b = (
                e.bounds,
                c.widgets_mut().get_mut(&e.id).expect("Lost Child")
            );
            f(b);
        }
    }

    fn link<'a>(&self, c: &'a mut E::Ctx) -> Link<'a,E> {
        Link{
            ctx: c,
            widget_id: self.own_id.clone()
        }
    }
}

/*pub struct FknIter<'a,E> where E: Env {
    ctx: &'a mut E::Ctx,
    childs: Vec<BoundedWidget<E>>,
    idx: usize,
}

impl<'a,E> Iterator for FknIter<'a,E> where E: Env {
    type Item = (Bounds,&'a mut E::DynWidget);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.idx;
        self.idx+=1;
        let c = self.childs.get(c);
        
        if let Some(e) = c {
            Some((
                e.bounds,
                self.ctx.widgets_mut().get_mut(&e.id).expect("Lost Child")
            ))
        }else{
            None
        }
    }
}*/