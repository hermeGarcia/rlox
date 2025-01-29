use rlox_ast::expr;
use rlox_ast::{Ast, AstElem, AstProperty, Expr, ExprId};
use rlox_source::SourceMetadata;

use crate::error;
use crate::token_stream::{Token, TokenKind};
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    expression(ctxt, ast)
}

fn expression(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    equality(ctxt, ast)
}

fn equality_operator(ctxt: &Context) -> Option<expr::BinaryOperator> {
    match ctxt.peek().kind {
        TokenKind::EqualEqual => Some(expr::BinaryOperator::Equal),
        TokenKind::BangEqual => Some(expr::BinaryOperator::NotEqual),
        _ => None,
    }
}

fn equality(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let mut expr = comparison(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, equality_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: comparison(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs);
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs);

        expr = ast.add(Expr::BinaryExpr(binary_expr));

        ast.attach(expr, SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            line_start: lhs_metadata.line_start,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn comparison_operator(ctxt: &Context) -> Option<expr::BinaryOperator> {
    match ctxt.peek().kind {
        TokenKind::Greater => Some(expr::BinaryOperator::Greater),
        TokenKind::Less => Some(expr::BinaryOperator::Less),
        TokenKind::GreaterEqual => Some(expr::BinaryOperator::GreaterOrEqual),
        TokenKind::LessEqual => Some(expr::BinaryOperator::LessOrEqual),
        _ => None,
    }
}

fn comparison(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let mut expr = term(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, comparison_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: term(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs);
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs);

        expr = ast.add(Expr::BinaryExpr(binary_expr));

        ast.attach(expr, SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            line_start: lhs_metadata.line_start,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn term_operator(ctxt: &Context) -> Option<expr::BinaryOperator> {
    match ctxt.peek().kind {
        TokenKind::Plus => Some(expr::BinaryOperator::Plus),
        TokenKind::Minus => Some(expr::BinaryOperator::Minus),
        _ => None,
    }
}

fn term(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let mut expr = factor(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, term_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: factor(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs);
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs);

        expr = ast.add(Expr::BinaryExpr(binary_expr));

        ast.attach(expr, SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            line_start: lhs_metadata.line_start,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn factor_operator(ctxt: &Context) -> Option<expr::BinaryOperator> {
    match ctxt.peek().kind {
        TokenKind::Slash => Some(expr::BinaryOperator::Division),
        TokenKind::Star => Some(expr::BinaryOperator::Multiply),
        _ => None,
    }
}

fn factor(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let mut expr = unary(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, factor_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: unary(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs);
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs);

        expr = ast.add(Expr::BinaryExpr(binary_expr));

        ast.attach(expr, SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            line_start: lhs_metadata.line_start,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn unary_operator(ctxt: &Context) -> Option<expr::UnaryOperator> {
    match ctxt.peek().kind {
        TokenKind::Bang => Some(expr::UnaryOperator::Negation),
        TokenKind::Minus => Some(expr::UnaryOperator::Minus),
        _ => None,
    }
}

fn unary(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let first_token = ctxt.peek();

    let Some(operator) = match_and_consume(ctxt, unary_operator) else {
        return primary(ctxt, ast);
    };

    let unary_expr = expr::Unary {
        operator,
        operand: primary(ctxt, ast)?,
    };

    let operand_metadata = *ast.get(unary_expr.operand);
    let unary = ast.add(Expr::UnaryExpr(unary_expr));

    ast.attach(unary, SourceMetadata {
        start: first_token.start,
        end: operand_metadata.end,
        line_start: first_token.line,
        source: ctxt.src_id,
    });

    Ok(unary)
}

fn primary(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    let token = ctxt.peek();
    let primary_id = match token.kind {
        TokenKind::False => Ok(ast.add(Expr::Boolean(false))),
        TokenKind::True => Ok(ast.add(Expr::Boolean(true))),
        TokenKind::Nil => Ok(ast.add(Expr::Nil)),
        TokenKind::Integer => Ok(ast.add(Expr::Natural(primitive_type(ctxt, token)?))),
        TokenKind::Decimal => Ok(ast.add(Expr::Decimal(primitive_type(ctxt, token)?))),
        TokenKind::LeftParen => nested_expression(ctxt, ast),

        _ => Err(Into::into(error::UnexpectedToken {
            start: token.start,
            end: token.end,
            line: token.line,
            source: ctxt.src_id,
            expected: vec![
                TokenKind::False,
                TokenKind::True,
                TokenKind::Nil,
                TokenKind::Integer,
                TokenKind::Decimal,
                TokenKind::LeftParen,
            ],
        })),
    }?;

    ctxt.consume();
    ast.attach(primary_id, SourceMetadata {
        start: token.start,
        end: ctxt.peek().start,
        line_start: token.line,
        source: ctxt.src_id,
    });

    Ok(primary_id)
}

fn nested_expression(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<ExprId> {
    ctxt.consume();

    let inner = expression(ctxt, ast)?;

    if !matches!(ctxt.peek().kind, TokenKind::RightParen) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            line: ctxt.peek().line,
            source: ctxt.src_id,
            expected: vec![TokenKind::RightParen],
        }));
    };

    Ok(inner)
}

fn primitive_type<T: std::str::FromStr>(ctxt: &Context, token: Token) -> ParserResult<T> {
    String::from_utf8_lossy(&ctxt.src[token.start..token.end]).parse().map_err(|_| {
        Into::into(error::TypeCouldNotBeParsed {
            start: token.start,
            end: token.end,
            line: token.line,
            source: ctxt.src_id,
        })
    })
}

fn match_and_consume<MatcherFn, T>(ctxt: &mut Context, matcher: MatcherFn) -> Option<T>
where
    MatcherFn: Fn(&Context) -> Option<T>,
{
    let match_result = matcher(ctxt);

    if match_result.is_some() {
        ctxt.consume();
    }

    match_result
}

#[cfg(test)]
mod tests {
    use expr::*;
    use rlox_source::Source;
    use test_case::test_case;

    use super::*;

    fn debug_fmt(expr_id: ExprId, ast: &Ast) -> String {
        match &ast[expr_id] {
            Expr::BinaryExpr(binary) => {
                let lhs = debug_fmt(binary.lhs, ast);
                let rhs = debug_fmt(binary.rhs, ast);
                format!("{:?}({lhs}, {rhs})", binary.operator)
            }
            Expr::UnaryExpr(unary) => {
                let operand = debug_fmt(unary.operand, ast);
                format!("{:?}({operand})", unary.operator)
            }

            other => format!("{other:?}"),
        }
    }

    #[test_case(b"-12 + (2)", &format!("{:?}({:?}({:?}), {:?})", BinaryOperator::Plus, UnaryOperator::Minus, Expr::Natural(12), Expr::Natural(2)))]
    #[test_case(b"-2", &format!("{:?}({:?})", UnaryOperator::Minus, Expr::Natural(2)))]
    #[test_case(b"12 * 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Multiply, Expr::Natural(12), Expr::Natural(3)))]
    #[test_case(b"12 + 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Plus, Expr::Natural(12), Expr::Natural(3)))]
    fn composed_parsing(source: &[u8], expected: &str) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr_id = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(&debug_fmt(expr_id, &ast), expected);

        let metadata: &SourceMetadata = ast.get(expr_id);
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(metadata.line_start, 0);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }

    #[test_case(b"12.34", Expr::Decimal(12.34))]
    #[test_case(b"12", Expr::Natural(12))]
    #[test_case(b"nil", Expr::Nil)]
    #[test_case(b"true", Expr::Boolean(true))]
    #[test_case(b"false", Expr::Boolean(false))]
    fn primary_parsing(source: &[u8], expected: Expr) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr_id = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(format!("{expected:?}"), format!("{:?}", ast[expr_id]));

        let metadata: &SourceMetadata = ast.get(expr_id);
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(metadata.line_start, 0);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }
}
