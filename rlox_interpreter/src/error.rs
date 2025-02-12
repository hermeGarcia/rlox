use rlox_errors::{Error, Message};
use rlox_source::{Source, SourceMetadata};

#[derive(Debug)]
pub enum RuntimeError {
    OperationNotDefined(OperationNotDefined),
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::OperationNotDefined(e) => e.into(),
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
