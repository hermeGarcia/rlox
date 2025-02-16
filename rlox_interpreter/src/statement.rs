use rlox_ast::stmt as rlox_stmt;
use rlox_ast::{Ast, ExprId, Stmt, StmtId};

use crate::EvalCtxt;
use crate::RuntimeResult;
use crate::expression;

type StmtResult = RuntimeResult<()>;

pub fn eval(stmt: StmtId, ast: &Ast, ctxt: &mut EvalCtxt) -> StmtResult {
    match &ast[stmt] {
        Stmt::Expr(id) => eval_expr_stmt(*id, ast, ctxt),
        Stmt::Print(inner) => eval_print(inner, ast, ctxt),
    }
}

fn eval_print(stmt: &rlox_stmt::Print, ast: &Ast, ctxt: &mut EvalCtxt) -> StmtResult {
    let expr_value = expression::eval(stmt.expr, ast, ctxt)?;

    println!("{expr_value}");

    Ok(())
}

fn eval_expr_stmt(expr_id: ExprId, ast: &Ast, ctxt: &mut EvalCtxt) -> StmtResult {
    expression::eval(expr_id, ast, ctxt)?;
    Ok(())
}
