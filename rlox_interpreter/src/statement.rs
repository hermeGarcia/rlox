use rlox_ast::{Ast, Stmt, StmtId};

use crate::RuntimeResult;
use crate::expression;
use crate::value_system::Value;

pub fn eval(stmt: StmtId, ast: &Ast) -> RuntimeResult<Value> {
    match &ast[stmt] {
        Stmt::Expr(id) => expression::eval(*id, ast),
    }
}
