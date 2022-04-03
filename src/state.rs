use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

use git2::Repository;
use mlua::prelude::*;

use crate::{Error, Result};

/// State object for bumping actions.
///
/// This struct contains all necessary stateful information for the different bumping actions.
///
/// All members are initialized lazily, so there will never be more work done than really
/// necessary. The actual initialization happens in the `impl` block, hence the raw members are
/// private. Use the `get_*` methods to access them.
#[derive(Default)]
pub(crate) struct State {
    lua: Option<Rc<Lua>>,
    repository: Option<Rc<Repository>>,
    workdir: Option<Rc<PathBuf>>,
    config_files: Option<Rc<Vec<PathBuf>>>,
    file_mapping: Option<Rc<HashMap<PathBuf, LuaRegistryKey>>>,
}

impl State {
    /// Get shared Lua instance.
    pub(crate) fn get_lua(&mut self) -> Rc<Lua> {
        Rc::clone(self.lua.get_or_insert_with(|| Rc::new(Lua::new())))
    }

    /// Get Repository object.
    pub(crate) fn get_repository(&mut self) -> Result<Rc<Repository>> {
        if let Some(repository) = &self.repository {
            Ok(Rc::clone(repository))
        } else {
            match git2::Repository::discover(".") {
                Ok(repository) => Ok(Rc::clone(self.repository.insert(Rc::new(repository)))),
                Err(_) => Err(Error::NotARepository),
            }
        }
    }

    /// Get root working directory of the current repository.
    pub(crate) fn get_workdir(&mut self) -> Result<Rc<PathBuf>> {
        if let Some(workdir) = &self.workdir {
            Ok(Rc::clone(workdir))
        } else {
            match self.get_repository()?.workdir() {
                Some(workdir) => Ok(Rc::clone(
                    self.workdir.insert(Rc::new(PathBuf::from(workdir))),
                )),
                None => Err(Error::BareRepositoryNotSupported),
            }
        }
    }

    /// Get list of available configuration files.
    pub(crate) fn get_config_files(&mut self) -> Result<Rc<Vec<PathBuf>>> {
        if let Some(config_files) = &self.config_files {
            Ok(Rc::clone(config_files))
        } else {
            let config_user =
                home::home_dir().and_then(|p| p.join(".git-bump.lua").canonicalize().ok());
            let config_repo_unshared = self
                .get_repository()?
                .path()
                .join("git-bump.lua")
                .canonicalize()
                .ok();
            let config_repo_shared = self
                .get_workdir()?
                .join(".git-bump.lua")
                .canonicalize()
                .ok();

            let config_files = [config_user, config_repo_unshared, config_repo_shared]
                .into_iter()
                .flatten()
                .collect();

            Ok(Rc::clone(self.config_files.insert(Rc::new(config_files))))
        }
    }

    /// Get map of existing files and Lua functions for bumping.
    pub(crate) fn get_file_mapping(&mut self) -> Result<Rc<HashMap<PathBuf, LuaRegistryKey>>> {
        if let Some(file_mapping) = &self.file_mapping {
            Ok(Rc::clone(file_mapping))
        } else {
            if self.get_config_files()?.is_empty() {
                return Ok(self
                    .file_mapping
                    .insert(Rc::new(Default::default()))
                    .clone());
            }

            let mut file_mapping = HashMap::new();
            for config in self.get_config_files()?.deref() {
                let content = fs::read_to_string(config);
                match content {
                    Ok(content) => {
                        let lua = self.get_lua();
                        let result = match lua.load(&content).eval::<HashMap<String, LuaFunction>>()
                        {
                            Ok(map) => {
                                for (file, func) in map {
                                    let file = self.get_workdir()?.join(file);

                                    if !file.exists() {
                                        continue;
                                    }

                                    let func = lua.create_registry_value(func)?;

                                    if let Some(key) = file_mapping.insert(file, func) {
                                        lua.remove_registry_value(key)?;
                                    };
                                }
                                Ok(())
                            }
                            Err(source) => Err(Error::LuaLoadingFailed { source }),
                        };
                        result
                    }
                    Err(_) => continue,
                }?;
            }

            Ok(Rc::clone(self.file_mapping.insert(Rc::new(file_mapping))))
        }
    }
}
