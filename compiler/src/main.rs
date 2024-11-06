use context::src_library::{FileLibrary, SourceFile, SourceKind};
use std::fs::read_to_string;
use std::io;
use std::io::Result as IoResult;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

macro_rules! abort {
    ($msg:expr) => {{
        eprintln!($msg);
        return ExitCode::FAILURE;
    }};
}

pub fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => prompt_mode(),
        2 => file_mode(&args[1]),
        other => abort!("Too many arguments, {other}"),
    }
}

fn file_mode(file_path: &str) -> ExitCode {
    let mut library = FileLibrary::default();

    let src_id = match read_source(file_path, &mut library) {
        Ok(id) => id,
        Err(err) => abort!("Could not read {file_path:?}: {err}"),
    };

    let Ok(()) = compile(SourceKind::File(src_id), &library[src_id].data) else {
        error_system::report(&library);
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn prompt_mode() -> ExitCode {
    let mut output = io::stdout();
    let mut buffer = String::new();
    let library = FileLibrary::default();

    loop {
        write!(&mut output, "> ").unwrap();
        output.flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();

        if let Err(()) = compile(SourceKind::Prompt, &buffer) {
            error_system::report(&library);
        }

        buffer.clear();
    }
}

fn compile(src_id: SourceKind, code: &str) -> Result<(), ()> {
    parser::parse(src_id, code.as_bytes())?;

    Ok(())
}

pub fn read_source<P: Into<PathBuf>>(path: P, library: &mut FileLibrary) -> IoResult<usize> {
    let path: PathBuf = path.into().canonicalize()?;
    let source = read_to_string(&path)?;

    Ok(library.add(SourceFile {
        path: path.to_string_lossy().to_string(),
        data: source,
    }))
}
