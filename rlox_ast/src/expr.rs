use std::ops::{Index, IndexMut};

use crate::{Ast, AstElem, AstIndex, AstVec, Identifier, StrId, define_id};

pub struct ExprNode<Inner> {
    pub expr_id: ExprId,
    pub inner: Inner,
}

#[macro_export]
macro_rules! expr_node {
    ($global:expr, $inner: expr) => {
        ExprNode {
            expr_id: $global.global_id(),
            inner: $inner,
        }
    };
}

pub use expr_node;

#[derive(Clone, Copy, Debug)]
pub enum ExprKind {
    Assign(AssignId),
    Binary(BinaryId),
    Call(CallId),
    Unary(UnaryId),
    Identifier(Identifier),
    String(StrId),
    Natural(u64),
    Decimal(f64),
    Boolean(bool),
    Nil,
}

#[derive(Clone, Copy, Debug)]
pub struct Nil;

impl From<Identifier> for ExprKind {
    fn from(value: Identifier) -> Self {
        ExprKind::Identifier(value)
    }
}

impl From<StrId> for ExprKind {
    fn from(value: StrId) -> Self {
        ExprKind::String(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(usize);

impl AstIndex for ExprId {
    fn inner(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Expr {
    global_id: ExprId,
    kind: ExprKind,
}

impl Expr {
    pub fn global_id(&self) -> ExprId {
        self.global_id
    }

    pub fn kind(&self) -> ExprKind {
        self.kind
    }
}

impl AstElem<Identifier, Expr> for Ast {
    fn add(&mut self, elem: Identifier) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: elem.into(),
        }
    }
}

impl AstElem<StrId, Expr> for Ast {
    fn add(&mut self, elem: StrId) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: elem.into(),
        }
    }
}

impl AstElem<u64, Expr> for Ast {
    fn add(&mut self, elem: u64) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Natural(elem),
        }
    }
}

impl AstElem<f64, Expr> for Ast {
    fn add(&mut self, elem: f64) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Decimal(elem),
        }
    }
}

impl AstElem<bool, Expr> for Ast {
    fn add(&mut self, elem: bool) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Boolean(elem),
        }
    }
}

impl AstElem<Nil, Expr> for Ast {
    fn add(&mut self, _: Nil) -> Expr {
        let global_id = self.expr_id;

        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Nil,
        }
    }
}

define_id!(AssignId);
pub(crate) type AssignVec = AstVec<Assign, AssignId>;

#[derive(Clone, Debug, Copy)]
pub struct Assign {
    pub lhs: Expr,
    pub rhs: Expr,
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

impl AstElem<Assign, Expr> for Ast {
    fn add(&mut self, elem: Assign) -> Expr {
        let global_id = self.expr_id;
        let kind = AssignId::new(self.assign_buffer.len());

        self.assign_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Assign(kind),
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
    LogicAnd,
    LogicOr,
}

#[derive(Clone, Debug, Copy)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub lhs: Expr,
    pub rhs: Expr,
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

impl AstElem<Binary, Expr> for Ast {
    fn add(&mut self, elem: Binary) -> Expr {
        let global_id = self.expr_id;
        let kind = BinaryId::new(self.binary_buffer.len());

        self.binary_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Binary(kind),
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
    pub operand: Expr,
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

impl AstElem<Unary, Expr> for Ast {
    fn add(&mut self, elem: Unary) -> Expr {
        let global_id = self.expr_id;
        let kind = UnaryId::new(self.unary_buffer.len());

        self.unary_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Unary(kind),
        }
    }
}

define_id!(CallId);
pub(crate) type CallVec = AstVec<Call, CallId>;

#[derive(Clone, Debug)]
pub struct Call {
    pub lhs: Expr,
    pub arguments: Vec<Expr>,
}

impl Index<CallId> for Ast {
    type Output = Call;

    fn index(&self, index: CallId) -> &Self::Output {
        &self.call_buffer[index]
    }
}

impl IndexMut<CallId> for Ast {
    fn index_mut(&mut self, index: CallId) -> &mut Self::Output {
        &mut self.call_buffer[index]
    }
}

impl AstElem<Call, Expr> for Ast {
    fn add(&mut self, elem: Call) -> Expr {
        let global_id = self.expr_id;
        let kind = CallId::new(self.call_buffer.len());

        self.call_buffer.push(elem);
        self.expr_metadata_buffer.push(None);
        self.expr_id += 1;

        Expr {
            global_id: ExprId(global_id),
            kind: ExprKind::Call(kind),
        }
    }
}
