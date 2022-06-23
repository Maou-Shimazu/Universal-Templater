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
        /// Generate a CLI project
        #[clap(short = 'T', long = "cli")]
        cli: bool,
        /// Generate a web project
        #[clap(short = 'W', long = "web")]
        web: bool,

        /// Deno project
        #[clap(short = 'D', long = "deno")]
        deno: bool,

        /// Npm project
        /// #[clap(short = 'N', long = "npm")]
        npm: bool,
    },
}

fn main() {
    let args = UT::parse();
    match args.commands {
        Some(Command::Ts {
            name,
            cli,
            web,
            deno,
            npm,
        }) => {
            #[allow(unused_mut)]
            let files: Vec<String>;
            let directories: Vec<String> = vec!["ts".to_string()];
            if cli {
                files = vec!["deps.ts".to_string(), "main.ts".to_string()];
            } else if web {
                files = vec![
                    "index.ts".to_string(),
                    "deps.ts".to_string(),
                    "tsconfig.json".to_string(),
                ];
            } else {
                files = vec!["deps.ts".to_string(), "index.ts".to_string()];
            }

            match Ts::configure(name, cli, web, files, directories)
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
