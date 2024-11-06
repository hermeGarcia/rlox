mod error;
mod tokenizer;

use context::src_library::SourceKind;
use error::UnknownToken;
use error_system::error;
use tokenizer::{TokenKind, TokenScanner};

pub fn parse(src_id: SourceKind, code: &[u8]) -> Result<(), ()> {
    for token in TokenScanner::new(code) {
        println!("{token:?}");

        if let TokenKind::Unknown = token.kind {
            error(UnknownToken {
                start: token.start,
                end: token.end,
                line: token.line + 1,
                source: src_id,
            });
            return Err(());
        }
    }

    Ok(())
}
