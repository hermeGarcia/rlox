use std::ops::{Index, IndexMut};

use crate::{Ast, AstElem, AstIndex, AstVec, ExprId, StrId, define_id};

#[derive(Clone, Copy, Debug)]
pub enum Stmt {
    Print(PrintId),
    Declaration(DeclarationId),
    Expr(ExprId),
}

impl AstElem<ExprId, StmtId> for Ast {
    fn add(&mut self, elem: ExprId) -> StmtId {
        let global_id = self.stmt_id;

        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        StmtId {
            global_id,
            kind: Stmt::Expr(elem),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StmtId {
    global_id: usize,
    pub kind: Stmt,
}
impl Eq for StmtId {}

impl std::hash::Hash for StmtId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.global_id.hash(state);
    }
}

impl Ord for StmtId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.global_id.cmp(&other.global_id)
    }
}

impl PartialOrd for StmtId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StmtId {
    fn eq(&self, other: &Self) -> bool {
        self.global_id == other.global_id
    }
}

impl AstIndex for StmtId {
    fn inner(&self) -> usize {
        self.global_id
    }
}

define_id!(PrintId);
pub(crate) type PrintVec = AstVec<Print, PrintId>;

#[derive(Clone, Debug)]
pub struct Print {
    pub expr: ExprId,
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

impl AstElem<Print, StmtId> for Ast {
    fn add(&mut self, elem: Print) -> StmtId {
        let global_id = self.stmt_id;
        let inner = PrintId::new(self.print_buffer.len());

        self.print_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        StmtId {
            global_id,
            kind: Stmt::Print(inner),
        }
    }
}

define_id!(DeclarationId);
pub(crate) type DeclarationVec = AstVec<Declaration, DeclarationId>;

#[derive(Clone, Debug)]
pub struct Declaration {
    pub identifier: StrId,
    pub value: Option<ExprId>,
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

impl AstElem<Declaration, StmtId> for Ast {
    fn add(&mut self, elem: Declaration) -> StmtId {
        let global_id = self.stmt_id;
        let inner = DeclarationId::new(self.declaration_buffer.len());

        self.declaration_buffer.push(elem);
        self.stmt_metadata_buffer.push(None);
        self.stmt_id += 1;

        StmtId {
            global_id,
            kind: Stmt::Declaration(inner),
        }
    }
}
