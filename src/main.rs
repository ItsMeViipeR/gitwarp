mod git;

use clap::Parser;
use git::{
    commands::{GitCommand, Gitwarp},
    exec::{clone, commit},
};

fn main() {
    let args: Gitwarp = Gitwarp::parse();

    match args.command {
        GitCommand::Clone(c) => clone(&c.url).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }),
        GitCommand::Commit(c) => commit(&c.message, c.all, c.push, c.files).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }),
    }
}
