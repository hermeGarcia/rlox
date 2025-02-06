mod error;
mod expression;
mod statement;
pub mod value_system;

pub use value_system::Value;

use rlox_ast::Ast;

type RuntimeResult<T> = Result<T, error::RuntimeError>;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeFailure;

pub fn eval(ast: &Ast) -> Result<Value, RuntimeFailure> {
    let mut result = Value::Nil;

    for stmt in ast.initial_block().iter().copied() {
        match statement::eval(stmt, ast) {
            Ok(value) => result = value,
            Err(error) => {
                rlox_errors::error(error);
                return Err(RuntimeFailure);
            }
        }
    }

    Ok(result)
}
