use rlox_ast::expr;
use rlox_ast::{Ast, AstElem, AstProperty, Expr};
use rlox_source::SourceMetadata;

use crate::error;
use crate::token_stream::{Token, TokenKind};
use crate::{Context, ParserResult};

pub fn parse(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    expression(ctxt, ast)
}

fn expression(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    assign(ctxt, ast)
}

fn assign_operator(ctxt: &Context) -> Option<()> {
    match ctxt.peek().kind {
        TokenKind::Equal => Some(()),
        _ => None,
    }
}

fn assign(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = equality(ctxt, ast)?;

    if match_and_consume(ctxt, assign_operator).is_some() {
        let assign_expr = expr::Assign {
            lhs: expr,
            rhs: equality(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(assign_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(assign_expr.rhs.global_id());

        expr = ast.add(assign_expr);

        ast.attach(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn equality_operator(ctxt: &Context) -> Option<expr::BinaryOperator> {
    match ctxt.peek().kind {
        TokenKind::EqualEqual => Some(expr::BinaryOperator::Equal),
        TokenKind::BangEqual => Some(expr::BinaryOperator::NotEqual),
        _ => None,
    }
}

fn equality(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = comparison(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, equality_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: comparison(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.attach(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
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

fn comparison(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = term(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, comparison_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: term(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.attach(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
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

fn term(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = factor(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, term_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: factor(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.attach(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
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

fn factor(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = unary(ctxt, ast)?;

    while let Some(operator) = match_and_consume(ctxt, factor_operator) {
        let binary_expr = expr::Binary {
            operator,
            lhs: expr,
            rhs: unary(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.attach(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
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

fn unary(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let first_token = ctxt.peek();

    let Some(operator) = match_and_consume(ctxt, unary_operator) else {
        return primary(ctxt, ast);
    };

    let unary_expr = expr::Unary {
        operator,
        operand: primary(ctxt, ast)?,
    };

    let operand_metadata = *ast.get(unary_expr.operand.global_id());
    let unary = ast.add(unary_expr);

    ast.attach(unary.global_id(), SourceMetadata {
        start: first_token.start,
        end: operand_metadata.end,
        source: ctxt.src_id,
    });

    Ok(unary)
}

fn primary(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let token = ctxt.peek();
    let primary = match token.kind {
        TokenKind::False => Ok(ast.add(false)),

        TokenKind::True => Ok(ast.add(true)),

        TokenKind::Nil => Ok(ast.add(expr::Nil)),

        TokenKind::Integer => Ok(ast.add(primitive_type::<u64>(ctxt, token)?)),

        TokenKind::Decimal => Ok(ast.add(primitive_type::<f64>(ctxt, token)?)),

        TokenKind::LeftParen => nested_expression(ctxt, ast),

        TokenKind::Identifier => {
            let identifier = ast.add(&ctxt.src[token.start..token.end]);
            Ok(ast.add(identifier))
        }

        _ => Err(Into::into(error::UnexpectedToken {
            start: token.start,
            end: token.end,
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
    ast.attach(primary.global_id(), SourceMetadata {
        start: token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(primary)
}

fn nested_expression(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    ctxt.consume();

    let inner = expression(ctxt, ast)?;

    if !matches!(ctxt.peek().kind, TokenKind::RightParen) {
        return Err(Into::into(error::UnexpectedToken {
            start: ctxt.peek().start,
            end: ctxt.peek().end,
            source: ctxt.src_id,
            expected: vec![TokenKind::RightParen],
        }));
    };

    Ok(inner)
}

pub fn primitive_type<T: std::str::FromStr>(ctxt: &Context, token: Token) -> ParserResult<T> {
    String::from_utf8_lossy(&ctxt.src[token.start..token.end]).parse().map_err(|_| {
        Into::into(error::TypeCouldNotBeParsed {
            start: token.start,
            end: token.end,
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
    use super::*;

    use rlox_ast::debug_utils::fmt_expr;
    use rlox_ast::expr::*;
    use rlox_source::Source;
    use test_case::test_case;

    #[test_case(b"var123", "var123")]
    #[test_case(b"my_var", "my_var")]
    #[test_case(b"var_a = 3", &format!("Assign(var_a, {:?})", ExprKind::Natural(3)))]
    #[test_case(b"-12 + (2)", &format!("{:?}({:?}({:?}), {:?})", BinaryOperator::Plus, UnaryOperator::Minus, ExprKind::Natural(12), ExprKind::Natural(2)))]
    #[test_case(b"-2", &format!("{:?}({:?})", UnaryOperator::Minus, ExprKind::Natural(2)))]
    #[test_case(b"12 * 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Multiply, ExprKind::Natural(12), ExprKind::Natural(3)))]
    #[test_case(b"12 + 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Plus, ExprKind::Natural(12), ExprKind::Natural(3)))]
    fn composed_parsing(source: &[u8], expected: &str) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(&fmt_expr(expr, &ast), expected);

        let metadata: &SourceMetadata = ast.get(expr.global_id());
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }

    #[test_case(b"12.34", ExprKind::Decimal(12.34))]
    #[test_case(b"12", ExprKind::Natural(12))]
    #[test_case(b"nil", ExprKind::Nil)]
    #[test_case(b"true", ExprKind::Boolean(true))]
    #[test_case(b"false", ExprKind::Boolean(false))]
    fn primary_parsing(source: &[u8], expected: ExprKind) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(format!("{expected:?}"), format!("{:?}", expr.kind()));

        let metadata: &SourceMetadata = ast.get(expr.global_id());
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }
}
