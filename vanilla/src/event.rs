use crate::traits::UpdateElementEvent;
use web_sys::Element;

pub struct Event<M> {
    pub name: String,
    pub func: Box<dyn Fn(&web_sys::Event) -> crate::Msg<M>>,
}

impl<M> UpdateElementEvent for Event<M>
where
    M: 'static + serde::Serialize,
{
    fn update_elem(self, el: &Element) {
        gloo_events::EventListener::new(el, self.name, move |e| {
            let msg = (*self.func)(e);
            let s = serde_json::to_string(&msg).unwrap();
            crate::send_event(s);
        })
        .forget();
    }
}

pub fn ev<M, F>(trigger: &str, handle: F) -> Event<M>
where
    F: Fn(&web_sys::Event) -> crate::Msg<M> + 'static,
{
    Event {
        name: trigger.to_string(),
        func: Box::new(handle),
    }
}
