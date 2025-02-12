macro_rules! token {
    ($kind:expr, $self:expr, $offset:expr) => {
        Token {
            kind: $kind,
            start: $self.current,
            end: $self.current + $offset,
        }
    };
}

/// States of the number recognition automata (NRA).
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum NraState {
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
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.current().as_ref().is_some_and(u8::is_ascii_whitespace) {
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
                return token!(TokenKind::Comment, self, comment_offset);
            }
        }

        token!(TokenKind::Comment, self, comment_offset)
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
        let mut state = NraState::State0;

        loop {
            match src_tail.get(offset).copied() {
                Some(b'.') if state == NraState::State0 => state = NraState::State1,
                Some(t) if t.is_ascii_digit() && state == NraState::State1 => state = NraState::State2,
                Some(t) if t.is_ascii_digit() => (),
                _ => break,
            }

            offset += 1;
        }

        match state {
            NraState::State0 => token!(TokenKind::Integer, self, offset),
            NraState::State1 => token!(TokenKind::Integer, self, offset - 1),
            NraState::State2 => token!(TokenKind::Decimal, self, offset),
        }
    }

    fn identifier(&self) -> Token {
        let src_tail = &self.src[self.current..];
        let belongs_in_identifier = |token_byte: &u8| token_byte.is_ascii_alphanumeric() || (*token_byte == b'_');

        let mut offset = 0;
        while src_tail.get(offset).is_some_and(belongs_in_identifier) {
            offset += 1;
        }

        let token_kind = keywords(&src_tail[..offset]).unwrap_or(TokenKind::Identifier);

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

#[inline]
fn keywords(text: &[u8]) -> Option<TokenKind> {
    match text {
        b"and" => Some(TokenKind::And),
        b"class" => Some(TokenKind::Class),
        b"else" => Some(TokenKind::Else),
        b"false" => Some(TokenKind::False),
        b"for" => Some(TokenKind::For),
        b"fun" => Some(TokenKind::Fun),
        b"if" => Some(TokenKind::If),
        b"nil" => Some(TokenKind::Nil),
        b"or" => Some(TokenKind::Or),
        b"print" => Some(TokenKind::Print),
        b"return" => Some(TokenKind::Return),
        b"super" => Some(TokenKind::Super),
        b"this" => Some(TokenKind::This),
        b"true" => Some(TokenKind::True),
        b"var" => Some(TokenKind::Var),
        b"while" => Some(TokenKind::While),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(b"// This is a comment", TokenKind::Comment; "comment")]
    #[test_case(b"@", TokenKind::Unknown; "unknown")]
    #[test_case(b"while", TokenKind::While; "while_stmt")]
    #[test_case(b"var", TokenKind::Var;"var")]
    #[test_case(b"true", TokenKind::True; "true_case")]
    #[test_case(b"this", TokenKind::This; "this")]
    #[test_case(b"super", TokenKind::Super; "super_op")]
    #[test_case(b"return", TokenKind::Return; "return_stmt")]
    #[test_case(b"print", TokenKind::Print; "print")]
    #[test_case(b"or", TokenKind::Or; "or")]
    #[test_case(b"nil", TokenKind::Nil; "nil")]
    #[test_case(b"if", TokenKind::If; "if_case")]
    #[test_case(b"for", TokenKind::For; "for_loop")]
    #[test_case(b"fun", TokenKind::Fun; "fun")]
    #[test_case(b"false", TokenKind::False; "false_case")]
    #[test_case(b"else", TokenKind::Else; "else_case")]
    #[test_case(b"class", TokenKind::Class; "class")]
    #[test_case(b"and", TokenKind::And; "and")]
    #[test_case(b"42.24", TokenKind::Decimal; "decimal")]
    #[test_case(b"42", TokenKind::Integer; "integer")]
    #[test_case(b"\"this is a string\"", TokenKind::String; "string")]
    #[test_case(b"id32_id", TokenKind::Identifier; "identifier_underscore")]
    #[test_case(b"id32", TokenKind::Identifier; "identifier_alphanumeric")]
    #[test_case(b"id", TokenKind::Identifier; "identifier_alpha")]
    #[test_case(b"<=", TokenKind::LessEqual; "less_equal")]
    #[test_case(b"<", TokenKind::Less; "less")]
    #[test_case(b">=", TokenKind::GreaterEqual; "greater_equal")]
    #[test_case(b">", TokenKind::Greater; "greater")]
    #[test_case(b"==", TokenKind::EqualEqual; "equal_equal")]
    #[test_case(b"=", TokenKind::Equal; "equal")]
    #[test_case(b"!=", TokenKind::BangEqual; "bang_equal")]
    #[test_case(b"!", TokenKind::Bang; "bang")]
    #[test_case(b"*", TokenKind::Star; "star")]
    #[test_case(b"/", TokenKind::Slash; "slash")]
    #[test_case(b";", TokenKind::Semicolon; "semicolon")]
    #[test_case(b"+", TokenKind::Plus; "plus")]
    #[test_case(b"-", TokenKind::Minus; "minus")]
    #[test_case(b".", TokenKind::Dot; "dot")]
    #[test_case(b",", TokenKind::Comma; "comma")]
    #[test_case(b"}", TokenKind::RightBrace; "right_brace")]
    #[test_case(b"{", TokenKind::LeftBrace; "left_brace")]
    #[test_case(b")", TokenKind::RightParen; "right_paren")]
    #[test_case(b"(", TokenKind::LeftParen; "left_paren")]
    fn single_token_test(source: &[u8], kind: TokenKind) {
        let mut stream = TokenStream::new(source);
        let token = stream.next_token();

        assert_eq!(token.kind, kind);
        assert_eq!(&source[token.start..token.end], source);

        let token = stream.next_token();

        assert_eq!(token.kind, TokenKind::Eof);
        assert_eq!(token.start, source.len());
        assert_eq!(token.end, source.len());
    }

    #[test]
    fn test_inline_comment() {
        let source = r#"
        // This is a comment
        if true {
            // This is also a comment
        }
        // This is my final comment"#;

        let mut stream = TokenStream::new(source.as_bytes());

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::Comment);
        assert_eq!(&source[token.start..token.end], "// This is a comment\n");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::If);
        assert_eq!(&source[token.start..token.end], "if");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::True);
        assert_eq!(&source[token.start..token.end], "true");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::LeftBrace);
        assert_eq!(&source[token.start..token.end], "{");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::Comment);
        assert_eq!(&source[token.start..token.end], "// This is also a comment\n");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::RightBrace);
        assert_eq!(&source[token.start..token.end], "}");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::Comment);
        assert_eq!(&source[token.start..token.end], "// This is my final comment");

        let token = stream.next_token();
        assert_eq!(token.kind, TokenKind::Eof);
        assert_eq!(source.len(), token.start);
    }
}
