mod tokenizer;

use error_system::formatted_error;
use tokenizer::{TokenKind, TokenScanner};

pub fn parse(code: &[u8]) {
    for token in TokenScanner::new(code) {
        println!("{token:?}");

        if let TokenKind::Unknown = token.kind {
            let token_start = token.start;
            let token_end = token.end;
            let raw_token = std::str::from_utf8(&code[token_start..token_end]).unwrap();
            formatted_error!("Unknown token: {raw_token} at {token_start}:{token_end}");
            break;
        }
    }
}
