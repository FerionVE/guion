pub struct PaneState<E> where E: Context {
    layouts: Vec<E::WidgetID>,
    invalid: bool,
}