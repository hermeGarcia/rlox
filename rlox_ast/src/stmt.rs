use crate::ExprId;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(ExprId),
}
