use rlox_ast::expr::{Binary, Unary};
use rlox_ast::{Ast, Expr, ExprId};
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

type Writer<W> = W;

pub fn graph<W: Write>(expr: ExprId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression(expr, ast, writer)
}

fn expression<W: Write>(expr: ExprId, ast: &Ast, writer: &mut Writer<W>) -> Result<()> {
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
