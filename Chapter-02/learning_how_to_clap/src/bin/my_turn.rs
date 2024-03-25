huse clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short='n')]
    skip_newline: bool,
    text: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli.skip_newline);
    println!("{:?}", cli.text);
}