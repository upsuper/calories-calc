use crate::expr::{Expr, Unit};
use std::rc::Rc;

#[derive(Default)]
pub struct State {
    /// Next item id to be pushed along with the expression.
    next_id: usize,
    /// List of items consist of an id and an expression.
    items: Vec<(usize, Rc<Expr>)>,
}

impl State {
    pub fn iter_items(&self) -> impl Iterator<Item = &(usize, Rc<Expr>)> {
        self.items.iter()
    }

    pub fn total(&self, unit: Unit) -> f32 {
        self.items.iter().map(|(_, e)| e.calc(unit)).sum()
    }

    pub fn add_new_item(&mut self, value: &str) -> Result<(), ()> {
        let expr = Expr::parse(value)?;
        self.items.push((self.next_id, Rc::new(expr)));
        self.next_id += 1;
        Ok(())
    }

    pub fn increase_item(&mut self, id: usize) -> Option<()> {
        let (idx, expr) = self.find_expr_by_id(id)?;
        let mut expr = expr.clone();
        Rc::make_mut(&mut expr).adjust_factor(1.);
        self.items[idx] = (id, expr);
        Some(())
    }

    pub fn decrease_item(&mut self, id: usize) -> Option<()> {
        let (idx, expr) = self.find_expr_by_id(id)?;
        let mut expr = expr.clone();
        Rc::make_mut(&mut expr).adjust_factor(-1.);
        if !expr.is_zero() {
            self.items[idx] = (id, expr);
        } else {
            self.items.remove(idx);
        }
        Some(())
    }

    fn find_expr_by_id(&mut self, id: usize) -> Option<(usize, &Rc<Expr>)> {
        self.items
            .iter()
            .enumerate()
            .find_map(|(idx, (i, e))| (*i == id).then(|| (idx, e)))
    }
}
