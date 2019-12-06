
use crate::core::ctx::Context;
use crate::standard::ctx::StandardCtx;
use crate::core::widget::Widget;
///tabulate through widget tree
pub fn tabulate<E: Context>(c: &mut E, selected: E::WidgetID, reverse: bool) -> E::WidgetID {
    //for recognizing infinite loops
    let initial_selected = selected.clone();
    let mut current = selected;
    //set if we need another pass
    let mut repeat = true;
    //set if should skip traversing childs in next pass
    let mut traverse_parents = false;

    while repeat {
        repeat = false;

        let w = c.widget(&current).expect("Lost Widget");

        if !traverse_parents {
            traverse_parents = true;
            if w.has_childs() {
                if let Some(c) = w.childs().next() {
                    current = c;
                    //traverse into child, skip parent traverse
                    traverse_parents = false;
                }
            }
        }
        if traverse_parents {
            traverse_parents = false;
            if let Some(p) = w.parent() {
                let pc = c.widget(&current).expect("Lost Widget").childs_vec();
                //find current child in parent
                let idx = pc.iter().position(|c| *c == current ).expect("Parent Lost Child Widget");

                if !reverse && pc.len()-idx-1 != 0 {
                    //traverse into next silbing
                    current = pc[idx+1].clone();
                } else if reverse && idx != 0 {
                    //traverse into next silbing
                    current = pc[idx-1].clone();
                }else{
                    //parent traverse end was reached, traverse grandpa
                    current = p.clone();
                    traverse_parents = true;
                    repeat = true;
                }
            }else {
                //root widget cycle complete, start over in next pass
            }
        }

        if !c.widget(&current).expect("Lost Widget").selectable() {
            repeat = true;
        }

        if current == initial_selected {
            return current;
        }
    }

    current
}