use error_system::formatted_error;

type ScannerResult = Result<Token, ()>;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Comment,
    Eof,
}

pub struct TokenScanner<'a> {
    src: &'a [u8],
    current: usize,
}

impl<'a> TokenScanner<'a> {
    pub fn new(src: &[u8]) -> TokenScanner {
        TokenScanner {
            src,
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> ScannerResult {
        while !self.is_finished() && self.src[self.current].is_ascii_whitespace() {
            self.current += 1;
        }

        if self.is_finished() {
            return Ok(Token {
                kind: TokenKind::Eof,
                start: self.current,
                end: self.current,
            });
        }

        let token = self.scan_token()?;
        self.current = token.end;
        Ok(token)
    }

    fn scan_token(&self) -> ScannerResult {
        match self.current() {
            b'(' => Ok(Token {
                kind: TokenKind::LeftParen,
                start: self.current,
                end: self.current + 1,
            }),

            b')' => Ok(Token {
                kind: TokenKind::RightParen,
                start: self.current,
                end: self.current + 1,
            }),

            b'{' => Ok(Token {
                kind: TokenKind::LeftBrace,
                start: self.current,
                end: self.current + 1,
            }),

            b'}' => Ok(Token {
                kind: TokenKind::LeftBrace,
                start: self.current,
                end: self.current + 1,
            }),

            b',' => Ok(Token {
                kind: TokenKind::Comma,
                start: self.current,
                end: self.current + 1,
            }),

            b'.' => Ok(Token {
                kind: TokenKind::Dot,
                start: self.current,
                end: self.current + 1,
            }),

            b'-' => Ok(Token {
                kind: TokenKind::Minus,
                start: self.current,
                end: self.current + 1,
            }),

            b'+' => Ok(Token {
                kind: TokenKind::Plus,
                start: self.current,
                end: self.current + 1,
            }),

            b';' => Ok(Token {
                kind: TokenKind::Semicolon,
                start: self.current,
                end: self.current + 1,
            }),

            b'*' => Ok(Token {
                kind: TokenKind::Start,
                start: self.current,
                end: self.current + 1,
            }),

            b'!' if self.matches(1, b'=') => Ok(Token {
                kind: TokenKind::BangEqual,
                start: self.current,
                end: self.current + 2,
            }),

            b'!' => Ok(Token {
                kind: TokenKind::Bang,
                start: self.current,
                end: self.current + 2,
            }),

            b'=' if self.matches(1, b'=') => Ok(Token {
                kind: TokenKind::EqualEqual,
                start: self.current,
                end: self.current + 2,
            }),

            b'=' => Ok(Token {
                kind: TokenKind::Equal,
                start: self.current,
                end: self.current + 2,
            }),

            b'<' if self.matches(1, b'=') => Ok(Token {
                kind: TokenKind::LessEqual,
                start: self.current,
                end: self.current + 2,
            }),

            b'<' => Ok(Token {
                kind: TokenKind::Less,
                start: self.current,
                end: self.current + 2,
            }),

            b'>' if self.matches(1, b'=') => Ok(Token {
                kind: TokenKind::GreaterEqual,
                start: self.current,
                end: self.current + 2,
            }),

            b'>' => Ok(Token {
                kind: TokenKind::Greater,
                start: self.current,
                end: self.current + 2,
            }),

            b'/' if self.matches(1, b'/') => {
                let start_of_comment = self.current + 1;
                let end_of_comment = self.src[start_of_comment..].iter().enumerate().find_map(|(id, &c)| {
                    if c == b'\n' {
                        Some(id + 1)
                    } else {
                        None
                    }
                });

                Ok(Token {
                    kind: TokenKind::Comment,
                    start: self.current,
                    end: end_of_comment.unwrap_or(self.src.len()),
                })
            }

            b'/' => Ok(Token {
                kind: TokenKind::Slash,
                start: self.current,
                end: self.current + 1,
            }),

            unknown => {
                let unknown: char = unknown.into();
                let start = self.current;
                let end = start + 1;
                formatted_error!("Unknown token: {unknown} at {start}:{end}");
                Err(())
            }
        }
    }

    fn matches(&self, offset: usize, expect: u8) -> bool {
        let possible_token = self.src.get(self.current + offset);
        possible_token.map_or(false, |token| *token == expect)
    }

    fn is_finished(&self) -> bool {
        self.current == self.src.len()
    }

    fn current(&self) -> u8 {
        self.src[self.current]
    }
}

pub fn parse(code: &[u8]) {
    let mut token_scanner = TokenScanner::new(code);
    loop {
        let Ok(token) = token_scanner.next_token() else {
            break;
        };

        println!("{token:?}");

        if let TokenKind::Eof = token.kind {
            break;
        }
    }
}
