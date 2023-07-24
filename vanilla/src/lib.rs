#![recursion_limit = "1024"]

pub use web_sys::{Element, Node};

mod attr;
mod js;
mod r#macro;
pub mod traits;
mod websocket;

use console_error_panic_hook::set_once as set_panic_hook;
pub use r#macro::document;
use web_sys::window;

pub use attr::Attr;
pub use js::console_log;
pub use traits::Component;
pub use traits::{UpdateElem, UpdateElemIterator};

pub fn start_app<T: Component>(elem: T) {
    set_panic_hook();
    // websocket::spawn("ws://localhost:5000").unwrap();
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");

    body.append_child(elem.view().as_ref().as_ref()).unwrap();
}
