//use super::*;
use qwutils::*;

#[derive(Clone)]
pub struct LazoutDir {
    pub min: u32,
    pub max: Option<u32>,
    pub optimal: Option<u32>,
    pub pressure: f32,
}

impl LazoutDir {
    //TODO may use Add/Sub impls
    pub fn add(&mut self, v: u32) {
        self.min += v;
        self.max.add_to(v);
        self.optimal.add_to(v);
        //TODO decide if we should alter the pressure
    }

    pub fn sub(&mut self, v: u32) {
        self.min -= v;
        self.max.sub_to(v);
        self.optimal.sub_to(v);
    }
}