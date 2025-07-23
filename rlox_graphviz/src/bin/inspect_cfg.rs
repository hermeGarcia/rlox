// Example of graphviz:
// dot out/block.dot -T svg -o out/block.svg

use std::fs::File;
use std::fs::read_to_string;
use std::io::BufWriter;
use std::io::Result as IoResult;
use std::path::PathBuf;
use std::process::ExitCode;

use rlox_cf_graph::build_cfg;
use rlox_graphviz::cfg::Ctxt;
use rlox_source::{Source, SourceFile, SourceLibrary};

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
        eprintln!("Could not create graph.dot");
        return ExitCode::FAILURE;
    };

    let main_sequence = ast.main();
    let cf_graph = build_cfg::from_sequence_of_stmts(main_sequence, &ast);
    let ctxt = Ctxt {
        library,
        ast: &ast,
        cf_graph: &cf_graph,
    };

    match rlox_graphviz::cfg::graph(ctxt, &mut output) {
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
