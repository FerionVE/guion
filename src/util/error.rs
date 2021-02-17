use std::error::Error;
use std::fmt::Display;

use super::*;

#[non_exhaustive]
#[derive(Debug)]
pub enum GuionError<E> where E: Env {
    TraitcastError{
        op: &'static str,
        src_type: Vec<&'static str>,
        dest_trait_type: &'static str,
    },
    ResolveError{
        op: &'static str,
        sub_path: E::WidgetPath,
        widget_type: Vec<&'static str>,
        child_info: Vec<GuionResolveErrorChildInfo<E>>,
    },
}

#[derive(Debug)]
pub struct GuionResolveErrorChildInfo<E> where E: Env {
    pub child_idx: usize,
    pub widget_type: Vec<&'static str>,
    pub widget_path_if_path: Option<E::WidgetPath>,
    pub widget_id: Option<E::WidgetID>,
}

impl<E> Error for GuionError<E> where E: Env {
    
}

impl<E> Display for GuionError<E> where E: Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TraitcastError { op, src_type, dest_trait_type } => {
                write!(f,"Failed to {}\n",op)?;
                for v in src_type {
                    write!(f,"\tsrc_type={}\n",v)?;
                }
                write!(f,"\tdest_type={}\n",dest_trait_type)?;
            }
            Self::ResolveError { op, sub_path, widget_type, child_info } => {
                write!(f,"Failed to {} child in Widget\n",op)?;
                write!(f,"\tsub_path={:?}\n",sub_path)?;
                for v in widget_type {
                    write!(f,"\ttype={}\n",v)?;
                }
                for c in child_info {
                    if let Some(v) = &c.widget_id {
                        write!(f,"\n\tChild #{} id={:?}\n",c.child_idx,v)?;
                    }
                    if let Some(v) = &c.widget_path_if_path {
                        write!(f,"\n\tChild #{} path={:?}\n",c.child_idx,v)?;
                    }
                    for v in &c.widget_type {
                        write!(f,"\tChild #{} type={}\n",c.child_idx,v)?;
                    }
                }
            }
        }
        Ok(())
    }
}
