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

// 該当の文字で強制的にtokenの区切りとなるか（通常はホワイトスペースだが、それ以外にもあるかどうか）
// 様々なscannerで共通のため、ここに置く
// したがって、新しいscannerでtokenを区切る(finishする)どうかの基準は、まずはこの関数を参考にする

// 以下、Numberより
// 区切り文字ならここで数値を終わらせる必要がある
// ただし、全ての区切り文字がここで判断されるわけではない
// '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),

// 以下、Alphabetより
// 区切り文字ならここでNameを終わらせる必要がある
// ただし、全ての区切り文字がここで判断されるわけではない
// b'[' | b']' | b'(' | b')' | b'{' | b'}' | b':' | b',' => self.finish_with_state(state),
pub fn is_token_end_delimiter(c: u8) -> bool {
    c == b'[' || c == b']'
}
