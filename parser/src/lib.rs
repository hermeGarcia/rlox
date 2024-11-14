mod error;
mod tokenizer;

use context::src_library::Source;
use error::{ParserError, UnknownToken};
use tokenizer::{TokenKind, TokenScanner};

pub fn parse(src_id: Source, code: &[u8]) -> Result<(), ParserError> {
    for token in TokenScanner::new(code) {
        println!("{token:?}");

        if let TokenKind::Unknown = token.kind {
            return Err(ParserError::from(UnknownToken {
                start: token.start,
                end: token.end,
                line: token.line,
                source: src_id,
            }));
        }
    }

    Ok(())
}
