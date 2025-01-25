mod error;
mod expression;
mod token_stream;

use context::Source;
use error::ParserError;
use rlox_ast::Ast;
use token_stream::{Token, TokenKind, TokenStream};

type ParserResult<T> = Result<T, ParserError>;

struct Context<'a> {
    src: &'a [u8],
    src_id: Source,
    current: Token,
    stream: TokenStream<'a>,
}

impl Context<'_> {
    fn new(src_id: Source, src: &[u8]) -> Context {
        let mut stream = TokenStream::new(src);
        let start = stream.next_token();

        Context {
            src,
            src_id,
            stream,
            current: start,
        }
    }

    fn peek(&self) -> Token {
        self.current
    }

    fn consume_if(&mut self, expected: TokenKind) -> bool {
        if self.peek().kind == expected {
            self.consume();
            true
        } else {
            false
        }
    }

    fn consume(&mut self) -> Token {
        let current = self.current;
        self.current = self.stream.next_token();

        self.skip_comments();

        current
    }

    fn skip_comments(&mut self) {
        while matches!(self.current.kind, TokenKind::Comment) {
            self.current = self.stream.next_token();
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current.kind, TokenKind::Eof)
    }
}

fn panic_mode(ctxt: &mut Context) {
    loop {
        if ctxt.is_at_end() {
            break;
        }

        if matches!(ctxt.peek().kind, TokenKind::Semicolon) {
            ctxt.consume();
            break;
        }

        ctxt.consume();
    }
}

pub fn parse(src_id: Source, code: &[u8]) -> Result<Ast, Ast> {
    let mut is_valid = true;
    let mut ast = Ast::default();
    let mut ctxt = Context::new(src_id, code);

    ctxt.skip_comments();

    while !ctxt.is_at_end() {
        if let Err(error) = expression::parse(&mut ctxt, &mut ast) {
            is_valid = false;
            error_system::error(error);
            panic_mode(&mut ctxt);

            continue;
        }

        if !ctxt.consume_if(TokenKind::Semicolon) {
            error_system::error(ParserError::from(error::UnexpectedToken {
                start: ctxt.peek().start,
                end: ctxt.peek().end,
                line: ctxt.peek().line,
                source: ctxt.src_id,
                expected: vec![TokenKind::Semicolon],
            }));

            panic_mode(&mut ctxt);
            is_valid = false;
        }

        ctxt.skip_comments();
    }

    if is_valid {
        Ok(ast)
    } else {
        Err(ast)
    }
}
