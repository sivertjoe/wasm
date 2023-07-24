use vanilla::*;

struct Page {
    stuff: Vec<Element>,
}

impl Component for Page {
    fn view(self) -> Box<dyn AsRef<Node>> {
        Box::new(div![
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
            self.stuff
        ])
    }
}

fn main() {
    let page = Page {
        stuff: vec![h2!["test"]],
    };
    start_app(page)
}
