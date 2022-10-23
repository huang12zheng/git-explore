use std::process;

use crate::*;
/// config new version on VersionOpts and tmp Config.
pub fn get_version(opt: &VersonOpts, config: &mut Config) -> Result<()> {
    let version = match opt.commit_version.to_owned() {
        Some(version) => version,
        None => {
            let mut version = Version::parse(&config.version).unwrap();
            match opt.kind {
                CommitKind::Patch => version.patch += 1,
                CommitKind::Minor => version.minor += 1,
                CommitKind::Major => version.major += 1,
            };
            version.to_string()
        }
    };

    config.version = version;

    Ok(())
}
pub fn sed_cargo(version: &str, config: &Config) -> Result<()> {
    let paths = &config.cargo_repos;
    let binding = vec!["s/\nversion.*\n/\nversion = '", version, "'\n/"].join("");

    sed(paths, binding.as_str())
}
pub fn sed_pubspec(version: &str, config: &Config) -> Result<()> {
    let paths = &config.pubspec_repos;
    let binding = vec!["s/\nversion.*\n/\nversion = ", version, "\n/"].join("");

    sed(paths, binding.as_str())
}
pub fn sed(paths: &Vec<String>, binding: &str) -> Result<()> {
    let cmd = ReplaceCommand::new(binding).unwrap();
    paths.iter().for_each(|path| {
        let file_path = path.to_path();
        let contents = file_path.get_content();

        let contents = cmd.execute(contents).into_owned();

        std::fs::write(file_path, contents).unwrap();
    });
    Ok(())
}
pub fn git_commit(version: &str, config: &Config) -> Result<()> {
    config.git_repos.iter().for_each(|git| {
        process::Command::new("git")
            .current_dir(git)
            .args(vec!["commit", "-am", version])
            .spawn()
            .expect(&format!("Failed to execute command git commit {}", git));
    });
    Ok(())
}
pub fn commit(opt: &mut CommitOpts) -> Result<()> {
    let mut config = Config::get(opt.base.base_dir.to_config_path().to_str().unwrap());

    get_version(&opt.version_opts, &mut config).unwrap();
    let version = &config.version;
    sed_cargo(version, &config).unwrap();
    sed_pubspec(version, &config).unwrap();
    config.set(opt.base.base_dir.to_config_path()).unwrap();
    git_commit(version, &config).unwrap();

    Ok(())
}

#[test]
pub fn tt() {
    let version = "0.1.2";
    // let binding = vec!["s/^version.*$/version = '", version, "'/"].join("");
    let binding = vec!["s/\nversion.*\n/\nversion = '", version, "'\n/"].join("");
    let cmd = ReplaceCommand::new(binding.as_str()).unwrap();
    println!("{:#?}", cmd.execute("version = \"0.1.0\""));
}
// #[test]
// pub fn run_commit() {
//     let cli = RepoCli::parse_from([KEY_COMMAND, "commit", "-d", "d:\\rust\\backend\\sdk1018"]);
//     if let Some(Command::Commit(mut opt)) = cli.command {
//         commit(&mut opt).unwrap();
//     }
// }
