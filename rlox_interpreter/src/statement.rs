use rlox_ast::AstProperty;
use rlox_ast::stmt::*;
use rlox_ast::{Ast, Expr, Stmt, StmtKind};

use crate::RuntimeResult;
use crate::error;
use crate::expression;
use crate::runtime::Runtime;
use crate::value_system::Value;

type StmtResult = RuntimeResult<()>;

pub fn eval<'a>(stmt: Stmt, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    match stmt.kind() {
        StmtKind::Expr(inner) => expr_stmt(inner, ast, runtime),
        StmtKind::Print(inner) => print(inner, ast, runtime),
        StmtKind::Declaration(inner) => declaration(inner, ast, runtime),
        StmtKind::Block(inner) => block(inner, ast, runtime),
        StmtKind::IfElse(inner) => if_else(stmt.global_id(), inner, ast, runtime),
    }
}

fn declaration<'a>(id: DeclarationId, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let declaration = &ast[id];

    let value = match declaration.value {
        None => Value::Nil,
        Some(expr) => expression::deref_expression(expr, ast, runtime)?,
    };

    runtime.insert(&ast[declaration.identifier], value);

    Ok(())
}

fn print<'a>(id: PrintId, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let expr_value = expression::deref_expression(ast[id].expr, ast, runtime)?;

    println!("{expr_value}");

    Ok(())
}

fn block<'a>(id: BlockId, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let block = &ast[id];

    runtime.enter_block();

    for stmt in block.iter().copied() {
        eval(stmt, ast, runtime)?;
    }

    runtime.leave_block();

    Ok(())
}

fn if_else<'a>(global_id: StmtId, id: IfElseId, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let stmt = &ast[id];

    let condition = match expression::deref_expression(stmt.condition, ast, runtime)? {
        Value::Boolean(inner) => inner,

        unexpected_value => {
            let stmt_metadata = ast.get(global_id);
            let condition_metadata = ast.get(stmt.condition.global_id());
            return Err(From::from(error::UnexpectedValue {
                start: stmt_metadata.start,
                end: condition_metadata.end,
                source: stmt_metadata.source,
                value: unexpected_value.to_string(),
            }));
        }
    };

    if condition {
        eval(stmt.if_branch, ast, runtime)
    } else if let Some(branch) = stmt.else_branch {
        eval(branch, ast, runtime)
    } else {
        Ok(())
    }
}

fn expr_stmt(expr_id: Expr, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    expression::deref_expression(expr_id, ast, runtime)?;
    Ok(())
}
