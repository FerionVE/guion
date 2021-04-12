use std::error::Error;
use std::fmt::{Debug,Display};

use super::*;

#[derive(Clone)]
#[non_exhaustive]
pub enum GuionError<E> where E: Env {
    TraitcastError(Box<TraitcastError>),
    ResolveError(Box<ResolveError<E>>),
    Empty,
}

#[derive(Clone)]
pub struct ResolveError<E> where E: Env {
    pub op: &'static str,
    pub sub_path: E::WidgetPath,
    pub widget_type: Vec<&'static str>,
    pub child_info: Vec<GuionResolveErrorChildInfo<E>>,
}
#[derive(Clone)]
pub struct TraitcastError {
    pub op: &'static str,
    pub src_type: Vec<&'static str>,
    pub dest_trait_type: &'static str,
}

#[derive(Clone)]
pub struct GuionResolveErrorChildInfo<E> where E: Env {
    pub child_idx: usize,
    pub widget_type: Vec<&'static str>,
    pub widget_path_if_path: Option<E::WidgetPath>,
    pub widget_id: Option<E::WidgetID>,
}

impl<E> Error for GuionError<E> where E: Env {
    
}

impl<E> From<()> for GuionError<E> where E: Env {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

impl<E> Display for GuionError<E> where E: Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TraitcastError(e) => {
                write!(f,"\n\nFailed to {}\n\n",e.op)?;
                for v in &e.src_type {
                    write!(f,"\tsrc  = {}\n",v)?;
                }
                write!(f,"\n\tdest = {}\n\n",e.dest_trait_type)?;
            }
            Self::ResolveError(e) => {
                write!(f,"\n\nFailed to {} child in Widget\n\n",e.op)?;
                write!(f,"\tsub_path = {:?}\n",e.sub_path)?;
                for v in &e.widget_type {
                    write!(f,"\ttype     = {}\n",v)?;
                }
                write!(f,"\n")?;
                for c in &e.child_info {
                    if let Some(v) = &c.widget_id {
                        write!(f,"\tChild #{} id   = {:?}\n",c.child_idx,v)?;
                    }
                    if let Some(v) = &c.widget_path_if_path {
                        write!(f,"\tChild #{} path = {:?}\n",c.child_idx,v)?;
                    }
                    for v in &c.widget_type {
                        write!(f,"\tChild #{} type = {}\n",c.child_idx,v)?;
                    }
                    write!(f,"\n")?;
                }
                
            }
            Self::Empty => {}, //TODO
        }
        Ok(())
    }
}

impl<E> Debug for GuionError<E> where E: Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Display::fmt(self,f)
    }
}
