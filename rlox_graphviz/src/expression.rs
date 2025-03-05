use rlox_ast::expr::*;
use rlox_ast::{Ast, Expr, StrId};
use std::io::{BufWriter, Result, Write};

pub fn graph<W: Write>(expr: Expr, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression(expr, ast, writer)
}

fn expression<W: Write>(expr: Expr, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match expr.kind() {
        ExprKind::Boolean(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::Natural(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::Decimal(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::Binary(inner) => binary(expr.global_id(), inner, ast, writer),
        ExprKind::Unary(inner) => unary(expr.global_id(), inner, ast, writer),
        ExprKind::Assign(inner) => assign(expr.global_id(), inner, ast, writer),
        ExprKind::Identifier(inner) => identifier(expr.global_id(), inner, ast, writer),
        ExprKind::Nil => writeln!(writer, "\"{expr:?}\" [label=\"nil\"]",),
    }
}

fn identifier<W: Write>(expr: ExprId, id: StrId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "\"{expr:?}\" [label=\"{}\"]", &ast[id])?;
    Ok(())
}

fn assign<W: Write>(expr: ExprId, id: AssignId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let assign = &ast[id];

    writeln!(writer, "\"{expr:?}\" [label=\"Assign\"]")?;
    writeln!(writer, "\"{expr:?}\" -> \"{:?}\"", assign.lhs.global_id())?;
    writeln!(writer, "\"{expr:?}\" -> \"{:?}\"", assign.rhs.global_id())?;

    expression(assign.lhs, ast, writer)?;
    expression(assign.rhs, ast, writer)?;

    Ok(())
}

fn binary<W: Write>(expr: ExprId, id: BinaryId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = &ast[id];

    writeln!(writer, "\"{expr:?}\" [label=\"{:?}\"]", data.operator)?;
    writeln!(writer, "\"{expr:?}\" -> \"{:?}\"", data.lhs.global_id())?;
    writeln!(writer, "\"{expr:?}\" -> \"{:?}\"", data.rhs.global_id())?;

    expression(data.lhs, ast, writer)?;
    expression(data.rhs, ast, writer)?;

    Ok(())
}

fn unary<W: Write>(expr: ExprId, id: UnaryId, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let data = &ast[id];

    writeln!(writer, "\"{expr:?}\" [label=\"{:?}\"]", data.operator)?;
    writeln!(writer, "\"{expr:?}\" -> \"{:?}\"", data.operand.global_id())?;

    expression(data.operand, ast, writer)?;

    Ok(())
}
