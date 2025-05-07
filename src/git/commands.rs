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
    #[clap(name = "commit")]
    Commit(Commit),
}

#[derive(Parser, Debug)]
pub struct Clone {
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct Commit {
    #[clap(long, short, help = "The commit message to use")]
    pub message: String,
    #[clap(
        long,
        short,
        help = "Add all files to the commit. If you don't want to add all files, use -f and specify the files to add"
    )]
    pub all: bool,
    #[clap(long, short, help = "Push the commit to the remote repository")]
    pub push: bool,
    #[clap(
        long,
        short,
        help = "The files to add to the commit. If you don't want to add all files, use -f and specify the files to add"
    )]
    pub files: Option<Vec<String>>,
}
