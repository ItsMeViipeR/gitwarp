use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "git")]
pub struct Gitwarp {
    #[clap(subcommand)]
    pub command: GitCommand,
}

#[derive(Parser, Debug)]
pub enum GitCommand {
    #[clap(name = "clone")]
    Clone(Clone),
}

#[derive(Parser, Debug)]
pub struct Clone {
    pub url: String,
}
