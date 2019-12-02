//use super::*;

#[derive(Clone)]
pub struct LazoutDir {
    pub min: u32,
    pub max: u32,
    pub optimal: u32,
    pub pressure: f32,
}

impl LazoutDir {
    //TODO may use Add/Sub impls
    pub fn add(&mut self, v: u32) {
        self.min += v;
        self.max += v;
        self.optimal += v;
        //TODO decide if we should alter the pressure
    }

    pub fn sub(&mut self, v: u32) {
        self.min -= v;
        self.max -= v;
        self.optimal -= v;
    }
}