use crate::*;
use ctx::*;
use widget::Widget;

pub fn tabulate<E: Env>(s: &E::Storage, selected: E::WidgetPath, reverse: bool) -> E::WidgetPath {
    let initial_selected = selected.refc();
    let mut current = selected;

    loop {
        if reverse {
            walk_reverse::<E>(&mut current,s);
        }else{
            walk_forward::<E>(&mut current,s);
        }

        if current.exact_eq(&initial_selected) {
            eprintln!("Tab Oof");
            return current;
        }

        if s.widget(current.refc()).expect("Lost Widget").focusable() {
            return current;
        }
    }

    fn walk_forward<E: Env>(current: &mut E::WidgetPath, s: &E::Storage) {
        {
            let w = s.widget(current.refc()).expect("Lost Widget");
            let pc = w.child_paths();
            if !pc.is_empty() {
                *current = pc[0].refc();
                return;
            }
        }
        loop {
            if let Some(parent) = current.parent() {
                let w = s.widget(parent.refc()).expect("Lost Widget");
                let pc = w.child_paths();

                let idx = pc.iter().position(|c| c.tip() == current.tip() ).expect("Parent Lost Child Widget");

                if idx < pc.len()-1 {
                    *current = pc[idx+1].clone();
                }else{
                    *current = parent;
                    continue;
                }
            }
            break;
        }
    }
    fn walk_reverse<E: Env>(current: &mut E::WidgetPath, s: &E::Storage) {
        if let Some(parent) = current.parent() {
            let w = s.widget(parent.refc()).expect("Lost Widget");
            let pc = w.child_paths();

            let idx = pc.iter().position(|c| c.tip() == current.tip() ).expect("Parent Lost Child Widget");

            if idx > 0 {
                *current = pc[idx-1].clone();
            }else{
                *current = parent;
                return;
            }
        }
        loop {
            let w = s.widget(current.refc()).expect("Lost Widget");
            let pc = w.child_paths();
            if pc.is_empty() {break;}
            *current = pc[pc.len()-1].refc();
        }
    }
}

/// tabulate through widget tree
/// returns the next widget from selected in the specific direction
pub fn tabulate_old<E: Env>(s: &E::Storage, selected: E::WidgetPath, reverse: bool) -> E::WidgetPath {
    //for recognizing infinite loops
    let initial_selected = selected.refc();
    let mut current = selected;
    //set if we need another pass
    let mut repeat = true;
    //set if should skip traversing childs in next pass
    let mut traverse_parents = false;

    let mut w = s.widget(current.refc()).expect("Lost Widget");

    while repeat {
        repeat = false;

        if !traverse_parents {
            traverse_parents = true;
            if w.childs() != 0 {
                if let Some(c) = w.child_paths().into_iter().next() {
                    current = c;
                    w = s.widget(current.refc()).expect("Lost Widget");
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

                if !reverse && idx < pc.len()-1 {
                    //traverse into next silbing
                    current = pc[idx+1].clone();
                    w = s.widget(current.refc()).expect("Lost Widget");
                } else if reverse && idx > 0 {
                    //traverse into next silbing
                    current = pc[idx-1].clone();
                    w = s.widget(current.refc()).expect("Lost Widget");
                }else{
                    //parent traverse end was reached, traverse grandpa
                    current = p.refc();
                    w = s.widget(current.refc()).expect("Lost Widget");
                    traverse_parents = true;
                    repeat = true;
                }
            }else {
                //root widget cycle complete, start over in next pass
            }
        }

        if !w.focusable() {
            repeat = true;
        }

        if current.exact_eq(&initial_selected) {
            return current;
        }
    }

    current
}
