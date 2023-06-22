use std::{error, ffi::OsString, fmt, path::PathBuf};

/// XDG Base Directory specification errors.
#[derive(Debug, PartialEq, Eq)]
pub enum XdgError {
    /// Unable to retrieve user's home directory.
    HomeNotFound,
    /// XDG environment variable contains a relative path.
    EnvVarRelativePath {
        /// XDG environment variable key (variable name).
        env_var_key: &'static str,
        /// XDG environment variable's relative path.
        path: PathBuf,
    },
    /// XDG Environment variable set to invalid unicode.
    InvalidUnicode {
        /// XDG environment variable key (variable name).
        env_var_key: &'static str,
        /// XDG environment variable value.
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
            XdgError::EnvVarRelativePath { env_var_key, path } => write!(
                f,
                "environment variable '{env_var_key}' contains a relative \
                    path '{path}' (paths in XDG environment variables must \
                    be asbolute)",
                path = path.display()
            ),
            XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            } => write!(
                f,
                "environment variable '{env_var_key}' contains invalid unicode \
                    {env_var_val:?}",
            ),
        }
    }
}

impl error::Error for XdgError {}
