use crate::attr::Attr;
use crate::traits::ToHtml;
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attr>,
    pub body: Vec<Box<dyn ToHtml>>,
}
