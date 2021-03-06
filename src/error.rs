use std::error;
use std::fmt;

pub enum Error {
    OciError(DbError),
    DpiError(DbError),
    InvalidColumnIndex(usize, usize),
    InvalidColumnName(String),
    InvalidTypeConversion(String, String),
    OutOfRange(String, String),
    NullConversionError,
    NoMoreData,
    InternalError(String),
}

#[derive(Eq, PartialEq, Clone)]
pub struct DbError {
    code: i32,
    offset: u16,
    message: String,
    fn_name: String,
    action: String,
}

impl DbError {
    pub fn new(code: i32, offset: u16, message: String, fn_name: String, action: String) -> DbError {
        DbError {
            code: code,
            offset: offset,
            message: message,
            fn_name: fn_name,
            action: action,
        }
    }

    /// Oracle error code if OciError. always zero if DpiError
    pub fn code(&self) -> i32 {
        self.code
    }

    /// ? (used for Batch Errors?)
    pub fn offset(&self) -> u16 {
        self.offset
    }

    /// error message
    pub fn message(&self) -> &String {
        &self.message
    }

    /// function name in ODPI-C used by rust-oracle
    pub fn fn_name(&self) -> &String {
        &self.fn_name
    }

    /// action name in ODPI-C used by rust-oracle
    pub fn action(&self) -> &String {
        &self.action
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OciError(ref err) =>
                write!(f, "OCI Error: {}", err.message),
            Error::DpiError(ref err) =>
                write!(f, "DPI Error: {}", err.message),
            Error::InvalidColumnIndex(idx, num_cols) =>
                write!(f, "Invalid column index {} for 1..{}", idx, num_cols),
            Error::InvalidColumnName(ref name) =>
                write!(f, "Invalid column name {}", name),
            Error::InvalidTypeConversion(ref from_type, ref to_type) =>
                write!(f, "Invalid type conversion from {} to {}", from_type, to_type),
            Error::OutOfRange(ref from_type, ref to_type) =>
                write!(f, "Out of range while converting {} to {}", from_type, to_type),
            Error::NullConversionError =>
                write!(f, "Null conversion error"),
            Error::NoMoreData =>
                write!(f, "No more data to be fetched"),
            Error::InternalError(ref msg) =>
                write!(f, "Internal Error: {}", msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OciError(ref err) =>
                write!(f, "OCI Error: (code: {}, offset: {}, message:{}, fn_name: {}, action: {})",
                       err.code, err.offset, err.message, err.fn_name, err.action),
            Error::DpiError(ref err) =>
                write!(f, "OCI Error: (code: {}, offset: {}, message:{}, fn_name: {}, action: {})",
                       err.code, err.offset, err.message, err.fn_name, err.action),
            Error::InvalidColumnIndex(_, _) |
            Error::InvalidColumnName(_) |
            Error::InvalidTypeConversion(_, _) |
            Error::OutOfRange(_, _) |
            Error::NullConversionError |
            Error::NoMoreData |
            Error::InternalError(_) =>
                write!(f, "{}", *self),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OciError(_) => "Oracle OCI error",
            Error::DpiError(_) => "Oracle DPI Error",
            Error::InvalidColumnIndex(_, _) => "Invalid column index",
            Error::InvalidColumnName(_) => "Invalid column name",
            Error::InvalidTypeConversion(_, _) => "Invalid type conversion",
            Error::OutOfRange(_, _) => "Out of range error",
            Error::NullConversionError => "Null conversion error",
            Error::NoMoreData => "No more data",
            Error::InternalError(_) => "Internal error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
