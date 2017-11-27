#[derive(Debug, Clone, PartialEq, Copy)]
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
