use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(help = "Input", min_values = 1, required = true)]
    text: Vec<String>,

    #[clap(short, help = "Do not output the trailing newline")]
    n: bool,
}
fn main() {
    let cli = Cli::parse();
    print!("{}{}", cli.text.join(" "), if cli.n { "" } else { "\n" });
}
