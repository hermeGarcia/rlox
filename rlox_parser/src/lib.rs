mod error;
mod expression;
mod statement;
mod token_stream;

use error::ParserError;
use rlox_ast::Ast;
use rlox_source::Source;
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

pub enum AstStatus {
    Complete,
    Incomplete,
}

pub struct AstWithStatus {
    pub status: AstStatus,
    pub inner: Ast,
}

impl Default for AstWithStatus {
    fn default() -> Self {
        AstWithStatus {
            status: AstStatus::Complete,
            inner: Ast::default(),
        }
    }
}

impl AsRef<Ast> for AstWithStatus {
    fn as_ref(&self) -> &Ast {
        &self.inner
    }
}

impl AsMut<Ast> for AstWithStatus {
    fn as_mut(&mut self) -> &mut Ast {
        &mut self.inner
    }
}

impl From<AstWithStatus> for Result<Ast, Box<Ast>> {
    fn from(value: AstWithStatus) -> Self {
        match value.status {
            AstStatus::Complete => Ok(value.inner),
            AstStatus::Incomplete => Err(Box::new(value.inner)),
        }
    }
}

pub fn parse(src_id: Source, code: &[u8]) -> Result<Ast, Box<Ast>> {
    let mut ast = AstWithStatus::default();
    let mut ctxt = Context::new(src_id, code);

    ctxt.skip_comments();

    while !ctxt.is_at_end() {
        match statement::parse(&mut ctxt, ast.as_mut()) {
            Ok(stmt_id) => {
                ast.as_mut().push_into_initial_block(stmt_id);
                ctxt.skip_comments();
            }
            Err(error) => {
                ast.status = AstStatus::Incomplete;
                rlox_errors::error(error);
                panic_mode(&mut ctxt);

                continue;
            }
        }
    }

    Result::from(ast)
}
