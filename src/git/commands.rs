use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "git")]
#[clap(about = "Manage your git repositories easily")]
pub struct Gitwarp {
    #[clap(subcommand)]
    pub command: GitCommand,
}

#[derive(Parser, Debug)]
pub enum GitCommand {
    #[clap(name = "clone")]
    #[clap(about = "Clone a repository")]
    #[clap(alias = "cl")]
    Clone(Clone),
    #[clap(name = "commit")]
    #[clap(about = "Commit changes to the repository")]
    #[clap(alias = "co")]
    Commit(Commit),
    #[clap(name = "branch")]
    #[clap(about = "Manage branches in the repository")]
    #[clap(alias = "br")]
    Branch(Branch),
    #[clap(name = "merge")]
    #[clap(about = "Merge branches in the repository")]
    #[clap(alias = "m")]
    Merge(Merge),
    #[clap(name = "pull")]
    #[clap(about = "Pull changes from the remote repository")]
    #[clap(alias = "pl")]
    #[clap(alias = "p")]
    Pull(Pull),
    Version,
}

#[derive(Parser, Debug)]
pub struct Clone {
    #[clap(help = "The URL of the repository to clone")]
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
    #[clap(about = "Create a new branch")]
    Create(CreateBranch),
    #[clap(name = "delete")]
    #[clap(about = "Delete a branch")]
    Delete(DeleteBranch),
    #[clap(name = "rename")]
    #[clap(about = "Rename a branch")]
    Rename(RenameBranch),
    #[clap(name = "list")]
    #[clap(about = "List all branches")]
    List,
    #[clap(name = "switch")]
    #[clap(about = "Switch to a different branch")]
    Switch(SwitchBranch),
}

#[derive(Parser, Debug)]
pub struct CreateBranch {
    #[clap(help = "The name of the branch to create")]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct DeleteBranch {
    #[clap(help = "The name of the branch to delete")]
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
    #[clap(help = "The name of the branch to switch to")]
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

#[derive(Parser, Debug)]
pub struct Version {
    #[clap(long, short, help = "Show the version of the program")]
    pub version: bool,
}
