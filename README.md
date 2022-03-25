# git-bump

Consistently bump your version numbers with Lua scripts.

## Motivation

When publishing a new software release, there are usually a couple of places
where you want to update the current version number:

-   configuration files like `Cargo.toml` and `package.json`
-   source files with defined constants for your application
-   a conventional `VERSION` file in your repository root
-   your changelog
-   maybe a lot of other places, depending on you specific needs and workflow

Also, depending on your workflow, you might want to first bump your version to
something like `1.2.3-RC`, then after some final testing `1.2.3` and
eventually to a development version `1.3.0-SNAPSHOT`.

Since these tasks can be nicely automated, you might want to have a small
script that does the bumping for you. I even ended up with a `bump.sh` in each
of my projects, which are all quite similar, especially the ones for the same
programming language. To avoid this kind of boilerplate code in every single
repository, I came up with `git-bump` which is configurable via Lua scripts.

`git-bump` searches for configuration files in certain
[locations](#configuration-file-locations), aggregates them, and calls a
custom Lua function for every defined file. This way it is possible to define
global version bump functions that can be used in each repository.

## Installation

`git-bump` can be installed easily through Cargo via `crates.io`:

```shell script
cargo install git-bump
```

## Usage

To bump your versions to `1.2.3`, it is as simple as:

```shell script
git-bump 1.2.3
```

Or, with Git subcommand syntax:

```shell script
git bump 1.2.3
```

Well, maybe not quite that easy. If you do not have any configuration files
yet, then you will be presented with an error:

```text
$ git bump 1.2.3
Error: No valid config files found
```

For a first success, let's start with a very simple configuration file in the
root of your Git repository. Name it `.git-bump.lua` (the leading `.` denotes
a hidden file in Linux and is quite usual for such configuration files) with
the following contents:

```lua
return {
    VERSION = function(version)
        return version
    end,
}
```

The configuration files are expected to return a Lua table. The keys are the
file names you want to run the bumper on, relative to the Git repository root.
The value is a Lua function, taking two parameters: The version that was given
as argument to `git-bump` and the contents of the file for conveniently
altering. If you do not need the current file content, you can ignore the
second parameter, Lua does not care about extraneous parameters. The functions
need to return the new contents of the file, which will then be written into
the according files.

In this example, the file `VERSION` will only contain the given version string.

More complex examples can be found in the section [Sample
Functions](#sample-functions).

Since such configurations could be shared across multiple, different
repositories, `git-bump` will not create new files, but only operate on
existing files. So, for this example, create `VERSION` and run the bumper
again:

```text
$ touch VERSION
$ git bump 1.2.3
$ cat VERSION
1.2.3
```

## Configuration File Locations

The bump config files will be searched in the following locations:

-   `$HOME/.git-bump.lua` (Unix) or `%USERPROFILE%\.git-bump.lua` (Windows)

    Per-user global config file.

-   `$GIT_DIR/git-bump.lua`

    Per-repository config file, not intended for sharing.

-   `$GIT_WORK_TREE/.git-bump.lua`

    Per-repository config file, may be checked into Git for sharing.

Those locations will be evaluated in order, a later file overrides mappings of
the previous ones if they have matching keys. Missing config files will be
silently ignored. However, all files missing results in an error, since
`git-bump` needs at least one config file to do something.

## Sample Functions

Find the latest sample config file here: 
https://github.com/FloGa/git-bump/blob/develop/.git-bump.lua

This is a non-exhaustive list of possible functions that can be used in your
config files. If you have ideas for more default functions, don't hesitate to
open a PR!