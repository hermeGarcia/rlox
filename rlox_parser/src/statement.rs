use rlox_ast::{Ast, AstElem, AstProperty, Stmt, StmtId};

use crate::expression;
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    let expr_id = expression::parse(ctxt, ast)?;
    let stmt_id = ast.add(Stmt::Expr(expr_id));
    let expr_metadata = *ast.get(expr_id);

    ast.attach(stmt_id, expr_metadata);
    Ok(stmt_id)
}
