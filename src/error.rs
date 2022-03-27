pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Not a Git repository")]
    NotARepository,
    #[error("Not supported on bare repositories")]
    BareRepositoryNotSupported,
    #[error("No valid config files found")]
    NoValidConfigFound,
    #[error("Failed to load Lua code: {source}")]
    LuaLoadingFailed { source: mlua::Error },
    #[error("Failed to execute Lua code: {source}")]
    LuaExecutionFailed { source: mlua::Error },
    #[error("Failed to execute pre function: {source}")]
    LuaPreFuncFailed { source: mlua::Error },
    #[error("Failed to execute post function: {source}")]
    LuaPostFuncFailed { source: mlua::Error },
    #[error(transparent)]
    LuaError(#[from] mlua::Error),
    #[error("Failed to read to file: {source}")]
    ReadFailed { source: std::io::Error },
    #[error("Failed to write to file: {source}")]
    WriteFailed { source: std::io::Error },
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
