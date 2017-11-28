use ::token::Tokn;

#[derive(Debug)]
pub enum Error {
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
