use crate::core::util::border::Border;
use qwutils::*;

#[derive(Clone)]
pub struct Size {
    pub x: SizeAxis,
    pub y: SizeAxis,
}

#[derive(Clone)]
pub struct SizeAxis {
    pub min: u32,
    pub preferred: u32,
    pub max: Option<u32>,
}

impl Size {
    pub fn fixed(w: u32, h: u32) -> Self {
        Self{
            x: SizeAxis{
                min: w,
                preferred: w,
                max: Some(w),
            },
            y: SizeAxis{
                min: h,
                preferred: h,
                max: Some(h),
            }
        }
    }

    pub fn with_border(&self, b: &Border) -> Self {
        let mut c = self.clone();
        c.add_border(b);
        c
    }

    pub fn add_border(&mut self, b: &Border) {
        self.x.add(b.left+b.right);
        self.y.add(b.top+b.bottom);
    }
}

impl SizeAxis {
    //TODO may use Add/Sub impls
    pub fn add(&mut self, v: u32) {
        self.min += v;
        self.preferred += v;
        self.max.add_to(v);
        //TODO decide if we should alter the pressure
    }

    pub fn sub(&mut self, v: u32) {
        self.min -= v;
        self.preferred -= v;
        self.max.sub_to(v);
    }
}