use rlox_ast::expr::*;
use rlox_ast::{Ast, Expr, ExprId, ExprWithId, StrId};
use std::io::{BufWriter, Result, Write};

pub fn graph<W: Write>(expr: ExprId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression(expr, ast, writer)
}

fn expression<W: Write>(expr: ExprId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match expr.kind {
        Expr::Boolean(inner) => writeln!(writer, "\"{expr:?}\" [label=\"{inner}\"]",),
        Expr::Natural(inner) => writeln!(writer, "\"{expr:?}\" [label=\"{inner}\"]",),
        Expr::Decimal(inner) => writeln!(writer, "\"{expr:?}\" [label=\"{inner}\"]",),
        Expr::Binary(inner) => binary(ExprWithId::new(expr, &ast[inner]), ast, writer),
        Expr::Unary(inner) => unary(ExprWithId::new(expr, &ast[inner]), ast, writer),
        Expr::Assign(inner) => assign(ExprWithId::new(expr, &ast[inner]), ast, writer),
        Expr::Identifier(inner) => identifier(ExprWithId::new(expr, inner), ast, writer),
        Expr::Nil => writeln!(writer, "\"{expr:?}\" [label=\"nil\"]",),
    }
}

fn identifier<W: Write>(expr: ExprWithId<StrId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "\"{:?}\" [label=\"{}\"]", expr.my_id, ast[expr.data])?;
    Ok(())
}

fn assign<W: Write>(expr: ExprWithId<&Assign>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "\"{:?}\" [label=\"Assign\"]", expr.my_id)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", expr.my_id, expr.data.lhs)?;
    writeln!(writer, "\"{:?}\" -> \"{:?}\"", expr.my_id, expr.data.rhs)?;

    expression(expr.data.lhs, ast, writer)?;
    expression(expr.data.rhs, ast, writer)?;

    Ok(())
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
