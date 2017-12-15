macro_rules! all_scanners {
    () => {
        vec![
            &DelimiterScanner,
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
pub mod evaluator;
pub mod printer;
