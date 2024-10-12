use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use error_system::formatted_error;

pub fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => prompt_mode(),
        2 => file_mode(&args[1]),
        other => {
            formatted_error!("Too many arguments, {other}");
            ExitCode::FAILURE
        }
    }
}

fn file_mode(file_path: &str) -> ExitCode {
    let file_path = Path::new(file_path);

    let Ok(file_path) = file_path.canonicalize() else {
        formatted_error!("Could not find {file_path:?}");
        return ExitCode::FAILURE;
    };

    match std::fs::read_to_string(&file_path) {
        Err(err) => formatted_error!("Could not read {file_path:?}: {err}"),
        Ok(source_code) => execute_code_pipeline(&source_code),
    }

    if error_system::error_flag() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn prompt_mode() -> ExitCode {
    let mut output = io::stdout();
    let mut buffer = String::new();

    loop {
        write!(&mut output, "> ").unwrap();
        output.flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        execute_code_pipeline(&buffer);
        buffer.clear();

        if error_system::error_flag() {
            return ExitCode::FAILURE;
        }
    }
}

fn execute_code_pipeline(code: &str) {
    parser::parse(code.as_bytes());
}
