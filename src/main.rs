mod git;

use clap::Parser;
use git::{
    commands::{BranchCommand, GitCommand, Gitwarp},
    exec::*,
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
        GitCommand::Branch(b) => match b.subcommand {
            BranchCommand::Create(c) => create_branch(&c.name).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }),
            BranchCommand::List => branches_list().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }),
            BranchCommand::Delete(d) => branch_delete(&d.name).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }),
            BranchCommand::Rename(r) => branch_rename(&r.name, &r.new_name).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }),
            BranchCommand::Switch(s) => branch_switch(&s.name).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }),
        },
        GitCommand::Merge(m) => merge(&m.branch).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }),
        GitCommand::Pull(p) => pull(p.branch).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }),
    }
}
