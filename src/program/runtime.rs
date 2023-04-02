use std::cell::RefCell;
use std::rc::Rc;
use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;
use crate::program::program::ProgramEnvs;
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::{StatementInterpreter, StatementRes, StmtVisitor};

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub args : Vec<ExpressionRes>,
    pub body: Statement,
    pub env: Rc<RefCell<Environment>>
}

impl Method {
    pub fn new(name: String, args : Vec<ExpressionRes>, body: Statement, captured: Environment) -> Method {
        Method {
            name,
            args,
            body,
            env : Rc::new(RefCell::new(captured)),
        }
    }

    // this should be just the params right now, not the captured stuff
    pub fn call(&self, statement_interpreter: StatementInterpreter, env: Rc<RefCell<ProgramEnvs>>) -> Result<StatementRes, String> {
        let mut interpreter = StatementInterpreter::new_with_envs(

                env);
        return interpreter.eval(&self.body);
    }
}