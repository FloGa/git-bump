use mlua::prelude::*;

use git_bump::bump;

fn main() -> LuaResult<()> {
    bump()
}
