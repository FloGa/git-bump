use std::collections::HashMap;
use std::env::args;
use std::fs;

use mlua::prelude::*;
use mlua::Function;

pub use crate::{error::Error, error::Result};

mod error;

pub fn bump() -> Result<()> {
    let repository = git2::Repository::discover(".").map_err(|_| Error::NotARepository)?;

    let workdir = repository
        .workdir()
        .ok_or_else(|| Error::BareRepositoryNotSupported)?;

    let bump_configs = {
        let config_user =
            home::home_dir().and_then(|p| p.join(".git-bump.lua").canonicalize().ok());
        let config_repo_unshared = repository.path().join("git-bump.lua").canonicalize().ok();
        let config_repo_shared = workdir.join(".git-bump.lua").canonicalize().ok();

        [config_user, config_repo_unshared, config_repo_shared]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    };

    if bump_configs.is_empty() {
        return Err(Error::NoValidConfigFound);
    }

    let version = args().nth(1).ok_or(Error::NoVersionGiven)?;

    let lua = Lua::new();

    let mut map = HashMap::new();
    for config in bump_configs {
        let content = fs::read_to_string(config);
        let chunk = match content {
            Ok(content) => lua.load(&content).eval::<HashMap<String, Function>>(),
            Err(_) => continue,
        }
        .map_err(|source| Error::LuaLoadingFailed { source })?;

        for (file, func) in chunk {
            map.insert(file, func);
        }
    }

    for (file, f) in map {
        let file = workdir.join(file);

        if !file.exists() {
            continue;
        }

        let contents = fs::read_to_string(&file).map_err(|source| Error::ReadFailed { source })?;

        let mut contents = f
            .call::<_, String>((version.clone(), contents))
            .map_err(|source| Error::LuaExecutionFailed { source })?;
        if !contents.ends_with('\n') {
            contents.push('\n')
        }

        fs::write(file, contents).map_err(|source| Error::WriteFailed { source })?;
    }

    Ok(())
}
