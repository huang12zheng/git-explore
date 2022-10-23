use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct PushOption {
    #[clap(flatten)]
    pub base: BaseOptions,
}
