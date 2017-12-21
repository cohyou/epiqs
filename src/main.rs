extern crate env_logger;
extern crate epiqs;
use epiqs::printer::*;

fn main() {
    env_logger::init().unwrap();

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
