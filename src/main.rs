use align::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    align::run(cli);
}
