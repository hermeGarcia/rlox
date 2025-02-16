mod error;
mod expression;
mod statement;
pub mod value_system;

pub use value_system::Value;

use rlox_ast::Ast;

type RuntimeResult<T> = Result<T, error::RuntimeError>;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeFailure;

#[derive(Debug, Clone, Copy)]
pub struct EvalReport;

struct EvalCtxt;

pub fn eval(ast: &Ast) -> Result<EvalReport, RuntimeFailure> {
    let mut ctxt = EvalCtxt;

    for stmt in ast.initial_block().iter().copied() {
        if let Err(error) = statement::eval(stmt, ast, &mut ctxt) {
            rlox_errors::error(error);
            return Err(RuntimeFailure);
        }
    }

    Ok(EvalReport)
}
