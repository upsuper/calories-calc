use crate::expr::{Expr, Unit};
use crate::wasm_utils::UnwrapAbort;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, Document, DocumentFragment, Element, Event, EventTarget, HtmlElement, HtmlInputElement,
    HtmlTemplateElement, KeyboardEvent, Node, Window,
};

const UNIT: Unit = Unit::Kj;

lazy_static! {
    static ref WIN: Window = window().unwrap_abort();
    static ref DOC: Document = WIN.document().unwrap_abort();
    static ref INPUT: HtmlInputElement = q("#input").unchecked_into();
    static ref RECORDS: Node = q("#records").into();
    static ref RECORD: HtmlTemplateElement = q("#record").unchecked_into();
    static ref TOTAL: Node = q("#total").into();
}

fn q(query: &str) -> Element {
    DOC.query_selector(query).unwrap_abort().unwrap_abort()
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
    add_listener!((DOC.document_element().unwrap_abort(), "click") => |evt| {
        let target: Element = match evt.target() {
            Some(target) => target.unchecked_into(),
            None => return Ok(()),
        };
        if target.id() == "add" {
            return add_item();
        }
        let class_name = target.class_name();
        match class_name.as_str() {
            "inc" => increase_item(target.closest(".record")?.unwrap_abort()),
            "dec" => decrease_item(target.closest(".record")?.unwrap_abort()),
            _ => Ok(()),
        }
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
    let expr_elem = new_record.query_selector(".expr")?.unwrap_abort();
    Node::from(expr_elem).set_text_content(Some(&format!("{}", expr)));
    let value_elem = new_record.query_selector(".value")?.unwrap_abort();
    Node::from(value_elem).set_text_content(Some(&format!("{} {}", value, UNIT)));
    RECORDS.insert_before(new_record.as_ref(), RECORDS.first_child().as_ref())?;
    update_total(value);
    Ok(())
}

fn increase_item(row: Element) -> Result<(), JsValue> {
    adjust_item(row, 1.0)
}

fn decrease_item(row: Element) -> Result<(), JsValue> {
    adjust_item(row, -1.0)
}

fn adjust_item(row: Element, delta: f32) -> Result<(), JsValue> {
    let expr_elem = Node::from(row.query_selector(".expr")?.unwrap_abort());
    let expr = expr_elem.text_content().unwrap_abort();
    let mut expr = Expr::parse(&expr).unwrap_abort();
    expr.adjust_factor(delta);
    expr_elem.set_text_content(Some(&format!("{}", expr)));

    let value_elem = Node::from(row.query_selector(".value")?.unwrap_abort());
    let value = value_elem.text_content().unwrap_abort();
    let value = Expr::parse(&value).unwrap_abort().calc(UNIT);
    update_total(-value);
    let value = expr.calc(UNIT).round();
    if (value - 0.0).abs() < 1e-5 {
        row.remove();
    } else {
        value_elem.set_text_content(Some(&format!("{} {}", value, UNIT)));
        update_total(value);
    }
    Ok(())
}

fn update_total(diff: f32) {
    let current_total = TOTAL.text_content().unwrap_abort();
    let current_total = Expr::parse(&current_total).unwrap_abort().calc(UNIT);
    let new_total = current_total + diff;
    TOTAL.set_text_content(Some(&format!("{} {}", new_total, UNIT)));
}
