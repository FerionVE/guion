use std::any::TypeId;

use crate::env::Env;
use crate::newpath::{PathResolvusDyn, PathResolvus};

#[derive(Clone)]
pub struct UpdateRoute<'a,E> where E: Env {
    scope: Option<UpdateScope<'a,E>>,
}

#[derive(Clone)]
struct UpdateScope<'a,E> where E: Env {
    /// Some(v) if v.inner().is_some(): target still not resolve
    /// Some(v) if v.inner().is_none(): target just resolved
    /// None: resolving is done, we are aleady in sub or child of resolve target
    resolve: Option<&'a (dyn PathResolvusDyn<E>+'a)>,
    zone: Option<UpdateZone>,
}

#[derive(Clone)]
struct UpdateZone {
    target_zone: TypeId,
    current_zone: TypeId,
    zone_activated: bool,
}

impl<'a,E> UpdateRoute<'a,E> where E: Env {
    pub fn none() -> Self {
        Self { scope: None }
    }

    pub fn resolving(&self) -> Option<&'a (dyn PathResolvusDyn<E>+'a)> {
        self.resolvus().filter(|resolvus| resolvus.inner().is_some() )
    }

    pub fn resolvus(&self) -> Option<&'a (dyn PathResolvusDyn<E>+'a)> {
        self.scope.as_ref().and_then(|scope| scope.resolve )
    }

    pub fn for_child_1(&self) -> Self {
        self.for_child_f(|resolvus| resolvus.inner().unwrap() )
    }

    pub fn for_child_f(&self, fun: impl FnOnce(&'a (dyn PathResolvusDyn<E>+'a)) -> &'a (dyn PathResolvusDyn<E>+'a)) -> Self {
        let mut this = self.clone();

        if let Some(scope) = &mut this.scope {
            if let Some(resolvus) = &mut scope.resolve {
                if let Some(_) = resolvus.inner() {
                    // still resolved, the child may or may not be the target
                    *resolvus = fun(&**resolvus);
                    if let Some(zone) = &mut scope.zone {
                        if resolvus.inner().is_none() && zone.current_zone == zone.target_zone {
                            zone.zone_activated = true;
                        }
                    }
                } else {
                    // we just were resolve target, for child of resolve target
                    scope.resolve = None;
                }
            } else {
                // we are aleady inside child of resolve target 
            }
        } else {
            // no scope or resolving
        }

        this
    }

    pub fn through_zone<Z>(mut self) -> Option<Self> where Z: 'static {
        let new_zone = TypeId::of::<Z>();

        if let Some(scope) = &mut self.scope {
            if let Some(mut zone) = scope.zone.as_mut() {
                if zone.current_zone == new_zone {
                    return Some(self);
                }
                
                if let Some(resolvus) = &mut scope.resolve {
                    // we either still have to resolve or are just on resolve target
                    if let Some(_) = resolvus.inner() {
                        // still not resolved
                        zone.current_zone = new_zone;
                    } else {
                        // we just are in resolve target
                        if zone.target_zone == new_zone {
                            zone.zone_activated = true;
                        } else if zone.zone_activated {
                            // we ending the activated zone before even going into any child
                            //return None;
                            scope.zone = None;
                            return Some(self);
                        }
                    }
                } else {
                    // we are aleady inside child of resolve target
                    if zone.current_zone != new_zone {
                        if zone.zone_activated && zone.target_zone != new_zone {
                            // we are leaving active zone
                            return None;
                        } else if !zone.zone_activated {
                            // right zone wasn't activated as the zone wasn't reached when reaching resolve target, so zoning is dead
                            scope.zone = None;
                            return Some(self);
                        }
                    }
                }
    
                zone.current_zone = new_zone;
            }
        } else {
            // zones don't matter if no scope
        }

        Some(self)
    }
}
