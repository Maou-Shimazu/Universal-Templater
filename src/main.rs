mod build;
mod ts;

use build::{ProjectBuilder};
use ts::Ts;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct UT {
    #[clap(subcommand)]
    commands: Command
}

#[derive(Subcommand)]
enum Command {
    // Generate a typescript project
    Ts {},
}

fn main() {
    let args = UT::parse();
    match args.commands {
        Command::Ts {} => {
            let ts = Ts::new();
            match ts.build() {
                Ok(_) => {},
                Err(_) => {
                    println!("Error");
                }
            };
        }
    }
}