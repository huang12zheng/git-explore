use crate::PathEx;

use super::*;

/// 1. if workspace is directory, push it
/// 2. if workspace is file, check content, and push it if it exists submodule
pub fn find_git(opt: &ListOption) -> Result<Vec<PathBuf>> {
    let gits = find_folders(opt.clone()).unwrap();

    let mut paths = Vec::new();
    gits.iter().for_each(|git| {
        match git.kind {
            crate::FilterKind::File => {
                let file_path = git.workspace.join(".git");
                // gitdir: ../.git/modules/common
                let content = file_path.get_content();
                let key: Vec<_> = content.split(':').collect();
                if key[0] == "gitdir" {
                    let module_path = git.workspace.join(key[1].trim());
                    if module_path.is_dir() {
                        paths.push(git.workspace.to_owned());
                    }
                }
            }
            crate::FilterKind::Directory => paths.push(git.workspace.to_owned()),
        }
    });

    Ok(paths)
}

#[test]
fn search_git() {
    use crate::*;
    use clap::Parser;
    let cli = RepoCli::parse_from([KEY_COMMAND, "list", "-d", "d:\\rust\\backend\\sdk1018"]);
    // println!("cli: {:#?}", cli);
    let ret = if let Some(Command::List(opt)) = cli.command {
        find_git(&opt)
    } else {
        todo!()
    };
    insta::assert_debug_snapshot!(ret)
}
