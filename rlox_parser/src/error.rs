use rlox_errors::{Error, Message};
use rlox_source::{Source, SourceMetadata};

use crate::token_stream::TokenKind;

#[derive(Debug)]
pub enum ParserError {
    UnknownToken(UnknownToken),
    UnexpectedToken(UnexpectedToken),
    TypeCouldNotBeParsed(TypeCouldNotBeParsed),
}

impl From<ParserError> for Error {
    fn from(value: ParserError) -> Self {
        match value {
            ParserError::UnknownToken(e) => e.into(),
            ParserError::TypeCouldNotBeParsed(e) => e.into(),
            ParserError::UnexpectedToken(e) => e.into(),
        }
    }
}

#[derive(Debug)]
pub struct UnknownToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) source: Source,
}

impl From<UnknownToken> for ParserError {
    fn from(value: UnknownToken) -> Self {
        ParserError::UnknownToken(value)
    }
}

impl Message for UnknownToken {
    fn description(&self) -> String {
        "Unknown token found".into()
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            line_start: self.line,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct TypeCouldNotBeParsed {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) source: Source,
}

impl From<TypeCouldNotBeParsed> for ParserError {
    fn from(value: TypeCouldNotBeParsed) -> Self {
        ParserError::TypeCouldNotBeParsed(value)
    }
}

impl Message for TypeCouldNotBeParsed {
    fn description(&self) -> String {
        "Natural value could not be parsed".into()
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            line_start: self.line,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) source: Source,
    pub(crate) expected: Vec<TokenKind>,
}

impl From<UnexpectedToken> for ParserError {
    fn from(value: UnexpectedToken) -> Self {
        ParserError::UnexpectedToken(value)
    }
}

impl Message for UnexpectedToken {
    fn description(&self) -> String {
        format!("Expected one of the following tokens: {:?}", self.expected)
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            line_start: self.line,
            source: self.source,
        }
    }
}
