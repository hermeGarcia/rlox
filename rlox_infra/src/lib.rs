/// rlox uses a data oriented pattern in several places, which means that
/// dealing with structs of vectors. To ease such scenarios this trait allows
/// client code to use the type system to specify Vec access.
///
/// TODO: this is not a clean abstraction, `assign` does not make sense here.
pub trait StructVec<Item, ElemId> {
    fn assign(&mut self, id: ElemId, property: Item);
    fn get(&self, id: ElemId) -> &Item;
    fn get_mut(&mut self, id: ElemId) -> &mut Item;
}
