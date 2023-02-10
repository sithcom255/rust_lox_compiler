pub trait Expression {
    fn accept(&mut self, visitor: &impl Visitor);
}

pub trait Visitor {
    fn execute_for_expr(&self, object: &impl Expression);
}

pub struct Expr {}

impl Expression for Expr {
    fn accept(&mut self, visitor: &impl Visitor) {
        visitor.execute_for_expr(self);
    }
}