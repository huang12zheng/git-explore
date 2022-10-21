//! source from https://github.com/kodemaniak/cargo-gitv/blob/main/src/build_context.rs

use crate::*;
pub use anyhow::{anyhow, Context, Result};
pub struct BuildContext {
    pub git_repository: Repository,
    pub cargo_metadata: Metadata,
    pub git_sha: String,
    pub git_latest_version: Option<Version>,
    pub git_commit_timestamp: Option<DateTime<Utc>>,
    pub cargo_version: Version,
}

impl BuildContext {
    pub fn dev_version(&self) -> Result<Version> {
        let timestamp: DateTime<Utc> = self
            .git_commit_timestamp
            .unwrap_or_else(|| SystemTime::now().into());
        let timestamp_formatted = timestamp.format("%Y%m%d%H%M%S");

        let dev_version = format!(
            "{}-dev.{}+{}",
            self.cargo_version, timestamp_formatted, self.git_sha
        );

        Ok(Version::parse(&dev_version).unwrap())
        // .with_context(|| "Failed to generate valid dev version.")
    }
}

// pub fn load_build_context() -> Result<BuildContext> {
//     let mut cmd = cargo_metadata::MetadataCommand::new();
//     cmd.manifest_path("./Cargo.toml");
//     let cargo_metadata = cmd
//         .exec()
//         .context("Could not find the cargo metadata. Tried ./Cargo.toml")?;

//     let git_repository = Repository::open("./")
//       .context("Could not find a git repository. Please run from the top-level folder of a git repository.")?;

//     let git_sha = get_git_sha(&git_repository)?;
//     let git_latest_version = get_git_latest_version(&git_repository)?;
//     let cargo_version = get_cargo_version(&cargo_metadata)?;
//     let git_commit_timestamp = get_git_commit_timestamp(&git_repository).ok();

//     let build_context = BuildContext {
//         git_repository,
//         cargo_metadata,
//         git_sha,
//         git_latest_version,
//         git_commit_timestamp,
//         cargo_version,
//     };

//     Ok(build_context)
// }

pub fn get_git_sha(git_repository: &Repository) -> Result<String> {
    let mut sha = git_repository
        .head()?
        .target()
        .ok_or_else(|| anyhow!("Could not determin git commit SHA!"))?
        .to_string();
    sha.truncate(7);

    Ok(sha)
}

pub fn get_git_latest_version(git_repository: &Repository) -> Result<Option<Version>> {
    let mut tags: Vec<Version> = git_repository
        .tag_names(Some("v*"))?
        .into_iter()
        .flatten()
        .flat_map(|v| v.strip_prefix('v'))
        .flat_map(semver::Version::parse)
        .collect();
    tags.sort();
    tags.reverse();

    let current_release_version = tags.first().cloned();

    Ok(current_release_version)
}

pub fn get_git_commit_timestamp(git_repository: &Repository) -> Result<DateTime<Utc>> {
    let head_ref = git_repository.find_reference("HEAD")?;
    let head_direct = head_ref.resolve()?;
    let head_refspec = head_direct.name().expect("invalid name");
    let head_oid = git_repository.refname_to_id(head_refspec)?;
    let commit = git_repository.find_commit(head_oid)?;
    let commit_timestamp = commit.time().seconds();
    let timestamp: DateTime<Utc> =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(commit_timestamp, 0), Utc);

    Ok(timestamp)
}

pub trait CargoVersion {
    fn cargo_version(&self) -> Result<Version>;
}
impl CargoVersion for Metadata {
    fn cargo_version(&self) -> Result<Version> {
        let root = self.workspace_members.first().unwrap();
        let package = self
            .packages
            .iter()
            .find(|p| p.id == *root)
            .ok_or_else(|| anyhow!("Could not determine cargo package version."))?;
        Ok(package.version.clone())
    }
}
impl CargoVersion for &str {
    fn cargo_version(&self) -> Result<Version> {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(self);
        let cargo_metadata = cmd
            .exec()
            .context("Could not find the cargo metadata. Tried ./Cargo.toml")?;
        cargo_metadata.cargo_version()
    }
}
