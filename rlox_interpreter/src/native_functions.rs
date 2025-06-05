use std::fs;

use rlox_ast::expr::ExprId;
use rlox_ast::{Ast, AstProperty};
use rlox_errors::compiler_log;

use crate::RuntimeResult;
use crate::error;
use crate::runtime::Runtime;
use crate::value_system::Value;

#[rustfmt::skip]
pub const REGISTRY: &[NativeFn] = &[
     READ_FILE,
     PRINTLN
];

#[derive(Copy, Clone, Debug)]
pub struct NativeFn {
    pub name: &'static str,
    pub function: fn(NativeFnContext, &Runtime) -> RuntimeResult<Value>,
}

pub struct NativeFnContext<'a> {
    pub args: Vec<Value>,
    pub caller: ExprId,
    pub ast: &'a Ast,
}

const READ_FILE: NativeFn = NativeFn {
    name: "read_file",
    function: read_file_to_string,
};

pub fn read_file_to_string(context: NativeFnContext, _runtime: &Runtime) -> RuntimeResult<Value> {
    if context.args.len() != 1 {
        let metadata = context.ast.get(context.caller);

        return Err(From::from(error::WrongNumberOfArgs {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
            got: context.args.len(),
            expect: 1,
        }));
    }

    let Value::String(file_path) = &context.args[0] else {
        let metadata = context.ast.get(context.caller);

        return Err(From::from(error::UnexpectedValue {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
            expected: "String".to_string(),
        }));
    };

    let fs_result = fs::read_to_string(file_path);

    compiler_log!("{fs_result:?}");

    Ok(fs_result.map_or(Value::Nil, Value::String))
}

const PRINTLN: NativeFn = NativeFn {
    name: "println",
    function: println,
};

pub fn println(context: NativeFnContext, _runtime: &Runtime) -> RuntimeResult<Value> {
    if context.args.len() != 1 {
        let metadata = context.ast.get(context.caller);

        return Err(From::from(error::WrongNumberOfArgs {
            start: metadata.start,
            end: metadata.end,
            source: metadata.source,
            got: context.args.len(),
            expect: 1,
        }));
    }

    println!("{}", context.args[0]);

    Ok(Value::Nil)
}
