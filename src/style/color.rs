pub trait Color: Clone {
    fn from_rgba8(c: [u8;4]) -> Self;
    fn to_rgba8(&self) -> [u8;4];
}

pub enum ColorVariant {
    Background(u32),
    Text(u32),
    Border(u32),
}
