use rlox_ast::Ast;
use rlox_ast::expr::Expr;
use rlox_ast::stmt::*;
use std::io::{BufWriter, Result, Write};

use crate::expression;

pub fn graph<W: Write>(stmt: Stmt, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match stmt.kind() {
        StmtKind::Print(inner) => print(stmt_node!(stmt, inner), ast, writer),
        StmtKind::Block(inner) => block(stmt_node!(stmt, inner), ast, writer),
        StmtKind::Declaration(inner) => declaration(stmt_node!(stmt, inner), ast, writer),
        StmtKind::IfElse(inner) => if_else(stmt_node!(stmt, inner), ast, writer),
        StmtKind::While(inner) => while_stmt(stmt_node!(stmt, inner), ast, writer),
        StmtKind::Expr(inner) => stmt_expr(stmt_node!(stmt, inner), ast, writer),
    }
}

fn stmt_expr<W: Write>(node: StmtNode<Expr>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let expr = node.inner;

    writeln!(writer, "\"{stmt_id:?}\" [label=\"StmtExpr\"]")?;
    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", expr.global_id())?;
    expression::graph(expr, ast, writer)
}

fn block<W: Write>(node: StmtNode<BlockId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let block = &ast[node.inner];

    writeln!(writer, "\"{stmt_id:?}\" [label=\"Block\"]")?;

    for inner_stmt in block.iter().copied() {
        writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", inner_stmt.global_id())?;
        graph(inner_stmt, ast, writer)?;
    }

    Ok(())
}

fn print<W: Write>(node: StmtNode<PrintId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let data = &ast[node.inner];

    expression::graph(data.expr, ast, writer)?;

    writeln!(writer, "\"{stmt_id:?}\" [label=\"Print\"]")?;
    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.expr.global_id())?;

    Ok(())
}

fn declaration<W: Write>(node: StmtNode<DeclarationId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{:?}\" [label=\"{}\"]", data.identifier, &ast[data.identifier])?;
    writeln!(writer, "\"{stmt_id:?}\" [label=\"Declaration\"]")?;
    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.identifier)?;

    if let Some(initialization) = data.value {
        expression::graph(initialization, ast, writer)?;
        writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", initialization.global_id())?;
    }

    Ok(())
}

fn if_else<W: Write>(node: StmtNode<IfElseId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{stmt_id:?}\" [label=\"IfElse\"]")?;

    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.condition.global_id())?;
    expression::graph(data.condition, ast, writer)?;

    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.if_branch.global_id())?;
    graph(data.if_branch, ast, writer)?;

    if let Some(else_branch) = data.else_branch {
        writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", else_branch.global_id())?;
        graph(else_branch, ast, writer)?;
    }

    Ok(())
}

fn while_stmt<W: Write>(node: StmtNode<WhileId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let stmt_id = node.stmt_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{stmt_id:?}\" [label=\"While\"]")?;

    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.condition.global_id())?;
    expression::graph(data.condition, ast, writer)?;

    writeln!(writer, "\"{stmt_id:?}\" -> \"{:?}\"", data.body.global_id())?;
    graph(data.body, ast, writer)?;

    Ok(())
}
