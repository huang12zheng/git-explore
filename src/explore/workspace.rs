use crate::*;

const MAX_DEPTH: usize = 100;
/// find workspace by filter_name and would return dir with type
pub fn find_folders(opt: ListOption) -> Result<Vec<Workspace>> {
    let mut paths = Vec::new();
    collect_folders(
        &opt,
        Path::new(&opt.base.base_dir),
        Path::new(""),
        0,
        &mut paths,
    )?;

    Ok(paths)
}
#[derive(Debug)]
pub struct Workspace {
    pub kind: FilterKind,
    pub workspace: PathBuf,
}

fn collect_folders<P, R>(
    opt: &ListOption,
    base_path: P,
    child_path: R,
    depth: usize,
    paths: &mut Vec<Workspace>,
) -> Result<()>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let full_path = base_path.as_ref().join(child_path).canonicalize()?;

    for entry_res in walkdir::WalkDir::new(&full_path).min_depth(1).max_depth(1) {
        let entry = entry_res?;
        let entry_path = entry.path();

        if opt.filter.name.eq(entry.file_name().to_str().unwrap()) {
            paths.push(Workspace {
                kind: if entry_path.is_dir() {
                    FilterKind::Directory
                } else {
                    FilterKind::File
                },
                workspace: full_path.clone(),
            });
            if !opt.base.deep_recurse {
                return Ok(());
            }
        }
        if depth < MAX_DEPTH && entry_path.is_dir() {
            collect_folders(opt, &full_path, entry_path, depth + 1, paths)?;
        }
    }

    Ok(())
}

#[test]
fn search() {
    use crate::*;
    use clap::Parser;
    let cli = RepoCli::parse_from([KEY_COMMAND, "list", "-d", KEY_BASEPATH]);
    // println!("cli: {:#?}", cli);
    let ret = if let Some(Command::List(opt)) = cli.command {
        find_folders(opt)
    } else {
        todo!()
    };
    insta::assert_debug_snapshot!(ret)
}
