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

/*
#[test]
#[ignore]
fn evaled_empty_ast() {
    // let empty_ast = &RefCell::new(AbstractSyntaxTree::new());
    // let mut evaluator = Evaluator::new(empty_ast);
    // let evaled_ast = evaluator.walk().unwrap();
    let vm = craete_vm();// Rc::new(RefCell::new(Heliqs::new()));
    let printer = Printer::new(vm);
    assert_eq!(printer.print(), "");
}
*/

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
    print_evaled_str(r"|> ; |! |\ |% ; [a b c] |> ; ^> -1 [|@ ; c |@ ; b] [6667 6668 6669]", "6668")
}


#[test]
fn primitive_function() {
    // print_evaled_str("|> ; |@ ; decr", "Prim(decr)");
    // print_evaled_str("|> ; |@ ; ltoreq", "Prim(ltoreq)");
    print_evaled_str("|> ; |! |> ; |@ ; decr [5]", "4");
    print_evaled_str("|> ; |! |> ; |@ ; ltoreq [5 4]", "^F"); // 5 <= 4
    // print_evaled_str("|> ; ^> -1 [|! |> ; |@ ; ltoreq [5 4] |! |> ; |@ ; decr [5]]", ";");
    // print_evaled_str(r"|> ; ^> -1 [|! |> ; |@ ; ltoreq [5 4] |! |> ; |\ |% ; [x] |> ; ^> -1 [|@ ; x][5642]]", ";");
}

#[test]
// #[ignore]
fn tarai() {
    /*
    print_evaled_str(
        r"|> ; ^> -1 [
          |# abc |\ |% ; [x] ^> -1 [|@ ; x]
          |! |> ; |@ ; abc [8888]
        ]",
        "8888",
    );
    */

    /*
    print_evaled_str(
        r"|> ; ^> -1 [
          |# abc |\ |% ; [x]
                    ^> -1 [
                       |? |> ; |! |> ; |@ ; ltoreq [|> ; |@ ; x 10]
                          |: 1234
                             4321
                       ]
          |! |> ; |@ ; abc [8888]
        ]",
        "8888",
    );
    */

    // (defun tak (x y z)
    //   (if (<= x y)
    //       y
    //       (tak (tak (1- x) y z)
    //            (tak (1- y) z x)
    //            (tak (1- z) x y))))
    // (tak 12 6 0)

    /*
    print_evaled_str(
        r"|> ; ^> -1
        [
            |# recursive |\ |% ; [x]
                      ^> -1 [
                         |? |> ; |! |> ; |@ ; ltoreq [|> ; |@ ; x 0]
                            |: ^T
                               |> ; |! |> ; |@ ; recursive [0]
                      ]

            |! |> ; |@ ; recursive [1]
        ]",
        r";",
    );
    */

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

            |! |> ; |@ ; tak [12 6 0]
        ]",
        r";",
    );
}
