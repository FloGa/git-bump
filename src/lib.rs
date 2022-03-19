use std::collections::HashMap;
use std::env::args;
use std::fs;

use mlua::prelude::*;
use mlua::Function;

mod error;

pub fn bump() -> LuaResult<()> {
    let repository = git2::Repository::discover(".").expect("Not a Git repository");

    let workdir = match repository.workdir() {
        Some(workdir) => workdir,
        None => panic!("git-bump is not supported on bare repositories"),
    };

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
        panic!("No valid config files found")
    }

    let version = args().nth(1).unwrap();

    let lua = Lua::new();

    let mut map = HashMap::new();
    for config in bump_configs {
        let content = fs::read_to_string(config);
        let chunk = match content {
            Ok(content) => lua.load(&content).eval::<HashMap<String, Function>>(),
            Err(_) => continue,
        };

        for (file, func) in chunk? {
            map.insert(file, func);
        }
    }

    for (file, f) in map {
        let mut contents = f.call::<_, String>(version.clone())?;
        if !contents.ends_with('\n') {
            contents.push('\n')
        }

        fs::write(workdir.join(file), contents).unwrap();
    }

    Ok(())
}
