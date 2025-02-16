use crate::{Ast, Expr, ExprId, Stmt, StmtId};

pub fn fmt_expr(expr_id: ExprId, ast: &Ast) -> String {
    match &ast[expr_id] {
        Expr::Binary(binary) => {
            let lhs = fmt_expr(binary.lhs, ast);
            let rhs = fmt_expr(binary.rhs, ast);
            format!("{:?}({lhs}, {rhs})", binary.operator)
        }

        Expr::Unary(unary) => {
            let operand = fmt_expr(unary.operand, ast);
            format!("{:?}({operand})", unary.operator)
        }

        other => format!("{other:?}"),
    }
}

pub fn fmt_stmt(stmt_id: StmtId, ast: &Ast) -> String {
    match &ast[stmt_id] {
        Stmt::Expr(inner) => fmt_expr(*inner, ast),

        Stmt::Print(inner) => {
            let operand = fmt_expr(inner.expr, ast);
            format!("Print({operand})")
        }
    }
}
