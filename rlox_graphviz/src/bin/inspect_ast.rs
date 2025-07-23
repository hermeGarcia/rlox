// Example of graphviz:
// dot out/block.dot -T svg -o out/block.svg

use rlox_source::{Source, SourceFile, SourceLibrary};
use std::fs::File;
use std::fs::read_to_string;
use std::io::BufWriter;
use std::io::Result as IoResult;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    let Some(file_path) = args.get(1) else {
        eprintln!("No input file specified");
        return ExitCode::FAILURE;
    };

    let Some(output_path) = args.get(2) else {
        eprintln!("No output file specified");
        return ExitCode::FAILURE;
    };

    let mut library = SourceLibrary::default();

    let Ok(src_index) = read_source(file_path, &mut library) else {
        eprintln!("Could not read {file_path}");
        return ExitCode::FAILURE;
    };

    let src_code = &library[src_index].data;
    let src_id = Source::File(src_index);

    compile(src_id, src_code, &library, output_path)
}

fn compile(src_id: Source, code: &str, library: &SourceLibrary, output_path: &str) -> ExitCode {
    let Ok(ast) = rlox_parser::parse(src_id, code.as_bytes()) else {
        rlox_errors::report(library);
        return ExitCode::FAILURE;
    };

    let Ok(mut output) = File::create(output_path).map(BufWriter::new) else {
        eprintln!("Could not created graph.dot");
        return ExitCode::FAILURE;
    };

    match rlox_graphviz::ast::graph(&ast, &mut output) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Failed writing {error:?}");
            ExitCode::FAILURE
        }
    }
}

pub fn read_source<P: Into<PathBuf>>(path: P, library: &mut SourceLibrary) -> IoResult<usize> {
    let path: PathBuf = path.into().canonicalize()?;
    let source = read_to_string(&path)?;

    Ok(library.add(SourceFile {
        path: path.to_string_lossy().to_string(),
        data: source,
    }))
}
