mod git;

use clap::Parser;
use git::commands::Gitwarp;

fn main() {
    let args: Gitwarp = Gitwarp::parse();

    match args.command {
        git::commands::GitCommand::Clone(clone) => {
            println!("URL: {}", clone.url);
        }
    }
}
