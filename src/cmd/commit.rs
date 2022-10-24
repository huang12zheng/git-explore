use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct MessageOpts {
    #[clap(long = "cm")]
    pub commit_message: Option<String>,
    #[clap(value_enum,short = 'k',default_value_t = CommitVersionKind::Patch)]
    pub kind: CommitVersionKind,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CommitVersionKind {
    Patch,
    Minor,
    Major,
}
#[derive(Parser, Debug, Clone)]
pub struct CommitOpts {
    #[clap(flatten)]
    pub base: BaseOptions,
    #[clap(flatten)]
    pub message_opts: MessageOpts,
    #[clap(skip)]
    pub config: Option<Config>,
}
