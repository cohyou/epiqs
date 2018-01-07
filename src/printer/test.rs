use super::*;

#[test]
// #[ignore]
fn symbol() {
    print_str("abc", "abc");
}

#[test]
// #[ignore]
fn number() {
    print_str("123", "123");
}

#[test]
// #[ignore]
fn unit() {
    print_str(";", ";");
}

#[test]
#[ignore]
fn tpiq() {
    // print_str("|: abc 123", ":< abc 123 >");
    print_str("|# abc 123", "#< abc 123 >");
}

#[test]
// #[ignore]
fn nested_tpiq() {
    print_str("|: |: cde |: abc 123 456", ":(:(cde :(abc 123)) 456)");
}

#[test]
// #[ignore]
fn list() {
    print_str("[abc 123]", ":(abc :(123 ;))");
}

#[test]
// #[ignore]
fn empty_env() {
    print_str("|% ; -1", "%(; -1)");
}

#[test]
// #[ignore]
fn resolve_piq() {
    print_str("|@ abc ;", "@(abc ;)");
}

#[test]
// #[ignore]
fn block() {
    print_str(
        r"|> ; |! |\ |% ; ; ^> -1 [|# abc 123 |@ ; abc] ;",
        r">(; !(\(%(; ;) >(-1 :(#(abc 123) :(@(; abc) ;)))) ;))"
    );
}

#[test]
// #[ignore]
fn evaled_symbol_ast() {
    print_evaled_str("abc", "abc");
}


#[test]
// #[ignore]
fn evaled_number_ast() {
    print_evaled_str("123", "123");
}

#[test]
// #[ignore]
fn evaled_define_ast() {
    // define symbol is number
    print_evaled_str("|> ; |# abc 123", ";");
}

/*
#[test]
#[ignore]
fn evaled_apply() {
    print_evaled_str(r"|> ; |! |\ |% ; ; 0 ;", r"0");
}
*/

#[test]
// #[ignore]
fn evaled_list() {
    print_evaled_str(r"|> ; ^> -1 [1 2 3]", r"3");
}

#[test]
// #[ignore]
fn evaled_defining_list() {
    // print_evaled_str(r"|> ; |# abc 123", r";");
    // print_evaled_str(r"|> ; |@ ; abc", r";");
    print_evaled_str(r"|> ; ^> -1 [|# abc 1234 |@ ; abc]", r"1234");
}

#[test]
// #[ignore]
fn access() {
    // print_str("|. a p", ".(a p)");
    // print_evaled_str("|> ; |. |: 1 3 p", "p")
    print_evaled_str("|> ; |. |: 1 3 q", "3")
}

#[test]
// #[ignore]
fn condition() {
    // print_str("|? abc 123", "?(abc 123)");
    // print_str("^T", "^T");
    // print_str("^F", "^F");
    print_evaled_str("|> ; |? ^T |: 1 0", "1");
    print_evaled_str("|> ; |? ^F |: 1 0", "0");
}

#[test]
// #[ignore]
fn exec_func() {
    // print_str(r"|% ; ;", ";a")
    // print_str(r"|> ; |! |\ |% ; ; 1 ;", ";a")
    print_evaled_str(r"|> ; |! |\ |% ; [a b c] ^> -1 [|@ ; c |@ ; b] [6667 6668 6669]", "6668")
}


#[test]
#[ignore]
fn primitive_function() {
    // print_evaled_str("|> ; |@ ; decr", "Prim(decr)");
    // print_evaled_str("|> ; |@ ; ltoreq", "Prim(ltoreq)");
    print_evaled_str("|> ; |! |> ; |@ ; decr [5]", "4");
    print_evaled_str("|> ; |! |> ; |@ ; ltoreq [5 4]", "^F"); // 5 <= 4
    // print_evaled_str("|> ; ^> -1 [|! |> ; |@ ; ltoreq [5 4] |! |> ; |@ ; decr [5]]", ";");
    // print_evaled_str(r"|> ; ^> -1 [|! |> ; |@ ; ltoreq [5 4] |! |> ; |\ |% ; [x] |> ; ^> -1 [|@ ; x][5642]]", ";");
}

#[test]
#[ignore]
fn tarai() {
    print_evaled_str(
        r"|> ; ^> -1
        [
            |# tak |\ |% ; [x y z]
                      ^> -1 [
                         |? |> ; |! |> ; |@ ; ltoreq [|> ; |@ ; x |> ; |@ ; y]
                            |: |> ; |@ ; y
                               |> ; |! |> ; |@ ; tak [
                                  |> ; |! |> ; |@ ; tak [|> ; |! |> ; |@ ; decr [|> ; |@ ; x] |> ; |@ ; y |> ; |@ ; z]
                                  |> ; |! |> ; |@ ; tak [|> ; |! |> ; |@ ; decr [|> ; |@ ; y] |> ; |@ ; z |> ; |@ ; x]
                                  |> ; |! |> ; |@ ; tak [|> ; |! |> ; |@ ; decr [|> ; |@ ; z] |> ; |@ ; x |> ; |@ ; y]
                               ]
                      ]

            |! |> ; |@ ; tak [2 1 0]
        ]",
        r"2"
    );
}

#[test]
#[ignore]
fn fib() {
    print_evaled_str(
        r"|> ; ^> -1
        [
            |# fib |\ |% ; [n]
                      ^> -1 [
                         |? |> ; |! |> ; |@ ; eq [|> ; |@ ; n 0]
                            |: 0
                               |> ; |? |> ; |! |> ; |@ ; eq [|> ; |@ ; n 1]
                                       |: 1
                                       |> ; |! |> ; |@ ; plus [
                                          |> ; |! |> ; |@ ; fib [|> ; |! |> ; |@ ; minus [|> ; |@ ; n 2]]
                                          |> ; |! |> ; |@ ; fib [|> ; |! |> ; |@ ; minus [|> ; |@ ; n 1]]
                                       ]
                      ]
            |! |> ; |@ ; fib [30]
        ]",
        r"832040"
    );
}

#[test]
#[ignore]
fn prim() {
    print_evaled_str(
        r"|> ; ^> -1
        [
            |# x 1
            |# y 2
            |! |> ; |@ ; ltoreq [|> ; |@ ; x |> ; |@ ; y]
        ]",
        r"^T"
    );
}

#[test]
// #[ignore]
fn text() {
    print_str(r#""o58nkry drtse""#, r#""o58nkry drtse""#);
}

#[test]
// #[ignore]
fn evaled_text() {
    print_evaled_str(r#""o58nkry drtse""#, r#""o58nkry drtse""#);
}

#[test]
// #[ignore]
fn evaled_text_in_list() {
    print_str(r#"["bbb" 0 "aaa"]"#, r#":("bbb" :(0 :("aaa" ;)))"#);
}

#[test]
fn access_in_list() {
    print_evaled_str("'> |. |: 0 1 p", "0");
}

#[test]
fn single_quote() {
    print_str("'> [a]", ">(; :(a ;))");
}

#[test]
fn resolve_syntax_sugar() {
    print_str("@abc", ">(; @(; abc))");
}

#[test]
#[ignore]
fn apply_syntax_sugar() {
    print_syntax_sugar("abc! [1 2]", "'> |! abc [1 2]");
}

#[test]
#[ignore]
fn resolve_and_apply_syntax_sugar() {
    print_syntax_sugar("@abc! [1 3]", "'> |! '> '@ abc [1 3]");
}

#[test]
fn lpiq() {
    print_syntax_sugar(r#"abc:"a""#, r#"|: abc "a""#);
}

#[test]
fn nested_lpiq() {
    print_syntax_sugar(r#"abc:"a":1"#, r#"|: abc |: "a" 1"#);
}

#[test]
#[ignore]
fn accessor() {
    print_syntax_sugar("a.b", "'> |. a b");
}

#[test]
#[ignore]
fn nested_accessor() {
    print_syntax_sugar("a.b.c", " '> |. '> |. a b c");
}

#[test]
// #[ignore]
fn print() {
    print_evaled_str(r#"@print! ["a"]"#, ";");
}

#[test]
fn compare_texts() {
    print_evaled_str(r#"@eq! ["a" "b"]"#, "^F");
}

#[test]
fn concat_texts() {
    print_evaled_str(r#"@concat! ["a", "b"]"#, r#""ab""#);
}

#[test]
fn evaled_list_syntax_sugar() {
    // print_syntax_sugar("^[1 2 3]", "^> -1 [1 2 3]");
    print_syntax_sugar("^[@plus! [1 2]]", "^> ; [@plus! [1 2]]");
}

#[test]
fn quote() {
    print_evaled_str("'> || a", "a");
}
