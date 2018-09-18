mod expr;
mod helpers;

use crate::expr::{Expr, Unit};
use crate::helpers::*;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, DocumentFragment, Element, Event, EventTarget, HtmlElement, HtmlInputElement,
    HtmlTemplateElement, KeyboardEvent, Node, Window,
};

const UNIT: Unit = Unit::Kj;

macro_rules! typed_element {
    ($id:expr => $ty:ty) => {{
        DOC.get_element_by_id($id).unwrap().unchecked_into::<$ty>()
    }};
}

lazy_static! {
    static ref DOC: Document = Window::document().unwrap();
    static ref INPUT: HtmlInputElement = typed_element!("input" => HtmlInputElement);
    static ref RECORDS: Node = DOC.get_element_by_id("records").unwrap().into();
    static ref RECORD: HtmlTemplateElement = typed_element!("record" => HtmlTemplateElement);
    static ref TOTAL: Node = DOC.get_element_by_id("total").unwrap().into();
}

macro_rules! add_listener {
    (($target:expr, $type:expr) => |$evt:ident| $block:block) => {{
        let func: Box<dyn FnMut(_) -> Result<_, JsValue>> = Box::new(|$evt: Event| $block);
        let closure = Closure::wrap(func);
        let result = ($target.as_ref() as &EventTarget)
            .add_event_listener_with_callback($type, closure.as_ref().unchecked_ref());
        closure.forget();
        result
    }};
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    if DOC.ready_state() != "loading" {
        add_event_listeners()
    } else {
        add_listener!((DOC, "DOMContentLoaded") => |_evt| {
            add_event_listeners()
        })
    }
}

fn add_event_listeners() -> Result<(), JsValue> {
    add_listener!((DOC.document_element().unwrap(), "click") => |evt| {
        let target = match evt.target() {
            Some(target) => target.unchecked_into::<Element>(),
            None => return Ok(()),
        };
        if target.id() == "add" {
            add_item()?;
        } else if target.class_name() == "remove" {
            remove_item(target.closest("tr")?.unwrap());
        }
        Ok(())
    })?;
    add_listener!((INPUT, "keypress") => |evt| {
        let evt: &KeyboardEvent = evt.unchecked_ref();
        if evt.key() == "Enter" {
            add_item()?;
        }
        Ok(())
    })?;
    Ok(())
}

fn add_item() -> Result<(), JsValue> {
    let input = INPUT.value();
    (INPUT.as_ref() as &HtmlElement).focus()?;
    let expr = match Expr::parse(&input) {
        Ok(expr) => expr,
        Err(_) => {
            (INPUT.as_ref() as &Element).set_class_name("error");
            return Ok(());
        }
    };
    (INPUT.as_ref() as &Element).set_class_name("");
    INPUT.set_value("");
    let value = expr.calc(UNIT).round();

    let new_record = DOC
        .import_node_with_deep(RECORD.content().as_ref(), true)?
        .unchecked_into::<DocumentFragment>();
    let expr_elem = new_record.query_selector_infallible(".expr");
    Node::from(expr_elem).set_text_content(Some(&format!("{}", expr)));
    let value_elem = new_record.query_selector_infallible(".value");
    Node::from(value_elem).set_text_content(Some(&format!("{} {}", value, UNIT)));
    RECORDS.insert_before(new_record.as_ref(), RECORDS.first_child().as_ref())?;
    update_total(value);
    Ok(())
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
