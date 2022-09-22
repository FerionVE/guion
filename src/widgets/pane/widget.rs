use crate::dispatchor::{AsWidgetsClosure, AsWidgetsAllClosure};
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds, for_child_widget};

use super::*;

impl<'w,E,T> Widget<E> for Pane<'w,E,T> where
    E: Env,
    for<'a> T: AsWidgets<'a,E>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        let render_props = StdRenderProps::new(stack)
            .inside_spacing_border();

        let child_bounds = self.child_bounds(stack, &render_props.absolute_bounds, true, root.fork(), ctx).unwrap();

        self.childs.all(
            AsWidgetsAllClosure::new(|idx,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                widget.render(
                    &for_child_widget(
                        render_props
                            .slice(&child_bounds[idx]),
                        widget
                    ),
                    renderer,
                    root,ctx
                )
            }),
            root,ctx
        );
        //TODO FIX viewport
    }

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);

        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.route_to_childs {return false;}

        let child_bounds = self.child_bounds(&stack, &bounds.bounds, true, root.fork(), ctx).unwrap();

        let mut passed = false;

        self.childs.all(
            AsWidgetsAllClosure::new(|idx,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                let stack = WithCurrentBounds {
                    inner: for_child_widget(&stack,widget),
                    bounds: bounds.bounds.slice(&child_bounds[idx]),
                    viewport: bounds.viewport.clone(),
                };
    
                passed |= widget.event_direct(&stack,event,root,ctx);
            }),
            root,ctx
        );

        passed
    }

    fn _size<P>(
        &self,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| {
                let mut s = ESize::<E>::empty();

                self.childs.all(
                    AsWidgetsAllClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>|
                        //TODO bounds could never be used in constraints calc, else we would already need to have the child bounds calculates, which also requires the constraints
                        s.add( &widget.size(&stack,root,ctx), self.orientation )
                    ),
                    root,ctx
                );

                s
            }
        );

        size
    }

    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        //TODO holy stack
        let mut child_sizes = Vec::with_capacity(self.childs());

        self.childs.all(
            AsWidgetsAllClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>|
                //TODO bounds could never be used in constraints calc, else we would already need to have the child bounds calculates, which also requires the constraints
                child_sizes.push( widget.size(&stack,root,ctx) )
            ),
            root,ctx
        );

        let bounds = calc_bounds(&b.size,&child_sizes,self.orientation); 

        Ok(bounds)
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        self.childs.by_index(
            i,
            AsWidgetsClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
            ),
            root,ctx
        ).unwrap_or_else(|| todo!()/*(callback)(Err(()),ctx)*/ )
    }

    fn focusable(&self) -> bool {
        false
    }
}

impl<'z,E,T> AsWidget<'z,E> for Pane<'z,E,T> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}
