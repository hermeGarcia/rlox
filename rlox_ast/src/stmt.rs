use std::ops::{Index, IndexMut};

use crate::{Ast, AstElem, AstIndex, AstVec, Expr, StrId, define_id};

#[derive(Clone, Copy, Debug)]
pub enum StmtKind {
    Print(PrintId),
    Declaration(DeclarationId),
    Block(BlockId),
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

define_id!(PrintId);
pub(crate) type PrintVec = AstVec<Print, PrintId>;

#[derive(Clone, Debug)]
pub struct Print {
    pub expr: Expr,
}

impl Index<PrintId> for Ast {
    type Output = Print;

    fn index(&self, index: PrintId) -> &Self::Output {
        &self.print_buffer[index]
    }
}

impl IndexMut<PrintId> for Ast {
    fn index_mut(&mut self, index: PrintId) -> &mut Self::Output {
        &mut self.print_buffer[index]
    }
}

impl AstElem<Print, Stmt> for Ast {
    fn add(&mut self, elem: Print) -> Stmt {
        let global_id = self.stmt_id;
        let inner = PrintId::new(self.print_buffer.len());

        self.print_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        Stmt {
            global_id: StmtId(global_id),
            kind: StmtKind::Print(inner),
        }
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
