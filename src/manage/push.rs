use std::process;

use crate::*;

pub fn git_push(config: &Config) -> Result<()> {
    config.git_repos.iter().for_each(|git| {
        process::Command::new("git")
            .current_dir(git)
            .args(vec!["push"])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute command git push {}", git));
    });
    Ok(())
}

pub fn push(opt: &PushOption) -> Result<()> {
    let config = Config::get(opt.base.base_dir.to_config_path().to_str().unwrap());

    git_push(&config).unwrap();

    Ok(())
}

#[test]
pub fn run_push() {
    use crate::*;
    let cli = RepoCli::parse_from([KEY_COMMAND, "push", "-d", "d:\\rust\\backend\\sdk1018"]);
    if let Some(Command::Push(mut opt)) = cli.command {
        push(&mut opt).unwrap();
    }
}
