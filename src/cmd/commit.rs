use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct VersonOpts {
    #[clap(long = "cv")]
    pub commit_version: Option<String>,
    #[clap(value_enum,short = 'k',default_value_t = CommitKind::Patch)]
    pub kind: CommitKind,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CommitKind {
    Patch,
    Minor,
    Major,
}
#[derive(Parser, Debug, Clone)]
pub struct CommitOpts {
    #[clap(flatten)]
    pub base: BaseOptions,
    #[clap(flatten)]
    pub version_opts: VersonOpts,
    #[clap(skip)]
    pub config: Option<Config>,
}
