pub mod expression;

use rlox_ast::{Ast, Stmt};
use std::io::{BufWriter, Result, Write};

pub fn graph<W: Write>(ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "digraph {{")?;

    for root in ast.roots().iter().copied() {
        match &ast[root] {
            Stmt::Expr(id) => expression::graph(*id, ast, writer)?,
        }
    }

    writeln!(writer, "}}")?;
    writer.flush()
}
