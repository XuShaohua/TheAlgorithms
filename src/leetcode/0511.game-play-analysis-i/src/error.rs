// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::enum_variant_names)]

use diesel::result::DatabaseErrorKind;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    ConfigError,

    DbConnError,
    DbGeneralError,
    DbUniqueViolationError,
    DbForeignKeyViolationError,
    DbNotFoundError,

    IoError,
}

unsafe impl Send for ErrorKind {}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

#[allow(dead_code)]
impl Error {
    #[must_use]
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_owned(),
        }
    }

    #[must_use]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, err.to_string())
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Self::from_string(ErrorKind::DbConnError, format!("r2d2 err: {err}"))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match &err {
            diesel::result::Error::DatabaseError(kind, _info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    Self::from_string(ErrorKind::DbUniqueViolationError, err.to_string())
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    Self::from_string(ErrorKind::DbForeignKeyViolationError, err.to_string())
                }
                _ => Self::from_string(ErrorKind::DbGeneralError, err.to_string()),
            },
            diesel::result::Error::NotFound => {
                Self::from_string(ErrorKind::DbNotFoundError, err.to_string())
            }
            _ => Self::from_string(ErrorKind::DbGeneralError, err.to_string()),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::from_string(ErrorKind::ConfigError, err.to_string())
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(err: std::ffi::OsString) -> Self {
        Self::from_string(
            ErrorKind::ConfigError,
            format!("OsString to String err: {err:?}"),
        )
    }
}

impl From<dotenvy::Error> for Error {
    fn from(err: dotenvy::Error) -> Self {
        Self::from_string(ErrorKind::ConfigError, format!("dotenv err: {err:?}"))
    }
}
