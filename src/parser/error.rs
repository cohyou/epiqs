use core::*;
use super::TokenState;

#[derive(Debug)]
pub enum Error {
    UnknownError(u8),
    TokenError(Tokn),
    // Int8CastError,
    // TextError,
    // NotAexpError,
    // NotAffxError,
    // NotCpiqError,
    // CanNotCloseParenError,
    // NotTrueListError,
    // NotDebruijnIndexError,
    // Next(String),
    Expression(TokenState),
    TpiqSingle(TokenState),
    
    NotMatchError,
    Unimplemented,
}
