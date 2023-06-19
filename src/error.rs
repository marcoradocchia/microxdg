use std::{error, ffi::OsString, fmt, path::PathBuf};

/// XDG Base Directory specification errors.
#[derive(Debug, PartialEq, Eq)]
pub enum XdgError {
    /// Unable to retrieve user's home directory.
    HomeNotFound,
    /// XDG environment variable set to a relative path.
    EnvVarRelativePath {
        env_var_key: &'static str,
        env_var_val: PathBuf,
    },
    /// XDG Environment variable set to invalid unicode.
    InvalidUnicode {
        env_var_key: &'static str,
        env_var_val: OsString,
    },
}

impl fmt::Display for XdgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XdgError::HomeNotFound => write!(
                f,
                "unable to locate user's home directory, \
                    neither HOME or USER environment variables set"
            ),
            XdgError::EnvVarRelativePath {
                env_var_key,
                env_var_val,
            } => write!(
                f,
                "environment variable '{env_var_key}' set to relative \
                    path '{env_var_val}'",
                env_var_val = env_var_val.display()
            ),
            XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            } => write!(
                f,
                "environment variable '{env_var_key}' set to invalid \
                    unicode value '{env_var_val:?}'"
            ),
        }
    }
}

impl error::Error for XdgError {}
