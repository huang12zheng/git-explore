#![feature(iter_intersperse)]
#![feature(absolute_path)]
pub(crate) use clap::{Args, Parser, Subcommand};
pub(crate) use semver::*;
pub(crate) use serde::*;

pub(crate) use cargo_metadata::Metadata;
pub(crate) use chrono::{DateTime, NaiveDateTime, Utc};
pub(crate) use git2::Repository;
pub(crate) use sedregex::ReplaceCommand;
use std::time::SystemTime;
pub(crate) use toml;

mod cmd;
mod err;
mod explore;
mod manage;
mod util;
pub use cmd::*;
pub use explore::*;
pub use manage::*;
pub use util::*;

pub const KEY_CONFIG_PATH: &str = "git_explore.db";
pub const KEY_COMMAND: &str = "git-explore";
