use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(usize);

#[derive(Clone, Debug, Default)]
pub struct ExprVec<T> {
    inner: Vec<T>,
}

impl<T> Index<ExprId> for ExprVec<T> {
    type Output = T;

    fn index(&self, index: ExprId) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl<T> IndexMut<ExprId> for ExprVec<T> {
    fn index_mut(&mut self, index: ExprId) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}

impl<T> ExprVec<T> {
    pub fn push(&mut self, elem: T) -> ExprId {
        let fresh_id = self.len();
        self.inner.push(elem);

        ExprId(fresh_id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
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
