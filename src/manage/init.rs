use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: String,
    pub git_repos: Vec<String>,
    pub cargo_repos: Vec<String>,
    pub pubspec_repos: Vec<String>,
}
impl Config {
    // fn ok(self) -> std::result::Result<Self, ()> {
    //     Ok(self)
    // }
    pub fn get(path: impl AsRef<Path>) -> Self {
        let content = path.as_ref().to_str().unwrap().get_content();
        toml::from_str(&content).unwrap()
    }
    pub fn set(&self, search_path: PathBuf) -> std::result::Result<(), std::io::Error> {
        std::fs::write(search_path, toml::to_string(self).unwrap())
    }
}
pub fn init(opt: &InitOption) -> std::result::Result<(), std::io::Error> {
    let search_path = opt.base.base_dir.to_config_path();
    let list_opts = ListOption {
        base: opt.base.clone(),
        filter: BaseFilterOptions {
            // kind: FilterKind::Directory,
            name: ".git".to_owned(),
        },
    };
    let git_vec = find_git(&list_opts).unwrap();
    let list_opts = ListOption {
        base: opt.base.clone(),
        filter: BaseFilterOptions {
            // kind: FilterKind::File,
            name: "Cargo.toml".to_owned(),
        },
    };
    let cargo_vec = find_file(&list_opts).unwrap();
    let list_opts = ListOption {
        base: opt.base.clone(),
        filter: BaseFilterOptions {
            // kind: FilterKind::File,
            name: "pubspec.yaml".to_owned(),
        },
    };
    let pubspec_vec = find_file(&list_opts).unwrap();
    let version = get_max_version(&cargo_vec).unwrap_or_else(|| Version::parse("0.0.1").unwrap());
    Config {
        version: (opt)
            .commit_version
            .to_owned()
            .unwrap_or_else(|| version.to_string()),
        git_repos: git_vec.to_string_vec(),
        cargo_repos: cargo_vec.to_string_vec(),
        pubspec_repos: pubspec_vec.to_string_vec(),
    }
    .set(search_path)
}

pub fn get_max_version(cargo_repos: &[PathBuf]) -> Option<Version> {
    cargo_repos
        .iter()
        .map(|path| {
            let p = path.to_str().unwrap();
            p.cargo_version().unwrap()
        })
        .max()
    // .unwrap()
}
#[test]
fn run_init() {
    let cli = RepoCli::parse_from([KEY_COMMAND, "init", "-d", KEY_BASEPATH]);
    // println!("cli: {:#?}", cli);
    if let Some(Command::Init(opt)) = cli.command {
        init(&opt).unwrap();
    }
    let content = ("d:\\rust\\backend\\sdk1018\\".to_string() + KEY_CONFIG_PATH).get_content();

    insta::assert_debug_snapshot!(content)
}
