use crate::*;
use crate::root::RootRef;
use super::*;

impl<SB,E> Handler<E> for StdHandlerLive<SB,E> where
    SB: HandlerBuilder<E>,
    E: Env,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    EEvent<E>: StdVarSup<E>
{
    fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's {
        &self.sup
    }

    #[inline] 
    fn _render(
        &self,
        l: Link<E>,
        r: &mut ERenderer<'_,E>,
        tail: &mut dyn FnMut(Link<E>,&mut ERenderer<'_,E>),
    ) {
        self.sup._render(l,r,tail)
        //todo!()
    }
    #[inline] 
    fn _event_direct(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp {
        if let Some(_) = e.event.is::<MouseMove>() {
            (self.access)(l.ctx).s.mouse.hovered = Some(l.ident());
        }
        self.sup._event_direct(l,e,tail)
        //todo!()
    }
    #[inline] 
    fn _send_event(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        child: E::WidgetPath,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>,E::WidgetPath)->Result<EventResp,E::Error>
    ) -> Result<EventResp,E::Error> {
        /*if let Some(_) = e.0.is::<MouseMove>() {
            (self.f)(l.ctx).s.mouse.hovered = Some(l.ident());
        }*/
        self.sup._send_event(l,e,child,tail)
    }
    //#[inline] 
    fn _event_root(
        &self,
        mut l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp { //TODO BUG handle sudden invalidation of hovered widget
        assert!(l.path().is_empty());
        if let Some(ee) = e.event.is::<RootEvent<E>>() {
            let mut passed = false;
            match ee {
                RootEvent::KbdDown{key} => {
                    //Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = (self.access)(l.ctx).s.kbd.focused.clone() {
                        if !l.widget.root.has_widget(id.refc().path,l.ctx) {
                            //drop event if widget is gone
                            return false;
                        }
                        (self.access)(l.ctx).s.key.down(
                            key.clone(),
                            id.clone(),
                            e.ts,
                            None,
                        );
                        //emit KbdDown event
                        let event = KbdDown{
                            key: key.clone(),
                        };
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(event)),
                            id.refc().path
                        ).unwrap();
                        /*let event = KbdPress{
                            key: key.clone(),
                            down_widget: id.refc(),
                            down_ts: e.2,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));*/
                        passed |= l._event_root(&e.with_event(Event::from(RootEvent::KbdPress{key})));
                    }
                    //l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                },
                RootEvent::KbdUp{key} => {
                    let old = (self.access)(l.ctx).s.key.up(key);
                    if let Some(p) = old {
                        let event = KbdUp{
                            key: p.key,
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(l.ctx).s.kbd.focused.clone() {
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event.clone())),
                                id.path,
                            ).unwrap_or(false);
                        }
                        //drop up if widget is gone TODO check if this is correct
                        passed |=  l.send_event(
                            &e.default_filter().with_event(Event::from(event.clone())),
                            p.down.refc().path,
                        ).unwrap_or(false);
                    }
                },
                RootEvent::KbdPress{key} => {
                    let old = (self.access)(l.ctx).s.key.get(key.clone());
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let event = KbdPress{
                            key: p.key.clone(),
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (self.access)(l.ctx).s.kbd.focused.clone() {
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event.clone())),
                                id.path.refc(),
                            ).unwrap_or(false);
                            let mut do_tab = false;
                            if let Ok(l) = l.with_widget(id.path.refc()) {
                                if key == MatchKeyCode::KbdTab && l.widget._tabulate_by_tab() {
                                    do_tab = true;
                                }
                            }
                            if do_tab {
                                let reverse = l.state().is_pressed(MatchKeyCode::KbdShift).is_some();
                                let dir = if reverse {TabulateDirection::Backward} else {TabulateDirection::Forward};
                                let path = tabi(l.reference(),id.path,dir).expect("TODO");
                                //better way than this hack to get the ident
                                (self.access)(l.ctx).s.kbd.focused = Some(WidgetIdent::from_path(path,&l.widget.root,l.ctx).expect("TODO"));
                            }
                        }
                    }
                }
                RootEvent::MouseDown{key} => {
                    passed |= self._event_root(l.reference(),&e.with_event(Event::from(RootEvent::MouseUp{key: key.clone()})),&mut |_,_|todo!());
                    //unfocus previously focused widget
                    passed |= self.unfocus(l.reference(),e.bounds,e.ts);

                    //the currently hovered widget
                    if let Some(pos) = (self.access)(l.ctx).s.mouse.pos {
                        if let Some(hovered) = (self.access)(l.ctx).s.mouse.hovered.clone() {
                            (self.access)(l.ctx).s.key.down(key.clone(),hovered.clone(),e.ts,Some(pos));

                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(MouseDown{key,pos})),
                                hovered.path.refc(),
                            ).unwrap_or(false);


                            //passed |= Self::focus(l,hovered.path.refc(),e.1,e.2).unwrap_or(false);

                            let focus = if let Ok(w) = l.with_widget(hovered.path.refc()) {
                                w.widget._focus_on_mouse_down()
                            }else{
                                false
                            };

                            if focus {
                                passed |= self.focus(l,hovered.path,e.bounds,e.ts).unwrap();
                            }
                        }
                    }
                }
                RootEvent::MouseUp{key} => {
                    let old = (self.access)(l.ctx).s.key.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        if let Some(pos) = (self.access)(l.ctx).s.mouse.pos {
                            let event = MouseUp{
                                key: p.key,
                                pos,
                                down_pos: p.cursor.expect("TODO"), //fails if a invalid press was inserted into the state
                                down_widget: p.down.refc(),
                                down_ts: p.ts
                            };
                            if let Some(hovered) = (self.access)(l.ctx).s.mouse.hovered.clone() {
                                if hovered != p.down {
                                    passed |= l.send_event(
                                        &e.default_filter().with_event(Event::from(event.clone())),
                                        hovered.path,
                                    ).unwrap_or(false);
                                }
                            }
                            //event dropped if widget gone
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event)),
                                p.down.refc().path,
                            ).unwrap_or(false);
                        }
                    }
                }
                RootEvent::MouseMove{pos} => {
                    //set pos
                    (self.access)(l.ctx).s.mouse.pos = Some(pos);
                    //previous hovered widget
                    if let Some(p) = (self.access)(l.ctx).s.mouse.hovered.take() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseLeave{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    //hover state will be updated as the event passes through the widget tree
                    passed |= l._event_root(&e.with_event(Event::from(MouseMove{pos})));
                    if let Some(p) = (self.access)(l.ctx).s.mouse.hovered.clone() {//TODO optimize clone
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseEnter{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    
                }
                RootEvent::MouseLeaveWindow{} => {
                    if let Some(p) = (self.access)(l.ctx).s.mouse.hovered.clone() {//TODO optimize clone
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseLeave{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    let mouse = &mut (self.access)(l.ctx).s.mouse;
                    mouse.pos = None;
                    mouse.hovered = None;
                }
                RootEvent::WindowMove{pos,size} => {
                    passed |= l._event_root(&e.with_event(Event::from(WindowMove{pos,size})))
                }
                RootEvent::WindowResize{size} => {
                    passed |= l._event_root(&e.with_event(Event::from(WindowResize{size})))
                }
                RootEvent::TextInput{text} => {
                    if let Some(id) = (self.access)(l.ctx).s.kbd.focused.clone() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(TextInput{text})),
                            id.path,
                        ).unwrap_or(false);
                    }
                }
                RootEvent::MouseScroll{x,y} => {
                    if let Some(hovered) = (self.access)(l.ctx).s.mouse.hovered.clone() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseScroll{x,y})),
                            hovered.path,
                        ).unwrap_or(false);
                    }
                }
            }
            passed
        }else{
            self.sup._event_root(l,e,tail)
        }
    }
    #[inline] 
    fn _size(
        &self,
        l: Link<E>,
        e: &EStyle<E>,
        tail: &mut dyn FnMut(Link<E>,&EStyle<E>)->ESize<E>,
    ) -> ESize<E> {
        //todo!();
        self.sup._size(l,e,tail)
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
