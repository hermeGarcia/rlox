use std::io;
use std::io::Write;
use std::process::ExitCode;

use error_system::formatted_error;

pub fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => prompt_mode(),
        2 => file_mode(&args[0]),
        other => {
            formatted_error!("Too many arguments, {other}");
            ExitCode::FAILURE
        }
    }
}

fn file_mode(_file_path: &str) -> ExitCode {
    todo!()
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

        if error_system::errors_found() {
            return ExitCode::FAILURE;
        }
    }
}

fn execute_code_pipeline(code: &str) {
    println!("[CODE] {code}");
    parser::parse(code.as_bytes());
}
