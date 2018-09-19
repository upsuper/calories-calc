use crate::expr::{Expr, Unit};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, DocumentFragment, Element, Event, EventTarget, HtmlElement, HtmlInputElement,
    HtmlTemplateElement, KeyboardEvent, Node, Window, window,
};

const UNIT: Unit = Unit::Kj;

lazy_static! {
    static ref WIN: Window = window().unwrap();
    static ref DOC: Document = WIN.document().unwrap();
    static ref INPUT: HtmlInputElement = q("#input").unchecked_into();
    static ref RECORDS: Node = q("#records").into();
    static ref RECORD: HtmlTemplateElement = q("#record").unchecked_into();
    static ref TOTAL: Node = q("#total").into();
}

fn q(query: &str) -> Element {
    DOC.query_selector(query).unwrap().unwrap()
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
        let target: Element = match evt.target() {
            Some(target) => target.unchecked_into(),
            None => return Ok(()),
        };
        if target.id() == "add" {
            add_item()?;
        } else if target.class_name() == "remove" {
            remove_item(target.closest("tr")?.unwrap())?;
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

    let new_record: DocumentFragment = DOC
        .import_node_with_deep(RECORD.content().as_ref(), true)?
        .unchecked_into();
    let expr_elem = new_record.query_selector(".expr")?.unwrap();
    Node::from(expr_elem).set_text_content(Some(&format!("{}", expr)));
    let value_elem = new_record.query_selector(".value")?.unwrap();
    Node::from(value_elem).set_text_content(Some(&format!("{} {}", value, UNIT)));
    RECORDS.insert_before(new_record.as_ref(), RECORDS.first_child().as_ref())?;
    update_total(value);
    Ok(())
}

fn remove_item(row: Element) -> Result<(), JsValue> {
    let value_elem = row.query_selector(".value")?.unwrap();
    let value = Node::from(value_elem).text_content().unwrap();
    let value = Expr::parse(&value).unwrap().calc(UNIT);
    row.remove();
    update_total(-value);
    Ok(())
}

fn update_total(diff: f32) {
    let current_total = TOTAL.text_content().unwrap();
    let current_total = Expr::parse(&current_total).unwrap().calc(UNIT);
    let new_total = current_total + diff;
    TOTAL.set_text_content(Some(&format!("{:.0} {}", new_total, UNIT)));
}
