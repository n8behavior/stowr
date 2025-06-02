use clap::Parser;

/// Welcome to the CLI for Stowr
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the asses to show
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("Asset [{}] not found...yet!", args.name);
}
