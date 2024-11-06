use context::src_library::SourceKind;
use error_system::CompilerMessage;

pub struct UnknownToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) source: SourceKind,
}

impl CompilerMessage for UnknownToken {
    fn message(&self) -> &str {
        "Unknown token found"
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn kind(&self) -> SourceKind {
        self.source
    }

    fn line(&self) -> usize {
        self.line
    }
}
