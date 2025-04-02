use rlox_ast::stmt;
use rlox_ast::{Ast, AstElem, AstProperty, Stmt};
use rlox_source::SourceMetadata;

use crate::error;
use crate::expression;
use crate::token_stream::TokenKind;
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    stmt(ctxt, ast)
}

fn stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    match ctxt.peek().kind {
        TokenKind::Var => var_stmt(ctxt, ast),
        TokenKind::Print => print_stmt(ctxt, ast),
        TokenKind::LeftBrace => block_stmt(ctxt, ast),
        TokenKind::If => if_else_stmt(ctxt, ast),
        TokenKind::While => while_stmt(ctxt, ast),
        _ => expr_stmt(ctxt, ast),
    }
}

fn while_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    let condition = expression::parse(ctxt, ast)?;
    let body = block_stmt(ctxt, ast)?;

    let stmt = ast.add(stmt::While {
        condition,
        body,
    });

    ast.attach(stmt.global_id(), SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt)
}

fn else_branch(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Option<Stmt>> {
    if !ctxt.consume_if(TokenKind::Else) {
        return Ok(None);
    }

    match ctxt.peek().kind {
        TokenKind::If => if_else_stmt(ctxt, ast).map(Some),
        TokenKind::LeftBrace => block_stmt(ctxt, ast).map(Some),

        _ => Err(From::from(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::If, TokenKind::LeftBrace],
        })),
    }
}

fn if_else_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    let condition = expression::parse(ctxt, ast)?;
    let if_branch = block_stmt(ctxt, ast)?;
    let else_branch = else_branch(ctxt, ast)?;

    let stmt = ast.add(stmt::IfElse {
        condition,
        if_branch,
        else_branch,
    });

    ast.attach(stmt.global_id(), SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt)
}

fn var_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    let identifier = match ctxt.peek().kind {
        TokenKind::Identifier => {
            let token = ctxt.consume();
            Ok(ast.add(&ctxt.src[token.start..token.end]))
        }

        _ => Err(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Identifier],
        }),
    }?;

    let value = match ctxt.peek().kind {
        TokenKind::Equal => {
            ctxt.consume();
            Some(expression::parse(ctxt, ast)?)
        }
        _ => None,
    };

    if !ctxt.consume_if(TokenKind::Semicolon) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Semicolon],
        }));
    }

    let stmt = ast.add(stmt::Declaration {
        identifier,
        value,
    });

    ast.attach(stmt.global_id(), SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt)
}

fn print_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    let print_stmt = stmt::Print {
        expr: expression::parse(ctxt, ast)?,
    };

    if !ctxt.consume_if(TokenKind::Semicolon) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Semicolon],
        }));
    }

    let stmt = ast.add(print_stmt);

    ast.attach(stmt.global_id(), SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt)
}

fn expr_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let expr = expression::parse(ctxt, ast)?;

    if !ctxt.consume_if(TokenKind::Semicolon) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::Semicolon],
        }));
    }

    let stmt = ast.add(expr);

    ast.attach(stmt.global_id(), SourceMetadata {
        end: ctxt.peek().start,
        ..*ast.get(expr.global_id())
    });

    Ok(stmt)
}

fn block_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    let mut block_stmts = vec![];

    while !matches!(ctxt.peek().kind, TokenKind::Eof | TokenKind::RightBrace) {
        let block_stmt = stmt(ctxt, ast)?;
        block_stmts.push(block_stmt);
    }

    if !ctxt.consume_if(TokenKind::RightBrace) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::RightBrace],
        }));
    }

    let stmt = ast.add(block_stmts.as_slice());

    ast.attach(stmt.global_id(), SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rlox_ast::debug_utils;
    use rlox_ast::expr::*;
    use rlox_source::Source;
    use test_case::test_case;

    #[rustfmt::skip]
    #[test_case(b"while true { 1 + 1; }", "While(Boolean(true),Block([\"Plus(Natural(1), Natural(1))\"]))")]
    #[test_case(b"if false { true; } else { false; }", "IfElse(Boolean(false),Block([\"Boolean(true)\"]),Block([\"Boolean(false)\"]))")]
    #[test_case(b"if false { true; }", "IfElse(Boolean(false),Block([\"Boolean(true)\"]),None)")]
    #[test_case(b"var a;", &format!("Declaration(a, None)"))]
    #[test_case(b"var a = 2;", &format!("Declaration(a, {:?})", ExprKind::Natural(2)))]
    #[test_case(b"print -12 + (2);", &format!("Print({:?}({:?}({:?}), {:?}))", BinaryOperator::Plus, UnaryOperator::Minus, ExprKind::Natural(12), ExprKind::Natural(2)))]
    #[test_case(b"print -2;", &format!("Print({:?}({:?}))", UnaryOperator::Minus, ExprKind::Natural(2)))]
    #[test_case(b"print 12 * 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Multiply, ExprKind::Natural(12), ExprKind::Natural(3)))]
    #[test_case(b"print 12 + 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Plus, ExprKind::Natural(12), ExprKind::Natural(3)))]
    fn stmt_parsing(source: &[u8], expected: &str) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let stmt = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(&debug_utils::fmt_stmt(stmt, &ast), expected);

        let metadata: &SourceMetadata = ast.get(stmt.global_id());
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }
}
