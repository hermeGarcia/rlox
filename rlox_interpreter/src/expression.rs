use rlox_ast::expr::{Binary, BinaryOperator, Unary, UnaryOperator};
use rlox_ast::{Ast, AstProperty, Expr, ExprId, ExprWithId};

use crate::EvalCtxt;
use crate::RuntimeResult;
use crate::error::OperationNotDefined;
use crate::value_system::{self, Value, VsResult};

pub fn eval(expr: ExprId, ast: &Ast, ctxt: &mut EvalCtxt) -> RuntimeResult<Value> {
    expression(expr, ast, ctxt)
}

fn expression(expr: ExprId, ast: &Ast, ctxt: &mut EvalCtxt) -> RuntimeResult<Value> {
    match expr.kind {
        Expr::Binary(inner) => binary(ExprWithId::new(expr, &ast[inner]), ast, ctxt),
        Expr::Unary(inner) => unary(ExprWithId::new(expr, &ast[inner]), ast, ctxt),
        Expr::Nil => Ok(Value::Nil),
        Expr::Boolean(inner) => Ok(Value::Boolean(inner)),
        Expr::Decimal(inner) => Ok(Value::Decimal(inner)),
        Expr::Natural(inner) => Ok(Value::Natural(inner)),
        Expr::Assign(_) => todo!(),
        Expr::Identifier(_) => todo!(),
    }
}

fn binary(binary: ExprWithId<&Binary>, ast: &Ast, ctxt: &mut EvalCtxt) -> RuntimeResult<Value> {
    let lhs = expression(binary.data.lhs, ast, ctxt)?;
    let rhs = expression(binary.data.rhs, ast, ctxt)?;

    let Ok(result) = apply_binary_operator(binary.data.operator, lhs, rhs) else {
        let metadata = *ast.get(binary.my_id);
        return Err(From::from(OperationNotDefined {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
        }));
    };

    Ok(result)
}

fn apply_binary_operator(operator: BinaryOperator, lhs: Value, rhs: Value) -> VsResult<Value> {
    match operator {
        BinaryOperator::Plus => value_system::add(lhs, rhs),
        BinaryOperator::Minus => value_system::sub(lhs, rhs),
        BinaryOperator::Multiply => value_system::mul(lhs, rhs),
        BinaryOperator::Division => value_system::div(lhs, rhs),
        BinaryOperator::Equal => value_system::equal(lhs, rhs),
        BinaryOperator::NotEqual => value_system::not_equal(lhs, rhs),
        BinaryOperator::Less => value_system::less(lhs, rhs),
        BinaryOperator::LessOrEqual => value_system::less_or_equal(lhs, rhs),
        BinaryOperator::Greater => value_system::greater(lhs, rhs),
        BinaryOperator::GreaterOrEqual => value_system::greater_or_equal(lhs, rhs),
    }
}

fn apply_unary_operator(operator: UnaryOperator, operand: Value) -> VsResult<Value> {
    match operator {
        UnaryOperator::Negation => value_system::not(operand),
        UnaryOperator::Minus => value_system::neg(operand),
    }
}

fn unary(unary: ExprWithId<&Unary>, ast: &Ast, ctxt: &mut EvalCtxt) -> RuntimeResult<Value> {
    let operand = expression(unary.data.operand, ast, ctxt)?;

    let Ok(result) = apply_unary_operator(unary.data.operator, operand) else {
        let metadata = *ast.get(unary.my_id);
        return Err(From::from(OperationNotDefined {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
        }));
    };

    Ok(result)
}
