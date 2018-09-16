mod expr;
mod helpers;

use crate::expr::{Expr, Unit};
use crate::helpers::*;
use lazy_static::lazy_static;
use std::mem::transmute;
use wasm_bindgen::prelude::*;
use web_sys::{
    Document, DocumentFragment, Element, Event, HtmlInputElement, HtmlTemplateElement, Node, Window,
};

const UNIT: Unit = Unit::Kj;

macro_rules! typed_element {
    ($id:expr => $ty:ty) => {{
        let elem = DOC.get_element_by_id($id).unwrap();
        unsafe { transmute::<_, $ty>(elem) }
    }};
}

lazy_static! {
    static ref DOC: Document = Window::document().unwrap();
    static ref INPUT: HtmlInputElement = typed_element!("input" => HtmlInputElement);
    static ref RECORDS: Node = DOC.get_element_by_id("records").unwrap().into();
    static ref RECORD: HtmlTemplateElement = typed_element!("record" => HtmlTemplateElement);
    static ref TOTAL: Node = DOC.get_element_by_id("total").unwrap().into();
}

#[wasm_bindgen]
pub fn handle_click(evt: Event) {
    let target = match evt.target() {
        Some(target) => unsafe { transmute::<_, Element>(target) },
        None => return,
    };
    if target.id() == "add" {
        add_item();
    } else if target.class_name() == "remove" {
        remove_item(target.closest("tr").unwrap().unwrap());
    }
}

fn add_item() {
    let input = INPUT.value();
    let expr = match Expr::parse(&input) {
        Ok(expr) => expr,
        Err(_) => unimplemented!(),
    };
    let value = expr.calc(UNIT);

    let new_record = DOC
        .import_node_with_deep(RECORD.content().as_ref(), true)
        .unwrap();
    let new_record = unsafe { transmute::<_, DocumentFragment>(new_record) };
    let expr_elem = new_record.query_selector_infallible(".expr");
    Node::from(expr_elem).set_text_content(Some(&format!("{}", expr)));
    let value_elem = new_record.query_selector_infallible(".value");
    Node::from(value_elem).set_text_content(Some(&format!("{:.0} {}", value, UNIT)));
    RECORDS.append_child(new_record.as_ref()).unwrap();
    update_total(value);
}

fn remove_item(row: Element) {
    let value_elem = row.query_selector_infallible(".value");
    let value = Node::from(value_elem).text_content().unwrap();
    let value = Expr::parse(&value).unwrap().calc(UNIT);
    row.remove();
    update_total(-value);
}

fn update_total(diff: f32) {
    let current_total = TOTAL.text_content().unwrap();
    let current_total = Expr::parse(&current_total).unwrap().calc(UNIT);
    let new_total = current_total + diff;
    TOTAL.set_text_content(Some(&format!("{:.0} {}", new_total, UNIT)));
}
