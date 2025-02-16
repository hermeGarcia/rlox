use crate::ExprId;

#[derive(Clone, Debug)]
pub enum Stmt {
    Print(Print),
    Expr(ExprId),
}

#[derive(Clone, Debug)]
pub struct Print {
    pub expr: ExprId,
}
