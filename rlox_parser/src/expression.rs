use rlox_ast::expr;
use rlox_ast::expr::BinaryOperator;
use rlox_ast::expr::Expr;
use rlox_ast::{Ast, AstElem, Identifier, StrId};
use rlox_infra::StructVec;
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
    let mut expr = logic_or(ctxt, ast)?;

    if match_and_consume(ctxt, assign_operator).is_some() {
        let assign_expr = expr::Assign {
            lhs: expr,
            rhs: logic_or(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(assign_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(assign_expr.rhs.global_id());

        expr = ast.add(assign_expr);

        ast.assign(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn logic_or(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = logic_and(ctxt, ast)?;

    if ctxt.match_consume(TokenKind::Or) {
        let binary_expr = expr::Binary {
            lhs: expr,
            rhs: logic_and(ctxt, ast)?,
            operator: BinaryOperator::LogicOr,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.assign(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: rhs_metadata.end,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
}

fn logic_and(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = equality(ctxt, ast)?;

    if ctxt.match_consume(TokenKind::And) {
        let binary_expr = expr::Binary {
            lhs: expr,
            rhs: equality(ctxt, ast)?,
            operator: BinaryOperator::LogicAnd,
        };

        let lhs_metadata: SourceMetadata = *ast.get(binary_expr.lhs.global_id());
        let rhs_metadata: SourceMetadata = *ast.get(binary_expr.rhs.global_id());

        expr = ast.add(binary_expr);

        ast.assign(expr.global_id(), SourceMetadata {
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

        ast.assign(expr.global_id(), SourceMetadata {
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

        ast.assign(expr.global_id(), SourceMetadata {
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

        ast.assign(expr.global_id(), SourceMetadata {
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
        TokenKind::Modulus => Some(expr::BinaryOperator::Modulus),
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

        ast.assign(expr.global_id(), SourceMetadata {
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
        return call(ctxt, ast);
    };

    let unary_expr = expr::Unary {
        operator,
        operand: call(ctxt, ast)?,
    };

    let operand_metadata = *ast.get(unary_expr.operand.global_id());
    let unary = ast.add(unary_expr);

    ast.assign(unary.global_id(), SourceMetadata {
        start: first_token.start,
        end: operand_metadata.end,
        source: ctxt.src_id,
    });

    Ok(unary)
}

fn call_arguments(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Vec<Expr>> {
    if matches!(ctxt.peek().kind, TokenKind::RightParen) {
        return Ok(Vec::with_capacity(0));
    }

    let mut args = vec![expression(ctxt, ast)?];

    while ctxt.match_consume(TokenKind::Comma) {
        args.push(expression(ctxt, ast)?);
    }

    Ok(args)
}

fn call(ctxt: &mut Context, ast: &mut Ast) -> ParserResult<Expr> {
    let mut expr = primary(ctxt, ast)?;

    while ctxt.match_consume(TokenKind::LeftParen) {
        let call_expr = expr::Call {
            lhs: expr,
            arguments: call_arguments(ctxt, ast)?,
        };

        let lhs_metadata: SourceMetadata = *ast.get(call_expr.lhs.global_id());
        let end_of_call = ctxt.try_consume(TokenKind::RightParen)?;

        expr = ast.add(call_expr);

        ast.assign(expr.global_id(), SourceMetadata {
            start: lhs_metadata.start,
            end: end_of_call.end,
            source: ctxt.src_id,
        });
    }

    Ok(expr)
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

        TokenKind::String => lox_str(ctxt, ast),

        TokenKind::Identifier => {
            let identifier: Identifier = ast.add(&ctxt.src[token.start..token.end]);
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
    ast.assign(primary.global_id(), SourceMetadata {
        start: token.start,
        end: ctxt.peek().start,
        source: ctxt.src_id,
    });

    Ok(primary)
}

fn lox_str(ctxt: &Context, ast: &mut Ast) -> ParserResult<Expr> {
    let token = ctxt.peek();

    // Double quotes do not need to be stored
    let str_start = token.start + 1;
    let str_end = token.end - 1;

    let lox_str: StrId = ast.add(&ctxt.src[str_start..str_end]);

    Ok(ast.add(lox_str))
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
    String::from_utf8_lossy(&ctxt.src[token.start..token.end])
        .parse()
        .map_err(|_| {
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

    #[rustfmt::skip]
    #[test_case(b"\"a string\"", "a string"; "string literal")]
    #[test_case(b"var123", "var123"; "var name with numbers")]
    #[test_case(b"my_var", "my_var"; "var use underscore")]
    #[test_case(b"var_a = 3", &format!("Assign(var_a, {:?})", ExprKind::Natural(3)); "var assignment")]
    #[test_case(b"-12 + (2)", &format!("{:?}({:?}({:?}), {:?})", BinaryOperator::Plus, UnaryOperator::Minus, ExprKind::Natural(12), ExprKind::Natural(2)); "complex arith expression")]
    #[test_case(b"-2", &format!("{:?}({:?})", UnaryOperator::Minus, ExprKind::Natural(2)); "neg expression")]
    #[test_case(b"12 * 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Multiply, ExprKind::Natural(12), ExprKind::Natural(3)); "mul expression")]
    #[test_case(b"12 + 3", &format!("{:?}({:?}, {:?})", BinaryOperator::Plus, ExprKind::Natural(12), ExprKind::Natural(3)); "add expression")]
    #[test_case(b"12 % 3", "Modulus(Natural(12), Natural(3))"; "modulus expression")]
    #[test_case(b"true and false", &format!("{:?}({:?}, {:?})", BinaryOperator::LogicAnd, ExprKind::Boolean(true), ExprKind::Boolean(false)); "simple boolean expression")]
    #[test_case(b"true or false and true", "LogicOr(Boolean(true), LogicAnd(Boolean(false), Boolean(true)))"; "nested boolean expression")]
    #[test_case(b"my_cool_function()", "Call(my_cool_function, [])"; "fn with no args")]
    #[test_case(b"function_with_args(12, 3, \"some string\")", "Call(function_with_args, [\"Natural(12)\", \"Natural(3)\", \"some string\"])"; "fn with several args")]
    #[test_case(b"my_cool_function(12)(3)", "Call(Call(my_cool_function, [\"Natural(12)\"]), [\"Natural(3)\"])"; "concat fn calls")]
    fn composed_parsing(source: &[u8], expected: &str) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr = parse(&mut ctxt, &mut ast).unwrap();

        assert_eq!(&fmt_expr(expr, &ast), expected);

        let metadata: &SourceMetadata = ast.get(expr.global_id());
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }

    #[test_case(b"12.34", ExprKind::Decimal(12.34); "parsing decimal")]
    #[test_case(b"12", ExprKind::Natural(12); "parsing natural")]
    #[test_case(b"nil", ExprKind::Nil; "parsing nil")]
    #[test_case(b"true", ExprKind::Boolean(true); "parsing true")]
    #[test_case(b"false", ExprKind::Boolean(false); "parsing false")]
    fn primary_parsing(source: &[u8], expected: ExprKind) {
        let mut ctxt = Context::new(Source::Prompt, source);
        let mut ast = Ast::default();
        let expr = parse(&mut ctxt, &mut ast).unwrap();

        rlox_errors::compiler_log!("HELLOOOO");

        assert_eq!(format!("{expected:?}"), format!("{:?}", expr.kind()));

        let metadata: &SourceMetadata = ast.get(expr.global_id());
        assert_eq!(metadata.source, Source::Prompt);
        assert_eq!(&source[metadata.start..metadata.end], source);
    }
}
