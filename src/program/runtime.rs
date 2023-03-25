use crate::expressions::expression::ExpressionRes;
use crate::statements::statement::Statement;
#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub args : Vec<ExpressionRes>,
    pub body: Box<Statement>,
}

impl Method {
    pub fn new(name: String, args : Vec<ExpressionRes>, body: Box<Statement>) -> Method {
        Method {
            name,
            args,
            body,
        }
    }
}