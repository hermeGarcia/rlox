use rlox_ast::expr::*;
use rlox_ast::{Ast, AstProperty, StrId};

use crate::RuntimeResult;
use crate::error;
use crate::native_functions::NativeFnContext;
use crate::runtime::Runtime;
use crate::value_system::{self, Value, VsResult};

pub fn deref_expression(expr: Expr, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    match expression(expr, ast, runtime)? {
        Value::Addr(address) => Ok(runtime.deref(address).clone()),
        other => Ok(other),
    }
}

pub fn expression(expr: Expr, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    match expr.kind() {
        ExprKind::Nil => Ok(Value::Nil),
        ExprKind::Boolean(inner) => Ok(Value::Boolean(inner)),
        ExprKind::Decimal(inner) => Ok(Value::Decimal(inner)),
        ExprKind::Natural(inner) => Ok(Value::Natural(inner)),
        ExprKind::String(inner) => Ok(Value::String(ast[inner].into())),

        ExprKind::Binary(inner) => binary(expr_node!(expr, inner), ast, runtime),
        ExprKind::Unary(inner) => unary(expr_node!(expr, inner), ast, runtime),
        ExprKind::Identifier(inner) => identifier(expr_node!(expr, inner), ast, runtime),
        ExprKind::Assign(inner) => assign(expr_node!(expr, inner), ast, runtime),
        ExprKind::Call(inner) => call(expr_node!(expr, inner), ast, runtime),
    }
}

fn assign(node: ExprNode<AssignId>, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    let assign = &ast[node.inner];

    let Value::Addr(address) = expression(assign.lhs, ast, runtime)? else {
        let metadata = *ast.get(node.expr_id);

        return Err(From::from(error::InvalidAssign {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
        }));
    };

    let new_value = deref_expression(assign.rhs, ast, runtime)?;

    runtime.memory[address] = new_value;

    Ok(Value::Nil)
}

fn identifier(node: ExprNode<StrId>, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    let Some(value) = runtime.address(&ast[node.inner]) else {
        let metadata = ast.get(node.expr_id);

        return Err(From::from(error::VarNotFound {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
        }));
    };

    Ok(Value::Addr(value))
}

fn binary(node: ExprNode<BinaryId>, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    let binary = &ast[node.inner];

    let lhs = deref_expression(binary.lhs, ast, runtime)?;

    // Applying lazy evaluation if possible
    match (binary.operator, &lhs) {
        (BinaryOperator::LogicAnd, Value::Boolean(false)) => return Ok(Value::Boolean(false)),
        (BinaryOperator::LogicOr, Value::Boolean(true)) => return Ok(Value::Boolean(true)),
        _ => (),
    }

    let rhs = deref_expression(binary.rhs, ast, runtime)?;

    let Ok(result) = apply_binary_operator(binary.operator, lhs, rhs) else {
        let metadata = *ast.get(node.expr_id);
        return Err(From::from(error::OperationNotDefined {
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
        BinaryOperator::LogicAnd => value_system::and(lhs, rhs),
        BinaryOperator::LogicOr => value_system::or(lhs, rhs),
    }
}

fn apply_unary_operator(operator: UnaryOperator, operand: Value) -> VsResult<Value> {
    match operator {
        UnaryOperator::Negation => value_system::not(operand),
        UnaryOperator::Minus => value_system::neg(operand),
    }
}

fn unary(node: ExprNode<UnaryId>, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    let unary = &ast[node.inner];

    let operand = deref_expression(unary.operand, ast, runtime)?;

    let Ok(result) = apply_unary_operator(unary.operator, operand) else {
        let metadata = *ast.get(node.expr_id);

        return Err(From::from(error::OperationNotDefined {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
        }));
    };

    Ok(result)
}

fn call(node: ExprNode<CallId>, ast: &Ast, runtime: &mut Runtime) -> RuntimeResult<Value> {
    let call = &ast[node.inner];

    let Value::Fn(callee) = deref_expression(call.caller, ast, runtime)? else {
        let metadata = ast.get(node.expr_id);

        return Err(From::from(error::UnexpectedValue {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
            expected: "Fn".to_string(),
        }));
    };

    let mut context = NativeFnContext {
        ast,
        args: Vec::with_capacity(call.arguments.len()),
        caller: node.expr_id,
    };

    for arg in call.arguments.iter().copied() {
        context.args.push(deref_expression(arg, ast, runtime)?);
    }

    (callee.function)(context, runtime)
}
