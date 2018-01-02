extern crate epiqs;

use std::fs::File;
use std::io::prelude::*;

// use epiqs::lexer::*;
// use epiqs::parser::*;
use epiqs::printer::*;

#[test]
// #[ignore]
fn read_file() {
    let file_name = "sample/text.iq";
    let mut file = File::open(file_name).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    print_evaled_str(&s, "20");
}

/*
#[test]
fn lex_pipe() {
    lex_from_str("symbol", "Chvc<symbol>");
    lex_from_str("|: a b", "Pipe Otag<:> Chvc<a> Chvc<b>");

    // lex_from_str("|: a 0", "Pipe Otag<:> Chvc<a> Nmbr<0>");
    // lex_from_str("|: 0 a", "Pipe Otag<:> Nmbr<0> Chvc<a>");

    lex_from_str("0", "Nmbr<0>");

    lex_from_str("1", "Nmbr<1>");
    lex_from_str("21", "Nmbr<21>");

    lex_from_str("|: 478 349578", "Pipe Otag<:> Nmbr<478> Nmbr<349578>");
}
*/
/*
#[test]
#[ignore]
fn parse_pipe() {
    parse_from_str("a", "a");
    parse_from_str("|: a b", ":<a b>");
    parse_from_str("|Defn a b", "Defn<a b>");
    parse_from_str("|: |: a |: b c d", ":< :<a :<b c>> d>");
}
*/
/*
    fn is_alphabetic(c: u8) -> bool {
        (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
    }

    #[test]
    fn test_alphabeticality() {
        assert!(is_alphabetic(b'a'));
        assert!(is_alphabetic(b'j'));
        assert!(is_alphabetic(b'z'));
        assert!(is_alphabetic(b'A'));
        assert!(is_alphabetic(b'L'));
        assert!(is_alphabetic(b'Z'));
        // assert!(!is_alphabetic('Ａ'));
        assert!(!is_alphabetic(b'0'));
        assert!(!is_alphabetic(b'%'));
    }

    #[test]
    fn lex_true() { lexer_from_str("T", "Chvc<T>"); }
    #[test]
    fn lex_false() { lexer_from_str("F", "Chvc<F>"); }
    #[test]
    fn lex_none() { lexer_from_str("N", "Chvc<N>"); }

    #[test]
    fn lex_symbol() {
        lexer_from_str("symbol", "Chvc<symbol>");
        lexer_from_str("SYMBOL", "Chvc<SYMBOL>");
        lexer_from_str("SYMBOL12345", "Chvc<SYMBOL12345>");
    }

    #[test]
    fn lex_double_quote() {
        // lexer_from_str("\"double quote\"", "Dbqt Chvc<double quote> Dbqt");
        lexer_from_str("\"我輩は猫である。名前はまだない。\"", "Dbqt Chvc<我輩は猫である。名前はまだない。> Dbqt");
    }

    #[test]
    fn lex_multiple_tokens() {
        lexer_from_str("T N F", "Chvc<T> Chvc<N> Chvc<F>");
    }

    #[test]
    fn lex_parens() {
        lexer_from_str("()", "Lprn Rprn");
        lexer_from_str("    (  ) ", "Lprn Rprn");
        lexer_from_str("(a)", "Lprn Chvc<a> Rprn");
        lexer_from_str("(a 0 \"c\")", "Lprn Chvc<a> Nmbr<0> Dbqt Chvc<c> Dbqt Rprn");
    }

    #[test]
    fn lex_brackets() {
        lexer_from_str("[]", "Lbkt Rbkt");
        lexer_from_str(" [   ]     ", "Lbkt Rbkt");
        lexer_from_str("[53420417]", "Lbkt Nmbr<53420417> Rbkt");
        lexer_from_str("[\"i t ' s\" bang \"m i r a c l e\" done 9999]",
                       "Lbkt Dbqt Chvc<i t ' s> Dbqt Chvc<bang> Dbqt Chvc<m i r a c l e> Dbqt Chvc<done> Nmbr<9999> Rbkt");
    }

    #[test]
    fn lex_curly_braces() {
        // lexer_from_str("{}", "Lcrl Rcrl");
        // lexer_from_str("  { }   ", "Lcrl Rcrl");
        lexer_from_str("{\"a\"}", "Lcrl Dbqt Chvc<a> Dbqt Rcrl");
        lexer_from_str("{\"波かっこの中です\"}", "Lcrl Dbqt Chvc<波かっこの中です> Dbqt Rcrl");
        lexer_from_str("{name \"baad\" age 288 favorite machine}",
                       "Lcrl Chvc<name> Dbqt Chvc<baad> Dbqt Chvc<age> Nmbr<288> Chvc<favorite> Chvc<machine> Rcrl");
    }

    #[test]
    fn lex_nested_sequences() {
        lexer_from_str("(())", "Lprn Lprn Rprn Rprn");
        lexer_from_str("[[]]", "Lbkt Lbkt Rbkt Rbkt");
        lexer_from_str("{{}}", "Lcrl Lcrl Rcrl Rcrl");

        lexer_from_str("((a))", "Lprn Lprn Chvc<a> Rprn Rprn");
        lexer_from_str("(a ())", "Lprn Chvc<a> Lprn Rprn Rprn");
        lexer_from_str("(() a)", "Lprn Lprn Rprn Chvc<a> Rprn");

        lexer_from_str("[ [   a   b ]]", "Lbkt Lbkt Chvc<a> Chvc<b> Rbkt Rbkt");
        lexer_from_str("[a   b[ ]]", "Lbkt Chvc<a> Chvc<b> Lbkt Rbkt Rbkt");
        lexer_from_str(" [ [ ]a b]   ", "Lbkt Lbkt Rbkt Chvc<a> Chvc<b> Rbkt");

        lexer_from_str("{a{b}}", "Lcrl Chvc<a> Lcrl Chvc<b> Rcrl Rcrl");
        lexer_from_str("{{a}b}", "Lcrl Lcrl Chvc<a> Rcrl Chvc<b> Rcrl");
        lexer_from_str("{a{}b}", "Lcrl Chvc<a> Lcrl Rcrl Chvc<b> Rcrl");

        // Plus(+)とStar(*)が不明瞭なままテストの中に書いてしまったが、ひとまず問題が洗い出せただけでも素晴らしい
        lexer_from_str("{SUM (+ 1 6 (* s 7)) MEMBER [\"Tom\" \"John\" {first: \"Kazuo\" family: \"Ishiguro\"}]}",
                       "Lcrl Chvc<SUM> Lprn Plus Nmbr<1> Nmbr<6> Lprn Star Chvc<s> Nmbr<7> Rprn Rprn Chvc<MEMBER> Lbkt Dbqt Chvc<Tom> Dbqt Dbqt Chvc<John> Dbqt Lcrl Chvc<first> Coln Dbqt Chvc<Kazuo> Dbqt Chvc<family> Coln Dbqt Chvc<Ishiguro> Dbqt Rcrl Rbkt Rcrl");
    }

    #[test]
    fn lex_code_block_aexp() {
        lexer_from_str("(\\_1 ;)", "Lprn Bksl Usnm<1> Smcl Rprn");
        lexer_from_str("|\\_1 ;", "Pipe Bksl Usnm<1> Smcl");
        lexer_from_str(".\\_1", "Stop Bksl Usnm<1>");
    }

    #[test]
    fn lex_symbol_aexp() {
        lexer_from_str("($ \"a\" ;)", "Lprn Dllr Dbqt Chvc<a> Dbqt Smcl Rprn");
        lexer_from_str("|$ \"a\" ;", "Pipe Dllr Dbqt Chvc<a> Dbqt Smcl");
        lexer_from_str(".$ \"a\"", "Stop Dllr Dbqt Chvc<a> Dbqt");
    }

    #[test]
    fn lex_environment_aexp() {
        lexer_from_str("(% [.$ \"a\":1 .$ \"b\":\"two\" .$ \"c\":.\\ 3] ;)",
                       concat!("Lprn Pcnt ",
                                    "Lbkt ",
                                    "Stop Dllr Dbqt Chvc<a> Dbqt Coln Nmbr<1> ",
                                    "Stop Dllr Dbqt Chvc<b> Dbqt Coln Dbqt Chvc<two> Dbqt ",
                                    "Stop Dllr Dbqt Chvc<c> Dbqt Coln Stop Bksl Nmbr<3> ",
                                    "Rbkt Smcl Rprn"));
        lexer_from_str("|% ; ;", "Pipe Pcnt Smcl Smcl");
        lexer_from_str(".% ;", "Stop Pcnt Smcl");
    }

    #[test]
    fn lex_define_aexp() {
        lexer_from_str("(%+ .$ \"identity\":.\\_1)", "Lprn Pcnt Plus Stop Dllr Dbqt Chvc<identity> Dbqt Coln Stop Bksl Usnm<1> Rprn");
        lexer_from_str("|%+ .$ \"identity\":.\\_1 ;", "Pipe Pcnt Plus Stop Dllr Dbqt Chvc<identity> Dbqt Coln Stop Bksl Usnm<1> Smcl");
        lexer_from_str(".%+ .$ \"identity\":.\\_1", "Stop Pcnt Plus Stop Dllr Dbqt Chvc<identity> Dbqt Coln Stop Bksl Usnm<1>");
    }

    #[test]
    fn lex_apply_aexp() {
        lexer_from_str("(! .\\ 3)", "Lprn Bang Stop Bksl Nmbr<3> Rprn");
        lexer_from_str("(! .\\ _1 (100))", "Lprn Bang Stop Bksl Usnm<1> Lprn Nmbr<100> Rprn Rprn");
        lexer_from_str("|! .\\ _1 (100)", "Pipe Bang Stop Bksl Usnm<1> Lprn Nmbr<100> Rprn");
        lexer_from_str(".! .\\ 3", "Stop Bang Stop Bksl Nmbr<3>");
    }

    #[test]
    fn lex_condition_aexp() {
        lexer_from_str("(? T 1:2)", "Lprn Qstn Chvc<T> Nmbr<1> Coln Nmbr<2> Rprn");
        lexer_from_str("|? T 1:2", "Pipe Qstn Chvc<T> Nmbr<1> Coln Nmbr<2>");
    }

    #[test]
    fn lex_tuple_aexp() {
        lexer_from_str("(& a:b .& c:d)", "Lprn Amps Chvc<a> Coln Chvc<b> Stop Amps Chvc<c> Coln Chvc<d> Rprn");
        lexer_from_str("|& a:b .& c:d", "Pipe Amps Chvc<a> Coln Chvc<b> Stop Amps Chvc<c> Coln Chvc<d>");
        lexer_from_str("{a:b c:d}", "Lcrl Chvc<a> Coln Chvc<b> Chvc<c> Coln Chvc<d> Rcrl");
    }

    #[test]
    fn lex_deref_aexp() {
        // 迷うが、シンボル剥がしは頻繁に行われるので、@aの形式でも良いかも。
        lexer_from_str("(@ .$ \"a\")", "Lprn Atsm Stop Dllr Dbqt Chvc<a> Dbqt Rprn");
        lexer_from_str(".@ .$ \"a\"", "Stop Atsm Stop Dllr Dbqt Chvc<a> Dbqt");
        lexer_from_str("@a", "Atsm Chvc<a>");
    }

    #[test]
    fn lex_slice_and_destructure_aexp() {
        // # に関してはまだ考える余地があるかもね
        lexer_from_str("|# 1 [1 2 3]", "Pipe Hash Nmbr<1> Lbkt Nmbr<1> Nmbr<2> Nmbr<3> Rbkt");
        // そもそも後置で使いたいかも
        lexer_from_str("[1 2 3]#1", "Lbkt Nmbr<1> Nmbr<2> Nmbr<3> Rbkt Hash Nmbr<1>");
        lexer_from_str("[1 2 3]#L", "Lbkt Nmbr<1> Nmbr<2> Nmbr<3> Rbkt Hash Chvc<L>");
    }

    #[test]
    fn lex_enum_aexp() {
        // enumはまだふわっとしている
        // 型指定とか、そもそもメソッド追加できるとか、最近っぽいenumにするにはどうしたらいいんやろうか
        // でもそれは、tupleはともかく構造体について考える必要があるので、なんとも困る。スキーマの話でもある。
        lexer_from_str(".+ [a b c d]", "Stop Plus Lbkt Chvc<a> Chvc<b> Chvc<c> Chvc<d> Rbkt");
    }

    #[test]
    fn lex_embedding_aexp() {
        // これは完全にsyntax sugar。まだ考え中ですね。
        // カンマを使うと、次に現れるものの外側のかっこが取れる。しかしそれって、次に来るものが明らかになってないと嫌だな。
        // 主に、rubyの引数の最後で連想配列のカッコをはがせる、みたいなのが想定にある。
        // () list なのか [] vector or keyword-listなのか {} tupleなのか。
        // 正直なんでもいいよね、という時に使いたい。

        // a:[b:c, d:e] -> a,b:c,d:e
        lexer_from_str("a, b:c, d:e", "Chvc<a> Comm Chvc<b> Coln Chvc<c> Comm Chvc<d> Coln Chvc<e>");
    }

    #[test]
    fn lex_metadata() {
        // (^ ) のつもりですが、いったん飛ばします。
    }

    #[test]
    fn lex_comment() {
        // .. がコメントなのは割といけてる気がしている
        lexer_from_str(" .. a \n w", "Stop Stop Chvc< a > Chvc<w>");
    }

    #[test]
    fn lex_catch_exception() {
        // と思ったけど、構造は複雑だし、!?という内容なので、マクロ化したい
    }

    #[test]
    fn lex_about_async() {
        // 非同期処理も置いておこう。
        // spawnはコードブロック`\`の中で付加情報だな。非同期かどうか、待つのか、とかね。
        // yieldは、むしろ`|!#`かな。返却するので。

        // ちなみにparallelのマクロ案は`\&`。
    }

    #[test]
    fn lex_event_dispatch() {
        // イベントディスパッチも、置いておこう
        // イベントと対応する関数との対をテーブルに登録できるように。
        // %+でもいいけど、以下のマクロの方がわかりやすい。
        // `|!< event-name execution-block` ..これはマクロかも。
        // |_< 取消はこんな感じかなあ、ともかく、マクロですね。
    }*/

    /*
    fn parser_from_str(text: &str) -> (Result<(), parser::ParseError>, String) {
        let mut iter = text.bytes();
        let lexer = Lexer::new(&mut iter);
        let mut parser = Parser::new(lexer);
        let ret = parser.parse();
        let s = parser.print_aexp(parser.max_index());
        (ret, s)
    }

    #[test]
    fn parse_zero() {
        let s = "0";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Int8(0)");
    }

    #[test]
    fn parse_number() {
        let s = "12345";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Int8(12345)");
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_big_number() {
        // 今は文字列から数値に変換する際に桁あふれを起こしているが、その対応はいずれ
        let s = "123459578462357356342";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    fn parse_empty_text() {
        let s = "\"\"";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Text(\"\")");
    }

    #[test]
    fn parse_text() {
        let s = "\"string\"";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Text(\"string\")");
    }

    #[test]
    #[should_panic]
    fn parse_true() {
        // まだ真偽値リテラルは実装されていない
        let s = "T";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_false() {
        // まだ真偽値リテラルは実装されていない
        let s = "F";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_none() {
        // まだNoneリテラルは実装されていない(代わりにUnitがある)
        let s = "N";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }
    */
// }
/*
fn lex_from_str(text: &str, right: &str) {
    let mut iter = text.bytes();
    let mut lexer = Lexer::new(&mut iter);
    let mut result = String::from("");
    /*
    loop {
        match lexer.next_token() {
            Ok(t) => {
                println!("Ok: {:?}", t);
                let s = &format!("{:?} ", t);
                result.push_str(s);
                break;
            },
            Err(epiqs::lexer::Error::EOF) => {
                println!("{:?}", "Error: EOF");
                break;
            },
            Err(e) => {
                println!("Error: {:?}", e);
                let s = &format!("{:?} ", e);
                result = s.clone();
                break;
            }
        }
    }*/

    while let Ok(t) = lexer.next_token() {
        let s = &format!("{:?} ", t);
        result.push_str(s);
        println!("{:?}", s);
    }

    if result.len() > 0 {
        let len = result.len() - 1;
        result.remove(len);
        assert_eq!(result, right);
    } else {
        assert_eq!("", right);
    }
}

fn parse_from_str(text: &str, right: &str) {
    let mut iter = text.bytes();
    let lexer = Lexer::new(&mut iter);
    let mut parser = Parser::new(lexer);

    let mut result = String::from("");
    match parser.parse() {
        Ok(p) => {
            let s = &format!("{}", p);
            result.push_str(s);
        },
        Err(e) => {
            let s = &format!("{:?}", e);
            result.push_str(s);
        },
    }

    assert_eq!(result, right);
}
*/
