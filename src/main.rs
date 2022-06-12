use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html, NodeRef};

use crate::expr::{Expr, Unit};

mod expr;
mod utils;

const UNIT: Unit = Unit::Kj;

fn main() {
    yew::start_app::<App>();
}

#[derive(Default)]
struct App {
    /// Next item id to be pushed along with the expression.
    next_id: usize,
    /// List of items consist of an id and an expression.
    items: Vec<(usize, Expr)>,
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
                match Expr::parse(&input.value()) {
                    Ok(expr) => {
                        self.input_error = false;
                        input.set_value("");
                        self.items.push((self.next_id, expr));
                        self.next_id += 1;
                    }
                    Err(_) => self.input_error = true,
                }
            }
            Message::Increase(id) => {
                self.find_expr_by_id_mut(id).unwrap().1.adjust_factor(1.);
            }
            Message::Decrease(id) => {
                let (idx, expr) = self.find_expr_by_id_mut(id).unwrap();
                expr.adjust_factor(-1.);
                let new_value = expr.calc(UNIT).round();
                if (new_value - 0.).abs() < 1e-5 {
                    self.items.remove(idx);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total: f32 = self.items.iter().map(|(_, e)| e.calc(UNIT)).sum();
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
                    { for self.items.iter().map(|(id, e)| self.view_item(link, *id, e)) }
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
    fn find_expr_by_id_mut(&mut self, id: usize) -> Option<(usize, &mut Expr)> {
        self.items
            .iter_mut()
            .enumerate()
            .find_map(|(idx, (i, e))| (*i == id).then(|| (idx, e)))
    }

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
