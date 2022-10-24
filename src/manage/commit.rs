use std::process;

// use anyhow::Ok;

use crate::*;
/// config new version on VersionOpts and tmp Config.
pub fn get_version(opt: &MessageOpts, config: &mut Config) -> Result<()> {
    // Get the version if message it is, or to be None
    let version = opt.commit_message.to_owned().filter(|m| m.starts_with("v"));

    let version = version.unwrap_or_else(|| {
        let mut version = Version::parse(&config.version).unwrap();
        match opt.kind {
            CommitVersionKind::Patch => version.patch += 1,
            CommitVersionKind::Minor => version.minor += 1,
            CommitVersionKind::Major => version.major += 1,
        };
        version.to_string()
    });
    config.version = version;

    Ok(())
}
pub fn get_message(opt: &mut MessageOpts, config: &Config) -> Result<()> {
    let mut binding = opt.commit_message.to_owned();
    let m = binding.get_or_insert(config.version.clone());
    opt.commit_message = Some(m.to_owned());

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
pub fn sed(paths: &[String], binding: &str) -> Result<()> {
    let cmd = ReplaceCommand::new(binding).unwrap();
    paths.iter().for_each(|path| {
        let file_path = path.to_path();
        let contents = file_path.get_content();

        let contents = cmd.execute(contents).into_owned();

        std::fs::write(file_path, contents).unwrap();
    });
    Ok(())
}
pub fn git_commit(commit_message: &str, config: &Config) -> Result<()> {
    config.git_repos.iter().for_each(|git| {
        process::Command::new("git")
            .current_dir(git)
            .args(vec!["commit", "-am", commit_message])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute command git commit {}", git));
    });
    Ok(())
}
pub fn commit(opt: &mut CommitOpts) -> Result<()> {
    let mut config = Config::get(opt.base.base_dir.to_config_path().to_str().unwrap());

    get_version(&opt.message_opts, &mut config).unwrap();
    get_message(&mut opt.message_opts, &config).unwrap();
    let commit_message = &opt.message_opts.commit_message.to_owned().unwrap();
    if commit_message.starts_with("v") {
        sed_cargo(commit_message, &config).unwrap();
        sed_pubspec(commit_message, &config).unwrap();
        config.set(opt.base.base_dir.to_config_path()).unwrap();
    }
    git_commit(commit_message, &config).unwrap();

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
//     let cli = RepoCli::parse_from([KEY_COMMAND, "commit", "-d", KEY_BASEPATH]);
//     if let Some(Command::Commit(mut opt)) = cli.command {
//         commit(&mut opt).unwrap();
//     }
// }
