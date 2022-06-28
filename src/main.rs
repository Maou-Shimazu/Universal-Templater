mod build;
mod structs;

use build::ProjectBuilder;
use clap::{Parser, Subcommand};
use structs::Ts;

/// Templater engine for all your favorite langs.
#[derive(Parser)]
#[clap(author, version, about)]
struct UT {
    #[clap(subcommand)]
    commands: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates a typescript project
    Ts {
        /// Name of project
        name: String,

        /// Deno project
        #[clap(short = 'D', long = "deno")]
        deno: bool,

        /// Npm project
        #[clap(short = 'N', long = "npm")]
        npm: bool,
    },
}

fn main() {
    let args = UT::parse();
    match args.commands {
        Some(Command::Ts {
            name,
            deno,
            npm,
        }) => {
            #[allow(unused_mut)]
            match Ts::new(name)
                .command(deno, npm)
                .build()
            {
                Ok(_) => {
                    println!("Successfully typescript built project");
                }
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
