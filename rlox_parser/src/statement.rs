use rlox_ast::stmt;
use rlox_ast::stmt::Stmt;
use rlox_ast::{Ast, AstElem, AstProperty};
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
        TokenKind::For => for_stmt(ctxt, ast),
        _ => expr_stmt(ctxt, ast),
    }
}

fn for_stmt(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Stmt> {
    let start_token = ctxt.consume();

    ctxt.try_consume(TokenKind::LeftParen)?;

    let declaration = if ctxt.match_consume(TokenKind::Semicolon) {
        None
    } else {
        Some(var_stmt(ctxt, ast)?)
    };

    let condition = if matches!(ctxt.peek().kind, TokenKind::Semicolon) {
        let token = ctxt.consume();
        let condition = ast.add(true);

        ast.attach(condition.global_id(), SourceMetadata {
            start: token.start,
            end: token.end,
            source: ctxt.src_id,
        });

        condition
    } else {
        let condition = expression::parse(ctxt, ast)?;
        ctxt.try_consume(TokenKind::Semicolon)?;

        condition
    };

    let increment = if ctxt.match_consume(TokenKind::RightParen) {
        None
    } else {
        let expr = expression::parse(ctxt, ast)?;
        let increment = ast.add(expr);
        ast.attach(increment.global_id(), *ast.get(expr.global_id()));
        ctxt.try_consume(TokenKind::RightParen)?;

        Some(increment)
    };

    let body = match increment {
        None => block_stmt(ctxt, ast)?,

        Some(increment) => {
            let inner_body = block_stmt(ctxt, ast)?;
            let outer_body = ast.add([inner_body, increment].as_slice());
            ast.attach(outer_body.global_id(), *ast.get(inner_body.global_id()));
            outer_body
        }
    };

    let metadata = SourceMetadata {
        start: start_token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    };

    let while_stmt = ast.add(stmt::While {
        condition,
        body,
    });

    ast.attach(while_stmt.global_id(), metadata);

    let Some(declaration) = declaration else {
        return Ok(while_stmt);
    };

    let full_loop = ast.add([declaration, while_stmt].as_slice());
    ast.attach(full_loop.global_id(), metadata);

    Ok(full_loop)
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
    if !ctxt.match_consume(TokenKind::Else) {
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

    let token = ctxt.try_consume(TokenKind::Identifier)?;
    let identifier = ast.add(&ctxt.src[token.start..token.end]);

    let value = if ctxt.match_consume(TokenKind::Equal) {
        Some(expression::parse(ctxt, ast)?)
    } else {
        None
    };

    ctxt.try_consume(TokenKind::Semicolon)?;

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

    ctxt.try_consume(TokenKind::Semicolon)?;

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

    ctxt.try_consume(TokenKind::Semicolon)?;

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

    ctxt.try_consume(TokenKind::RightBrace)?;

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
    #[test_case(b"for(;;){ 12; }", "While(Boolean(true),Block([\"Natural(12)\"]))"; "empty for")]
    #[test_case(b"for(; x < 10;){ 12; }", "While(Less(x, Natural(10)),Block([\"Natural(12)\"]))"; "for only condition")]
    #[test_case(b"for(; x < 10; x = x + 1){ 12; }", "While(Less(x, Natural(10)),Block([\"Block([\\\"Natural(12)\\\"])\", \"Assign(x, Plus(x, Natural(1)))\"]))"; "for no declaration")]
    #[test_case(b"for(var x = 0; x < 10; x = x + 1){ 12; }", "Block([\"Declaration(x, Natural(0))\", \"While(Less(x, Natural(10)),Block([\\\"Block([\\\\\\\"Natural(12)\\\\\\\"])\\\", \\\"Assign(x, Plus(x, Natural(1)))\\\"]))\"])"; "full for")]
    #[test_case(b"while true { 1 + 1; }", "While(Boolean(true),Block([\"Plus(Natural(1), Natural(1))\"]))"; "while expression")]
    #[test_case(b"if false { true; } else { false; }", "IfElse(Boolean(false),Block([\"Boolean(true)\"]),Block([\"Boolean(false)\"]))"; "if with else")]
    #[test_case(b"if false { true; }", "IfElse(Boolean(false),Block([\"Boolean(true)\"]),None)"; "simple if")]
    #[test_case(b"var a;", &format!("Declaration(a, None)"); "var declaration without assignment")]
    #[test_case(b"var a = 2;", &format!("Declaration(a, {:?})", ExprKind::Natural(2)); "var declaration with assignment")]
    #[test_case(b"print -12 + (2);", &format!("Print({:?}({:?}({:?}), {:?}))", BinaryOperator::Plus, UnaryOperator::Minus, ExprKind::Natural(12), ExprKind::Natural(2)); "print complex arith expression")]
    #[test_case(b"print -2;", &format!("Print({:?}({:?}))", UnaryOperator::Minus, ExprKind::Natural(2)); "print neg expression")]
    #[test_case(b"print 12 * 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Multiply, ExprKind::Natural(12), ExprKind::Natural(3)); "print mul expression")]
    #[test_case(b"print 12 + 3;", &format!("Print({:?}({:?}, {:?}))", BinaryOperator::Plus, ExprKind::Natural(12), ExprKind::Natural(3)); "print add expression")]
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
