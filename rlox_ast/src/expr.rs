use std::ops::{Index, IndexMut};

use crate::{Ast, AstElem, AstIndex, AstVec, StrId, define_id};

#[derive(Clone, Copy, Debug)]
pub enum Expr {
    Assign(AssignId),
    Binary(BinaryId),
    Unary(UnaryId),
    Identifier(StrId),
    Natural(u64),
    Decimal(f64),
    Boolean(bool),
    Nil,
}

#[derive(Clone, Copy, Debug)]
pub struct Nil;

#[derive(Clone, Copy, Debug)]
pub struct ExprId {
    global_id: usize,
    pub kind: Expr,
}
impl Eq for ExprId {}

impl std::hash::Hash for ExprId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.global_id.hash(state);
    }
}

impl Ord for ExprId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.global_id.cmp(&other.global_id)
    }
}

impl PartialOrd for ExprId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExprId {
    fn eq(&self, other: &Self) -> bool {
        self.global_id == other.global_id
    }
}

impl AstIndex for ExprId {
    fn inner(&self) -> usize {
        self.global_id
    }
}

impl AstElem<StrId, ExprId> for Ast {
    fn add(&mut self, elem: StrId) -> ExprId {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Identifier(elem),
        }
    }
}

impl AstElem<u64, ExprId> for Ast {
    fn add(&mut self, elem: u64) -> ExprId {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Natural(elem),
        }
    }
}

impl AstElem<f64, ExprId> for Ast {
    fn add(&mut self, elem: f64) -> ExprId {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Decimal(elem),
        }
    }
}

impl AstElem<bool, ExprId> for Ast {
    fn add(&mut self, elem: bool) -> ExprId {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Boolean(elem),
        }
    }
}

impl AstElem<Nil, ExprId> for Ast {
    fn add(&mut self, _: Nil) -> ExprId {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Nil,
        }
    }
}

define_id!(AssignId);
pub(crate) type AssignVec = AstVec<Assign, AssignId>;

#[derive(Clone, Debug, Copy)]
pub struct Assign {
    pub lhs: ExprId,
    pub rhs: ExprId,
}

impl Index<AssignId> for Ast {
    type Output = Assign;

    fn index(&self, index: AssignId) -> &Self::Output {
        &self.assign_buffer[index]
    }
}

impl IndexMut<AssignId> for Ast {
    fn index_mut(&mut self, index: AssignId) -> &mut Self::Output {
        &mut self.assign_buffer[index]
    }
}

impl AstElem<Assign, ExprId> for Ast {
    fn add(&mut self, elem: Assign) -> ExprId {
        let global_id = self.expr_id;
        let kind = AssignId::new(self.assign_buffer.len());

        self.assign_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Assign(kind),
        }
    }
}

define_id!(BinaryId);
pub(crate) type BinaryVec = AstVec<Binary, BinaryId>;

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

#[derive(Clone, Debug, Copy)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub lhs: ExprId,
    pub rhs: ExprId,
}

impl Index<BinaryId> for Ast {
    type Output = Binary;

    fn index(&self, index: BinaryId) -> &Self::Output {
        &self.binary_buffer[index]
    }
}

impl IndexMut<BinaryId> for Ast {
    fn index_mut(&mut self, index: BinaryId) -> &mut Self::Output {
        &mut self.binary_buffer[index]
    }
}

impl AstElem<Binary, ExprId> for Ast {
    fn add(&mut self, elem: Binary) -> ExprId {
        let global_id = self.expr_id;
        let kind = BinaryId::new(self.binary_buffer.len());

        self.binary_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Binary(kind),
        }
    }
}

define_id!(UnaryId);
pub(crate) type UnaryVec = AstVec<Unary, UnaryId>;

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Minus,
    Negation,
}

#[derive(Clone, Copy, Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub operand: ExprId,
}

impl Index<UnaryId> for Ast {
    type Output = Unary;

    fn index(&self, index: UnaryId) -> &Self::Output {
        &self.unary_buffer[index]
    }
}

impl IndexMut<UnaryId> for Ast {
    fn index_mut(&mut self, index: UnaryId) -> &mut Self::Output {
        &mut self.unary_buffer[index]
    }
}

impl AstElem<Unary, ExprId> for Ast {
    fn add(&mut self, elem: Unary) -> ExprId {
        let global_id = self.expr_id;
        let kind = UnaryId::new(self.unary_buffer.len());

        self.unary_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        ExprId {
            global_id,
            kind: Expr::Unary(kind),
        }
    }
}
