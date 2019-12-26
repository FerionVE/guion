use super::*;

/// Size +/+= Border
#[derive(Clone)]
pub struct Size {
    pub x: SizeAxis,
    pub y: SizeAxis,
}

/// SizeAxis +/+= SizeAxis
/// SizeAxis &/&= SizeAxis
#[derive(Clone)]
pub struct SizeAxis {
    pub min: u32,
    pub preferred: u32,
    pub max: Option<u32>,
}

impl Size {
    pub const fn fixed(w: u32, h: u32) -> Self {
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

    pub fn add(&mut self, o: &Self, dir: Orientation) {
        match dir {
            Orientation::Horizontal() => self.add_x(o),
            Orientation::Vertical() => self.add_y(o),
        }
    }

    pub fn add_x(&mut self, o: &Self) {
        self.x += &o.x;
        self.y &= &o.y;
    }

    pub fn add_y(&mut self, o: &Self) {
        self.x &= &o.x;
        self.y += &o.y;
    }

    pub const fn empty() -> Self {
        Size {
            x: SizeAxis::empty(),
            y: SizeAxis::empty(),
        }
    }
}

impl SizeAxis {
    pub const fn empty() -> Self {
        SizeAxis {
            min: 0,
            preferred: 0,
            max: None,
        }
    }
}