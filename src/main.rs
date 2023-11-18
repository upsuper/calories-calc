use gloo_storage::{SessionStorage, Storage};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::{classes, function_component, html};
use yew::{Callback, Component, Context, Html, NodeRef, Properties};

use crate::expr::{Expr, Unit};
use crate::state::State;

mod expr;
mod state;

const UNIT: Unit = Unit::Kj;
const BACKSPACE_KEY: &str = "\u{8}";
const ENTER_KEY: &str = "\r";

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Default)]
struct App {
    state: State,
    scrollable_ref: NodeRef,
    /// Reference to the input element for expression.
    input_ref: NodeRef,
    /// Whether the latest input has error on parsing.
    input_error: bool,
}

enum Message {
    AddNew,
    Increase(usize),
    Decrease(usize),
    Clear,
    Input(String),
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut app = Self::default();
        let _ = app.restore_session();
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddNew => {
                let input = self.input();
                let _ = input.focus();
                let result = self.state.add_new_item(&input.value());
                self.input_error = result.is_err();
                if result.is_ok() {
                    input.set_value("");
                }
            }
            Message::Increase(id) => {
                self.state.increase_item(id);
            }
            Message::Decrease(id) => {
                self.state.decrease_item(id);
            }
            Message::Clear => self.state.clear(),
            Message::Input(c) => {
                let input = self.input();
                let mut value: String = input.value();
                match c.as_str() {
                    ENTER_KEY => ctx.link().send_message(Message::AddNew),
                    BACKSPACE_KEY => {
                        let mut value = value.as_str().trim_end();
                        if let Some((idx, _)) = value.char_indices().next_back() {
                            value = value[0..idx].trim_end();
                        }
                        input.set_value(value);
                    }
                    c => {
                        if !value.is_empty()
                            && !value.ends_with(|c: char| c.is_ascii_whitespace())
                            && (!c.starts_with(|c: char| c.is_ascii_digit())
                                || !value.ends_with(|c: char| c.is_ascii_digit()))
                        {
                            value.push(' ');
                        }
                        value.push_str(c);
                        input.set_value(&value);
                    }
                }
                return false;
            }
        }
        self.save_session();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total = self.state.total(UNIT);
        let link = ctx.link();
        let on_keydown =
            link.batch_callback(|e: KeyboardEvent| (e.key() == "Enter").then_some(Message::AddNew));
        let on_clear = link.callback(|_| Message::Clear);
        let on_add_item = link.callback(|_| Message::AddNew);
        let on_increase = link.callback(Message::Increase);
        let on_decrease = link.callback(Message::Decrease);
        let on_input = link.callback(Message::Input);
        html! {
            <div class="root">
                <div ref={self.scrollable_ref.clone()} class="scrollable">
                    <ul class="items">
                        { for self.state.iter_items().map(|(id, expr)| html! {
                            <ItemView
                                key={*id}
                                id={*id}
                                expr={expr.clone()}
                                on_increase={on_increase.clone()}
                                on_decrease={on_decrease.clone()}
                            />
                        }) }
                    </ul>
                </div>
                <div class="total">
                    <div class="expr">
                        { format!("Total: {:.0} {}", total, UNIT) }
                    </div>
                    <div class="controls">
                        if !self.state.is_empty() {
                            <button onclick={on_clear}>{ "\u{2715}" }</button>
                        }
                    </div>
                </div>
                <div class="input">
                    <input
                        ref={self.input_ref.clone()}
                        class={classes!(self.input_error.then_some("error"))}
                        placeholder="985kJ / 6 * 2"
                        inputmode="none"
                        onkeydown={on_keydown}
                    />
                    <div class="controls">
                        <button onclick={on_add_item}>{ "+" }</button>
                    </div>
                </div>
                <Keypad on_input={on_input}/>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let scrollable = self.scrollable_ref.cast::<HtmlDivElement>().unwrap();
        let max_scroll_top = scrollable.scroll_height() - scrollable.client_height();
        scrollable.set_scroll_top(max_scroll_top);
    }
}

impl App {
    fn input(&self) -> HtmlInputElement {
        self.input_ref.cast::<HtmlInputElement>().unwrap()
    }

    fn restore_session(&mut self) -> Result<(), ()> {
        let items: Vec<String> = SessionStorage::get("items").map_err(|_| ())?;
        for item in items {
            self.state.add_new_item(&item)?;
        }
        Ok(())
    }

    fn save_session(&self) {
        let items = self
            .state
            .iter_items()
            .map(|(_, expr)| expr.to_string())
            .collect::<Vec<_>>();
        let _ = SessionStorage::set("items", items);
    }
}

#[derive(Properties, PartialEq)]
struct ItemViewProps {
    id: usize,
    expr: Rc<Expr>,
    on_increase: Callback<usize>,
    on_decrease: Callback<usize>,
}

#[function_component(ItemView)]
fn item_view(props: &ItemViewProps) -> Html {
    let id = props.id;
    let on_increase = props.on_increase.clone();
    let on_increase = move |_| on_increase.emit(id);
    let on_decrease = props.on_decrease.clone();
    let on_decrease = move |_| on_decrease.emit(id);
    html! {
        <li class="item">
            <div class="expr">
                { format!("{} = {:.0} {}", props.expr, props.expr.calc(UNIT), UNIT) }
            </div>
            <div class="controls">
                <button onclick={on_increase}>{ "+" }</button>
                <button onclick={on_decrease}>{ "-" }</button>
            </div>
        </li>
    }
}

#[derive(Properties, PartialEq)]
struct KeypadProps {
    on_input: Callback<String>,
}

#[function_component(Keypad)]
fn keypad(props: &KeypadProps) -> Html {
    let on_input = props.on_input.clone();
    let on_click = move |e: MouseEvent| {
        if let Some(elem) = e
            .target()
            .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok())
        {
            on_input.emit(elem.value());
        }
    };
    let buttons = [
        ("1", "1", "one"),
        ("2", "2", "two"),
        ("3", "3", "three"),
        ("4", "4", "four"),
        ("5", "5", "five"),
        ("6", "6", "six"),
        ("7", "7", "seven"),
        ("8", "8", "eight"),
        ("9", "9", "nine"),
        ("0", "0", "zero"),
        ("kJ", "kJ", "kj"),
        ("kcal", "kcal", "kcal"),
        ("*", "*", "mul"),
        ("/", "/", "div"),
        (BACKSPACE_KEY, "\u{2190}", "bs"),
        (ENTER_KEY, "\u{21b5}", "cr"),
    ];
    let buttons = buttons
        .iter()
        .map(|&(value, content, area)| {
            let style = format!("grid-area: {area}");
            let onclick = on_click.clone();
            html! { <button {value} {style} {onclick}>{content}</button> }
        })
        .collect::<Html>();
    html! { <div class="keyboard">{ buttons }</div> }
}
