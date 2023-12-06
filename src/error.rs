use std::{error, ffi::OsString, fmt, path::PathBuf};

/// [_XDG Base Directory Specification_](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)
/// errors.
#[derive(Debug, PartialEq, Eq)]
pub enum XdgError {
    /// Unable to retrieve user's home directory.
    HomeNotFound,
    /// XDG environment variable contains a relative path (only absolute paths allowed).
    RelativePath {
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XdgError::HomeNotFound => formatter.write_str(
                "Unable to retrieve user's home directory, \
                neither HOME nor USER environment variable set",
            ),
            XdgError::RelativePath { env_var_key, path } => formatter.write_fmt(format_args!(
                "The `{env_var_key}` environment variable contains a relative \
                path, while paths in XDG environment variables must be asbolute: `{path}`",
                path = path.display()
            )),
            XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            } => formatter.write_fmt(format_args!(
                "The `{env_var_key}` environment variable contains invalid unicode: \
                {env_var_val:?}",
            )),
        }
    }
}

impl error::Error for XdgError {}

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, ffi::OsStr, os::unix::prelude::OsStrExt};

    const INVALID_UNICODE_BYTES: [u8; 4] = [0xF0, 0x90, 0x80, 0x67];

    #[test]
    fn display_error() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            "Unable to retrieve user's home directory, \
            neither HOME nor USER environment variable set",
            XdgError::HomeNotFound.to_string()
        );
        assert_eq!(
            "The `XDG_CONFIG_HOME` environment variable contains a relative \
            path, while paths in XDG environment variables must be asbolute: `./config`",
            XdgError::RelativePath {
                env_var_key: "XDG_CONFIG_HOME",
                path: PathBuf::from("./config"),
            }
            .to_string(),
        );
        assert_eq!(
            "The `XDG_CONFIG_HOME` environment variable contains invalid unicode: \
            \"\\xF0\\x90\\x80g\"",
            XdgError::InvalidUnicode {
                env_var_key: "XDG_CONFIG_HOME",
                env_var_val: OsStr::from_bytes(&INVALID_UNICODE_BYTES).to_os_string(),
            }
            .to_string(),
        );

        Ok(())
    }
}
