mod expression;
mod statement;

use rlox_ast::Ast;
use std::io::{BufWriter, Result, Write};

pub fn graph<W: Write>(ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "digraph {{")?;

    for root in ast.main().iter().copied() {
        statement::graph(root, ast, writer)?;
    }

    writeln!(writer, "}}")?;
    writer.flush()
}
