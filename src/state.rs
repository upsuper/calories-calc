use crate::expr::{Expr, Unit};

#[derive(Default)]
pub struct State {
    /// Next item id to be pushed along with the expression.
    next_id: usize,
    /// List of items consist of an id and an expression.
    items: Vec<(usize, Expr)>,
}

impl State {
    pub fn iter_items(&self) -> impl Iterator<Item = &(usize, Expr)> {
        self.items.iter()
    }

    pub fn total(&self, unit: Unit) -> f32 {
        self.items.iter().map(|(_, e)| e.calc(unit)).sum()
    }

    pub fn add_new_item(&mut self, value: &str) -> Result<(), ()> {
        let expr = Expr::parse(value)?;
        self.items.push((self.next_id, expr));
        self.next_id += 1;
        Ok(())
    }

    pub fn increase_item(&mut self, id: usize) -> Option<()> {
        self.find_expr_by_id_mut(id)?.1.adjust_factor(1.);
        Some(())
    }

    pub fn decrease_item(&mut self, id: usize) -> Option<()> {
        let (idx, expr) = self.find_expr_by_id_mut(id)?;
        expr.adjust_factor(-1.);
        if expr.is_zero() {
            self.items.remove(idx);
        }
        Some(())
    }

    fn find_expr_by_id_mut(&mut self, id: usize) -> Option<(usize, &mut Expr)> {
        self.items
            .iter_mut()
            .enumerate()
            .find_map(|(idx, (i, e))| (*i == id).then(|| (idx, e)))
    }
}
