use std::collections::HashMap;
use std::env::args;
use std::fs;

use mlua::prelude::*;
use mlua::Function;

fn main() -> LuaResult<()> {
    let version = args().nth(1).unwrap();

    let lua = Lua::new();

    let map = lua
        .load(&fs::read_to_string("bump.lua").unwrap())
        .eval::<HashMap<String, Function>>()?;

    for (file, f) in map {
        let mut contents = f.call::<_, String>(version.clone())?;
        if !contents.ends_with('\n') {
            contents.push('\n')
        }

        fs::write(file, contents).unwrap();
    }

    Ok(())
}
