use vanilla::*;

fn main() {
    start_app(div![
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
        h2!["Hello, World"]
    ])
}
