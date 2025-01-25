macro_rules! token {
    ($kind:expr, $self:expr, $offset:expr) => {
        Token {
            kind: $kind,
            start: $self.current,
            end: $self.current + $offset,
            line: $self.line_number,
        }
    };
}

/// States of the number recognition automata (NRA).
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum NRAState {
    /// The integer part of a number is being parsed.
    State0,
    /// The integer part of a number was parsed and a dot was found.
    State1,
    /// Parsing the decimal part of a number.
    State2,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    /// First byte of the token in the source file.
    pub start: usize,
    /// End of the token in the source file.
    pub end: usize,
    /// Line where the token starts.
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    Star,
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
    Integer,
    Decimal,
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
    Unknown,
}

pub struct TokenStream<'a> {
    src: &'a [u8],
    line_number: usize,
    current: usize,
}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        match token.kind {
            TokenKind::Eof => None,
            _ => Some(token),
        }
    }
}

impl TokenStream<'_> {
    pub fn new(src: &[u8]) -> TokenStream {
        TokenStream {
            src,
            current: 0,
            line_number: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.current().as_ref().is_some_and(u8::is_ascii_whitespace) {
            if let Some(b'\n') = self.current() {
                self.line_number += 1;
            }

            self.current += 1;
        }

        let token = self.scan_token();
        self.current = token.end;

        token
    }

    fn scan_token(&mut self) -> Token {
        let Some(current_token) = self.current() else {
            return Token {
                kind: TokenKind::Eof,
                start: self.current,
                end: self.current,
                line: self.line_number,
            };
        };

        match current_token {
            b'(' => token!(TokenKind::LeftParen, self, 1),

            b')' => token!(TokenKind::RightParen, self, 1),

            b'{' => token!(TokenKind::LeftBrace, self, 1),

            b'}' => token!(TokenKind::RightBrace, self, 1),

            b',' => token!(TokenKind::Comma, self, 1),

            b'.' => token!(TokenKind::Dot, self, 1),

            b'-' => token!(TokenKind::Minus, self, 1),

            b'+' => token!(TokenKind::Plus, self, 1),

            b';' => token!(TokenKind::Semicolon, self, 1),

            b'*' => token!(TokenKind::Star, self, 1),

            b'!' if self.matches(1, b'=') => token!(TokenKind::BangEqual, self, 2),
            b'!' => token!(TokenKind::Bang, self, 1),

            b'=' if self.matches(1, b'=') => token!(TokenKind::EqualEqual, self, 2),
            b'=' => token!(TokenKind::Equal, self, 1),

            b'<' if self.matches(1, b'=') => token!(TokenKind::LessEqual, self, 2),
            b'<' => token!(TokenKind::Less, self, 1),

            b'>' if self.matches(1, b'=') => token!(TokenKind::GreaterEqual, self, 2),
            b'>' => token!(TokenKind::Greater, self, 1),

            b'/' if self.matches(1, b'/') => self.inline_comment(),
            b'/' => token!(TokenKind::Slash, self, 1),

            b'"' => self.string(),

            token if token.is_ascii_digit() => self.number(),

            token if token.is_ascii_alphabetic() => self.identifier(),

            _ => token!(TokenKind::Unknown, self, 1),
        }
    }

    fn inline_comment(&mut self) -> Token {
        let mut comment_offset = 0;

        for (offset, &i) in self.src[self.current..].iter().enumerate() {
            comment_offset = offset + 1;

            if i == b'\n' {
                break;
            }
        }

        let token = token!(TokenKind::Comment, self, comment_offset);
        self.line_number += 1;

        token
    }

    fn string(&self) -> Token {
        let mut src_iter = self.src[self.current..].iter().enumerate();

        src_iter.next();
        let end_of_string = src_iter.find(|(_, &token)| token == b'"').map(|t| t.0 + 1);

        match end_of_string {
            Some(end) => token!(TokenKind::String, self, end),
            None => token!(TokenKind::Unknown, self, self.src[self.current..].len()),
        }
    }

    fn number(&self) -> Token {
        let src_tail = &self.src[self.current..];
        let mut offset = 0;
        let mut state = NRAState::State0;

        loop {
            match src_tail.get(offset).copied() {
                Some(b'.') if state == NRAState::State0 => state = NRAState::State1,
                Some(t) if t.is_ascii_digit() && state == NRAState::State1 => state = NRAState::State2,
                Some(t) if t.is_ascii_digit() => (),
                _ => break,
            }

            offset += 1;
        }

        match state {
            NRAState::State0 => token!(TokenKind::Integer, self, offset),
            NRAState::State1 => token!(TokenKind::Integer, self, offset - 1),
            NRAState::State2 => token!(TokenKind::Decimal, self, offset),
        }
    }

    fn identifier(&self) -> Token {
        let src_tail = &self.src[self.current..];
        let belongs_in_identifier = |token_byte: &u8| token_byte.is_ascii_alphanumeric() || (*token_byte == b'_');

        let mut offset = 0;
        while src_tail.get(offset).is_some_and(belongs_in_identifier) {
            offset += 1;
        }

        let token_kind = KEYWORDS
            .iter()
            .find(|(key, _)| *key == &src_tail[..offset])
            .map(|kw| kw.1)
            .unwrap_or(TokenKind::Identifier);

        token!(token_kind, self, offset)
    }

    /// Checks if the byte at `self.current + offset` matches the
    /// expected one, false if the final index is out of bounds.
    fn matches(&self, offset: usize, expect: u8) -> bool {
        let possible_token = self.src.get(self.current + offset);
        possible_token.is_some_and(|token| *token == expect)
    }

    fn current(&self) -> Option<u8> {
        self.src.get(self.current).copied()
    }
}

const KEYWORDS: [(&[u8], TokenKind); 16] = [
    (b"and", TokenKind::And),
    (b"class", TokenKind::Class),
    (b"else", TokenKind::Else),
    (b"false", TokenKind::False),
    (b"for", TokenKind::For),
    (b"fun", TokenKind::Fun),
    (b"if", TokenKind::If),
    (b"nil", TokenKind::Nil),
    (b"or", TokenKind::Or),
    (b"print", TokenKind::Print),
    (b"return", TokenKind::Return),
    (b"super", TokenKind::Super),
    (b"this", TokenKind::This),
    (b"true", TokenKind::True),
    (b"var", TokenKind::Var),
    (b"while", TokenKind::While),
];
