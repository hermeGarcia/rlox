use context::src_library::{FileLibrary, Source, SourceMetadata};
use std::io::{Stdout, Write, stdout};
use std::sync::{Arc, LazyLock, Mutex};

pub trait Message: Sync + Send + 'static {
    fn description(&self) -> &str;
    fn source_metadata(&self) -> SourceMetadata;
}

pub struct Error(Arc<dyn Message>);
impl<T: Message> From<T> for Error {
    fn from(value: T) -> Self {
        Error(Arc::new(value))
    }
}

pub struct Warning(Arc<dyn Message>);
impl<T: Message> From<T> for Warning {
    fn from(value: T) -> Self {
        Warning(Arc::new(value))
    }
}

type Buffer = Vec<Arc<dyn Message>>;

static ERRORS: LazyLock<Mutex<Buffer>> = LazyLock::new(|| Mutex::new(Buffer::default()));
static WARNINGS: LazyLock<Mutex<Buffer>> = LazyLock::new(|| Mutex::new(Buffer::default()));

pub fn error<E: Into<Error>>(error: E) {
    let error: Error = error.into();
    let mut error_log = ERRORS.lock().unwrap();
    error_log.push(error.0);
}

pub fn warning<W: Into<Warning>>(warning: W) {
    let warning: Warning = warning.into();
    let mut warning_log = WARNINGS.lock().unwrap();
    warning_log.push(warning.0);
}

pub fn report(library: &FileLibrary) {
    let warnings: Buffer = std::mem::take(WARNINGS.lock().unwrap().as_mut());
    let errors: Buffer = std::mem::take(ERRORS.lock().unwrap().as_mut());
    let mut stdout = stdout();

    for warning in warnings {
        let message = warning.description();
        writeln!(&mut stdout, "[WARNING] {message}.").unwrap();
        print_source(&mut stdout, warning.as_ref(), library);
    }

    for error in errors {
        let message = error.description();
        writeln!(&mut stdout, "[ERROR] {message}.").unwrap();

        print_source(&mut stdout, error.as_ref(), library);
    }
}

fn print_source<Msg>(stdout: &mut Stdout, msg: &Msg, library: &FileLibrary)
where
    Msg: Message + ?Sized,
{
    let metadata = msg.source_metadata();

    let Source::File(index) = metadata.source else {
        return;
    };

    let source = &library[index];
    let source_path = &source.path;

    let data_as_bytes = source.data.as_bytes();
    let relevant_part = String::from_utf8_lossy(&data_as_bytes[metadata.start..metadata.end]);
    let mut line_offset = metadata.line_start + 1;

    for line in relevant_part.lines() {
        writeln!(stdout, "  {line_offset}| {line}").unwrap();
        line_offset += 1;
    }
    writeln!(stdout, "At {source_path}").unwrap();
}

#[macro_export]
macro_rules! compiler_log {
    ($msg:expr) => {{
        use std::io::{Write, stdout};
        let msg = format!($msg);
        let mut std_error = stdout();
        core::writeln!(std_error, "[LOG] {msg}").expect("Can not report");
        std_error.flush().expect("Can not flush");
    }};
}
