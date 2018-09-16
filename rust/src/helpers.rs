use web_sys::{Document, DocumentFragment, Element};

pub trait QuerySelectorInfallible {
    fn query_selector_infallible(&self, selectors: &str) -> Element;
}

macro_rules! impl_query_selector_infallible {
    ($($ty:ty),+) => {
        $(
            impl QuerySelectorInfallible for $ty {
                fn query_selector_infallible(&self, selectors: &str) -> Element {
                    self.query_selector(selectors).unwrap().unwrap()
                }
            }
        )+
    }
}
impl_query_selector_infallible!(Document, DocumentFragment, Element);
