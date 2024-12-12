pub mod src_library;

pub use src_library::FileLibrary;

use rlox_ast::{ExprVec, Expression};

pub struct Context {
    pub file_library: FileLibrary,
    pub expr_arena: ExprVec<Expression>,
}
