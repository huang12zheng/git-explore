use std::process;

use crate::*;

pub fn git_pull(config: &Config) -> Result<()> {
    config.git_repos.iter().for_each(|git| {
        process::Command::new("git")
            .current_dir(git)
            .args(vec!["pull"])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute command git pull {}", git));
    });
    Ok(())
}

pub fn pull(opt: &PushOption) -> Result<()> {
    let config = Config::get(opt.base.base_dir.to_config_path().to_str().unwrap());

    git_pull(&config).unwrap();

    Ok(())
}

#[test]
pub fn run_pull() {
    use crate::*;
    let cli = RepoCli::parse_from([KEY_COMMAND, "pull", "-d", "d:\\rust\\sdk10212"]);
    if let Some(Command::Pull(mut opt)) = cli.command {
        pull(&mut opt).unwrap();
    }
}
