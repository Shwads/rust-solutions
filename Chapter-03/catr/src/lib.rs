use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short='n', long="number", help="number lines")]
    number_lines: bool,
    #[arg(short='b', long="number-nonblank", help="number nonblank lines")]
    number_nonblank_lines: bool,
    #[arg(default_value="-")]
    files: Vec<String>
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _   => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

pub fn run() -> MyResult<()> {

    let mut output_lines: Vec<Option<String>> = vec![];

    let config = Config::parse();
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}", file_name, err),
            Ok(file) => {
                for line in file.lines() {
                    let line = line.unwrap();
                    output_lines.push(Some(line));
                }
            },
        }
        output_lines.push(None);
    }
    let mut line_count = 1;
    if config.number_lines && !config.number_nonblank_lines {
        for line in output_lines {
            match line {
                Some(text) => {
                    println!("{:>6}\t{}", line_count, text);
                    line_count += 1;
                }
                None => line_count = 1,
            }
        }
    } else if config.number_nonblank_lines {
        for line in output_lines {
            match line {
                Some(text) => {
                    if !(text.len() == 0) {
                        println!("{:>6}\t{}", line_count, text);
                        line_count += 1;
                    } else {
                        println!("{}", text);
                    }
                }
                None => line_count = 1,
            }
        }
    } else {
        for line in output_lines {
            if let Some(text) = line {
                println!("{}", text);
            }
        }
    }

    Ok(())
}