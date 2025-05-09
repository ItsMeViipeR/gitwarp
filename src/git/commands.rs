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
    #[clap(name = "branch")]
    Branch(Branch),
    #[clap(name = "merge")]
    Merge(Merge),
    #[clap(name = "pull")]
    Pull(Pull),
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

#[derive(Parser, Debug)]
pub struct Branch {
    #[clap(subcommand)]
    pub subcommand: BranchCommand,
}

#[derive(Parser, Debug)]
pub enum BranchCommand {
    #[clap(name = "create")]
    Create(CreateBranch),
    #[clap(name = "delete")]
    Delete(DeleteBranch),
    #[clap(name = "rename")]
    Rename(RenameBranch),
    #[clap(name = "list")]
    List,
    #[clap(name = "switch")]
    Switch(SwitchBranch),
}

#[derive(Parser, Debug)]
pub struct CreateBranch {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct DeleteBranch {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct RenameBranch {
    #[clap(help = "The current name of the branch")]
    pub name: String,
    #[clap(help = "The new name for the branch")]
    pub new_name: String,
}

#[derive(Parser, Debug)]
pub struct SwitchBranch {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct Merge {
    #[clap(help = "The branch to merge into the current branch")]
    pub branch: String,
}

#[derive(Parser, Debug)]
pub struct Pull {
    #[clap(help = "The branch to pull from")]
    pub branch: Option<String>,
    #[clap(
        long,
        short,
        help = "The remote repository to pull from. Defaults to 'origin'"
    )]
    pub remote: Option<String>,
}
