extern crate env_logger;
extern crate epiqs;
use std::env;
use epiqs::printer::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let file_name = &args[1];
        // let file = File::open(file_name).unwrap();
        // let reader = BufReader::new(file);
        println!("{}", evaled_reader(file_name));
        // let mut s = String::new();
        // file.read_to_string(&mut s).unwrap();
        // println!("{}", evaled_str(&s));
    } else {
        println!("{}", "sorry, REPL is now developing, please wait a bit...");
    }
}
