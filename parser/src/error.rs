use context::src_library::{Source, SourceMetadata};
use error_system::{Error, Message};

pub enum ParserError {
    UnknownTokenError(UnknownToken),
}

impl From<ParserError> for Error {
    fn from(value: ParserError) -> Self {
        match value {
            ParserError::UnknownTokenError(e) => e.into(),
        }
    }
}

pub struct UnknownToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) source: Source,
}

impl From<UnknownToken> for ParserError {
    fn from(value: UnknownToken) -> Self {
        ParserError::UnknownTokenError(value)
    }
}

impl Message for UnknownToken {
    fn description(&self) -> &str {
        "Unknown token found"
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            line: self.line,
            source: self.source,
        }
    }
}
