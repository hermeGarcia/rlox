#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceKind {
    Prompt,
    File(usize),
}

#[derive(Clone)]
pub struct SourceFile {
    pub path: String,
    pub data: String,
}

#[derive(Default, Clone)]
pub struct FileLibrary {
    source: Vec<SourceFile>,
}

impl std::ops::Index<usize> for FileLibrary {
    type Output = SourceFile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.source[index]
    }
}

impl std::ops::IndexMut<usize> for FileLibrary {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.source[index]
    }
}

impl FileLibrary {
    pub fn new() -> FileLibrary {
        FileLibrary::default()
    }

    pub fn add(&mut self, source: SourceFile) -> usize {
        let src_id = self.source.len();
        self.source.push(source);

        src_id
    }
}
