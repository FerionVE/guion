use super::*;

pub struct EventCompound<E>(EEvent<E>,Bounds,u64,bool) where E: Env;

