use rlox_ast::expr::*;
use rlox_ast::{Ast, StrId};
use std::io::{BufWriter, Result, Write};

pub fn graph<W: Write>(expr: Expr, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    expression(expr, ast, writer)
}

fn expression<W: Write>(expr: Expr, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    match expr.kind() {
        ExprKind::Binary(inner) => binary(expr_node!(expr, inner), ast, writer),
        ExprKind::Unary(inner) => unary(expr_node!(expr, inner), ast, writer),
        ExprKind::Assign(inner) => assign(expr_node!(expr, inner), ast, writer),
        ExprKind::Identifier(inner) => identifier(expr_node!(expr, inner), ast, writer),
        ExprKind::Call(inner) => call(expr_node!(expr, inner), ast, writer),

        ExprKind::Nil => writeln!(writer, "\"{:?}\" [label=\"nil\"]", expr.global_id()),
        ExprKind::Boolean(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::Natural(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::Decimal(inner) => writeln!(writer, "\"{:?}\" [label=\"{inner}\"]", expr.global_id()),
        ExprKind::String(inner) => writeln!(writer, "\"{:?}\" [label=\"Str({})\"]", expr.global_id(), &ast[inner]),
    }
}

fn identifier<W: Write>(node: ExprNode<StrId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let expr = node.expr_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{expr:?}\" [label=\"{data}\"]")
}

fn assign<W: Write>(id: ExprNode<AssignId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let expr_id = id.expr_id;
    let assign = &ast[id.inner];

    writeln!(writer, "\"{expr_id:?}\" [label=\"Assign\"]")?;
    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", assign.lhs.global_id())?;
    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", assign.rhs.global_id())?;

    expression(assign.lhs, ast, writer)?;
    expression(assign.rhs, ast, writer)?;

    Ok(())
}

fn binary<W: Write>(node: ExprNode<BinaryId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let expr_id = node.expr_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{expr_id:?}\" [label=\"{:?}\"]", data.operator)?;
    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", data.lhs.global_id())?;
    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", data.rhs.global_id())?;

    expression(data.lhs, ast, writer)?;
    expression(data.rhs, ast, writer)?;

    Ok(())
}

fn unary<W: Write>(node: ExprNode<UnaryId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let expr_id = node.expr_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{expr_id:?}\" [label=\"{:?}\"]", data.operator)?;
    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", data.operand.global_id())?;

    expression(data.operand, ast, writer)?;

    Ok(())
}

fn call<W: Write>(node: ExprNode<CallId>, ast: &Ast, writer: &mut BufWriter<W>) -> Result<()> {
    let expr_id = node.expr_id;
    let data = &ast[node.inner];

    writeln!(writer, "\"{expr_id:?}\" [label=\"Call\"]")?;

    writeln!(writer, "\"{expr_id:?}\" -> \"{:?}\"", data.caller.global_id())?;
    expression(data.caller, ast, writer)?;

    writeln!(writer, "\"args_of_{expr_id:?}\" [label=\"Args\"]")?;
    writeln!(writer, "\"{expr_id:?}\" -> \"args_of_{expr_id:?}\"")?;

    for argument in data.arguments.iter().copied() {
        writeln!(writer, "\"args_of_{expr_id:?}\" -> \"{:?}\"", argument.global_id())?;
        expression(argument, ast, writer)?;
    }

    Ok(())
}
