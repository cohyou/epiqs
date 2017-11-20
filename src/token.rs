use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Tokn {
    /* dispatcher */
    Pipe, // | vertical bar

    /* literal */
    Chvc(String), // Charactor Vector 単なる文字の並び

    /*
    Nmbr(String), // Number

    Usnm(String), // under score and number (e.g. _0 _34)

    // asciiのうち、記号は32(スペースを除く、また7Fも対象外)

    Dbqt, // " double quotation

    Lbkt, // [ left bracket
    Rbkt, // ] right bracket
    Lprn, // ( left parentheses
    Rprn, // ) right parentheses
    Lcrl, // { left curly brace
    Rcrl, // } right curly brace

    Coln, // : colon

    Crrt, // ^ carret
    Dllr, // $ dollar
    Smcl, // ; semi colon
    Bang, // ! exclamation

    // 残りの記号も列挙
    Plus, // + plus
    Star, // * asterisk
    Bksl, // \ back slash
    Stop, // . full stop (period)

    Pcnt, // % percent
    Qstn, // ? question mark
    Amps, // & ampersand
    Atsm, // @ at symbol
    Hash, // # hash

    Comm, // , comma
    */

    /*
    Slsh, // / slash

    Sgqt, // ' single quotation

    Hphn, // - hyphen-minus
    Less, // < less than
    Grtr, // > greater than
    Eqls, // = equal sign

    Udsc, // _ underscore Usnmとは別に一度定義しておく
    Tild, // ~ tilde
    Bkqt, // ` back quote
    */
}

impl fmt::Debug for Tokn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Tokn::Pipe => write!(f, "Pipe"),

            &Tokn::Chvc(ref s) => write!(f, "Chvc<{}>", s),

            /*
            &Tokn::Nmbr(ref s) => write!(f, "Nmbr<{}>", s),
            &Tokn::Usnm(ref s) => write!(f, "Usnm<{}>", s),

            &Tokn::Dbqt => write!(f, "Dbqt"),

            &Tokn::Lbkt => write!(f, "Lbkt"),
            &Tokn::Rbkt => write!(f, "Rbkt"),
            &Tokn::Lprn => write!(f, "Lprn"),
            &Tokn::Rprn => write!(f, "Rprn"),
            &Tokn::Lcrl => write!(f, "Lcrl"),
            &Tokn::Rcrl => write!(f, "Rcrl"),

            &Tokn::Coln => write!(f, "Coln"),

            &Tokn::Crrt => write!(f, "Crrt"),
            &Tokn::Dllr => write!(f, "Dllr"),
            &Tokn::Smcl => write!(f, "Smcl"),
            &Tokn::Bang => write!(f, "Bang"),

            // 扱いが不明瞭だがひとまず足しておく
            &Tokn::Plus => write!(f, "Plus"),
            &Tokn::Star => write!(f, "Star"),
            &Tokn::Stop => write!(f, "Stop"),
            &Tokn::Bksl => write!(f, "Bksl"),

            &Tokn::Pcnt => write!(f, "Pcnt"),
            &Tokn::Qstn => write!(f, "Qstn"),
            &Tokn::Amps => write!(f, "Amps"),
            &Tokn::Atsm => write!(f, "Atsm"),
            &Tokn::Hash => write!(f, "Hash"),

            &Tokn::Comm => write!(f, "Comm"),
            */

            _ => write!(f, "????"),
        }
    }
}
