use crate::attr::Attr;
use web_sys::window;
use web_sys::Element;

pub trait UpdateElem {
    fn update_elem(self, elem: &Element);
}

pub trait UpdateElemIterator {
    fn update_elem(self, el: &Element);
}

pub trait Component {
    fn id(&self) -> String;

    fn update(&self) {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access document");

        let id = self.id();

        let new = self.view();
        let elem = document
            .get_element_by_id(&id)
            .expect("could not get element by id");
        elem.replace_with_with_node_1(&new)
            .expect("could not replace");
    }

    fn view(&self) -> web_sys::Element;
}

impl Component for Element {
    fn id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
    fn view(&self) -> web_sys::Element {
        self.clone()
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl<C: Component> Component for Rc<RefCell<C>> {
    fn id(&self) -> String {
        self.borrow().id()
    }
    fn view(&self) -> web_sys::Element {
        self.borrow().view()
    }
}

impl<T: UpdateElem, I: IntoIterator<Item = T>> UpdateElemIterator for I {
    fn update_elem(self, elem: &Element) {
        for el in self {
            el.update_elem(elem);
        }
    }
}

impl UpdateElem for Attr {
    fn update_elem(self, elem: &Element) {
        elem.set_attribute(&self.name, &self.value).unwrap();
    }
}

impl UpdateElem for Element {
    fn update_elem(self, elem: &Element) {
        elem.append_with_node_1(&self).unwrap();
    }
}

impl UpdateElem for &str {
    fn update_elem(self, elem: &Element) {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access document");
        let node = document.create_text_node(self);

        elem.append_with_node_1(node.as_ref()).unwrap();
    }
}
