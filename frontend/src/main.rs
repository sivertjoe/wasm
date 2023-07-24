use std::cell::RefCell;
use std::rc::Rc;
use vanilla::*;

struct Page {
    stuff: Vec<Element>,
    id: vanilla::Uuid,
}

impl Component for Page {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn view(&self) -> Element {
        console_log("I got called?!");
        div![
            attrs! {
                "id" => self.id()
            },
            styles! {
                "background" => "red"
            },
            div![
                styles! {
                    "background" => "blue"
                },
                h1!["div1"]
            ],
            div![
                styles! {
                    "background" => "green"
                },
                h1!["div1"]
            ],
            h2!["Hello, World"],
            self.stuff.clone()
        ]
    }
}

fn main() {
    let page = Page {
        id: vanilla::Uuid::new_v4(),
        stuff: vec![h2!["test"], h1!("Sofie")],
    };

    let page = Rc::new(RefCell::new(page));

    let cpy = Rc::clone(&page);
    spawn_local(async move {
        Timeout::new(3_000).await;
        cpy.borrow_mut().stuff.push(h2!("new!!"));
        console_log("update!");
        cpy.update();
    });
    start_app(page)
}
