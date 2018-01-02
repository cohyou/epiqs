macro_rules! all_scanners {
    () => {
        vec![
            &DelimiterScanner,
            &TextScanner,
            &AlphabetScanner,
            &ZeroScanner,
            &IntegerScanner,
            &EOFScanner,
        ]
    }
}

pub mod util;
pub mod core;
pub mod lexer;
pub mod parser;
pub mod walker;
pub mod printer;
