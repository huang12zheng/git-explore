mod commit;
mod init;
mod list;
mod pull;
mod push;
use crate::*;
pub use commit::*;
pub use init::*;
pub use list::*;
pub use pull::*;
pub use push::*;

#[derive(Parser, Debug)]
#[command(
    name = clap::crate_name!(),
    version = clap::crate_version!(),
    author = clap::crate_authors!("\n"),
    about = clap::crate_description!(),
    long_version = clap::crate_version!(),
    propagate_version = true,
)]
pub struct RepoCli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(visible_alias = "l", about = "List repositories")]
    List(ListOption),
    #[clap(visible_alias = "i", about = "Init with command List")]
    Init(InitOption),
    #[clap(visible_alias = "cv", about = "Set Version and commit")]
    Commit(CommitOpts),
    #[clap(about = "git pull for repositories")]
    Pull(PushOption),
    #[clap(about = "git push for repositories")]
    Push(PushOption),
}
