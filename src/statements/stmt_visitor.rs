use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;


use crate::expressions::expression::ExprResType::Variable;
use crate::expressions::visitor::ExpressionInterpreter;
use crate::statements::statement::Statement;
use crate::program::program::ProgramEnvs;


pub trait StmtVisitor {
    fn eval(&mut self, object: &Statement);
}

pub struct StatementInterpreter {
    pub expression_visitor: ExpressionInterpreter,
    pub program: Rc<RefCell<ProgramEnvs>>,
}

impl StmtVisitor for StatementInterpreter {

    fn eval(&mut self, object: &Statement) {
        match object {
            Statement::Stmt { expr } => {
                expr.accept(Rc::new(&self.expression_visitor));
            }
            Statement::PrintStatement { expr } => {
                let res = expr.accept(Rc::new(&self.expression_visitor));
                println!("{}", res.print())
            }
            Statement::VarDeclaration { identifier, expr } => {
                let identifier_res = identifier.as_ref().accept(Rc::new(&self.expression_visitor));
                let content = expr.as_ref().unwrap().accept(Rc::new(&self.expression_visitor));
                println!("{identifier_res:?}  {content:?}");
                if identifier_res.type_ == Variable {
                    let rc = self.program.clone();
                    let mut ref_mut = rc.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    envs.assign_value_to_var(0, identifier_res.str, content);
                };
            }
        }

    }
}

impl StatementInterpreter {

    pub fn new(expression_visitor: ExpressionInterpreter) -> StatementInterpreter {
        StatementInterpreter { expression_visitor, program: Rc::new(RefCell::new(ProgramEnvs::new())) }
    }

    pub fn interpret(&mut self, program: Vec<Box<Statement>>) {
        for statement in program {
            self.eval(&*statement)
        }
    }
}
