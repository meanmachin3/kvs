use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
            // println!("'kvs get' was used, key is: {:?}", key)
        }
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
            // println!("'kvs set' was used, key is: {:?} and value is: {:?}", key, value)
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
            // println!("'kvs rm' was used, key is: {:?}", key)
        }
        _ => unreachable!(),
    }
}
