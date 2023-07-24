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
    fn id() -> String {
        String::new()
    }
    fn view(self) -> Box<dyn AsRef<web_sys::Node>>;
}

impl Component for Element {
    fn view(self) -> Box<dyn AsRef<web_sys::Node>> {
        Box::new(self)
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
