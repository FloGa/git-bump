//! # git-bump
//!
//! [![badge github]][url github]
//! [![badge crates.io]][url crates.io]
//! [![badge docs.rs]][url docs.rs]
//! [![badge license]][url license]
//!
//! [badge github]: https://img.shields.io/badge/github-FloGa%2Fgit--bump-green
//! [badge crates.io]: https://img.shields.io/crates/v/git-bump
//! [badge docs.rs]: https://img.shields.io/docsrs/git-bump
//! [badge license]: https://img.shields.io/crates/l/git-bump
//!
//! [url github]: https://github.com/FloGa/git-bump
//! [url crates.io]: https://crates.io/crates/git-bump
//! [url docs.rs]: https://docs.rs/git-bump
//! [url license]: https://github.com/FloGa/git-bump/blob/develop/LICENSE
//!
//! Consistently bump your version numbers with Lua scripts.
//!
//! ## Motivation
//!
//! When publishing a new software release, there are usually a couple of places
//! where you want to update the current version number:
//!
//! -   configuration files like `Cargo.toml` and `package.json`
//! -   source files with defined constants for your application
//! -   a conventional `VERSION` file in your repository root
//! -   your changelog
//! -   maybe a lot of other places, depending on you specific needs and workflow
//!
//! Also, depending on your workflow, you might want to first bump your version to
//! something like `1.2.3-RC`, then after some final testing `1.2.3` and
//! eventually to a development version `1.3.0-SNAPSHOT`.
//!
//! Since these tasks can be nicely automated, you might want to have a small
//! script that does the bumping for you. I even ended up with a `bump.sh` in each
//! of my projects, which are all quite similar, especially the ones for the same
//! programming language. To avoid this kind of boilerplate code in every single
//! repository, I came up with `git-bump` which is configurable via Lua scripts.
//!
//! `git-bump` searches for configuration files in certain
//! [locations](#configuration-file-locations), aggregates them, and calls a
//! custom Lua function for every defined file. This way it is possible to define
//! global version bump functions that can be used in each repository.
//!
//! ## Installation
//!
//! `git-bump` can be installed easily through Cargo via `crates.io`:
//!
//! ```shell script
//! cargo install --locked git-bump
//! ```
//!
//! Please note that the `--locked` flag is necessary here to have the exact same
//! dependencies as when the application was tagged and tested. Without it, you
//! might get more up-to-date versions of dependencies, but you have the risk of
//! undefined and unexpected behavior if the dependencies changed some
//! functionalities. The application might even fail to build if the public API of
//! a dependency changed too much.
//!
//! Alternatively, pre-built binaries can be downloaded from the [GitHub
//! releases][gh-releases] page.
//!
//! [gh-releases]: https://github.com/FloGa/git-bump/releases
//!
//! ## Usage
//!
//! ```text
//! Usage: git-bump <NEW_VERSION|--list-files|--print-sample-config>
//!
//! Arguments:
//!   [NEW_VERSION]  Version to set
//!
//! Options:
//!       --list-files           List files that would be updated
//!       --print-sample-config  Print sample config file
//!   -h, --help                 Print help
//!   -V, --version              Print version
//! ```
//!
//! The following examples require that `git-bump` is accessible from your current
//! shell, so ensure that the executable lies in a directory that is part of your
//! `$PATH` (Linux and MacOS) or `%PATH%` (Windows) variable.
//!
//! To bump your versions to `1.2.3`, it is as simple as:
//!
//! ```shell script
//! git-bump 1.2.3
//! ```
//!
//! Or, with Git subcommand syntax:
//!
//! ```shell script
//! git bump 1.2.3
//! ```
//!
//! Well, maybe not quite that easy. If you do not have any configuration files
//! yet, nothing will happen.
//!
//! For a first success, let's start with a very simple configuration file in the
//! root of your Git repository. Name it `.git-bump.lua` (the leading `.` denotes
//! a hidden file in Linux and is quite usual for such configuration files) with
//! the following contents:
//!
//! ```lua
//! return {
//!     VERSION = function(version)
//!         return version
//!     end,
//! }
//! ```
//!
//! The configuration files are expected to return a Lua table. The keys are the
//! file names you want to run the bumper on, relative to the Git repository root.
//! The value is a Lua function, taking two parameters: The version that was given
//! as argument to `git-bump` and the contents of the file for conveniently
//! altering. If you do not need the current file content, you can ignore the
//! second parameter, Lua does not care about extraneous parameters. The functions
//! need to return the new contents of the file, which will then be written into
//! the according files.
//!
//! In this example, the file `VERSION` will only contain the given version string.
//!
//! More complex examples can be found in the section [Sample
//! Functions](#sample-functions).
//!
//! Since such configurations could be shared across multiple, different
//! repositories, `git-bump` will not create new files, but only operate on
//! existing files. So, for this example, create `VERSION` and run the bumper
//! again:
//!
//! ```text
//! $ touch VERSION
//! $ git bump 1.2.3
//! $ cat VERSION
//! 1.2.3
//! ```
//!
//! To create a sample configuration file with several ready-to-use recipes, run:
//!
//! ```shell script
//! git bump --print-sample-config >.git-bump.lua
//! ```
//!
//! To print out a list of existing files that are configured in the config files
//! and would be processed during bumping, run:
//!
//! ```shell script
//! git bump --list-files
//! ```
//!
//! ## Hook Functions
//!
//! Along with the new contents for a specified file, one can also define hook
//! functions that should be run *before* or *after* the new content is written to
//! the file.
//!
//! The `pre_func` could be used, for example, to create a backup of the file
//! prior to updating it. The `post_func` might be used to do some house keeping
//! with modified config files.
//!
//! The hooks must be returned as a Lua table with the members `pre_func` and
//! `post_func`. Both members are optional. If a hook function does not exist, it
//! will be silently ignored.
//!
//! The following is a simple, imaginary example to demonstrate the usage of hook
//! functions. For a proper example, take a look at the section [Sample
//! Functions](#sample-functions).
//!
//! ```lua
//! return {
//!     VERSION = function(version)
//!         local os = require("os")
//!
//!         local pre_func = function()
//!             os.execute("cp VERSION VERSION.old")
//!         end
//!
//!         local post_func = function()
//!             os.execute("git commit -m 'Update VERSION' VERSION")
//!         end
//!
//!         return version, {pre_func = pre_func, post_func = post_func}
//!     end
//! }
//! ```
//!
//! ## Configuration File Locations
//!
//! The bump config files will be searched in the following locations:
//!
//! -   `$HOME/.git-bump.lua` (Unix) or `%USERPROFILE%\.git-bump.lua` (Windows)
//!
//!     Per-user global config file.
//!
//! -   `$GIT_DIR/git-bump.lua`
//!
//!     Per-repository config file, not intended for sharing.
//!
//! -   `$GIT_WORK_TREE/.git-bump.lua`
//!
//!     Per-repository config file, may be checked into Git for sharing.
//!
//! Those locations will be evaluated in order, a later file overrides mappings of
//! the previous ones if they have matching keys. Missing config files will be
//! silently ignored.
//!
//! If you want to explicitly ignore a bumping function of a "higher"
//! configuration, you must declare it in a "lower" config file like so:
//!
//! ```lua
//! return {
//!     -- ...
//!
//!     ["dummy.txt"] = function(_, content)
//!         -- no bumping, just return unaltered content
//!         return content
//!     end
//!
//!     -- ...
//! }
//! ```
//!
//! ## Sample Functions
//!
//! Find the latest sample config file here:
//! <https://github.com/FloGa/git-bump/blob/develop/.git-bump.lua>
//!
//! This is a non-exhaustive list of possible functions that can be used in your
//! config files. If you have ideas for more default functions, don't hesitate to
//! open a PR!

use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

use mlua::prelude::*;

use crate::state::State as BumpState;
use crate::{cli::run, error::Error, error::Result};

mod cli;
mod error;
mod state;

/// Bump files to a given version.
fn bump(version: String) -> Result<()> {
    let mut bump_state = BumpState::default();

    let map = bump_state.get_file_mapping()?;

    let lua = bump_state.get_lua();
    for (file, f) in map.deref() {
        let f = lua.registry_value::<LuaFunction>(f)?;

        let contents = fs::read_to_string(&file).map_err(|source| Error::ReadFailed { source })?;

        let (mut contents, hooks) = f
            .call::<_, (String, Option<HashMap<String, LuaFunction>>)>((version.clone(), contents))
            .map_err(|source| Error::LuaExecutionFailed { source })?;
        if !contents.ends_with('\n') {
            contents.push('\n')
        }

        if let Some(hooks) = &hooks {
            if let Some(pre_func) = hooks.get("pre_func") {
                pre_func
                    .call::<_, ()>(())
                    .map_err(|source| Error::LuaPreFuncFailed { source })?;
            }
        }

        fs::write(file, contents).map_err(|source| Error::WriteFailed { source })?;

        if let Some(hooks) = &hooks {
            if let Some(post_func) = hooks.get("post_func") {
                post_func
                    .call::<_, ()>(())
                    .map_err(|source| Error::LuaPostFuncFailed { source })?;
            }
        }
    }

    Ok(())
}

/// Print file paths that would be bumped.
fn list_files() -> Result<()> {
    let mut bump_state = BumpState::default();

    let map = bump_state.get_file_mapping()?;

    let mut keys = map.deref().keys().collect::<Vec<_>>();
    keys.sort();

    for file in keys {
        println!("{}", file.to_string_lossy());
    }

    Ok(())
}

/// Print sample `git-bump.lua`.
fn print_sample_config() {
    println!("{}", include_str!("../.git-bump.lua"))
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
