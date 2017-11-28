use lexer::token::Tokn;

#[derive(Debug)]
pub enum ParseError {
    UnknownError(u8),
    TokenError(Tokn)
    // Int8CastError,
    // TextError,
    // NotAexpError,
    // NotAffxError,
    // NotCpiqError,
    // CanNotCloseParenError,
    // NotTrueListError,
    // NotDebruijnIndexError,
    // Next(String),
}
