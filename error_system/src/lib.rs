use context::src_library::{FileLibrary, SourceKind};
use std::io::{Stdout, Write, stdout};
use std::sync::{Arc, LazyLock, Mutex};

pub trait CompilerMessage: Sync + Send + 'static {
    fn message(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn kind(&self) -> SourceKind;
    fn line(&self) -> usize;
}

type Buffer = Vec<Arc<dyn CompilerMessage>>;

static ERRORS: LazyLock<Mutex<Buffer>> = LazyLock::new(|| Mutex::new(Buffer::default()));
static WARNINGS: LazyLock<Mutex<Buffer>> = LazyLock::new(|| Mutex::new(Buffer::default()));

pub fn error<Msg: CompilerMessage>(msg: Msg) {
    let msg = Arc::new(msg);
    let mut error_log = ERRORS.lock().unwrap();
    error_log.push(msg);
}

pub fn warning<Msg: CompilerMessage>(msg: Msg) {
    let msg = Arc::new(msg);
    let mut warning_log = WARNINGS.lock().unwrap();
    warning_log.push(msg);
}

pub fn report(library: &FileLibrary) {
    let warnings: Buffer = std::mem::take(WARNINGS.lock().unwrap().as_mut());
    let errors: Buffer = std::mem::take(ERRORS.lock().unwrap().as_mut());
    let mut stdout = stdout();

    for warning in warnings {
        let message = warning.message();
        writeln!(&mut stdout, "[WARNING] {message}.").unwrap();
        print_source(&mut stdout, warning.as_ref(), library);
    }

    for error in errors {
        let message = error.message();
        writeln!(&mut stdout, "[ERROR] {message}.").unwrap();

        print_source(&mut stdout, error.as_ref(), library);
    }
}

fn print_source<Msg>(stdout: &mut Stdout, msg: &Msg, library: &FileLibrary)
where
    Msg: CompilerMessage + ?Sized,
{
    let SourceKind::File(index) = msg.kind() else {
        return;
    };

    let source = &library[index];
    let source_path = &source.path;

    let data_as_bytes = source.data.as_bytes();
    let relevant_part = String::from_utf8_lossy(&data_as_bytes[msg.start()..msg.end()]);
    let mut line_offset = msg.line();

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
