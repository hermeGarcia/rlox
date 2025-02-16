use crate::ExprId;

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Natural(u64),
    Decimal(f64),
    Boolean(bool),
    Nil,
}

#[derive(Clone, Debug, Copy)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub lhs: ExprId,
    pub rhs: ExprId,
}

#[derive(Clone, Copy, Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub operand: ExprId,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Plus,
    Minus,
    Multiply,
    Division,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Minus,
    Negation,
}
