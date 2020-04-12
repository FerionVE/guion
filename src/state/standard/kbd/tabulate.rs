use crate::*;
use ctx::*;
use widget::Widget;
/// tabulate through widget tree
/// returns the next widget from selected in the specific direction
pub fn tabulate<E: Env>(s: &E::Storage, selected: E::WidgetPath, reverse: bool) -> E::WidgetPath {
    //for recognizing infinite loops
    let initial_selected = selected.clone();
    let mut current = selected;
    //set if we need another pass
    let mut repeat = true;
    //set if should skip traversing childs in next pass
    let mut traverse_parents = false;

    while repeat {
        repeat = false;

        let w = s.widget(current.refc()).expect("Lost Widget");

        if !traverse_parents {
            traverse_parents = true;
            if w.widget().childs() != 0 {
                if let Some(c) = w.child_paths().into_iter().next() {
                    current = c;
                    //traverse into child, skip parent traverse
                    traverse_parents = false;
                }
            }
        }
        if traverse_parents {
            traverse_parents = false;
            if let Some(p) = current.parent() {
                let pc = w.child_paths();
                //find current child in parent
                let idx = pc.iter().position(|c| c.tip() == current.tip() ).expect("Parent Lost Child Widget");

                if !reverse && pc.len()-idx-1 != 0 {
                    //traverse into next silbing
                    current = pc[idx+1].clone();
                } else if reverse && idx != 0 {
                    //traverse into next silbing
                    current = pc[idx-1].clone();
                }else{
                    //parent traverse end was reached, traverse grandpa
                    current = p.refc();
                    traverse_parents = true;
                    repeat = true;
                }
            }else {
                //root widget cycle complete, start over in next pass
            }
        }

        if !s.widget(current.refc()).expect("Lost Widget").widget().focusable() {
            repeat = true;
        }

        if current == initial_selected {
            return current;
        }
    }

    current
}