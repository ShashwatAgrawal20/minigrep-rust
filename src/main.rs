use std::env;
// use std::fs;
use std::process;

use minigrep::ArgsVal;

fn main() {
    // --------------------    My shitty code    --------------------

    // let args: Vec<String> = env::args().collect();
    // let to_search = &args[1];
    // let file = &args[2];
    // let file_contents = fs::read_to_string(file).expect("Error reading file");
    // for line in file_contents.lines() {
    //     if line.contains(to_search) {
    //         println!("line:- {}", line);
    //     }
    // }

    // --------------------  My shitty code ends --------------------


    // --------------------   Books Shitty Code  --------------------

    let args: Vec<String> = env::args().collect();
    let arg_val = ArgsVal::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(arg_val) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
