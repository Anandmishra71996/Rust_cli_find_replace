use std::{env, fs};

use regex::{self, Match, Regex};
use text_colorizer::Colorize;
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("usage: <targetString> <replacementString> <input_file> <output_file>");
}

fn parse_args() -> Arguments {
    let args: Vec<_> = env::args().skip(1).collect(); // Skip the first argument (program name)
    if args.len() != 4 {
        eprintln!("{} Wrong No of Args given.", "Error".red().bold());
        print_help();
        std::process::exit(1);
    }
    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}
fn replace(pattern: &String, data: &String, target: &String) -> Result<String, regex::Error> {
    let regex = Regex::new(pattern)?;
    Ok(regex.replace_all(data, target).to_string())
}

fn read_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} can not read file", "Error".red().bold());
            print_help();
            std::process::exit(1)
        }
    };
    let replaced_data = match replace(&args.pattern, &data, &args.replace) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} Error while replacing", "Error".red().bold());
            std::process::exit(1);
        }
    };
    match fs::write(&args.output_file, &replaced_data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} can not write file", "Error".red().bold())
        }
    }
}
pub fn run() {
    let args: Arguments = parse_args();
    read_write(&args);
}
