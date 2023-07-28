#![recursion_limit = "1024"]

pub use web_sys::{Element, Node};

mod attr;
mod event;
mod js;
mod r#macro;
mod traits;
mod websocket;

use console_error_panic_hook::set_once as set_panic_hook;
pub use r#macro::document;
use web_sys::window;

pub use attr::Attr;
pub use js::console_log;
pub use traits::Component;
pub use traits::{UpdateElem, UpdateElemIterator, UpdateElementEvent};

pub use gloo_timers::future::TimeoutFuture as Timeout;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use uuid::Uuid;
pub use wasm_bindgen_futures::spawn_local;

pub use event::ev;
pub use wasm_bindgen::JsCast;
pub use web_sys::*;

#[derive(Serialize, Deserialize)]
pub enum Msg<M> {
    System,
    User(M),
}

pub struct Context {}
impl Context {
    pub fn url(&self) -> String {
        window().unwrap().location().href().unwrap()
    }
}

static mut SENDER: Option<async_channel::Sender<String>> = None;
static mut RECV: Option<async_channel::Receiver<String>> = None;

unsafe fn init_channel() {
    if SENDER.is_none() {
        let (send, r) = async_channel::unbounded::<String>();
        SENDER = Some(send);
        RECV = Some(r);
    }
}
fn send_event(evt: String) {
    unsafe {
        init_channel();
        SENDER.as_ref().unwrap().send_blocking(evt).unwrap();
    }
}

pub fn start_app<C, F, M>(elem: C, f: F)
where
    C: Component,
    F: Fn(Msg<M>) + 'static,
    M: Serialize + DeserializeOwned,
{
    set_panic_hook();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");

    spawn_local(async move {
        unsafe {
            init_channel();
            while let Ok(evt) = RECV.as_ref().unwrap().recv().await {
                let m: Msg<M> = serde_json::from_str(&evt).unwrap();
                f(m);
            }
        }
    });

    body.append_child(elem.view().as_ref()).unwrap();
}
