#[derive(Clone)]
pub struct Weight {
    pub x: WeightAxis,
    pub y: WeightAxis,
}

#[derive(Clone)]
pub struct WeightAxis {
    ///expansion pressure
    pub pressure: f32,
    ///0.0 = align to left
    ///0.5 = align to center
    ///1.0 = align to right
    pub gravitation: f32,
}
