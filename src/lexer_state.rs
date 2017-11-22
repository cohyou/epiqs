#[derive(Debug, Clone, PartialEq)]
pub enum LexerState {
    Normal,
    InnerTag,
    InnerName,

    // ZeroNumber,
    // InnerNumber,
    // InnerText,
    // FinishText,
    // AfterUnderscore,
    // AfterDot,
    // InnerComment,
}
