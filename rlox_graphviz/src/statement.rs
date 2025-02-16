use rlox_ast::stmt as rlox_stmt;
use rlox_ast::{Ast, Stmt, StmtId};
use std::io::{BufWriter, Result, Write};

use crate::DataWithId;
use crate::expression;

type StmtWithId<Data> = DataWithId<StmtId, Data>;

pub fn graph<W: Write>(stmt_id: StmtId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match &ast[stmt_id] {
        Stmt::Expr(id) => expression::graph(*id, ast, writer),
        Stmt::Print(inner) => graph_print(DataWithId::new(stmt_id, inner), ast, writer),
    }
}

fn graph_print<W: Write>(stmt: StmtWithId<&rlox_stmt::Print>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression::graph(stmt.data.expr, ast, writer)?;

    writeln!(writer, "\"{:?}\" [label=\"Print\"]", stmt.my_id)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", stmt.my_id, stmt.data.expr)?;

    Ok(())
}
