use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct InitOption {
    #[clap(flatten)]
    pub base: BaseOptions,
    #[clap(long = "cv")]
    pub commit_version: Option<String>,
}
