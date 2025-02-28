use rlox_ast::stmt::*;
use rlox_ast::{Ast, Expr, Stmt, StmtKind};

use crate::RuntimeResult;
use crate::expression;
use crate::runtime::Runtime;
use crate::value_system::Value;

type StmtResult = RuntimeResult<()>;

pub fn eval(stmt: Stmt, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    match stmt.kind() {
        StmtKind::Expr(inner) => expr_stmt(inner, ast, runtime),
        StmtKind::Print(inner) => print(inner, ast, runtime),
        StmtKind::Declaration(inner) => declaration(inner, ast, runtime),
        StmtKind::Block(inner) => block(inner, ast, runtime),
    }
}

fn declaration(id: DeclarationId, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    let declaration = &ast[id];

    let value = match declaration.value {
        None => Value::Nil,
        Some(expr) => expression::deref_expression(expr, ast, runtime)?,
    };

    runtime.insert(declaration.identifier, value);

    Ok(())
}

fn print(id: PrintId, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    let expr_value = expression::deref_expression(ast[id].expr, ast, runtime)?;

    println!("{expr_value}");

    Ok(())
}

fn block(id: BlockId, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    let block = &ast[id];

    runtime.enter_block();

    for stmt in block.stmts.iter().copied() {
        eval(stmt, ast, runtime)?;
    }

    runtime.leave_block();

    Ok(())
}

fn expr_stmt(expr_id: Expr, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    expression::deref_expression(expr_id, ast, runtime)?;
    Ok(())
}
