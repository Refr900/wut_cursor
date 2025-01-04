#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub len: u32,
}

impl Token {
    pub const DUMMY: Self = Self::new(Kind::Eof, 0);
    pub const EOF: Self = Self::new(Kind::Eof, 0);

    #[inline]
    pub const fn new(kind: Kind, len: u32) -> Self {
        Self { kind, len }
    }
}

impl Token {
    #[inline]
    pub const fn is_eof(&self) -> bool {
        self.kind.is_eof()
    }

    #[inline]
    pub const fn like_ident(&self) -> bool {
        self.kind.like_ident()
    }

    #[inline]
    pub const fn is_space(&self) -> bool {
        self.kind.is_space()
    }

    #[inline]
    pub const fn is_newline(&self) -> bool {
        self.kind.is_newline()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    /// Any spacing
    Space {
        newline: bool,
    },
    /// `ident`, `enum`, `_var123`
    Ident,
    /// No ascii idents
    InvalidIdent,
    /// Look [LiteralKind]
    Literal(LiteralKind),
    // Punctuation
    /// `"`
    Quote,
    /// `'`
    Apostrophe,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    Unknown,
    Eof,
}

impl Kind {
    #[inline]
    pub const fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }

    #[inline]
    pub const fn like_ident(&self) -> bool {
        matches!(self, Self::Ident | Self::InvalidIdent)
    }

    #[inline]
    pub const fn is_space(&self) -> bool {
        matches!(self, Self::Space { .. })
    }

    #[inline]
    pub const fn is_newline(&self) -> bool {
        matches!(self, Self::Space { newline: true })
    }
}

/// We are not parse chars and strings.
/// Let the upper levels define the syntax of strings and characters.
/// 
/// Or use ready-made
/// - [Char](crate::Cursor::parse_char_continue)
/// - [String](crate::Cursor::parse_str_continue)
#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    Int { base: Base, empty: bool },
    Float { base: Base },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}
