#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcID(usize);

#[derive(Default)]
pub struct SrcLibrary {
    inner: Vec<String>,
}

impl std::ops::Index<SrcID> for SrcLibrary {
    type Output = String;

    fn index(&self, index: SrcID) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl std::ops::IndexMut<SrcID> for SrcLibrary {
    fn index_mut(&mut self, index: SrcID) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}
