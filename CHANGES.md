# Changes in 0.3.1

-   Update dependencies

-   Update git2 to 0.20.4

-   Update mlua to 0.11.6

-   Update thiserror to 2.0.18

# Changes in 0.3.0

-   Convert application into binary only

    There is no actual library code that could be shared, so let's stay with
    the main.rs and ditch the lib.rs for now.

-   Update README

-   Update dependencies

# Changes in 0.2.0

-   Add two additional errors for hooks

-   Support pre and post hook functions

    Along with the new contents for a specified file, one can also define
    hook functions that should be run *before* or *after* the new content is
    written to the file.

    The `pre_func` could be used, for example, to create a backup of the
    file prior to updating it. The `post_func` might be used to do some
    house keeping with modified config files.

    The hooks must be returned as a Lua table with the members `pre_func`
    and `post_func`. Both members are optional. If a hook function does not
    exist, it will be silently ignored.

    Example:

    ```lua
    return {
        VERSION = function(version)
            local os = require("os")

            local pre_func = function()
                os.execute("cp VERSION VERSION.old")
            end

            local post_func = function()
                os.execute("git commit -m 'Update VERSION' VERSION")
            end

            return version, {pre_func = pre_func, post_func = post_func}
        end
    }
    ```

-   Add post_func example to git-bump.lua

    This example will run `cargo check` after updating the version in
    Cargo.toml. Not only will this validate the modified config, it will
    also update Cargo.lock accordingly.

-   Add function to print out sample config

-   Create method for printing file paths

    This list will include absolute file paths of all files that will be
    considered when bumping versions.

-   Adjust README regarding missing config files

    Previously, the README mentioned that it is an error if no configuration
    files exist. This was true in the very beginning, but after a few
    practical tests I came to the conclusion that missing configuration
    files should not cause an error, but just be silently ignored. This will
    be better for automation scripts that might run on many repositories,
    including those without any bump configurations.

# Changes in 0.1.0

Initial release.
