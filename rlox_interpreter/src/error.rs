use rlox_errors::{Error, Message};
use rlox_source::{Source, SourceMetadata};

use crate::value_system::Value;

#[derive(Debug)]
pub enum RuntimeError {
    OperationNotDefined(OperationNotDefined),
    VarNotFound(VarNotFound),
    InvalidAssign(InvalidAssign),
    UnexpectedValue(UnexpectedValue),
    WrongNumberOfArgs(WrongNumberOfArgs),
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::VarNotFound(e) => e.into(),
            RuntimeError::OperationNotDefined(e) => e.into(),
            RuntimeError::InvalidAssign(e) => e.into(),
            RuntimeError::UnexpectedValue(e) => e.into(),
            RuntimeError::WrongNumberOfArgs(e) => e.into(),
        }
    }
}

#[derive(Debug)]
pub struct OperationNotDefined {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) source: Source,
}

impl From<OperationNotDefined> for RuntimeError {
    fn from(value: OperationNotDefined) -> Self {
        RuntimeError::OperationNotDefined(value)
    }
}

impl Message for OperationNotDefined {
    fn description(&self) -> String {
        "The operation is not defined".into()
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct VarNotFound {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) source: Source,
}

impl From<VarNotFound> for RuntimeError {
    fn from(value: VarNotFound) -> Self {
        RuntimeError::VarNotFound(value)
    }
}

impl Message for VarNotFound {
    fn description(&self) -> String {
        "Variable not found".into()
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct InvalidAssign {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) source: Source,
}

impl From<InvalidAssign> for RuntimeError {
    fn from(value: InvalidAssign) -> Self {
        RuntimeError::InvalidAssign(value)
    }
}

impl Message for InvalidAssign {
    fn description(&self) -> String {
        "Assignments need a memory location".to_string()
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedValue {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) source: Source,
    pub(crate) found: Value,
}

impl From<UnexpectedValue> for RuntimeError {
    fn from(value: UnexpectedValue) -> Self {
        RuntimeError::UnexpectedValue(value)
    }
}

impl Message for UnexpectedValue {
    fn description(&self) -> String {
        format!("Found: {}", self.found)
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub struct WrongNumberOfArgs {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) source: Source,
    pub(crate) got: usize,
    pub(crate) expect: usize,
}

impl From<WrongNumberOfArgs> for RuntimeError {
    fn from(value: WrongNumberOfArgs) -> Self {
        RuntimeError::WrongNumberOfArgs(value)
    }
}

impl Message for WrongNumberOfArgs {
    fn description(&self) -> String {
        format!("Expected {} arguments, but got {}", self.expect, self.got)
    }

    fn source_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            start: self.start,
            end: self.end,
            source: self.source,
        }
    }
}
