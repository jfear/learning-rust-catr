use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'n', long = "number", help = "Number lines")]
    number_lines: bool,
    #[arg(short = 'b', long = "number-nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let mut counter = 1;
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(stream) => {
                for line in stream.lines().map(|l| l.unwrap()) {
                    if args.number_lines {
                        println!("{:>6}\t{}", counter, line);
                        counter += 1;
                    } else if args.number_nonblank_lines && line.is_empty() {
                        println!("{:>6}\t{}", counter, line);
                        counter += 1;
                    } else {
                        println!("{}", line)
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
