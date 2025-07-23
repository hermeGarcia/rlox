pub mod build_cfg;

use std::ops::{Index, IndexMut};

use rlox_ast::expr::Expr;
use rlox_ast::stmt::{self, StmtNode};
use rlox_infra::StructVec;

pub struct CfgVec<T> {
    inner: Vec<T>,
}

impl<T> Index<BasicBlockId> for CfgVec<T> {
    type Output = T;

    fn index(&self, index: BasicBlockId) -> &Self::Output {
        &self.inner[index.inner]
    }
}

impl<T> IndexMut<BasicBlockId> for CfgVec<T> {
    fn index_mut(&mut self, index: BasicBlockId) -> &mut Self::Output {
        &mut self.inner[index.inner]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BasicBlockId {
    inner: usize,
}

impl std::fmt::Display for BasicBlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBId({})", self.inner)
    }
}

impl BasicBlockId {
    fn new(inner: usize) -> BasicBlockId {
        BasicBlockId {
            inner,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BasicBlockValue {
    Declaration(StmtNode<stmt::DeclarationId>),
    StmtExpr(StmtNode<Expr>),
    Condition(Expr),
    EnterBlock,
    LeaveBlock,
    EntryPoint,
    EndPoint,
}

impl From<Expr> for BasicBlockValue {
    fn from(value: Expr) -> Self {
        BasicBlockValue::Condition(value)
    }
}

impl From<StmtNode<stmt::DeclarationId>> for BasicBlockValue {
    fn from(value: StmtNode<stmt::DeclarationId>) -> Self {
        BasicBlockValue::Declaration(value)
    }
}

impl From<StmtNode<Expr>> for BasicBlockValue {
    fn from(value: StmtNode<Expr>) -> Self {
        BasicBlockValue::StmtExpr(value)
    }
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub stmt: BasicBlockValue,
}

impl BasicBlock {
    fn new(stmt: BasicBlockValue) -> BasicBlock {
        BasicBlock {
            stmt,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeKind {
    True,
    False,
    Unconditional,
}

#[derive(Debug, Clone, Default)]
pub struct Edges {
    edge_kind: Vec<EdgeKind>,
    goes_to: Vec<BasicBlockId>,
}

impl StructVec<EdgeKind, usize> for Edges {
    fn assign(&mut self, id: usize, property: EdgeKind) {
        self.edge_kind[id] = property;
    }

    fn get(&self, id: usize) -> &EdgeKind {
        &self.edge_kind[id]
    }

    fn get_mut(&mut self, id: usize) -> &mut EdgeKind {
        &mut self.edge_kind[id]
    }
}

impl StructVec<BasicBlockId, usize> for Edges {
    fn assign(&mut self, id: usize, property: BasicBlockId) {
        self.goes_to[id] = property;
    }

    fn get(&self, id: usize) -> &BasicBlockId {
        &self.goes_to[id]
    }

    fn get_mut(&mut self, id: usize) -> &mut BasicBlockId {
        &mut self.goes_to[id]
    }
}

impl Edges {
    pub fn new() -> Edges {
        Edges::default()
    }

    pub fn len(&self) -> usize {
        self.goes_to.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default)]
pub struct ControlFlowGraph {
    nodes: Vec<BasicBlock>,
    edges: Vec<Edges>,
}

impl StructVec<BasicBlock, BasicBlockId> for ControlFlowGraph {
    fn assign(&mut self, id: BasicBlockId, item: BasicBlock) {
        self.nodes[id.inner] = item;
    }

    fn get(&self, id: BasicBlockId) -> &BasicBlock {
        &self.nodes[id.inner]
    }

    fn get_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        &mut self.nodes[id.inner]
    }
}

impl StructVec<Edges, BasicBlockId> for ControlFlowGraph {
    fn assign(&mut self, id: BasicBlockId, item: Edges) {
        self.edges[id.inner] = item;
    }

    fn get(&self, id: BasicBlockId) -> &Edges {
        &self.edges[id.inner]
    }

    fn get_mut(&mut self, id: BasicBlockId) -> &mut Edges {
        &mut self.edges[id.inner]
    }
}

impl ControlFlowGraph {
    pub fn new() -> ControlFlowGraph {
        ControlFlowGraph::default()
    }

    pub fn fresh_block(&mut self, stmt: BasicBlockValue) -> BasicBlockId {
        let block_id = BasicBlockId::new(self.nodes.len());

        self.nodes.push(BasicBlock::new(stmt));
        self.edges.push(Edges::default());

        block_id
    }

    pub fn basic_block_ids(&self) -> impl Iterator<Item = BasicBlockId> {
        (0..self.nodes.len()).map(BasicBlockId::new)
    }
}
