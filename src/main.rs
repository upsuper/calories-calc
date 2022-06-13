use std::rc::Rc;
use web_sys::{HtmlDivElement, HtmlInputElement, KeyboardEvent};
use yew::{classes, function_component, html};
use yew::{Callback, Component, Context, Html, NodeRef, Properties};

use crate::expr::{Expr, Unit};
use crate::state::State;

mod expr;
mod state;

const UNIT: Unit = Unit::Kj;

fn main() {
    yew::start_app::<App>();
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
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddNew => {
                let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
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
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total = self.state.total(UNIT);
        let link = ctx.link();
        let on_keydown =
            link.batch_callback(|e: KeyboardEvent| (e.key() == "Enter").then(|| Message::AddNew));
        let on_add_item = link.callback(|_| Message::AddNew);
        let on_increase = link.callback(Message::Increase);
        let on_decrease = link.callback(Message::Decrease);
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
                        { format!("Total: {} {}", total, UNIT) }
                    </div>
                    <div class="controls"/>
                </div>
                <div class="input">
                    <input
                        ref={self.input_ref.clone()}
                        class={classes!(self.input_error.then(|| "error"))}
                        placeholder="985kJ / 6 * 2"
                        onkeydown={on_keydown}
                    />
                    <div class="controls">
                        <button onclick={on_add_item}>{ "+" }</button>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let scrollable = self.scrollable_ref.cast::<HtmlDivElement>().unwrap();
        let max_scroll_top = scrollable.scroll_height() - scrollable.client_height();
        scrollable.set_scroll_top(max_scroll_top);
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
