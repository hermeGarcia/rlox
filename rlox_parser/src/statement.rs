use rlox_ast::{Ast, AstElem, AstProperty, Stmt, StmtId};
use rlox_source::SourceMetadata;

use crate::error;
use crate::expression;
use crate::token_stream::TokenKind;
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    stmt(ctxt, ast)
}

pub fn stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    stmt_expr(ctxt, ast)
}

pub fn stmt_expr(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    let expr_id = expression::parse(ctxt, ast)?;

    if !ctxt.consume_if(TokenKind::Semicolon) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Semicolon],
        }));
    }

    let stmt_id = ast.add(Stmt::Expr(expr_id));

    ast.attach(stmt_id, SourceMetadata {
        end: ctxt.peek().start,
        ..*ast.get(expr_id)
    });

    Ok(stmt_id)
}
