use clap::{Command, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
struct Args {
    lines: u64,
    bytes: Option<u64>,
    files: Vec<String>,
}

fn get_args() -> Args {
    let matches = Command::new("headr")
        .version("1.0")
        .author("Adam Rasool | github.com/Shwads")
        .about("Unix's head in Rust")
        .arg(Arg::new("lines")
            .short('n')
            .long("lines")
            .value_name("LINES")
            .help("Number of lines")
            .value_parser(clap::value_parser!(u64))
            .default_value("10"),
        )
        .arg(Arg::new("bytes")
            .short('c')
            .long("bytes")
            .value_name("BYTES")
            .value_parser(clap::value_parser!(u64))
            .conflicts_with("lines")
            .help("Number of bytes"),
        )
        .arg(Arg::new("files")
            .value_name("FILES")
            .num_args(1..)
            .help("Input files")
            .default_value("-"),
        )
        .try_get_matches()
        .unwrap_or_else(|err| {
            //eprintln!("{err}");
            // eprintln!("Couldn't unwrap the arguments");
            eprintln!("{}", err);
            std::process::exit(1);
       });

    Args {
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
        files: matches.get_many("files").unwrap().cloned().collect(),
    }
}

// ---------------------------------------------------------------------------------------------------------

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    //println!("{:?}", args);

    match args.bytes {
        Some(x) => {
            let mut file_count = 1;

            for filename in &args.files {
                let file = open(&filename);
                match file {
                    Err(err) => eprintln!("{}: {}", filename, err),
                    _ => {

                        if args.files.len() > 1 {
                            println!("==> {} <==", filename);
                        }

                        let mut store_bytes: Vec<u8> = vec![];
                        let mut as_bytes = file.unwrap().bytes();

                        for _n in 0..x {
                            match as_bytes.next() {
                                Some(Ok(byte)) => store_bytes.push(byte),
                                _ => break,
                            }
                        }

                        let bytes_as_string = String::from_utf8_lossy(&store_bytes);
                        print!("{}", bytes_as_string);

                        if file_count < args.files.len() {
                            println!();
                        }
                    },
                }
                file_count += 1;
            }

        },
        None => {
            let mut count = 1;
            for filename in &args.files {
                let file = open(&filename);

                match file {
                    Err(err) => eprintln!("{}: {}", filename, err),
                    _ => {
                        if args.files.len() > 1 {
                            println!("==> {} <==", filename);
                        }
                        let mut buf: Vec<u8> = vec![];
                        let mut unwrapped_file = file.unwrap();

                        for _count in 0..args.lines {
                            let _num_bytes = unwrapped_file.read_until(b'\n', &mut buf);
                            print!("{}", String::from_utf8_lossy(&buf));
                            buf.clear();
                        }

                        // let mut as_iterator = file.unwrap().lines();

                        // for _count in 0..args.lines {
                        //     let next_line = as_iterator.next();
                        //     match next_line {
                        //         Some(Ok(line)) => println!("{}", line),
                        //         _ => break,
                        //     }
                        // }
                        if count < args.files.len() {
                            println!();
                        }
                    },
                }
                count += 1;
            }
        },
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------

fn open(filename: &str) -> Result<Box< dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _   => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}