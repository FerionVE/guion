use crate::*;
use widget::Widget;

#[deprecated]
#[allow(deprecated)]
pub fn tabulate<E: Env>(s: &E::RootRef<'_>, selected: E::WidgetPath, reverse: bool) -> E::WidgetPath {
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

    fn walk_forward<E: Env>(current: &mut E::WidgetPath, s: &E::RootRef<'_>) {
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
                if let Ok(w) = s.widget(parent.refc()) {
                    let pc = w.child_paths();

                    let idx = pc.iter().position(|c| c.dest_eq(current) ).expect("Parent Lost Child Widget");

                    if idx < pc.len()-1 {
                        *current = pc[idx+1].clone();
                    }else{
                        *current = parent;
                        continue;
                    }
                }
            }
            break;
        }
    }
    fn walk_reverse<E: Env>(current: &mut E::WidgetPath, s: &E::RootRef<'_>) {
        if let Some(parent) = current.parent() {
            if let Ok(w) = s.widget(parent.refc()) {
                let pc = w.child_paths();

                let idx = pc.iter().position(|c| c.dest_eq(current) ).expect("Parent Lost Child Widget");

                if idx > 0 {
                    *current = pc[idx-1].clone();
                }else{
                    *current = parent;
                    return;
                }
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
