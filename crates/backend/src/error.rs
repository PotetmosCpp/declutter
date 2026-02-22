use {
    std::{
        env::VarError,
        io,
    },
    freedesktop_file_parser::ParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Var(#[from] VarError),
    #[error(transparent)]
    Alpm(#[from] alpm::Error),
    #[error(transparent)]
    Trash(#[from] trash::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
}
