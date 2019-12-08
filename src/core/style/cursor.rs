pub trait Cursor {
    fn default() -> Self;
    fn arrow() -> Self;
    fn ibeam() -> Self;
    fn wait() -> Self;
    fn crosshair() -> Self;
    fn wait_arrow() -> Self;
    fn size_nwse() -> Self;
    fn size_nesw() -> Self;
    fn size_we() -> Self;
    fn size_ns() -> Self;
    fn size_all() -> Self;
    fn no() -> Self;
    fn hand() -> Self;
}
