pub trait Color {
    fn from_rgba8(c: [u8;4]) -> Self;
    fn into_rgba8(&self) -> [u8;4];
}