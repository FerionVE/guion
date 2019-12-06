use crate::core::env::Env;

pub trait Question {
    fn default_answer(&self) -> usize;
}

pub enum Questions<Q> where Q: Question {
    CanDrag(Q),
    CanDrop(Q),
    Other(Q),
}
