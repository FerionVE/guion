pub struct PaneState<E> where E: Env {
    layouts: Vec<E::WidgetID>,
    invalid: bool,
}