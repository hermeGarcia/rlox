use std::io::{BufWriter, Result, Write};

use rlox_ast::Ast;
use rlox_ast::expr::ExprId;
use rlox_ast::stmt::StmtId;
use rlox_cf_graph::{BasicBlock, BasicBlockId, BasicBlockValue, ControlFlowGraph, EdgeKind, Edges};
use rlox_infra::StructVec;
use rlox_source::{Source, SourceLibrary};

#[derive(Clone, Copy)]
pub struct Ctxt<'a> {
    pub cf_graph: &'a ControlFlowGraph,
    pub ast: &'a Ast,
    pub library: &'a SourceLibrary,
}

pub fn graph<W: Write>(ctxt: Ctxt, writer: &mut BufWriter<W>) -> Result<()> {
    writeln!(writer, "digraph {{")?;

    for bb_id in ctxt.cf_graph.basic_block_ids() {
        graph_block(bb_id, ctxt, writer)?;
    }

    writeln!(writer, "}}")?;
    writer.flush()
}

fn expr_to_string(expr: ExprId, ctxt: Ctxt) -> String {
    let metadata = ctxt.ast.get(expr);
    let Source::File(id) = metadata.source else {
        panic!("Prompt input is not supported")
    };

    let file = &ctxt.library[id];
    file.data[metadata.start..metadata.end].to_string()
}

fn stmt_to_string(stmt: StmtId, ctxt: Ctxt) -> String {
    let metadata = ctxt.ast.get(stmt);

    let Source::File(id) = metadata.source else {
        panic!("Prompt input is not supported")
    };

    let file = &ctxt.library[id];
    file.data[metadata.start..metadata.end].to_string()
}

struct GraphvizConfig {
    label: String,
    shape: String,
}

fn basic_block_stmt_to_graphviz_config(bb_stmt: BasicBlockValue, ctxt: Ctxt) -> GraphvizConfig {
    match bb_stmt {
        BasicBlockValue::EnterBlock => GraphvizConfig {
            label: "_enter_block_".to_string(),
            shape: "box".to_string(),
        },

        BasicBlockValue::LeaveBlock => GraphvizConfig {
            label: "_leave_block_".to_string(),
            shape: "box".to_string(),
        },

        BasicBlockValue::StmtExpr(inner) => GraphvizConfig {
            label: stmt_to_string(inner.stmt_id, ctxt),
            shape: "box".to_string(),
        },

        BasicBlockValue::Declaration(inner) => GraphvizConfig {
            label: stmt_to_string(inner.stmt_id, ctxt),
            shape: "box".to_string(),
        },

        BasicBlockValue::EntryPoint => GraphvizConfig {
            label: "".to_string(),
            shape: "doublecircle".to_string(),
        },

        BasicBlockValue::EndPoint => GraphvizConfig {
            label: "".to_string(),
            shape: "point".to_string(),
        },

        BasicBlockValue::Condition(expr) => GraphvizConfig {
            label: expr_to_string(expr.global_id(), ctxt),
            shape: "diamond".to_string(),
        },
    }
}

fn edge_to_graphviz_label(edge_kind: EdgeKind, _ctxt: Ctxt) -> String {
    match edge_kind {
        EdgeKind::True => "True".to_string(),
        EdgeKind::False => "False".to_string(),
        EdgeKind::Unconditional => "".to_string(),
    }
}

fn graph_block<W: Write>(bb_id: BasicBlockId, ctxt: Ctxt, writer: &mut BufWriter<W>) -> Result<()> {
    let basic_block: &BasicBlock = ctxt.cf_graph.get(bb_id);
    let mut config = basic_block_stmt_to_graphviz_config(basic_block.stmt, ctxt);

    config.label = config.label.replace('"', "\\\"");
    config.label = config.label.replace('(', "\\(");
    config.label = config.label.replace(')', "\\)");

    writeln!(writer, "\"{bb_id}\" [label=\"{}\", shape=\"{}\", center=true]", config.label, config.shape)?;

    let edges: &Edges = ctxt.cf_graph.get(bb_id);

    for index in 0..edges.len() {
        let edge_kind: EdgeKind = *edges.get(index);
        let goes_to: BasicBlockId = *edges.get(index);

        let mut label = edge_to_graphviz_label(edge_kind, ctxt);

        label = label.replace('"', "\\\"");
        label = label.replace('(', "\\(");
        label = label.replace(')', "\\)");

        writeln!(writer, "\"{bb_id}\" -> \"{goes_to}\" [label=\"{label}\"]")?;
    }

    Ok(())
}
