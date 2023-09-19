use std::env;

mod builder;
mod cli;
mod parser;
mod recursive_descent_parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let parsed_args = match parser::ArgsParser::new(args.iter().peekable()).parse_args() {
        Ok(parsed_args) => parsed_args,
        Err(e) => {
            println!("{e:?}");
            return;
        }
    };
    println!("{parsed_args:?}");

    let cmd = match builder::build_args(parsed_args) {
        Ok(value) => value,
        Err(e) => {
            println!("{e:?}");
            return;
        }
    };
    println!("{cmd:?}");

    let _ = cli::run(cmd);
}
