/**
 * File: error.rs
 * Author: alukard <alukard6942@github>
 * Date: 11.06.2022
 * Last Modified Date: 11.06.2022
 */


use thiserror::Error;


#[derive(Debug, Error)]
pub enum TftpError {
    #[error("unenable to connect to socket")]
    SockError,

    #[error("data makes no sence in this context")]
    InvalidData,

    #[error("packet has not enough size for data")]
    PackError,

    #[error("invalide mode")]
    InvalidMode,

    #[error("Nonspecific Error, should not be used !!!Temporary!!!")]
    TODO,

    #[error("too many failed tryes")]
    TooManyFailTryes,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TftpError>;
