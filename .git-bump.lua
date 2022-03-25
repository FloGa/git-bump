return {
    ["CHANGES.md"] = function(version, content)
        -- either replace the first line with the concrete release number or
        -- add an "upcoming" line, depending on the given version string

        local current_first_line = content:match("^([^\n]*)\n") or ""
        local current_version = current_first_line:match("%d+%.%d+%.%d+")
        local pure_version = version:match("%d+%.%d+%.%d+")

        local format_string
        if current_version == nil or current_version == pure_version then
            -- if the current version is an upcoming version, or if both pure
            -- versions are the same, replace the first line
            format_string = "%s\n%s"
            content = content:gsub("^[^\n]*\n", "", 1)
        else
            -- otherwise, prepend a new line
            format_string = "%s\n\n%s"
        end

        local first_line
        if version:find("%-SNAPSHOT$") then
            -- if the new version is a snapshot, use an "upcoming" header
            first_line = "# Changes since latest release"
        else
            -- otherwise use the concrete version
            first_line = "# Changes in " .. version
        end

        return (format_string):format(first_line, content)
    end,

    ["Cargo.toml"] = function(version, content)
        -- replace the first "version" attribute, which is most likely our
        -- own, with the current version string
        return content:gsub(
                   'version = %b""', ('version = "%s"'):format(version), 1
               )
    end,

    VERSION = function(version)
        return version
    end
}
