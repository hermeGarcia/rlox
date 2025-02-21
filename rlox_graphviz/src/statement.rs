use rlox_ast::stmt::*;
use rlox_ast::{Ast, Stmt, StmtId, StmtWithId};
use std::io::{BufWriter, Result, Write};

use crate::expression;

pub fn graph<W: Write>(stmt_id: StmtId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match stmt_id.kind {
        Stmt::Expr(id) => expression::graph(id, ast, writer),
        Stmt::Print(inner) => print(StmtWithId::new(stmt_id, &ast[inner]), ast, writer),
        Stmt::Declaration(inner) => declaration(StmtWithId::new(stmt_id, &ast[inner]), ast, writer),
    }
}

fn print<W: Write>(stmt: StmtWithId<&Print>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression::graph(stmt.data.expr, ast, writer)?;

    writeln!(writer, "\"{:?}\" [label=\"Print\"]", stmt.my_id)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", stmt.my_id, stmt.data.expr)?;

    Ok(())
}

fn declaration<W: Write>(stmt: StmtWithId<&Declaration>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let identifier_id = stmt.data.identifier;
    writeln!(writer, "\"{:?}\" [label=\"{}\"]", identifier_id, ast[identifier_id])?;
    writeln!(writer, "\"{:?}\" [label=\"Declaration\"]", stmt.my_id)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", stmt.my_id, identifier_id)?;

    if let Some(initialization) = stmt.data.value {
        expression::graph(initialization, ast, writer)?;
        writeln!(writer, "\"{:?}\" -> \"{:?}\"", stmt.my_id, initialization)?;
    }

    Ok(())
}
