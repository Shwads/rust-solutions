use clap::Parser;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short='n', help="do not print newline.")]
    skip_newline: bool,
    #[arg(help="text to be echoed back.")]
    text: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    let text = cli.text.join(" ");

    if !cli.skip_newline {
        println!("{}", text);
    } else {
        io::stdout().flush().unwrap();
        print!("{}", text);
    }
}
