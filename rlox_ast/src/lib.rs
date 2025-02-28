#[cfg(debug_assertions)]
pub mod debug_utils;
pub mod expr;
pub mod stmt;

pub use expr::{Expr, ExprId, ExprKind};
pub use stmt::{Stmt, StmtId, StmtKind};

use rlox_source::SourceMetadata;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(usize);

        impl $crate::AstIndex for $name {
            fn inner(&self) -> usize {
                self.0
            }
        }

        impl $name {
            pub fn new(inner: usize) -> $name {
                $name(inner)
            }
        }
    };
}

pub trait AstElem<Elem, ElemId> {
    fn add(&mut self, elem: Elem) -> ElemId;
}

pub trait AstProperty<Property, ElemId> {
    fn attach(&mut self, id: ElemId, property: Property);
    fn get(&self, id: ElemId) -> &Property;
    fn get_mut(&mut self, id: ElemId) -> &mut Property;
}

trait AstIndex {
    fn inner(&self) -> usize;
}

struct AstVec<T, Index: AstIndex> {
    inner: Vec<T>,
    _index: PhantomData<Index>,
}

impl<T, I: AstIndex> Default for AstVec<T, I> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _index: Default::default(),
        }
    }
}

impl<Elem, Id: AstIndex> Index<Id> for AstVec<Elem, Id> {
    type Output = Elem;

    fn index(&self, index: Id) -> &Self::Output {
        &self.inner[index.inner()]
    }
}

impl<Elem, Id: AstIndex> IndexMut<Id> for AstVec<Elem, Id> {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        &mut self.inner[index.inner()]
    }
}

impl<Elem, Index: AstIndex> AstVec<Elem, Index> {
    pub fn push(&mut self, elem: Elem) {
        self.inner.push(elem);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

define_id!(StrId);
pub(crate) type StrVec = AstVec<String, StrId>;

impl Index<StrId> for Ast {
    type Output = String;

    fn index(&self, index: StrId) -> &Self::Output {
        &self.str_buffer[index]
    }
}

impl IndexMut<StrId> for Ast {
    fn index_mut(&mut self, index: StrId) -> &mut Self::Output {
        &mut self.str_buffer[index]
    }
}

impl AstElem<String, StrId> for Ast {
    fn add(&mut self, elem: String) -> StrId {
        for (id, value) in self.str_buffer.inner.iter().enumerate() {
            if value == &elem {
                return StrId(id);
            }
        }

        let id = StrId::new(self.str_buffer.len());
        self.str_buffer.push(elem);

        id
    }
}

#[derive(Default)]
pub struct Ast {
    stmt_id: usize,
    expr_id: usize,

    initial_block: Vec<Stmt>,
    str_buffer: StrVec,

    // Expression buffers
    assign_buffer: expr::AssignVec,
    binary_buffer: expr::BinaryVec,
    unary_buffer: expr::UnaryVec,
    expr_metadata_buffer: AstVec<Option<SourceMetadata>, ExprId>,

    // Statement buffers
    print_buffer: stmt::PrintVec,
    declaration_buffer: stmt::DeclarationVec,
    block_buffer: stmt::BlockVec,
    stmt_metadata_buffer: AstVec<Option<SourceMetadata>, StmtId>,
}

impl AstProperty<SourceMetadata, ExprId> for Ast {
    fn attach(&mut self, id: ExprId, property: SourceMetadata) {
        self.expr_metadata_buffer[id] = Some(property);
    }

    fn get(&self, id: ExprId) -> &SourceMetadata {
        let Some(metadata) = &self.expr_metadata_buffer[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }

    fn get_mut(&mut self, id: ExprId) -> &mut SourceMetadata {
        let Some(metadata) = &mut self.expr_metadata_buffer[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }
}

impl AstProperty<SourceMetadata, StmtId> for Ast {
    fn attach(&mut self, id: StmtId, property: SourceMetadata) {
        self.stmt_metadata_buffer[id] = Some(property);
    }

    fn get(&self, id: StmtId) -> &SourceMetadata {
        let Some(metadata) = &self.stmt_metadata_buffer[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }

    fn get_mut(&mut self, id: StmtId) -> &mut SourceMetadata {
        let Some(metadata) = &mut self.stmt_metadata_buffer[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }
}

impl Ast {
    pub fn push_into_initial_block(&mut self, stmt: Stmt) {
        self.initial_block.push(stmt);
    }

    pub fn initial_block(&self) -> &[Stmt] {
        &self.initial_block
    }
}
