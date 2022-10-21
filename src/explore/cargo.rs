use super::*;

/// 1. if workspace is directory, push it
/// 2. if workspace is file, check content, and push it if it exists submodule
pub fn find_file(opt: &ListOption) -> Result<Vec<PathBuf>> {
    let files = find_folders(opt.clone()).unwrap();

    let mut paths = Vec::new();
    files.iter().for_each(|file| {
        if let crate::FilterKind::File = file.kind {
            let file_path = file.workspace.join(&opt.filter.name);
            paths.push(file_path);
        }
    });

    Ok(paths)
}

#[test]
fn search_git() {
    use crate::*;
    use clap::Parser;
    let cli = RepoCli::parse_from([
        "git-explore",
        "list",
        "-d",
        "d:\\rust\\backend\\sdk1018",
        "-n",
        "Cargo.toml",
    ]);
    // println!("cli: {:#?}", cli);
    let ret = if let Some(Command::List(opt)) = cli.command {
        find_file(&opt)
    } else {
        todo!()
    };
    insta::assert_debug_snapshot!(ret)
}
