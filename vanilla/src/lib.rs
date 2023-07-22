#![recursion_limit = "1024"]

mod attr;
mod element;
mod js;
mod r#macro;
pub mod traits;
mod websocket;

use console_error_panic_hook::set_once as set_panic_hook;
use web_sys::window;

pub use attr::Attr;
pub use element::Element;
pub use js::console_log;
pub use traits::{ToHtml, UpdateElem, UpdateElemIterator};

pub fn start_app<T: ToHtml>(elem: T) {
    set_panic_hook();
    websocket::spawn("ws://localhost:5000").unwrap();
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");

    body.append_child(elem.to_html(&document).as_ref().as_ref())
        .unwrap();
}
