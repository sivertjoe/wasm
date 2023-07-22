use crate::attr::Attr;
use crate::element::Element;
use web_sys::Document;
pub trait UpdateElem {
    fn update_elem(self, elem: &mut Element)
    where
        Self: Sized;
}

pub trait UpdateElemIterator {
    fn update_elem(self, el: &mut Element);
}

pub trait ToHtml {
    fn to_html(&self, _doc: &Document) -> Box<dyn AsRef<web_sys::Node>>;
}

impl<T: UpdateElem, I: IntoIterator<Item = T>> UpdateElemIterator for I {
    fn update_elem(self, elem: &mut Element) {
        for el in self {
            el.update_elem(elem);
        }
    }
}

impl UpdateElem for Attr {
    fn update_elem(self, elem: &mut Element)
    where
        Self: Sized,
    {
        elem.attrs.push(self);
    }
}

impl UpdateElem for Element {
    fn update_elem(self, elem: &mut Element)
    where
        Self: Sized,
    {
        elem.children.push(Box::new(self));
    }
}

impl<S: AsRef<str>> UpdateElem for S {
    fn update_elem(self, elem: &mut Element) {
        elem.children.push(Box::new(self.as_ref().to_owned()));
    }
}

impl<T> ToHtml for T
where
    T: ToString,
{
    fn to_html(&self, doc: &Document) -> Box<dyn AsRef<web_sys::Node>> {
        Box::new(doc.create_text_node(&self.to_string()))
    }
}

impl ToHtml for Element {
    fn to_html(&self, doc: &Document) -> Box<dyn AsRef<web_sys::Node>>
    where
        Self: Sized,
    {
        let elem = doc.create_element(&self.name).unwrap();
        for attr in self.attrs.iter() {
            elem.set_attribute(&attr.name, &attr.value).unwrap();
        }

        for el in self.children.iter() {
            let e = (*el).to_html(&doc);
            elem.append_child(e.as_ref().as_ref()).unwrap();
        }
        Box::new(elem)
    }
}
