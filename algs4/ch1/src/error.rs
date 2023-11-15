// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub enum ErrorKind {
    FmtError,
    IoError,
    ParseIntError,
    DotError,
    ParamError,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, format!("io error: {}", err))
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::from_string(ErrorKind::FmtError, format!("fmt error: {}", err))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::from_string(
            ErrorKind::ParseIntError,
            format!("parse int error: {}", err),
        )
    }
}
