use rlox_ast::stmt::*;
use rlox_ast::{Ast, Expr, Stmt, StmtId, StmtKind};
use std::io::{BufWriter, Result, Write};

use crate::expression;

pub fn graph<W: Write>(stmt: Stmt, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match stmt.kind() {
        StmtKind::Print(inner) => print(stmt.global_id(), inner, ast, writer),
        StmtKind::Block(inner) => block(stmt.global_id(), inner, ast, writer),
        StmtKind::Declaration(inner) => declaration(stmt.global_id(), inner, ast, writer),
        StmtKind::Expr(inner) => stmt_expr(stmt.global_id(), inner, ast, writer),
    }
}

fn stmt_expr<W: Write>(stmt: StmtId, id: Expr, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "\"{stmt:?}\" [label=\"StmtExpr\"]")?;
    writeln!(writer, "\"{stmt:?}\" -> \"{:?}\"", id.global_id())?;
    expression::graph(id, ast, writer)
}

fn block<W: Write>(stmt: StmtId, id: BlockId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let block = &ast[id];

    writeln!(writer, "\"{stmt:?}\" [label=\"Block\"]")?;

    for inner_stmt in block.stmts.iter().copied() {
        writeln!(writer, "\"{stmt:?}\" -> \"{:?}\"", inner_stmt.global_id())?;
        graph(inner_stmt, ast, writer)?;
    }

    Ok(())
}

fn print<W: Write>(stmt: StmtId, id: PrintId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = &ast[id];

    expression::graph(data.expr, ast, writer)?;

    writeln!(writer, "\"{stmt:?}\" [label=\"Print\"]")?;
    writeln!(writer, "\"{stmt:?}\" -> \"{:?}\"", data.expr.global_id())?;

    Ok(())
}

fn declaration<W: Write>(stmt: StmtId, id: DeclarationId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = &ast[id];

    writeln!(writer, "\"{:?}\" [label=\"{}\"]", data.identifier, ast[data.identifier])?;
    writeln!(writer, "\"{stmt:?}\" [label=\"Declaration\"]")?;
    writeln!(writer, "\"{stmt:?}\" -> \"{:?}\"", data.identifier)?;

    if let Some(initialization) = data.value {
        expression::graph(initialization, ast, writer)?;
        writeln!(writer, "\"{stmt:?}\" -> \"{:?}\"", initialization.global_id())?;
    }

    Ok(())
}
