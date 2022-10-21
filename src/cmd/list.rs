use crate::*;
#[derive(Parser, Debug, Clone)]
pub struct ListOption {
    #[clap(flatten)]
    pub base: BaseOptions,
    #[clap(flatten)]
    pub filter: BaseFilterOptions,
}

#[derive(Args, Debug, Clone)]
pub struct BaseOptions {
    /// The base directory that all of your repositories are inside
    #[clap(
        short = 'd',
        long = "base-dir",
        env = "GIT_PROJECT_BASE_DIR",
        default_value_t =  String::from("."),
    )]
    pub base_dir: String,
    /// Do not stop recursing when a .git folder is found
    #[clap(short = 'r', long = "deep-recurse", default_value_t = true)]
    pub deep_recurse: bool,
}

#[derive(Args, Debug, Clone)]
pub struct BaseFilterOptions {
    #[clap( short = 'n', default_value_t = String::from(".git"))]
    pub name: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum FilterKind {
    File,
    Directory,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum FilterName {
    Git,
    Cargo,
    Pubspec,
}

pub fn run_list(opt: &ListOption) -> Result<()> {
    let ret = find_git(&opt);
    println!("{:#?}", ret);
    Ok(())
}
