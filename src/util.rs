pub fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\t'
}

pub fn is_alphanumeric(c: u8) -> bool {
    is_alphabetic(c) || is_digit(c)
}

pub fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

pub fn is_alphabetic(c: u8) -> bool {
    is_alphabetic_uppercase(c) || is_alphabetic_lowercase(c)
}

pub fn is_alphabetic_uppercase(c: u8) -> bool {
    (c >= b'A' && c <= b'Z')
}

pub fn is_alphabetic_lowercase(c: u8) -> bool {
    (c >= b'a' && c <= b'z')
}
