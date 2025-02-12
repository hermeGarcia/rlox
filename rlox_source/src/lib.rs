#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceMetadata {
    pub start: usize,
    pub end: usize,
    pub source: Source,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Prompt,
    File(usize),
}

#[derive(Clone)]
pub struct SourceFile {
    pub path: String,
    pub data: String,
}

#[derive(Default, Clone)]
pub struct SourceLibrary {
    source: Vec<SourceFile>,
}

impl std::ops::Index<usize> for SourceLibrary {
    type Output = SourceFile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.source[index]
    }
}

impl std::ops::IndexMut<usize> for SourceLibrary {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.source[index]
    }
}

impl SourceLibrary {
    pub fn new() -> SourceLibrary {
        SourceLibrary::default()
    }

    pub fn add(&mut self, source: SourceFile) -> usize {
        let src_id = self.source.len();
        self.source.push(source);

        src_id
    }
}
