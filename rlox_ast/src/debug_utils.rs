use crate::{Ast, Expr, ExprId, Stmt, StmtId};

pub fn fmt_expr(expr_id: ExprId, ast: &Ast) -> String {
    match expr_id.kind {
        Expr::Assign(id) => {
            let assign = &ast[id];
            let lhs = fmt_expr(assign.lhs, ast);
            let rhs = fmt_expr(assign.rhs, ast);
            format!("Assign({lhs}, {rhs})")
        }

        Expr::Binary(id) => {
            let binary = &ast[id];
            let lhs = fmt_expr(binary.lhs, ast);
            let rhs = fmt_expr(binary.rhs, ast);
            format!("{:?}({lhs}, {rhs})", binary.operator)
        }

        Expr::Unary(id) => {
            let unary = &ast[id];
            let operand = fmt_expr(unary.operand, ast);
            format!("{:?}({operand})", unary.operator)
        }

        Expr::Identifier(string_id) => ast[string_id].clone(),

        other => format!("{other:?}"),
    }
}

pub fn fmt_stmt(stmt_id: StmtId, ast: &Ast) -> String {
    match stmt_id.kind {
        Stmt::Expr(inner) => fmt_expr(inner, ast),

        Stmt::Print(id) => {
            let print = &ast[id];
            let operand = fmt_expr(print.expr, ast);
            format!("Print({operand})")
        }

        Stmt::Declaration(id) => {
            let declaration = &ast[id];
            let identifier = &ast[declaration.identifier];
            let value = match declaration.value {
                Some(id) => fmt_expr(id, ast),
                None => "None".to_string(),
            };

            format!("Declaration({identifier}, {value})")
        }
    }
}
