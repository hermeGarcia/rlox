pub mod expression;
pub mod statement;

use rlox_ast::Ast;
use std::io::{BufWriter, Result, Write};

#[derive(Clone, Copy)]
struct DataWithId<Id, Data> {
    my_id: Id,
    data: Data,
}

impl<Id, Data> DataWithId<Id, Data> {
    pub fn new(my_id: Id, data: Data) -> DataWithId<Id, Data> {
        DataWithId {
            my_id,
            data,
        }
    }
}

pub fn graph<W: Write>(ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "digraph {{")?;

    for root in ast.initial_block().iter().copied() {
        statement::graph(root, ast, writer)?;
    }

    writeln!(writer, "}}")?;
    writer.flush()
}
