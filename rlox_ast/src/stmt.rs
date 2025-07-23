use std::ops::{Index, IndexMut};

use crate::{Ast, AstElem, AstIndex, AstVec, Expr, StrId, define_id};

#[derive(Debug, Clone, Copy)]
pub struct StmtNode<Inner> {
    pub stmt_id: StmtId,
    pub inner: Inner,
}

#[macro_export]
macro_rules! stmt_node {
    ($global:expr, $inner: expr) => {
        rlox_ast::stmt::StmtNode {
            stmt_id: $global.global_id(),
            inner: $inner,
        }
    };
}

pub use stmt_node;

#[derive(Clone, Copy, Debug)]
pub enum StmtKind {
    Declaration(DeclarationId),
    Block(BlockId),
    IfElse(IfElseId),
    While(WhileId),
    Expr(Expr),
}

impl AstElem<Expr, Stmt> for Ast {
    fn add(&mut self, elem: Expr) -> Stmt {
        let global_id = self.stmt_id;

        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::Expr(elem),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StmtId(usize);

#[derive(Clone, Copy, Debug)]
pub struct Stmt {
    global_id: StmtId,
    kind: StmtKind,
}

impl AstIndex for StmtId {
    fn inner(&self) -> usize {
        self.0
    }
}

impl Stmt {
    pub fn global_id(&self) -> StmtId {
        self.global_id
    }

    pub fn kind(&self) -> StmtKind {
        self.kind
    }
}

define_id!(DeclarationId);
pub(crate) type DeclarationVec = AstVec<Declaration, DeclarationId>;

#[derive(Clone, Debug)]
pub struct Declaration {
    pub identifier: StrId,
    pub value: Option<Expr>,
}

impl Index<DeclarationId> for Ast {
    type Output = Declaration;

    fn index(&self, index: DeclarationId) -> &Self::Output {
        &self.declaration_buffer[index]
    }
}

impl IndexMut<DeclarationId> for Ast {
    fn index_mut(&mut self, index: DeclarationId) -> &mut Self::Output {
        &mut self.declaration_buffer[index]
    }
}

impl AstElem<Declaration, Stmt> for Ast {
    fn add(&mut self, elem: Declaration) -> Stmt {
        let global_id = self.stmt_id;
        let inner = DeclarationId::new(self.declaration_buffer.len());

        self.declaration_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::Declaration(inner),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId {
    start: usize,
    end: usize,
}

pub(crate) type BlockVec = Vec<Stmt>;

impl Index<BlockId> for Ast {
    type Output = [Stmt];

    fn index(&self, index: BlockId) -> &Self::Output {
        &self.stmt_buffer[index.start..index.end]
    }
}

impl IndexMut<BlockId> for Ast {
    fn index_mut(&mut self, index: BlockId) -> &mut Self::Output {
        &mut self.stmt_buffer[index.start..index.end]
    }
}

impl AstElem<&[Stmt], Stmt> for Ast {
    fn add(&mut self, elem: &[Stmt]) -> Stmt {
        let global_id = self.stmt_id;
        let block_start = self.stmt_buffer.len();

        self.stmt_buffer.extend_from_slice(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        let block_end = self.stmt_buffer.len();

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::Block(BlockId {
                start: block_start,
                end: block_end,
            }),
        }
    }
}

define_id!(IfElseId);
pub(crate) type IfElseVec = AstVec<IfElse, IfElseId>;

#[derive(Clone, Debug)]
pub struct IfElse {
    pub condition: Expr,
    pub if_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

impl Index<IfElseId> for Ast {
    type Output = IfElse;

    fn index(&self, index: IfElseId) -> &Self::Output {
        &self.ifelse_buffer[index]
    }
}

impl IndexMut<IfElseId> for Ast {
    fn index_mut(&mut self, index: IfElseId) -> &mut Self::Output {
        &mut self.ifelse_buffer[index]
    }
}

impl AstElem<IfElse, Stmt> for Ast {
    fn add(&mut self, elem: IfElse) -> Stmt {
        let global_id = self.stmt_id;
        let inner = IfElseId::new(self.ifelse_buffer.len());

        self.ifelse_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::IfElse(inner),
        }
    }
}

define_id!(WhileId);
pub(crate) type WhileVec = AstVec<While, WhileId>;

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Stmt,
}

impl Index<WhileId> for Ast {
    type Output = While;

    fn index(&self, index: WhileId) -> &Self::Output {
        &self.while_buffer[index]
    }
}

impl IndexMut<WhileId> for Ast {
    fn index_mut(&mut self, index: WhileId) -> &mut Self::Output {
        &mut self.while_buffer[index]
    }
}

impl AstElem<While, Stmt> for Ast {
    fn add(&mut self, elem: While) -> Stmt {
        let global_id = self.stmt_id;
        let inner = WhileId::new(self.while_buffer.len());

        self.while_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::While(inner),
        }
    }
}
