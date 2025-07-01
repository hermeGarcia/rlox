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
    let condition = expression::deref_expression(stmt.condition, ast, runtime)?;

    let Value::Boolean(condition) = condition else {
        let stmt_metadata = ast.get(node.stmt_id);
        let condition_metadata = ast.get(stmt.condition.global_id());

        return Err(From::from(error::UnexpectedValue {
            start: stmt_metadata.start,
            end: condition_metadata.end,
            source: stmt_metadata.source,
            found: condition,
        }));
    };

    if condition {
        eval(stmt.if_branch, ast, runtime)
    } else if let Some(branch) = stmt.else_branch {
        eval(branch, ast, runtime)
    } else {
        Ok(())
    }
}

fn while_stmt<'a>(node: StmtNode<WhileId>, ast: &'a Ast, runtime: &mut Runtime<'a>) -> StmtResult {
    let stmt = &ast[node.inner];

    loop {
        let condition = expression::deref_expression(stmt.condition, ast, runtime)?;

        let Value::Boolean(condition) = condition else {
            let stmt_metadata = ast.get(node.stmt_id);
            let condition_metadata = ast.get(stmt.condition.global_id());

            return Err(From::from(error::UnexpectedValue {
                start: stmt_metadata.start,
                end: condition_metadata.end,
                source: stmt_metadata.source,
                found: condition,
            }));
        };

        if condition {
            eval(stmt.body, ast, runtime)?;
        } else {
            return Ok(());
        }
    }
}

fn expr_stmt(node: StmtNode<Expr>, ast: &Ast, runtime: &mut Runtime) -> StmtResult {
    expression::deref_expression(node.inner, ast, runtime)?;
    Ok(())
}
