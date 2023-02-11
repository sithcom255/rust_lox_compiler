use std::fmt::Debug;

pub trait Expression {
    fn accept(& self, visitor: Box<dyn Visitor>);
}

#[derive(Debug)]
pub struct Expr {
    value: String,
}

impl Expression for Expr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_expr( self);

    }
}

pub trait Visitor {
    fn execute_for_expr(&self, object: & Expr);
}

struct HelloWorldVisitor {}

impl Visitor for HelloWorldVisitor {
    fn execute_for_expr(&self, object:& Expr) {
        println!("Hello-world {}", object.value);
    }
}

#[test]
fn visitor_test() {
    let mut expr = Expr { value: String::from("here") };
    let visitor = HelloWorldVisitor {};
    let x = expr.accept(Box::new(visitor));
    println!("{:?}", x)
}