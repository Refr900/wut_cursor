macro_rules! Kind {
    ['"']  => { $crate::Kind::Quote };
    ['\''] => { $crate::Kind::Apostrophe };
    [';']  => { $crate::Kind::Semi };
    [':']  => { $crate::Kind::Colon };
    [',']  => { $crate::Kind::Comma };
    ['.']  => { $crate::Kind::Dot };
    ['@']  => { $crate::Kind::At };
    ['#']  => { $crate::Kind::Pound };
    ['~']  => { $crate::Kind::Tilde };
    ['?']  => { $crate::Kind::Question };
    ['$']  => { $crate::Kind::Dollar };
    ['=']  => { $crate::Kind::Eq };
    ['!']  => { $crate::Kind::Bang };
    ['<']  => { $crate::Kind::Lt };
    ['>']  => { $crate::Kind::Gt };
    ['&']  => { $crate::Kind::And };
    ['|']  => { $crate::Kind::Or };
    ['+']  => { $crate::Kind::Plus };
    ['-']  => { $crate::Kind::Minus };
    ['*']  => { $crate::Kind::Star };
    ['/']  => { $crate::Kind::Slash };
    ['^']  => { $crate::Kind::Caret };
    ['%']  => { $crate::Kind::Percent };
    ['(']  => { $crate::Kind::OpenParen };
    [')']  => { $crate::Kind::CloseParen };
    ['{']  => { $crate::Kind::OpenBrace };
    ['}']  => { $crate::Kind::CloseBrace };
    ['[']  => { $crate::Kind::OpenBracket };
    [']']  => { $crate::Kind::CloseBracket };
}

pub(crate) use Kind;
