use rlox_ast::Ast;
use rlox_ast::AstProperty;
use rlox_ast::expr::Expr;
use rlox_ast::stmt::*;

use crate::RuntimeResult;
use crate::error;
use crate::expression;
use crate::runtime::Runtime;
use crate::value_system::Value;

type StmtResult = RuntimeResult<()>;

pub fn eval<'a>(stmt: Stmt, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    match stmt.kind() {
        StmtKind::Expr(inner) => expr_stmt(stmt_node!(stmt, inner), ast, runtime),
        StmtKind::Print(inner) => print(stmt_node!(stmt, inner), ast, runtime),
        StmtKind::Declaration(inner) => declaration(stmt_node!(stmt, inner), ast, runtime),
        StmtKind::Block(inner) => block(stmt_node!(stmt, inner), ast, runtime),
        StmtKind::IfElse(inner) => if_else(stmt_node!(stmt, inner), ast, runtime),
        StmtKind::While(inner) => while_stmt(stmt_node!(stmt, inner), ast, runtime),
    }
}

fn declaration<'a>(node: StmtNode<DeclarationId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let declaration = &ast[node.inner];

    let value = match declaration.value {
        None => Value::Nil,
        Some(expr) => expression::deref_expression(expr, ast, runtime)?,
    };

    runtime.insert(&ast[declaration.identifier], value);

    Ok(())
}

fn print<'a>(node: StmtNode<PrintId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let stmt = &ast[node.inner];
    let expr_value = expression::deref_expression(stmt.expr, ast, runtime)?;

    println!("{expr_value}");

    Ok(())
}

fn block<'a>(node: StmtNode<BlockId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let block = &ast[node.inner];

    runtime.enter_block();

    for stmt in block.iter().copied() {
        eval(stmt, ast, runtime)?;
    }

    runtime.leave_block();

    Ok(())
}

fn if_else<'a>(node: StmtNode<IfElseId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let stmt = &ast[node.inner];

    let catch = |unexpected_value: Value| -> error::RuntimeError {
        let stmt_metadata = ast.get(node.stmt_id);
        let condition_metadata = ast.get(stmt.condition.global_id());

        From::from(error::UnexpectedValue {
            start: stmt_metadata.start,
            end: condition_metadata.end,
            source: stmt_metadata.source,
            value: unexpected_value.to_string(),
        })
    };

    match expression::deref_expression(stmt.condition, ast, runtime)? {
        Value::Boolean(true) => eval(stmt.if_branch, ast, runtime),

        Value::Boolean(false) => match stmt.else_branch {
            Some(branch) => eval(branch, ast, runtime),
            None => Ok(()),
        },

        unexpected_value => Err(catch(unexpected_value)),
    }
}

fn while_stmt<'a>(node: StmtNode<WhileId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let stmt = &ast[node.inner];

    let catch = |unexpected_value: Value| -> error::RuntimeError {
        let stmt_metadata = ast.get(node.stmt_id);
        let condition_metadata = ast.get(stmt.condition.global_id());

        From::from(error::UnexpectedValue {
            start: stmt_metadata.start,
            end: condition_metadata.end,
            source: stmt_metadata.source,
            value: unexpected_value.to_string(),
        })
    };

    loop {
        match expression::deref_expression(stmt.condition, ast, runtime)? {
            Value::Boolean(true) => eval(stmt.body, ast, runtime)?,

            Value::Boolean(false) => break Ok(()),

            unexpected_value => break Err(catch(unexpected_value)),
        }
    }
}

fn expr_stmt(node: StmtNode<Expr>, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    expression::deref_expression(node.inner, ast, runtime)?;
    Ok(())
}
