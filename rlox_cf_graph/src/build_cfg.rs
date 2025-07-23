use rlox_ast::Ast;
use rlox_ast::stmt::{self as ast_stmt, stmt_node};
use rlox_infra::StructVec;

use crate::{BasicBlockId, BasicBlockValue, ControlFlowGraph, EdgeKind, Edges};

struct State {
    graph: ControlFlowGraph,
    parents: Vec<(BasicBlockId, EdgeKind)>,
}

pub fn from_sequence_of_stmts(sequence: &[ast_stmt::Stmt], ast: &Ast) -> ControlFlowGraph {
    let mut graph = ControlFlowGraph::new();
    let entry_id = graph.fresh_block(BasicBlockValue::EntryPoint);
    let end_id = graph.fresh_block(BasicBlockValue::EndPoint);

    let mut builder = State {
        graph,
        parents: vec![(entry_id, EdgeKind::Unconditional)],
    };

    for stmt in sequence.iter().copied() {
        let leaves = stmt_dispatch(stmt, ast, &mut builder);
        builder.parents = leaves;
    }

    for (parent, edge_kind) in builder.parents {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.goes_to.push(end_id);
        edges.edge_kind.push(edge_kind);
    }

    builder.graph
}

fn stmt_dispatch(stmt: ast_stmt::Stmt, ast: &Ast, builder: &mut State) -> Vec<(BasicBlockId, EdgeKind)> {
    match stmt.kind() {
        ast_stmt::StmtKind::Declaration(inner) => emit_singleton_block(stmt_node!(stmt, inner), builder),
        ast_stmt::StmtKind::Expr(inner) => emit_singleton_block(stmt_node!(stmt, inner), builder),
        ast_stmt::StmtKind::Block(inner) => block_dispatch(inner, ast, builder),
        ast_stmt::StmtKind::IfElse(inner) => branch_dispatch(inner, ast, builder),
        ast_stmt::StmtKind::While(inner) => while_dispatch(inner, ast, builder),
    }
}

fn while_dispatch(id: ast_stmt::WhileId, ast: &Ast, builder: &mut State) -> Vec<(BasicBlockId, EdgeKind)> {
    let data = &ast[id];
    let loop_header = builder.graph.fresh_block(data.condition.into());

    for (parent, edge_kind) in builder.parents.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.edge_kind.push(edge_kind);
        edges.goes_to.push(loop_header);
    }

    builder.parents[0] = (loop_header, EdgeKind::True);
    builder.parents.truncate(1);

    let body_leaves = stmt_dispatch(data.body, ast, builder);

    for (leaf, _) in body_leaves.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(leaf);
        edges.edge_kind.push(EdgeKind::Unconditional);
        edges.goes_to.push(loop_header);
    }

    vec![(loop_header, EdgeKind::False)]
}

fn branch_dispatch(id: ast_stmt::IfElseId, ast: &Ast, builder: &mut State) -> Vec<(BasicBlockId, EdgeKind)> {
    let data = &ast[id];

    let condition = builder.graph.fresh_block(data.condition.into());

    for (parent, edge_kind) in builder.parents.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.edge_kind.push(edge_kind);
        edges.goes_to.push(condition);
    }

    builder.parents[0] = (condition, EdgeKind::True);
    builder.parents.truncate(1);

    let if_leaves = stmt_dispatch(data.if_branch, ast, builder);

    let Some(else_branch) = data.else_branch else {
        let mut leaves = if_leaves;
        leaves.push((condition, EdgeKind::False));

        return leaves;
    };

    builder.parents[0] = (condition, EdgeKind::False);
    builder.parents.truncate(1);

    let else_leaves = stmt_dispatch(else_branch, ast, builder);

    let mut leaves = if_leaves;
    leaves.extend(else_leaves);

    leaves
}

fn block_dispatch(id: ast_stmt::BlockId, ast: &Ast, builder: &mut State) -> Vec<(BasicBlockId, EdgeKind)> {
    let enter_block = builder.graph.fresh_block(BasicBlockValue::EnterBlock);

    for (parent, edge_kind) in builder.parents.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.edge_kind.push(edge_kind);
        edges.goes_to.push(enter_block);
    }

    // This access is safe because there is always at least
    // one parent node (except for the entry point).
    builder.parents[0] = (enter_block, EdgeKind::Unconditional);
    builder.parents.truncate(1);

    for stmt in ast[id].iter().copied() {
        let leaves = stmt_dispatch(stmt, ast, builder);
        builder.parents = leaves;
    }

    let leave_block = builder.graph.fresh_block(BasicBlockValue::LeaveBlock);

    for (parent, edge_kind) in builder.parents.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.edge_kind.push(edge_kind);
        edges.goes_to.push(leave_block);
    }

    vec![(leave_block, EdgeKind::Unconditional)]
}

fn emit_singleton_block<S>(stmt: S, builder: &mut State) -> Vec<(BasicBlockId, EdgeKind)>
where
    BasicBlockValue: From<S>,
{
    let fresh_block = builder.graph.fresh_block(stmt.into());

    for (parent, edge_kind) in builder.parents.iter().copied() {
        let edges: &mut Edges = builder.graph.get_mut(parent);
        edges.goes_to.push(fresh_block);
        edges.edge_kind.push(edge_kind);
    }

    vec![(fresh_block, EdgeKind::Unconditional)]
}
