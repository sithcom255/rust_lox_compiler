pub trait Expression {
    fn accept<T: Visitor + ?Sized>(&mut self, visitor: &T);
}

pub trait Visitor {
    fn execute_for_expr(&self, object: &impl Expression);
}

pub struct Expr {}

impl Expression for Expr {
    fn accept<T: Visitor + ?Sized>(&mut self, visitor: &T) {
        visitor.execute_for_expr(self);
    }
}


struct HelloWorldVisitor {}

impl Visitor for HelloWorldVisitor{
    fn execute_for_expr(&self, object: &impl Expression) {
        println!("Hello-world")
    }
}

#[test]
fn visitor_test() {
    let mut expr = Expr {};
    let visitor = HelloWorldVisitor {};
    expr.accept(&visitor)
}