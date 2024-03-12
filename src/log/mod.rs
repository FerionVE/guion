use std::error::Error;
use std::fmt::Arguments;

use crate::env::Env;

pub trait Logger {
    //TODO static, Context, or extra immutable type shit which then needs to be added to every single fn
    type LW: LogWriter;

    fn info<R>(f: impl FnOnce(&mut Self::LW) -> R) -> R;
    fn warn<R>(f: impl FnOnce(&mut Self::LW) -> R) -> R;
    fn debug<R>(f: impl FnOnce(&mut Self::LW) -> R) -> R;
    fn trace<R>(f: impl FnOnce(&mut Self::LW) -> R) -> R;
}

pub trait LogWriter {
    type Err: Error;

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Self::Err>;
}

fn akw<E>(id: &E::WidgetID)
where
    E: Env,
{
    E::Log::info(|w| write!(w, "{}: {:?}", "FuZ", id)).unwrap();
}
