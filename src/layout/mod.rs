use super::*;

pub mod size;
pub mod calc;
mod imp;

pub use size::*;

#[derive(Clone,Copy,PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    #[inline]
    fn flip(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

//TODO move to trait submodule
pub trait Gonstraints: From<StdGonstraints> + Into<StdGonstraints> + Clone + PartialEq {
    type Axis: GonstraintAxis;

    fn x(&self) -> Self::Axis;
    fn y(&self) -> Self::Axis;

    fn set_x(&mut self, x: &Self::Axis);
    fn set_y(&mut self, y: &Self::Axis);

    #[inline]
    fn par(&self, dir: Orientation) -> Self::Axis {
        match dir {
            Orientation::Horizontal => self.x(),
            Orientation::Vertical => self.y(),
        }
    }
    #[inline]
    fn unpar(&self, dir: Orientation) -> Self::Axis {
        self.par(dir.flip())
    }

    #[inline]
    fn set_par(&mut self, dir: Orientation, v: &Self::Axis) {
        match dir {
            Orientation::Horizontal => self.set_x(v),
            Orientation::Vertical => self.set_y(v),
        }
    }
    #[inline]
    fn set_unpar(&mut self, dir: Orientation, v: &Self::Axis) {
        self.set_par(dir.flip(), v)
    }
    
    fn empty() -> Self;
    fn fixed(w: u32, h: u32) -> Self;

    fn from_axis(x: &Self::Axis, y: &Self::Axis) -> Self;

    #[inline]
    fn from_par(dir: Orientation, par: &Self::Axis, unpar: &Self::Axis) -> Self {
        match dir {
            Orientation::Horizontal => Self::from_axis(par,unpar),
            Orientation::Vertical => Self::from_axis(unpar,par),
        }
    }

    #[inline]
    fn add(&mut self, o: &Self, dir: Orientation) {
        match dir {
            Orientation::Horizontal => self.add_x(o),
            Orientation::Vertical => self.add_y(o),
        }
    }
    fn add_x(&mut self, o: &Self);
    fn add_y(&mut self, o: &Self);

    fn and(&self, o: &Self) -> Self;
    fn max(&self, o: &Self) -> Self;

    #[inline]
    fn flip(&mut self) {
        let x = self.x();
        let y = self.y();
        self.set_x(&y);
        self.set_y(&x);
    }

    #[inline]
    fn add_space(&mut self, v: u32, dir: Orientation) {
        let mut a = self.par(dir);
        a.op_add_u32(v);
        self.set_par(dir, &a);
    }

    //TODO Constraints<E>
    fn add_border(&mut self, b: &Border);
}

pub trait GonstraintAxis: Clone {
    fn empty() -> Self;
    fn fixed(v: u32) -> Self;
    fn op_add_u32(&mut self, v: u32);
    fn op_add(&mut self, v: &Self);
    fn op_and(&self, v: &Self) -> Self;
    fn op_max(&self, o: &Self) -> Self;

    fn min(&self) -> u32;
    fn preferred(&self) -> u32;
    fn max(&self) -> Option<u32>;
    fn pressure(&self) -> f32;

    fn set_min(&mut self, v: u32);
    fn set_preferred(&mut self, v: u32);
    fn set_max(&mut self, v: Option<u32>);
    fn set_pressure(&mut self, v: f32);
}

impl Gonstraints for StdGonstraints {
    type Axis = StdGonstraintAxis;

    #[inline]
    fn x(&self) -> Self::Axis {
        self.x
    }
    #[inline]
    fn y(&self) -> Self::Axis {
        self.y
    }
    #[inline]
    fn set_x(&mut self, x: &Self::Axis) {
        self.x = *x
    }
    #[inline]
    fn set_y(&mut self, y: &Self::Axis) {
        self.y = *y
    }
    #[inline]
    fn empty() -> Self {
        Self{
            x: StdGonstraintAxis::empty(),
            y: StdGonstraintAxis::empty(),
        }
    }
    #[inline]
    fn fixed(w: u32, h: u32) -> Self {
        StdGonstraints::fixed_const(w,h)
    }
    #[inline]
    fn from_axis(x: &Self::Axis, y: &Self::Axis) -> Self {
        StdGonstraints{
            x: *x,
            y: *y,
        }
    }

    //flip std::mem::swap(&mut self.x, &mut self.y)

    #[inline]
    fn add_x(&mut self, o: &Self) {
        self.x += &o.x;
        self.y &= &o.y;
    }
    #[inline]
    fn add_y(&mut self, o: &Self) {
        self.x &= &o.x;
        self.y += &o.y;
    }
    #[inline]
    fn and(&self, o: &Self) -> Self {
        Self{
            x: self.x.op_and(&o.x),
            y: self.y.op_and(&o.y),
        }
    }
    #[inline]
    fn max(&self, o: &Self) -> Self {
        Self{
            x: self.x.op_max(&o.x),
            y: self.y.op_max(&o.y),
        }
    }
    #[inline]
    fn add_border(&mut self, b: &Border) {
        let mut senf = *self;
        senf += b
    }
}

impl GonstraintAxis for StdGonstraintAxis {
    #[inline]
    fn op_add_u32(&mut self, v: u32) {
        let mut senf = *self;
        senf += v;
    }
    #[inline]
    fn op_add(&mut self, v: &Self) {
        let mut senf = *self;
        senf += v;
    }
    #[inline]
    fn op_and(&self, v: &Self) -> Self {
        self & v
    }
    #[inline]
    fn op_max(&self, o: &Self) -> Self {
        let mut s = self.clone();

        s.min = s.min.max(o.min);
        s.preferred = s.preferred.max(o.preferred);
        s.max = s.max.with_if( &o.max, #[inline] |s,r| (*s).max(*r) );

        s.max.map(#[inline] |m| s.preferred = s.preferred.min(m) );
        s.preferred = s.preferred.max(s.min);
        
        s.pressure = s.pressure.max(o.pressure);

        s
    }
    #[inline]
    fn empty() -> Self {
        StdGonstraintAxis::empty_const()
    }
    #[inline]
    fn fixed(v: u32) -> Self {
        StdGonstraintAxis::fixed_const(v)
    }
    #[inline]
    fn min(&self) -> u32 {
        self.min
    }
    #[inline]
    fn preferred(&self) -> u32 {
        self.preferred
    }
    #[inline]
    fn max(&self) -> Option<u32> {
        self.max
    }
    #[inline]
    fn set_min(&mut self, v: u32) {
        self.min = v
    }
    #[inline]
    fn set_preferred(&mut self, v: u32) {
        self.preferred = v
    }
    #[inline]
    fn set_max(&mut self, v: Option<u32>) {
        self.max = v
    }
    #[inline]
    fn pressure(&self) -> f32 {
        self.pressure
    }
    #[inline]
    fn set_pressure(&mut self, v: f32) {
        self.pressure = v
    }
}