use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use vanilla::*;

#[derive(Default)]
struct Page {
    input: String,
    items: Vec<Element>,
}

impl Component for Page {
    fn id(&self) -> String {
        "1".to_string()
    }
    fn view(&self) -> vanilla::Element {
        div![
            attrs! {
                "id" => self.id()
            },
            h1!("Add todo item"),
            input![ev("input", |e| {
                let mm: &vanilla::InputEvent = e.dyn_ref().unwrap();
                vanilla::Msg::User(Msg::Input(mm.data().unwrap()))
            })],
            button![
                ev("mousedown", |_e| { vanilla::Msg::User(Msg::Submit) }),
                "Add"
            ],
            h2!["TODOS:"],
            self.items.clone()
        ]
    }
}

#[derive(Serialize, Deserialize)]
enum Msg {
    Input(String),
    Submit,
}

fn main() {
    let page = Rc::new(RefCell::new(Page::default()));

    let cpy = Rc::clone(&page);
    let update = move |msg| match msg {
        vanilla::Msg::User(msg) => match msg {
            Msg::Input(ch) => {
                cpy.borrow_mut().input.push_str(&ch);
            }
            Msg::Submit => {
                let txt = cpy.borrow().input.clone();
                cpy.borrow_mut().items.push(h2!(txt));
                cpy.borrow_mut().input.clear();
                cpy.update(); // Updates the component
            }
        },
        _ => {}
    };

    start_app(page, update);
}
