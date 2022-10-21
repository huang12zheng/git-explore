use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct InitOption {
    #[clap(flatten)]
    pub base: BaseOptions,
    #[clap(flatten)]
    pub version_opts: VersonOpts,
}
