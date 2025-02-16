use rlox_ast::expr::{Binary, Unary};
use rlox_ast::{Ast, Expr, ExprId};
use std::io::{BufWriter, Result, Write};

use crate::DataWithId;

type ExprWithId<Data> = DataWithId<ExprId, Data>;

pub fn graph<W: Write>(expr: ExprId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression(expr, ast, writer)
}

fn expression<W: Write>(expr: ExprId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match &ast[expr] {
        Expr::Binary(data) => binary(DataWithId::new(expr, data), ast, writer),
        Expr::Unary(data) => unary(DataWithId::new(expr, data), ast, writer),
        Expr::Boolean(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Natural(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Decimal(data) => writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]",),
        Expr::Nil => writeln!(writer, "\"{expr:?}\" [label=\"nil\"]",),
    }
}

fn binary<W: Write>(binary: ExprWithId<&Binary>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = binary.data;

    writeln!(writer, "\"{:?}\" [label=\"{:?}\"]", binary.my_id, data.operator)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.lhs)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.rhs)?;

    expression(data.lhs, ast, writer)?;
    expression(data.rhs, ast, writer)?;

    Ok(())
}

fn unary<W: Write>(binary: ExprWithId<&Unary>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = binary.data;

    writeln!(writer, "\"{:?}\" [label=\"{:?}\"]", binary.my_id, data.operator)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", binary.my_id, data.operand)?;

    expression(data.operand, ast, writer)?;

    Ok(())
}
