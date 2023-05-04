use crate::aliases::{EEvent, ERenderer, ESize};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::{MouseMove, RootEvent, KbdDown, KbdPress, MouseDown, MouseUp, MouseLeave, WindowResize, WindowMove, TextInput, MouseScroll, KbdUp, MouseEnter};
use crate::event_new::Event;
use crate::invalidation::Invalidation;
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::queron::dyn_tunnel::QueronDyn;
use crate::render::StdRenderProps;
use crate::root::RootRef;
use crate::{event_new, EventResp};
use crate::event_new::variants::StdVariant;
use crate::intercept::{WidgetIntercept, InterceptBuilder};
use crate::newpath::{PathStack, PathResolvusDyn, FwdCompareStat, PathResolvus};
use crate::queron::Queron;
use crate::state::{CtxStdState, StdState};
use crate::util::tabulate::{TabulateDirection, tabi};
use crate::widget::Widget;

use super::StdInterceptLive;

impl<SB,E> WidgetIntercept<E> for StdInterceptLive<SB,E> where
    SB: InterceptBuilder<E>,
    E: Env,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    EEvent<E>: StdVarSup<E>
{
    // fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's {
    //     &self.sup
    // }

    #[inline] 
    fn _render<W>(
        &self,
        widget: &mut W,
        path: &mut NewPathStack,
        stack: StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    )
    where
        W: Widget<E> + ?Sized
    {
        self.sup._render(widget, path, stack, renderer, force_render, cache, root, ctx)
        //todo!()
    }
    #[inline] 
    fn _event_direct<W>(
        &self,
        widget: &mut W,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation
    where
        W: Widget<E> + ?Sized
    {
        //let widget_data = QueryCurrentWidget.query_in(stack).unwrap();

        //TODO it can't be! this could never know of the spacing borders, so we can't properly trace the hover here

        (self.access)(ctx).state.mouse.hover_last_seen = Some(widget.id());

        let vali = self.sup._event_direct(widget, path, stack, event, route_to_widget, root, ctx);

        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self | (event_mode.route_to_childs & event_mode.child_filter_point.is_some());

        if receive_self && event.query_variant::<MouseMove>(path,stack).is_some() {
            let self_id = widget.id();

            let mouse_state = &mut (self.access)(ctx).state.mouse;
            if mouse_state.hover_last_seen == Some(self_id) && mouse_state.hovered.as_ref().map_or(true, |(_,id)| self_id != *id ) {
                mouse_state.prev_hovered = mouse_state.hovered.take();
                mouse_state.hovered = Some((path.left_slice().to_owned(), self_id));
            }
        }

        vali
        //todo!()
    }
    //#[inline] 
    fn _event_root<W>(
        &self,
        root_widget: &mut W,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation
    where
        W: Widget<E> + ?Sized
    { //TODO BUG handle sudden invalidation of hovered widget
        let root_path_slice = path.left_slice().fetch();
        
        assert!(root_path_slice.is_empty()); //TODO stupid path.is_empty() check

        if let Some(ee) = event.query_variant::<RootEvent<E>>(path,stack) {
            let ee = ee.clone();
            let ts = event.ts();
            let mut passed = Invalidation::valid();
            match ee {
                RootEvent::KbdDown{key} => {
                    //Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                        if root_widget.resolve(id.0.as_slice(), root.fork(), ctx).is_err() {
                            //drop event if widget is gone
                            return Invalidation::valid();
                        }
                        (self.access)(ctx).state.key.down(
                            key.clone(),
                            id.clone(),
                            ts,
                            None,
                        );
                        //emit KbdDown event
                        let event = KbdDown{
                            key: key.clone(),
                        };
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(event,ts),//.with_filter_path_strict(id.path),
                            Some(id.0.as_slice()),
                            root.fork(), ctx,
                        );
                        /*let event = KbdPress{
                            key: key.clone(),
                            down_widget: id.clone(),
                            down_ts: e.2,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));*/
                        passed |= self._event_root(
                            root_widget,
                            path,
                            stack,
                            &StdVariant::new(RootEvent::KbdPress{key},ts),
                            route_to_widget, root, ctx,
                        ); // TODO discards filters from current RootEvent
                    }
                    //l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                },
                RootEvent::KbdUp{key} => {
                    let old = (self.access)(ctx).state.key.up(key);
                    if let Some(p) = old {
                        let event = KbdUp{
                            key: p.key,
                            down_widget: p.down.clone(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event.clone(),ts),//.with_filter_path_strict(id.path),
                                Some(id.0.as_slice()),
                                root.fork(), ctx,
                            );
                        }
                        //drop up if widget is gone TODO check if this is correct
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(event,ts),//.with_filter_path_strict(p.down.path),
                            Some(p.down.0.as_slice()),
                            root, ctx,
                        );
                    }
                },
                RootEvent::KbdPress{key} => {
                    let old = (self.access)(ctx).state.key.get(key.clone());
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let event = KbdPress{
                            key: p.key.clone(),
                            down_widget: p.down.clone(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event,ts),//.with_filter_path_strict(id.path.clone()),
                                Some(id.0.as_slice()),
                                root.fork(), ctx,
                            );
                            let mut do_tab = false;
                            if
                                key == MatchKeyCode::KbdTab &&
                                root_widget.resolve(id.0.as_slice(), root.fork(), ctx).map_or(false,|w| w.widget._tabulate_by_tab() )
                            {
                                do_tab = true;
                            }
                            if do_tab {
                                let reverse = ctx.state().is_pressed(MatchKeyCode::KbdShift).is_some();
                                let dir = if reverse {TabulateDirection::Backward} else {TabulateDirection::Forward};
                                let path = tabi(root_widget,path,stack,id.clone(),dir,root.fork(),ctx).expect("TODO");
                                (self.access)(ctx).state.kbd.focused = Some(path);
                            }
                        }
                    }
                }
                RootEvent::MouseDown{key} => {
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(RootEvent::MouseUp{key: key.clone()},ts),
                        route_to_widget, root.fork(), ctx,
                    ); // TODO discards filters from current RootEvent
                    //unfocus previously focused widget
                    passed |= self.unfocus(root_widget, path, stack, ts, root.fork(), ctx);

                    //the currently hovered widget
                    if let Some(pos) = (self.access)(ctx).state.mouse.pos {
                        if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                            (self.access)(ctx).state.key.down(key.clone(),hovered.clone(),ts,Some(pos));

                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(MouseDown{key,pos},ts),//.with_filter_path_strict(hovered.path.clone()),
                                Some(hovered.0.as_slice()),
                                root.fork(), ctx,
                            );


                            //passed |= Self::focus(l,hovered.path.clone(),e.1,e.2).unwrap_or(false);

                            let focus = root_widget.resolve(hovered.0.as_slice(), root.fork(), ctx)
                                .map_or(false,|w| w.widget._focus_on_mouse_down() );

                            if focus {
                                passed |= self.focus(root_widget, path, hovered, stack, ts, root, ctx).unwrap();
                            }
                        }
                    }
                }
                RootEvent::MouseUp{key} => {
                    let old = (self.access)(ctx).state.key.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        if let Some(pos) = (self.access)(ctx).state.mouse.pos {
                            let event = MouseUp{
                                key: p.key,
                                pos,
                                down_pos: p.cursor.expect("TODO"), //fails if a invalid press was inserted into the state
                                down_widget: p.down.clone(),
                                down_ts: p.ts
                            };
                            if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                                if hovered.1 != p.down.1 { //TODO is this correct
                                    passed |= root_widget.event_direct(
                                        path,
                                        stack,
                                        &StdVariant::new(event.clone(),ts),//.with_filter_path_strict(hovered.path),
                                        Some(hovered.0.as_slice()),
                                        root.fork(), ctx,
                                    );
                                }
                            }
                            //event dropped if widget gone
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event,ts),//.with_filter_path_strict(p.down.path),
                                Some(p.down.0.as_slice()),
                                root, ctx,
                            );
                        }
                    }
                }
                RootEvent::MouseMove{pos} => {
                    //set pos
                    (self.access)(ctx).state.mouse.pos = Some(pos);
                    //previous hovered widget
                    
                    //hover state will be updated as the event passes through the widget tree
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(MouseMove{pos},ts).with_filter_point(pos), //TODO infer path filter from RootEvent //TODO keep the path filter or allow at non-root widget, to allow e.g. multi-window
                        None, root.fork(), ctx,
                    ); // TODO discards filters from current RootEvent

                    if let Some(prev_hovered) = (self.access)(ctx).state.mouse.prev_hovered.take() {
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseLeave{},ts).direct_only(),//.with_filter_path_strict(p.path),
                            Some(prev_hovered.0.as_slice()),
                            root.fork(), ctx,
                        );

                        if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {//TODO optimize clone
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(MouseEnter{},ts).direct_only(),//.with_filter_path_strict(p.path),
                                Some(hovered.0.as_slice()),
                                root, ctx,
                            );
                        }
                    }
                }
                RootEvent::MouseLeaveWindow{} => {
                    if let Some(p) = (self.access)(ctx).state.mouse.hovered.clone() {//TODO optimize clone
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseLeave{},ts),//.with_filter_path_strict(p.path),
                            Some(p.0.as_slice()),
                            root, ctx,
                        );
                    }
                    let mouse = &mut (self.access)(ctx).state.mouse;
                    mouse.pos = None;
                    mouse.hovered = None;
                }
                RootEvent::WindowMove{pos,size} => {
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(WindowMove{pos,size},ts),
                        route_to_widget, root, ctx,
                    ); // TODO discards filters from current RootEvent
                }
                RootEvent::WindowResize{size} => {
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(WindowResize{size},ts),
                        route_to_widget, root, ctx,
                    ); // TODO discards filters from current RootEvent
                }
                RootEvent::TextInput{text} => {
                    if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(TextInput{text},ts),//.with_filter_path_strict(id.path),
                            Some(id.0.as_slice()),
                            root, ctx,
                        );
                    }
                }
                RootEvent::MouseScroll{x,y} => {
                    if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseScroll{x,y},ts),//.with_filter_path_strict(hovered.path),
                            Some(hovered.0.as_slice()),
                            root, ctx,
                        );
                    }
                }
            }
            passed
        }else{
            self.sup._event_root(root_widget, path, stack, event, route_to_widget, root, ctx)
        }
    }
    #[inline] 
    fn _size<W>(
        &self,
        w: &mut W,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>
    where
        W: Widget<E> + ?Sized
    {
        //todo!();
        self.sup._size(w, path, stack, root, ctx)
    }

    #[inline]
    fn respond_query<'a>(&'a self, _: crate::traitcast::WQueryResponder<'_,'a,E>) {}
    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, _: crate::traitcast::WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: crate::traitcast::WQueryGeneric<E> + ?Sized, G: ?Sized {}
}

/*impl<S,E> AsHandler<Self,E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context<'_>: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context<'_>) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &E::Context<'_>) -> &Self {
        c._handler()
    }
}

impl<S,E> AsHandler<S,E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context<'_>: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context<'_>) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &E::Context<'_>) -> &S {
        &c._handler().sup
    }
}*/

/*impl<S,C,T> AsHandler<S,C> for T where S: Handler<C>, C: Context, C::Handler: AsHandler<StdHandler<S,C>,C> + 'static {
    fn as_mut(c: &mut C) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &C) -> &S {
        &c._handler().sup
    }
}*/
