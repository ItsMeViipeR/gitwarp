mod git;

use clap::Parser;
use git::{commands::Gitwarp, exec::clone};

fn main() {
    let args: Gitwarp = Gitwarp::parse();

    match args.command {
        git::commands::GitCommand::Clone(c) => clone(&c.url).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }),
    }
}
