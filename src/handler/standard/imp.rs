use crate::*;
use crate::newpath::{PathStack, PathResolvus, FwdCompareStat};
use crate::queron::query::Query;
use crate::root::RootRef;
use super::*;

impl<SB,E> Handler<E> for StdHandlerLive<SB,E> where
    SB: HandlerBuilder<E>,
    E: Env,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    EEvent<E>: StdVarSup<E>
{
    // fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's {
    //     &self.sup
    // }

    #[inline] 
    fn _render<W,Ph,S>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    )
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized
    {
        self.sup._render(widget, path, stack, renderer, force_render, cache, root, ctx)
        //todo!()
    }
    #[inline] 
    fn _event_direct<W,Ph,S,Evt>(
        &self,
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized
    {
        //let widget_data = QueryCurrentWidget.query_in(stack).unwrap();

        //TODO it can't be! this could never know of the spacing borders, so we can't properly trace the hover here

        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        if event_mode.receive_self && event.query_variant::<MouseMove,_,_>(path,stack).is_some() {
            (self.access)(ctx).state.mouse.hovered = Some(path.into_resolvus());
        }

        self.sup._event_direct(widget, path, stack, event, route_to_widget, cache, root, ctx)
        //todo!()
    }
    //#[inline] 
    fn _event_root<W,Ph,S,Evt>(
        &self,
        root_widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized
    { //TODO BUG handle sudden invalidation of hovered widget
        assert!(path.inner().is_none()); //TODO stupid path.is_empty() check

        if let Some(ee) = event.query_variant::<RootEvent<E>,_,_>(path,stack) {
            let ee = ee.clone();
            let ts = event.ts();
            let mut passed = false;
            match ee {
                RootEvent::KbdDown{key} => {
                    //Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                        if !root.has_widget(&*id,ctx) {
                            //drop event if widget is gone
                            return false;
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
                            Some(&*id),
                            cache, root.fork(), ctx,
                        );
                        /*let event = KbdPress{
                            key: key.clone(),
                            down_widget: id.refc(),
                            down_ts: e.2,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));*/
                        passed |= self._event_root(
                            root_widget,
                            path,
                            stack,
                            &StdVariant::new(RootEvent::KbdPress{key},ts),
                            route_to_widget, cache, root, ctx,
                        ); // TODO discards filters from current RootEvent
                    }
                    //l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                },
                RootEvent::KbdUp{key} => {
                    let old = (self.access)(ctx).state.key.up(key);
                    if let Some(p) = old {
                        let event = KbdUp{
                            key: p.key,
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event.clone(),ts),//.with_filter_path_strict(id.path),
                                Some(&*id),
                                cache, root.fork(), ctx,
                            );
                        }
                        //drop up if widget is gone TODO check if this is correct
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(event,ts),//.with_filter_path_strict(p.down.path),
                            Some(&*p.down),
                            cache, root, ctx,
                        );
                    }
                },
                RootEvent::KbdPress{key} => {
                    let old = (self.access)(ctx).state.key.get(key.clone());
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let event = KbdPress{
                            key: p.key.clone(),
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event,ts),//.with_filter_path_strict(id.path.clone()),
                                Some(&*id),
                                cache, root.fork(), ctx,
                            );
                            let mut do_tab = false;
                            if
                                key == MatchKeyCode::KbdTab &&
                                root.with_widget(&*id,|w,_| w.map_or(false,|w| w._tabulate_by_tab() ), ctx)
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
                        route_to_widget, cache, root.fork(), ctx,
                    ); // TODO discards filters from current RootEvent
                    //unfocus previously focused widget
                    passed |= self.unfocus(root_widget, path, stack, ts, cache, root.fork(), ctx);

                    //the currently hovered widget
                    if let Some(pos) = (self.access)(ctx).state.mouse.pos {
                        if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                            (self.access)(ctx).state.key.down(key.clone(),hovered.clone(),ts,Some(pos));

                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(MouseDown{key,pos},ts),//.with_filter_path_strict(hovered.path.clone()),
                                Some(&*hovered),
                                cache, root.fork(), ctx,
                            );


                            //passed |= Self::focus(l,hovered.path.refc(),e.1,e.2).unwrap_or(false);

                            let focus = root.with_widget(
                                &*hovered,
                                |w,_| w.map_or(false,|w| w._focus_on_mouse_down() ),
                                ctx
                            );

                            if focus {
                                passed |= self.focus(root_widget, path, hovered, stack, ts, cache, root, ctx).unwrap();
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
                                down_widget: p.down.refc(),
                                down_ts: p.ts
                            };
                            if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                                if hovered.fwd_compare(&*p.down) != FwdCompareStat::Equal { //TODO is this correct
                                    passed |= root_widget.event_direct(
                                        path,
                                        stack,
                                        &StdVariant::new(event.clone(),ts),//.with_filter_path_strict(hovered.path),
                                        Some(&*hovered),
                                        cache, root.fork(), ctx,
                                    );
                                }
                            }
                            //event dropped if widget gone
                            passed |= root_widget.event_direct(
                                path,
                                stack,
                                &StdVariant::new(event.clone(),ts),//.with_filter_path_strict(p.down.path),
                                Some(&*p.down),
                                cache, root, ctx,
                            );
                        }
                    }
                }
                RootEvent::MouseMove{pos} => {
                    //set pos
                    (self.access)(ctx).state.mouse.pos = Some(pos);
                    //previous hovered widget
                    if let Some(p) = (self.access)(ctx).state.mouse.hovered.take() {
                        //TODO only send MouseLeave and MouseEnter if hovered widget actually changes
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseLeave{},ts),//.with_filter_path_strict(p.path),
                            Some(&*p),
                            cache, root.fork(), ctx,
                        );
                    }
                    //hover state will be updated as the event passes through the widget tree
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(MouseMove{pos},ts).with_filter_point(pos), //TODO infer path filter from RootEvent //TODO keep the path filter or allow at non-root widget, to allow e.g. multi-window
                        None, cache, root.fork(), ctx,
                    ); // TODO discards filters from current RootEvent

                    if let Some(p) = (self.access)(ctx).state.mouse.hovered.clone() {//TODO optimize clone
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseEnter{},ts),//.with_filter_path_strict(p.path),
                            Some(&*p),
                            cache, root, ctx,
                        );
                    }
                    
                }
                RootEvent::MouseLeaveWindow{} => {
                    if let Some(p) = (self.access)(ctx).state.mouse.hovered.clone() {//TODO optimize clone
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseLeave{},ts),//.with_filter_path_strict(p.path),
                            Some(&*p),
                            cache, root, ctx,
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
                        route_to_widget, cache, root, ctx,
                    ); // TODO discards filters from current RootEvent
                }
                RootEvent::WindowResize{size} => {
                    passed |= self._event_root(
                        root_widget,
                        path,
                        stack,
                        &StdVariant::new(WindowResize{size},ts),
                        route_to_widget, cache, root, ctx,
                    ); // TODO discards filters from current RootEvent
                }
                RootEvent::TextInput{text} => {
                    if let Some(id) = (self.access)(ctx).state.kbd.focused.clone() {
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(TextInput{text},ts),//.with_filter_path_strict(id.path),
                            Some(&*id),
                            cache, root, ctx,
                        );
                    }
                }
                RootEvent::MouseScroll{x,y} => {
                    if let Some(hovered) = (self.access)(ctx).state.mouse.hovered.clone() {
                        passed |= root_widget.event_direct(
                            path,
                            stack,
                            &StdVariant::new(MouseScroll{x,y},ts),//.with_filter_path_strict(hovered.path),
                            Some(&*hovered),
                            cache, root, ctx,
                        );
                    }
                }
            }
            passed
        }else{
            self.sup._event_root(root_widget, path, stack, event, route_to_widget, cache, root, ctx)
        }
    }
    #[inline] 
    fn _size<W,Ph,S>(
        &self,
        w: &W,
        path: &Ph,
        stack: &S,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized
    {
        //todo!();
        self.sup._size(w, path, stack, cache, root, ctx)
    }
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
