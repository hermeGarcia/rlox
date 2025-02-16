use rlox_ast::stmt;
use rlox_ast::{Ast, AstElem, AstProperty, Stmt, StmtId};
use rlox_source::SourceMetadata;

use crate::error;
use crate::expression;
use crate::token_stream::TokenKind;
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    stmt(ctxt, ast)
}

fn stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    match ctxt.peek().kind {
        TokenKind::Print => print_stmt(ctxt, ast),
        _ => expr_stmt(ctxt, ast),
    }
}

fn print_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
    let start_token = ctxt.consume();

    let stmt = Stmt::Print(stmt::Print {
        expr: expression::parse(ctxt, ast)?,
    });

    if !ctxt.consume_if(TokenKind::Semicolon) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Semicolon],
        }));
    }

    let stmt_id: StmtId = ast.add(stmt);

    ast.attach(stmt_id, SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt_id)
}

fn expr_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<StmtId> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use rlox_ast::debug_utils;
    use rlox_ast::expr::*;
    use rlox_source::Source;
    use test_case::test_case;

    #[test_case(b"print -12 + (2);", &format!("Print({:?}({:?}({:?}), {:?}))", BinaryOperator::Plus, UnaryOperator::Minus, Expr::Natural(12), Expr::Natural(2)))]
    #[test_case(b"print -2;", &format!("Print({:?}({:?}))", UnaryOperator::Minus, Expr::Natural(2)))]
    #[test_case(b"print 12 * 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Multiply, Expr::Natural(12), Expr::Natural(3)))]
    #[test_case(b"print 12 + 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Plus, Expr::Natural(12), Expr::Natural(3)))]
    fn stmt_parsing(source: &[u8], expected: &str) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let stmt_id = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(&debug_utils::fmt_stmt(stmt_id, &ast), expected);

        let metadata: &SourceMetadata = ast.get(stmt_id);
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }
}
