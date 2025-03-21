use crate::{Ast, Expr, ExprKind, Stmt, StmtKind};

pub fn fmt_expr(expr: Expr, ast: &Ast) -> String {
    match expr.kind() {
        ExprKind::Assign(id) => {
            let assign = &ast[id];
            let lhs = fmt_expr(assign.lhs, ast);
            let rhs = fmt_expr(assign.rhs, ast);
            format!("Assign({lhs}, {rhs})")
        }

        ExprKind::Binary(id) => {
            let binary = &ast[id];
            let lhs = fmt_expr(binary.lhs, ast);
            let rhs = fmt_expr(binary.rhs, ast);
            format!("{:?}({lhs}, {rhs})", binary.operator)
        }

        ExprKind::Unary(id) => {
            let unary = &ast[id];
            let operand = fmt_expr(unary.operand, ast);
            format!("{:?}({operand})", unary.operator)
        }

        ExprKind::Identifier(inner) | ExprKind::String(inner) => ast[inner].into(),

        other => format!("{other:?}"),
    }
}

pub fn fmt_stmt(stmt: Stmt, ast: &Ast) -> String {
    match stmt.kind() {
        StmtKind::Expr(inner) => fmt_expr(inner, ast),

        StmtKind::Block(id) => {
            let stmts = ast[id].iter();
            let inner: Vec<_> = stmts.map(|stmt| fmt_stmt(*stmt, ast)).collect();

            format!("Block({inner:?})")
        }

        StmtKind::IfElse(id) => {
            let stmt = &ast[id];

            let condition = fmt_expr(stmt.condition, ast);

            let if_branch = fmt_stmt(stmt.if_branch, ast);

            let else_branch = match stmt.else_branch {
                Some(branch) => fmt_stmt(branch, ast),
                None => "None".into(),
            };

            format!("IfElse({condition},{if_branch},{else_branch})")
        }

        StmtKind::Print(id) => {
            let print = &ast[id];
            let operand = fmt_expr(print.expr, ast);
            format!("Print({operand})")
        }

        StmtKind::Declaration(id) => {
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
