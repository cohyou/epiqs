extern crate env_logger;
extern crate epiqs;
use epiqs::printer::*;

fn main() {
    env_logger::init().unwrap();
    /*
    only_evaluate(
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
        ]"
    );*/

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
        r";"
    );
}
