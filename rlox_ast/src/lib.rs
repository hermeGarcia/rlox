pub mod expr;
pub mod stmt;

pub use expr::Expr;

use rlox_source::SourceMetadata;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub trait AstElem<Elem, ElemId>: Index<ElemId, Output = Elem> + IndexMut<ElemId> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(usize);

impl AstIndex for ExprId {
    fn inner(&self) -> usize {
        self.0
    }
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

#[derive(Default)]
pub struct Ast {
    exprs: AstVec<Expr, ExprId>,
    exprs_metadata: AstVec<Option<SourceMetadata>, ExprId>,
}

impl Index<ExprId> for Ast {
    type Output = Expr;

    fn index(&self, index: ExprId) -> &Self::Output {
        &self.exprs[index]
    }
}

impl IndexMut<ExprId> for Ast {
    fn index_mut(&mut self, index: ExprId) -> &mut Self::Output {
        &mut self.exprs[index]
    }
}

impl AstElem<Expr, ExprId> for Ast {
    fn add(&mut self, elem: Expr) -> ExprId {
        let expr_id = ExprId(self.exprs.len());

        self.exprs.push(elem);
        self.exprs_metadata.push(None);

        expr_id
    }
}

impl AstProperty<SourceMetadata, ExprId> for Ast {
    fn attach(&mut self, id: ExprId, property: SourceMetadata) {
        self.exprs_metadata[id] = Some(property);
    }

    fn get(&self, id: ExprId) -> &SourceMetadata {
        let Some(metadata) = &self.exprs_metadata[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }

    fn get_mut(&mut self, id: ExprId) -> &mut SourceMetadata {
        let Some(metadata) = &mut self.exprs_metadata[id] else {
            panic!("{id:?} does not have metadata");
        };

        metadata
    }
}

impl Ast {
    pub fn expr_ids(&self) -> impl DoubleEndedIterator<Item = ExprId> {
        (0..self.exprs.inner.len()).map(ExprId)
    }
}
