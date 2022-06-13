use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html, NodeRef};

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
        html! {
            <table>
                <colgroup>
                    <col id="expr_col"/>
                    <col id="op_col"/>
                </colgroup>
                <tbody>
                    { for self.state.iter_items().map(|(id, e)| self.view_item(link, *id, e)) }
                </tbody>
                <tfoot>
                    <tr>
                        <td class="total">{ format!("Total: {} {}", total, UNIT) }</td>
                        <td/>
                    </tr>
                    <tr>
                        <td>
                            <input
                                ref={self.input_ref.clone()}
                                class={classes!(self.input_error.then(|| "error"))}
                                placeholder="985kJ / 6 * 2"
                                onkeydown={on_keydown}
                            />
                        </td>
                        <td><button onclick={on_add_item}>{ "+" }</button></td>
                    </tr>
                </tfoot>
            </table>
        }
    }
}

impl App {
    fn view_item(&self, link: &Scope<Self>, id: usize, expr: &Expr) -> Html {
        let on_increase = link.callback(move |_| Message::Increase(id));
        let on_decrease = link.callback(move |_| Message::Decrease(id));
        html! {
            <tr key={id}>
                <td>{ format!("{} = {:.0} {}", expr, expr.calc(UNIT), UNIT) }</td>
                <td>
                    <button onclick={on_increase}>{ "+" }</button>
                    <button onclick={on_decrease}>{ "-" }</button>
                </td>
            </tr>
        }
    }
}
