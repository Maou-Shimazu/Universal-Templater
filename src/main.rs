mod build;
mod ts;

use build::ProjectBuilder;
use clap::{Parser, Subcommand};
use ts::Ts;

/// Templater engine for all your favorite langs.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct UT {
    #[clap(subcommand)]
    commands: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates a typescript project
    Ts {},
}

fn main() {
    let args = UT::parse();
    match args.commands {
        Some(Command::Ts {}) => {
            let ts = Ts::new();
            match ts.build() {
                Ok(_) => {}
                Err(e) => {
                    println!("There was an error creating ts project: {:?}. Report to a developer. Exiting", e);
                    std::process::exit(0);
                }
            };
        }
        None {} => {
            std::process::Command::new(env!("CARGO_BIN_NAME"))
                .arg("-h")
                .spawn()
                .unwrap();
        }
    }
}
