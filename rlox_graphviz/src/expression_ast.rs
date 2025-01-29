use rlox_ast::expr::{Binary, Unary};
use rlox_ast::{Ast, Expr, ExprId};
use std::collections::HashSet;
use std::io::{BufWriter, Result, Write};

#[derive(Clone, Copy)]
struct DataWithId<Data> {
    my_id: ExprId,
    data: Data,
}

impl<Data> DataWithId<Data> {
    pub fn new(my_id: ExprId, data: Data) -> DataWithId<Data> {
        DataWithId {
            my_id,
            data,
        }
    }
}

struct Writer<W> {
    writer: W,
    visited: HashSet<ExprId>,
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl<W> Writer<W> {
    pub fn visited(&mut self, expr: ExprId) {
        self.visited.insert(expr);
    }

    pub fn is_visited(&self, expr: ExprId) -> bool {
        self.visited.contains(&expr)
    }
}

pub fn write<W: Write>(ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let mut writer = Writer {
        writer,
        visited: HashSet::new(),
    };

    writeln!(writer, "digraph {{")?;
    for expr in ast.expr_ids().rev() {
        if writer.is_visited(expr) {
            continue;
        }

        expression(expr, ast, &mut writer)?;
    }

    writeln!(writer, "}}")?;
    writer.flush()
}

fn expression<W: Write>(expr: ExprId, ast: &Ast, writer: &mut Writer<W>) -> Result<()> {
    writer.visited(expr);

    match &ast[expr] {
        Expr::BinaryExpr(data) => binary(DataWithId::new(expr, data), ast, writer),
        Expr::UnaryExpr(data) => unary(DataWithId::new(expr, data), ast, writer),
        Expr::Boolean(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Natural(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Decimal(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Nil => writeln!(writer, "\"{expr:?}\" [label=\"nil\"]",),
    }
}

fn binary<W: Write>(binary: DataWithId<&Binary>, ast: &Ast, writer: &mut Writer<W>) -> Result<()> {
    let data = binary.data;

    writeln!(writer, "\"{:?}\" [label=\"{:?}\"]", binary.my_id, data.operator)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.lhs)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.rhs)?;

    expression(data.lhs, ast, writer)?;
    expression(data.rhs, ast, writer)?;

    Ok(())
}

fn unary<W: Write>(binary: DataWithId<&Unary>, ast: &Ast, writer: &mut Writer<W>) -> Result<()> {
    let data = binary.data;

    writeln!(writer, "\"{:?}\" [label=\"{:?}\"]", binary.my_id, data.operator)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.operand)?;

    expression(data.operand, ast, writer)?;

    Ok(())
}
