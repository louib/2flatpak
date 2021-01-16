use std::env;
use std::process::exit;
use std::io::{self, BufRead};

fn main() {
    let mut exit_code = 0;

    // TODO might need to use std::env::args_os instead, if
    // the args contain unicode.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Require 1 argument.");
        exit(1);
    }

    let command_name = &args[1];

    if command_name == &"import-modules".to_string() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line.unwrap());
        }
        println!("ici");
    }

    exit(exit_code);
}
