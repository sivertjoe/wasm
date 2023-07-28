use vanilla::*;

fn main() {
    let page = div![
        styles! {
            "background" => "blue",
        },
        h1!("Hello, World!"),
        h2! {
            styles! {
                "color" => "red"
            },
        "Wow, what an amazing page!"
        }
    ];
    start_app(page, |_e: Msg<()>| {});
}
