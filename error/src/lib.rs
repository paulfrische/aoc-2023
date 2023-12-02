use thiserror::Error;

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("parser error")]
    ParserError,
}

pub type Result<T> = std::result::Result<T, AOCError>;

impl<T> From<nom::Err<T>> for AOCError {
    fn from(_: nom::Err<T>) -> Self {
        AOCError::ParserError
    }
}
