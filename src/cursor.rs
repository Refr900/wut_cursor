use crate::{Base, Kind, LiteralKind, Token};

/// Lightweight iterator for character sequence
///
/// _Anything other than direct text analysis..._
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Cursor<'a>(&'a str);

const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self(source)
    }
}

impl<'a> Cursor<'a> {
    pub fn next(&mut self) -> Token {
        let start = self.len();
        let kind = self.match_kind();
        let len = self.token_len(start);
        Token::new(kind, len)
    }

    pub fn skip(&mut self) {
        self.next();
    }
}

impl<'a> Cursor<'a> {
    pub fn peek(&self, n: usize) -> Token {
        let mut cursor = self.clone();
        for _ in 0..n {
            cursor.skip();
        }
        cursor.next()
    }

    pub fn first(&self) -> Token {
        self.clone().next()
    }

    pub fn second(&self) -> Token {
        let mut lexer = self.clone();
        lexer.skip();
        lexer.next()
    }
}

impl<'a> Cursor<'a> {
    pub fn tokens<const N: usize>(&self) -> [Token; N] {
        let mut lexer = self.clone();
        let mut tokens = [Token::DUMMY; N];
        for i in 0..N {
            tokens[i] = lexer.next();
        }
        tokens
    }

    pub fn skip_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(Token) -> bool,
    {
        let token = self.first();
        while !token.is_eof() && predicate(token) {
            self.skip();
        }
    }

    pub fn skip_until<F>(&mut self, mut predicate: F)
    where
        F: FnMut(Token) -> bool,
    {
        self.skip_while(|token| !predicate(token));
    }
}

impl<'a> Cursor<'a> {
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_eof(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> Cursor<'a> {
    fn match_kind(&mut self) -> Kind {
        let Some(first) = self.next_char() else {
            return Kind::Eof;
        };

        match first {
            c if c.is_whitespace() => self.space(c),
            c if is_ident_start(c) => self.ident(),
            c if !c.is_ascii() => self.invalid_ident(),
            '0' => {
                let base = match self.base() {
                    Ok(base) => base,
                    Err(JustZero) => {
                        return Kind::Literal(LiteralKind::Int {
                            base: Base::Decimal,
                            empty: false,
                        })
                    }
                };
                Kind::Literal(self.number_with_base(base))
            }
            '1'..='9' => Kind::Literal(self.decimal_number()),
            '\'' => Kind!['\''],
            '"' => Kind!['"'],
            ';' => Kind![';'],
            ':' => Kind![':'],
            ',' => Kind![','],
            '.' => Kind!['.'],
            '@' => Kind!['@'],
            '#' => Kind!['#'],
            '~' => Kind!['~'],
            '?' => Kind!['?'],
            '$' => Kind!['$'],
            '=' => Kind!['='],
            '!' => Kind!['!'],
            '<' => Kind!['<'],
            '>' => Kind!['>'],
            '-' => Kind!['-'],
            '&' => Kind!['&'],
            '|' => Kind!['|'],
            '+' => Kind!['+'],
            '*' => Kind!['*'],
            '/' => Kind!['/'],
            '^' => Kind!['^'],
            '%' => Kind!['%'],
            '(' => Kind!['('],
            ')' => Kind![')'],
            '{' => Kind!['{'],
            '}' => Kind!['}'],
            '[' => Kind!['['],
            ']' => Kind![']'],
            _ => self.unknown(),
        }
    }
}

impl<'a> Cursor<'a> {
    const fn token_len(&self, start: usize) -> u32 {
        (start - self.len()) as u32
    }
}

impl<'a> Cursor<'a> {
    fn space(&mut self, first: char) -> Kind {
        let mut newline = first == '\n';
        self.skip_while_char(|c| {
            if c == '\n' {
                newline = true;
                return true;
            }
            c.is_whitespace()
        });
        Kind::Space { newline }
    }
}

impl<'a> Cursor<'a> {
    fn ident(&mut self) -> Kind {
        self.skip_while_char(is_ident_continue);
        if !self.first_char().is_ascii() {
            return self.invalid_ident();
        }
        Kind::Ident
    }

    fn invalid_ident(&mut self) -> Kind {
        self.skip_while_char(char::is_alphanumeric);
        Kind::InvalidIdent
    }
}

#[derive(Debug, Clone, Copy)]
struct JustZero;

impl<'a> Cursor<'a> {
    fn base(&mut self) -> Result<Base, JustZero> {
        // We've already missed the starting `0`
        // Check special character
        let base = match self.first_char() {
            'b' => Base::Binary,
            'o' => Base::Octal,
            'x' => Base::Hexadecimal,
            // number continue
            '0'..='9' => return Ok(Base::Decimal),
            // Just zero
            _ => return Err(JustZero),
        };
        // Skip special character
        self.skip_char();
        Ok(base)
    }

    fn number_with_base(&mut self, base: Base) -> LiteralKind {
        // We already have at least one number
        if matches!(base, Base::Decimal) {
            self.skip_decimal();
        } else {
            if matches!(self.skip_hex(), HasDigits::No) {
                return LiteralKind::Int {
                    base: Base::Decimal,
                    empty: true,
                };
            }
        }
        self.maybe_float(base)
    }

    fn decimal_number(&mut self) -> LiteralKind {
        self.skip_decimal();
        self.maybe_float(Base::Decimal)
    }

    fn maybe_float(&mut self, base: Base) -> LiteralKind {
        let [first, second] = self.two_chars();

        if first != '.' {
            return LiteralKind::Int { base, empty: false };
        }
        // skip `.`
        self.skip_char();

        if let Base::Hexadecimal = base {
            if second.is_ascii_hexdigit() {
                self.skip_hex();
            }
        } else {
            if second.is_ascii_digit() {
                self.skip_decimal();
            }
        }

        LiteralKind::Float { base }
    }
}

#[derive(Debug, Clone, Copy)]
enum HasDigits {
    Yes,
    No,
}

impl<'a> Cursor<'a> {
    fn skip_decimal(&mut self) -> HasDigits {
        let mut has_digits = HasDigits::No;
        loop {
            match self.first_char() {
                '0'..='9' => has_digits = HasDigits::Yes,
                '_' => (),
                _ => break,
            }
            self.skip_char();
        }
        has_digits
    }

    fn skip_hex(&mut self) -> HasDigits {
        let mut has_digits = HasDigits::No;
        loop {
            match self.first_char() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => has_digits = HasDigits::Yes,
                '_' => (),
                _ => break,
            }
            self.skip_char();
        }
        has_digits
    }
}

impl<'a> Cursor<'a> {
    fn unknown(&mut self) -> Kind {
        self.skip_while_char(|c| !c.is_ascii() && c.is_alphanumeric());
        Kind::Unknown
    }
}

// region: ----- Space -----

#[derive(Debug, Clone, Copy)]
pub struct Space {
    pub newline: bool,
    pub len: u32,
}

impl<'a> Cursor<'a> {
    pub fn parse_space(&mut self) -> Space {
        let start = self.len();
        let mut newline = false;
        self.skip_while_char(|c| {
            if c == '\n' {
                newline = true;
                return true;
            }
            c.is_whitespace()
        });
        Space {
            newline,
            len: self.token_len(start),
        }
    }
}

// endregion: ----- Space -----

// region: ----- Char -----

#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub terminated: bool,
    pub is_one_symbol: bool,
    pub len: u32,
}

impl<'a> Cursor<'a> {
    /// Parse char but skipping parsing first apostrophe but including it in len
    pub fn parse_char_continue(&mut self) -> Char {
        // include apostrophe
        let start = self.len() + 1;

        if self.second_char() == '\'' && self.first_char() != '\\' {
            // skip char
            self.skip_char();
            // skip '\''
            self.skip_char();
            return Char {
                terminated: true,
                is_one_symbol: true,
                len: self.token_len(start),
            };
        }

        let mut char_count = 0;
        loop {
            if self.is_eof() {
                break;
            }

            match self.first_char() {
                // end '\''
                '\'' => {
                    self.skip_char();
                    return Char {
                        terminated: true,
                        is_one_symbol: char_count == 1,
                        len: self.token_len(start),
                    };
                }
                // newline is not supported
                '\n' => break,
                // escaped character
                '\\' => {
                    self.skip_char();
                    self.skip_char();
                }
                // just character
                _ => self.skip_char(),
            }
            char_count += 1;
        }

        Char {
            terminated: false,
            is_one_symbol: false,
            len: self.token_len(start),
        }
    }
}

// endregion: ----- Char -----

// region: ----- String -----

#[derive(Debug, Clone, Copy)]
pub struct Str {
    pub terminated: bool,
    pub len: u32,
}

impl<'a> Cursor<'a> {
    /// Parse string but skipping parsing first quote but including it in len
    pub fn parse_str_continue(&mut self) -> Str {
        // include quote
        let start = self.len() + 1;
        loop {
            if self.is_eof() {
                break;
            }

            match self.first_char() {
                // end `"`
                '"' => {
                    self.skip_char();
                    return Str {
                        terminated: true,
                        len: self.token_len(start),
                    };
                }
                // escaped character
                '\\' => {
                    self.skip_char();
                    self.skip_char();
                }
                // just character
                _ => self.skip_char(),
            }
        }

        Str {
            terminated: false,
            len: self.token_len(start),
        }
    }
}

// endregion: ----- String -----

impl<'a> Cursor<'a> {
    /// Returns the first character and moving to the next character.
    #[inline]
    pub fn next_char(&mut self) -> Option<char> {
        let mut chars = self.0.chars();
        let char = chars.next();
        self.0 = chars.as_str();
        char
    }

    /// Skip the current character.
    #[inline(always)]
    pub fn skip_char(&mut self) {
        self.next_char();
    }
}

impl<'a> Cursor<'a> {
    pub fn first_char(&mut self) -> char {
        self.0.chars().next().unwrap_or(EOF)
    }

    pub fn second_char(&self) -> char {
        let mut chars = self.0.chars();
        chars.next();
        chars.next().unwrap_or(EOF)
    }

    pub fn two_chars(&mut self) -> [char; 2] {
        let mut chars = self.0.chars();
        let first = chars.next().unwrap_or(EOF);
        let second = chars.next().unwrap_or(EOF);
        [first, second]
    }
}

impl<'a> Cursor<'a> {
    pub fn skip_while_char<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while !self.is_eof() && predicate(self.first_char()) {
            self.skip_char();
        }
    }
}

#[inline]
fn is_ident_start(c: char) -> bool {
    // we support only ascii for identifiers
    c.is_ascii_alphabetic() || c == '_'
}

#[inline]
fn is_ident_continue(c: char) -> bool {
    // we support only ascii for identifiers
    c.is_ascii_alphanumeric() || c == '_'
}
