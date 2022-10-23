use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct PullOption {
    #[clap(flatten)]
    pub base: BaseOptions,
}
