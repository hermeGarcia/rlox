pub mod value_system;

mod error;
mod expression;
mod native_functions;
mod runtime;
mod statement;

pub use value_system::Value;

use rlox_ast::Ast;
use runtime::Runtime;

type RuntimeResult<T> = Result<T, error::RuntimeError>;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeFailure;

#[derive(Debug, Clone, Copy)]
pub struct EvalReport;

pub fn eval(ast: &Ast) -> Result<EvalReport, RuntimeFailure> {
    let mut runtime = Runtime::new();

    for stmt in ast.main().iter().copied() {
        if let Err(error) = statement::eval(stmt, ast, &mut runtime) {
            rlox_errors::error(error);
            return Err(RuntimeFailure);
        }
    }

    Ok(EvalReport)
}
