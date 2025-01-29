use rlox_source::{Source, SourceFile, SourceLibrary};
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
    let mut library = SourceLibrary::default();

    let src_id = match read_source(file_path, &mut library) {
        Ok(id) => id,
        Err(err) => abort!("Could not read {file_path:?}: {err}"),
    };

    compile(Source::File(src_id), &library[src_id].data, &library)
}

fn prompt_mode() -> ! {
    let mut output = io::stdout();
    let mut buffer = String::new();
    let library = SourceLibrary::default();

    loop {
        write!(&mut output, "> ").unwrap();
        output.flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();

        compile(Source::Prompt, &buffer, &library);

        buffer.clear();
    }
}

fn compile(src_id: Source, code: &str, library: &SourceLibrary) -> ExitCode {
    let Ok(_ast) = rlox_parser::parse(src_id, code.as_bytes()) else {
        rlox_errors::report(library);
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

pub fn read_source<P: Into<PathBuf>>(path: P, library: &mut SourceLibrary) -> IoResult<usize> {
    let path: PathBuf = path.into().canonicalize()?;
    let source = read_to_string(&path)?;

    Ok(library.add(SourceFile {
        path: path.to_string_lossy().to_string(),
        data: source,
    }))
}
