//! A minimal [_XDG Base Directory Specification_](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)
//! library for the Rust programming language.
//! ```rust
//! use microxdg::{Xdg, XdgError};
//!
//! fn main() -> Result<(), XdgError> {
//!     let xdg = Xdg::new()?;
//!
//!     let cache_dir = xdg.cache()?;
//!     let config_dir = xdg.config()?;
//!     let data_dir = xdg.data()?;
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]

mod error;

use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

pub use error::XdgError;

/// XDG Base Directory Specification's directories.
enum XdgDir {
    Cache,
    Config,
    Data,
    State,
}

impl XdgDir {
    /// Returns the XDG environment variable associated to the XDG base
    /// directory.
    fn env_var(&self) -> &'static str {
        match self {
            XdgDir::Cache => "XDG_CACHE_HOME",
            XdgDir::Config => "XDG_CONFIG_HOME",
            XdgDir::Data => "XDG_DATA_HOME",
            XdgDir::State => "XDG_STATE_HOME",
        }
    }

    /// Returns the _user-specific_ fallback directory in the case the XDG
    /// environment variable is not set.
    fn fallback(&self) -> &'static str {
        match self {
            XdgDir::Cache => ".cache",
            XdgDir::Config => ".config",
            XdgDir::Data => ".local/share",
            XdgDir::State => ".local/state",
        }
    }
}

/// XDG Base Directory Specification's _system-wide_ directories.
enum XdgSysDirs {
    Config,
    Data,
}

impl XdgSysDirs {
    /// Returns the XDG environment variable associated to the XDG base
    /// directories.
    fn env_var(&self) -> &'static str {
        match self {
            XdgSysDirs::Config => "XDG_CONFIG_DIRS",
            XdgSysDirs::Data => "XDG_DATA_DIRS",
        }
    }

    /// Returns an owned vector of paths from a slice of string literals.
    #[inline]
    fn from_paths(paths: &[&str]) -> Vec<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    /// Returns the fallback directories in the case the XDG environment
    /// variable is not set.
    fn fallback(&self) -> Vec<PathBuf> {
        match self {
            XdgSysDirs::Config => XdgSysDirs::from_paths(&["/etc/xdg"]),
            XdgSysDirs::Data => XdgSysDirs::from_paths(&["/usr/local/share", "/usr/share"]),
        }
    }
}

/// _An implementation of the [XDG Base Directory Specification](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)_.
///
/// Allows to retrieve:
/// - _user-specific_ XDG Base Directories:
///     - [_cache_](method@Xdg::cache);
///     - [_configuration_](method@Xdg::config);
///     - [_data_](method@Xdg::data);
///     - [_state_](method@Xdg::state);
///     - [_executable_](method@Xdg::exec);
///     - [_runtime_](method@Xdg::runtime);
/// - _system-wide_, preference ordered (order denotes importance):
///     - [_configuration_](method@Xdg::sys_config);
///     - [_data_](method@Xdg::sys_data).
///
/// Each of the base directories methods privileges the relative environment
/// variable's value and falls back to the corresponding default whenever the
/// environment variable is not set or set to an empty value.
///
/// # Examples
///
/// The example below retrieves the _user-specific XDG configuration directory_
/// by reading the value of the `XDG_CONFIG_HOME` environment variable
/// (similarly the other XDG directories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{Xdg, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::set_var("XDG_CONFIG_HOME", "/home/user/.config");
///
/// let xdg = Xdg::new()?;
/// assert_eq!(PathBuf::from("/home/user/.config"), xdg.config()?);
/// # Ok(())
/// # }
/// ```
///
/// In the case the `XDG_CONFIG_DIR` environment variable is not set,
/// `$HOME/.config` is used as a fallback (similarly the other XDG directories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{Xdg, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::set_var("HOME", "/home/user");
///
/// let xdg = Xdg::new()?;
/// assert_eq!(PathBuf::from("/home/user/.config"), xdg.config()?);
/// # Ok(())
/// # }
/// ```
///
/// Ultimately, if also the `HOME` environment variable is not set (very
/// unlikely), `/home/$USER/.config` is used as a fallback (similarly the other
/// XDG directories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{Xdg, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::remove_var("HOME");
/// std::env::set_var("USER", "user");
///
/// let xdg = Xdg::new()?;
/// assert_eq!(PathBuf::from("/home/user/.config"), xdg.config()?);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Xdg {
    /// User's home directory.
    home: PathBuf,
}

impl Xdg {
    /// Constructs a new [`Xdg`] instance from a string representation of the
    /// `HOME` path.
    #[inline]
    fn new_from_string(home: String) -> Xdg {
        Xdg {
            home: PathBuf::from(home),
        }
    }

    /// Constructs a new [`Xdg`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment
    /// variable is set.
    pub fn new() -> Result<Xdg, XdgError> {
        if let Ok(home) = env::var("HOME") {
            return Ok(Xdg::new_from_string(home));
        }

        if let Ok(user) = env::var("USER") {
            return Ok(Xdg::new_from_string(format!("/home/{user}")));
        }

        Err(XdgError::HomeNotFound)
    }

    /// Constructs a new [`XdgApp`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment
    /// variable is set.
    pub fn new_app(app_name: &'static str) -> Result<XdgApp, XdgError> {
        XdgApp::new(app_name)
    }

    /// Returns the **home** directory of the user owning the process.
    #[inline]
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// Returns a validated path from an XDG environment variable.
    ///
    /// # Errors
    ///
    /// This function returns an error if the value represents a relative path:
    /// XDG environment variables must be set to absolute paths.
    #[inline]
    fn validate_path<S>(env_var_key: &'static str, env_var_val: S) -> Result<PathBuf, XdgError>
    where
        S: Into<String>,
    {
        let path = PathBuf::from(env_var_val.into());

        if path.is_relative() {
            // XDG environment contains a relative path.
            return Err(XdgError::EnvVarRelativePath { env_var_key, path });
        }

        Ok(path)
    }

    /// Returns the path set to an XDG environment variable or a fallback
    /// in the case the environment variable is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set a relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn get_path(&self, dir: XdgDir) -> Result<PathBuf, XdgError> {
        let env_var_key = dir.env_var();
        match env::var(env_var_key) {
            // XDG environment variable is set and non-empty.
            Ok(env_var_val) if !env_var_val.is_empty() => {
                Xdg::validate_path(env_var_key, env_var_val)
            }
            // XDG environment variable is set but contains invalid unicode.
            Err(VarError::NotUnicode(env_var_val)) => Err(XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            }),
            // XDG environment variable is not set or set but empty.
            _ => Ok(self.home.join(dir.fallback())),
        }
    }

    /// Returns the _user-specific_ XDG **cache** directory specified by
    /// the `XDG_CACHE_HOME` environment variable. Falls back to
    /// `$HOME/.cache` if `XDG_CACHE_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set to a relative path;
    /// - the `XDG_CACHE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let cache_dir = xdg.cache()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgDir::Cache)
    }

    /// Returns the _user-specific_ XDG **configuration** directory specified by
    /// the `XDG_CONFIG_HOME` environment variable. Falls back to
    /// `$HOME/.config` if `XDG_CONFIG_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let config_dir = xdg.config()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgDir::Config)
    }

    /// Returns the _user-specific_ XDG **data** directory specified by
    /// the `XDG_DATA_HOME` environment variable. Falls back to
    /// `$HOME/.local/share` if `XDG_DATA_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set to a relative path;
    /// - the `XDG_DATA_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let data_dir = xdg.data()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgDir::Data)
    }

    /// Returns the _user-specific_ XDG **state** directory specified by
    /// the `XDG_STATE_HOME` environment variable. Falls back to
    /// `$HOME/.local/state` if `XDG_STATE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set to a relative path;
    /// - the `XDG_STATE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let state_dir = xdg.state()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgDir::State)
    }

    /// Returns the XDG **runtime** directory specified by the
    /// `XDG_RUNTIME_DIR` environment variable.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the `XDG_RUNTIME_DIR` environment variable is set;
    /// - `None` if the `XDG_RUNTIME_DIR` environment variable is missing or
    ///   set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_RUNTIME_DIR` environment variable is set to a relative path;
    /// - the `XDG_RUNTIME_DIR` environment variable contains invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// match xdg.runtime()? {
    ///     Some(runtime_dir) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn runtime(&self) -> Result<Option<PathBuf>, XdgError> {
        const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

        match env::var(XDG_RUNTIME_DIR) {
            // The runtime environment variable is set.
            Ok(env_var_val) if !env_var_val.is_empty() => {
                Ok(Some(Xdg::validate_path(XDG_RUNTIME_DIR, env_var_val)?))
            }
            // The runtime environment variable is set but contains invalid unicode.
            Err(VarError::NotUnicode(env_var_val)) => Err(XdgError::InvalidUnicode {
                env_var_key: XDG_RUNTIME_DIR,
                env_var_val,
            }),
            _ => Ok(None),
        }
    }

    /// Returns the _user-specific_ XDG **executable** directory specified by
    /// `$HOME/.local/bin`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let exec_dir = xdg.exec();
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn exec(&self) -> PathBuf {
        self.home.join(".local/bin")
    }

    /// Returns the preference-ordered _system-widw_ paths set to a system XDG
    /// environment variable or a fallback in the case the environment variable
    /// is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn get_sys_paths(&self, dirs: XdgSysDirs) -> Result<Vec<PathBuf>, XdgError> {
        let env_var_key = dirs.env_var();
        match env::var(env_var_key) {
            // XDG environment variable is set and non-empty.
            Ok(env_var_val) if !env_var_val.is_empty() => env_var_val
                .split(':')
                .map(|path_str| Xdg::validate_path(env_var_key, path_str))
                .collect(),
            // XDG environment variable is set but contains invalid unicode.
            Err(VarError::NotUnicode(env_var_val)) => Err(XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            }),
            // XDG environment variable is not set or set but empty.
            _ => Ok(dirs.fallback()),
        }
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration**
    /// directories specified by the `XDG_CONFIG_DIRS` environment variable,
    /// Falls back to `/usr/local/share:/usr/share` if `XDG_CONFIG_DIRS` is not
    /// set or is set to an empty value.
    ///
    /// # Note
    ///
    /// Used to search for config files in addition to the `XDG_CONFIG_HOME`
    /// user-specific base directory.
    ///
    /// The order denotes of the directories denotes the importance: the first
    /// directory is the most important, the last directory is the least
    /// important.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_DIRS` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let mut config_dirs = xdg.sys_config()?;
    /// config_dirs.push(xdg.config()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn sys_config(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_sys_paths(XdgSysDirs::Config)
    }

    /// Returns the system-wide, preference-ordered, XDG **data**
    /// directories specified by the `XDG_DATA_DIRS` environment variable,
    /// Falls back to `/etc/xdg` if `XDG_DATA_DIRS` is not set or is set to an
    /// empty value.
    ///
    /// # Note
    ///
    /// Used to search for data files in addition to the `XDG_DATA_HOME`
    /// user-specific base directory.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_DIRS` environment variable is set to a relative path;
    /// - the `XDG_DATA_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let mut data_dirs = xdg.sys_data()?;
    /// data_dirs.push(xdg.data()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_sys_paths(XdgSysDirs::Data)
    }
}

/// _An implementation of the [XDG Base Directory Specification](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)_
/// with extent to application specific subdirectories.
///
/// Allows to retrieve:
/// - _user-specific_ XDG Base Directories :
///     - [_cache_](method@XdgApp::cache);
///     - [_configuration_](method@XdgApp::config);
///     - [_data_](method@XdgApp::data);
///     - [_state_](method@XdgApp::state);
///     - [_executable_](method@XdgApp::exec);
///     - [_runtime_](method@XdgApp::runtime);
/// - _user-specific_ **application subdirectories**:
///     - [_app cache_](method@XdgApp::app_cache);
///     - [_app configuration_](method@XdgApp::app_config);
///     - [_app data_](method@XdgApp::app_data);
///     - [_app state_](method@XdgApp::app_state);
/// - _system-wide_, preference ordered (order denotes importance):
///     - [_configuration_](method@XdgApp::sys_config);
///     - [_data_](method@XdgApp::sys_data);
/// - _system-wide_, preference ordered (order denotes importance),
///   **application subdirectories**:
///     - [_app configuration_](method@XdgApp::app_sys_config);
///     - [_app data_](method@XdgApp::app_sys_data).
///
/// Each of the base directories methods privileges the relative environment
/// variable's value and falls back to the corresponding default whenever the
/// environment variable is not set or set to an empty value.
///
/// # Examples
///
/// The example below retrieves the _user-specific XDG app configuration
/// subdirectory_ by reading the value of the `XDG_CONFIG_HOME` environment
/// variable as `$XDG_CONFIG_HOME/app_name` (similarly the other XDG
/// application subdirectories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{XdgApp, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::set_var("XDG_CONFIG_HOME", "/home/user/.config");
///
/// let xdg = XdgApp::new("app_name")?;
/// assert_eq!(PathBuf::from("/home/user/.config/app_name"), xdg.app_config()?);
/// # Ok(())
/// # }
/// ```
///
/// In the case the `XDG_CONFIG_DIR` environment variable is not set,
/// `$HOME/.config/app_name` is used as a fallback (similarly the other XDG
/// application subdirectories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{XdgApp, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::set_var("HOME", "/home/user");
///
/// let xdg = XdgApp::new("app_name")?;
/// assert_eq!(PathBuf::from("/home/user/.config/app_name"), xdg.app_config()?);
/// # Ok(())
/// # }
/// ```
///
/// Ultimately, if also the `HOME` environment variable is not set (very
/// unlikely), `/home/$USER/.config/app_name` is used as a fallback (similarly
/// the other XDG directories):
/// ```rust
/// # use std::{error::Error, path::PathBuf};
/// # use microxdg::{XdgApp, XdgError};
/// # fn main() -> Result<(), XdgError> {
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::remove_var("HOME");
/// std::env::set_var("USER", "user");
///
/// let xdg = XdgApp::new("app_name")?;
/// assert_eq!(PathBuf::from("/home/user/.config/app_name"), xdg.app_config()?);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct XdgApp {
    /// XDG instance.
    xdg: Xdg,
    /// Application name.
    name: &'static str,
}

impl XdgApp {
    /// Constructs a new [`XdgApp`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment
    /// variable is set.
    pub fn new(app_name: &'static str) -> Result<XdgApp, XdgError> {
        Ok(XdgApp {
            xdg: Xdg::new()?,
            name: app_name,
        })
    }

    /// Converts an [`Xdg`] instance to [`XdgApp`].
    pub fn from_xdg(xdg: Xdg, app_name: &'static str) -> XdgApp {
        XdgApp {
            xdg,
            name: app_name,
        }
    }

    /// Returns the **home** directory of the user owning the process.
    #[inline]
    pub fn home(&self) -> &Path {
        self.xdg.home()
    }

    /// Returns the _user-specific_ XDG **cache** directory specified by
    /// the `XDG_CACHE_HOME` environment variable. Falls back to
    /// `$HOME/.cache` if `XDG_CACHE_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set to a relative path;
    /// - the `XDG_CACHE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let cache_dir = xdg.cache()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.xdg.cache()
    }

    /// Returns the _user-specific_ XDG **configuration** directory specified by
    /// the `XDG_CONFIG_HOME` environment variable. Falls back to
    /// `$HOME/.config` if `XDG_CONFIG_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let config_dir = xdg.config()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.xdg.config()
    }

    /// Returns the _user-specific_ XDG **data** directory specified by
    /// the `XDG_DATA_HOME` environment variable. Falls back to
    /// `$HOME/.local/share` if `XDG_DATA_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set to a relative path;
    /// - the `XDG_DATA_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let data_dir = xdg.data()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.xdg.data()
    }

    /// Returns the _user-specific_ XDG **state** directory specified by
    /// the `XDG_STATE_HOME` environment variable. Falls back to
    /// `$HOME/.local/state` if `XDG_STATE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set to a relative path;
    /// - the `XDG_STATE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let state_dir = xdg.state()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.xdg.state()
    }

    /// Returns the XDG **runtime** directory specified by the
    /// `XDG_RUNTIME_DIR` environment variable.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the `XDG_RUNTIME_DIR` environment variable is set;
    /// - `None` if the `XDG_RUNTIME_DIR` environment variable is missing or
    ///   set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_RUNTIME_DIR` environment variable is set to a relative path;
    /// - the `XDG_RUNTIME_DIR` environment variable contains invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.runtime()? {
    ///     Some(runtime_dir) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn runtime(&self) -> Result<Option<PathBuf>, XdgError> {
        self.xdg.runtime()
    }

    /// Returns the _user-specific_ XDG **executable** directory specified by
    /// `$HOME/.local/bin`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let exec_dir = xdg.exec();
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn exec(&self) -> PathBuf {
        self.xdg.exec()
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration**
    /// directories specified by the `XDG_CONFIG_DIRS` environment variable,
    /// Falls back to `/etc/xdg` if `XDG_CONFIG_DIRS` is not set or is set to
    /// an empty value.
    ///
    /// # Note
    ///
    /// Used to search for config files in addition to the `XDG_CONFIG_HOME`
    /// user-specific base directory.
    ///
    /// The order denotes of the directories denotes the importance: the first
    /// directory is the most important, the last directory is the least
    /// important.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_DIRS` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let mut config_dirs = xdg.sys_config()?;
    /// config_dirs.push(xdg.config()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn sys_config(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.xdg.sys_config()
    }

    /// Returns the system-wide, preference-ordered, XDG **data**
    /// directories specified by the `XDG_DATA_DIRS` environment variable,
    /// Falls back to `/usr/local/share:/usr/share` if `XDG_DATA_DIRS` is not
    /// set or is set to an
    /// empty value.
    ///
    /// # Note
    ///
    /// Used to search for data files in addition to the `XDG_DATA_HOME`
    /// user-specific base directory.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_DIRS` environment variable is set to a relative path;
    /// - the `XDG_DATA_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let mut data_dirs = xdg.sys_data()?;
    /// data_dirs.push(xdg.data()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.xdg.sys_data()
    }

    /// Appends the app name to `path`.
    #[inline]
    fn append_app_name(&self, mut path: PathBuf) -> PathBuf {
        path.push(self.name);
        path
    }

    /// Returns the user-specific XDG **cache** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG cache directory specified by the
    /// `XDG_CACHE_HOME` if available. Falls back to `$HOME/.cache/<app_name>`
    /// if `XDG_CACHE_HOME` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::cache`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set to a relative path;
    /// - the `XDG_CACHE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let app_cache_dir = xdg.app_cache()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn app_cache(&self) -> Result<PathBuf, XdgError> {
        Ok(self.append_app_name(self.xdg.cache()?))
    }

    /// Returns the user-specific XDG **configuration** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG configuration directory specified by the
    /// `XDG_CONFIG_HOME` if available. Falls back to `$HOME/.config/<app_name>`
    /// if `XDG_CONFIG_HOME` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::config`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let config_dir = xdg.config()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn app_config(&self) -> Result<PathBuf, XdgError> {
        Ok(self.append_app_name(self.xdg.config()?))
    }

    /// Returns the user-specific XDG **data** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG state directory specified by the
    /// `XDG_DATA_HOME` if available. Falls back to `$HOME/.local/share/<app_name>`
    /// if `XDG_DATA_HOME` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::data`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set to a relative path;
    /// - the `XDG_DATA_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let data_dir = xdg.data()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn app_data(&self) -> Result<PathBuf, XdgError> {
        Ok(self.append_app_name(self.xdg.data()?))
    }

    /// Returns the user-specific XDG **state** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG state directory specified by the
    /// `XDG_STATE_HOME` if available. Falls back to `$HOME/.local/state/<name>`
    /// if `XDG_STATE_HOME` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::state`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set to a relative path;
    /// - the `XDG_STATE_HOME` environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let state_dir = xdg.state()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn app_state(&self) -> Result<PathBuf, XdgError> {
        Ok(self.append_app_name(self.xdg.state()?))
    }

    /// Appends app names to each element of `paths`.
    #[inline]
    fn append_app_name_each(&self, mut paths: Vec<PathBuf>) -> Vec<PathBuf> {
        for path in paths.iter_mut() {
            path.push(self.name);
        }

        paths
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration**
    /// directories for the current application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered config directories specified by
    /// the `XDG_CONFIG_DIRS` environment variable. Falls back to `/etc/xdg`
    /// if `XDG_CONFIG_DIRS` is not set or is set
    /// to an empty value.
    ///
    /// See [`XdgApp::sys_config`] for further details.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_DIRS` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let mut config_dirs = xdg.sys_config()?;
    /// config_dirs.push(xdg.app_config()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn app_sys_config(&self) -> Result<Vec<PathBuf>, XdgError> {
        Ok(self.append_app_name_each(self.sys_config()?))
    }

    /// Returns the _system-wide_, preference-ordered, XDG **data** directories
    /// for the current application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered config directories specified by
    /// the `XDG_DATA_DIRS` environment variable. Falls back to
    /// `/usr/local/share:/usr/share` if `XDG_DATA_DIRS` is not set or is set
    /// to an empty value.
    ///
    /// See [`XdgApp::sys_data`] for further details.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_DIRS` environment variable is set to a relative path;
    /// - the `XDG_DATA_DIRS` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let mut data_dirs = xdg.sys_data()?;
    /// data_dirs.push(xdg.app_data()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn app_sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        Ok(self.append_app_name_each(self.sys_data()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, ffi::OsStr, os::unix::prelude::OsStrExt, path::Path};

    const INVALID_UNICODE_BYTES: [u8; 4] = [0xF0, 0x90, 0x80, 0x67];

    #[test]
    fn new_xdg() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user2");
        assert_eq!(Path::new("/home/user1"), Xdg::new()?.home());
        assert_eq!(Path::new("/home/user1"), Xdg::new_app("app_name")?.home());
        assert_eq!(Path::new("/home/user1"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user1"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home()
        );

        env::remove_var("HOME");
        assert_eq!(Path::new("/home/user2"), Xdg::new()?.home());
        assert_eq!(Path::new("/home/user2"), Xdg::new_app("app_name")?.home());
        assert_eq!(Path::new("/home/user2"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user2"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home()
        );

        env::remove_var("USER");
        assert_eq!(XdgError::HomeNotFound, Xdg::new().unwrap_err());
        assert_eq!(
            XdgError::HomeNotFound,
            Xdg::new_app("app_name").unwrap_err()
        );
        assert_eq!(XdgError::HomeNotFound, XdgApp::new("app_name").unwrap_err());

        Ok(())
    }

    #[test]
    fn usr_base_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");
        env::remove_var("XDG_RUNTIME_DIR");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user1");

        let xdg = Xdg::new()?;
        assert_eq!(Path::new("/home/user1/.local/bin"), xdg.exec());

        assert_eq!(Path::new("/home/user1"), xdg.home());
        assert_eq!(Path::new("/home/user1/.cache"), xdg.cache()?);
        assert_eq!(Path::new("/home/user1/.config"), xdg.config()?);
        assert_eq!(Path::new("/home/user1/.local/share"), xdg.data()?);
        assert_eq!(Path::new("/home/user1/.local/state"), xdg.state()?);
        assert_eq!(None, xdg.runtime()?);

        env::set_var("XDG_CACHE_HOME", "/home/user2/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user2/.config");
        env::set_var("XDG_DATA_HOME", "/home/user2/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user2/.local/state");
        env::set_var("XDG_RUNTIME_DIR", "/run/user/1000");
        assert_eq!(Path::new("/home/user2/.cache"), xdg.cache()?);
        assert_eq!(Path::new("/home/user2/.config"), xdg.config()?);
        assert_eq!(Path::new("/home/user2/.local/share"), xdg.data()?);
        assert_eq!(Path::new("/home/user2/.local/state"), xdg.state()?);
        assert_eq!(Some(PathBuf::from("/run/user/1000")), xdg.runtime()?);

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");
        env::set_var("XDG_RUNTIME_DIR", "");
        assert_eq!(Path::new("/home/user1/.cache"), xdg.cache()?);
        assert_eq!(Path::new("/home/user1/.config"), xdg.config()?);
        assert_eq!(Path::new("/home/user1/.local/share"), xdg.data()?);
        assert_eq!(Path::new("/home/user1/.local/state"), xdg.state()?);
        assert_eq!(None, xdg.runtime()?);

        env::set_var("XDG_CACHE_HOME", "./cache");
        env::set_var("XDG_CONFIG_HOME", "./config");
        env::set_var("XDG_DATA_HOME", "./data");
        env::set_var("XDG_STATE_HOME", "./state");
        env::set_var("XDG_RUNTIME_DIR", "./runtime");
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_CACHE_HOME",
                path: PathBuf::from("./cache")
            },
            xdg.cache().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_CONFIG_HOME",
                path: PathBuf::from("./config")
            },
            xdg.config().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_DATA_HOME",
                path: PathBuf::from("./data")
            },
            xdg.data().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_STATE_HOME",
                path: PathBuf::from("./state")
            },
            xdg.state().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_RUNTIME_DIR",
                path: PathBuf::from("./runtime")
            },
            xdg.runtime().unwrap_err()
        );

        let invalid_unicode = OsStr::from_bytes(&INVALID_UNICODE_BYTES);
        env::set_var("XDG_CACHE_HOME", invalid_unicode);
        env::set_var("XDG_CONFIG_HOME", invalid_unicode);
        env::set_var("XDG_DATA_HOME", invalid_unicode);
        env::set_var("XDG_STATE_HOME", invalid_unicode);
        env::set_var("XDG_RUNTIME_DIR", invalid_unicode);
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_CACHE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.cache().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_CONFIG_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.config().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_DATA_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.data().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_STATE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.state().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_RUNTIME_DIR",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.runtime().unwrap_err(),
        );

        Ok(())
    }

    #[test]
    fn usr_app_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user1");

        let xdg = XdgApp::new("app_name")?;
        assert_eq!(Path::new("/home/user1/.cache/app_name"), xdg.app_cache()?);
        assert_eq!(Path::new("/home/user1/.config/app_name"), xdg.app_config()?);
        assert_eq!(
            Path::new("/home/user1/.local/share/app_name"),
            xdg.app_data()?
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/app_name"),
            xdg.app_state()?
        );

        env::set_var("XDG_CACHE_HOME", "/home/user2/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user2/.config");
        env::set_var("XDG_DATA_HOME", "/home/user2/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user2/.local/state");
        assert_eq!(Path::new("/home/user2/.cache/app_name"), xdg.app_cache()?);
        assert_eq!(Path::new("/home/user2/.config/app_name"), xdg.app_config()?);
        assert_eq!(
            Path::new("/home/user2/.local/share/app_name"),
            xdg.app_data()?
        );
        assert_eq!(
            Path::new("/home/user2/.local/state/app_name"),
            xdg.app_state()?
        );

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");
        assert_eq!(Path::new("/home/user1/.cache/app_name"), xdg.app_cache()?);
        assert_eq!(Path::new("/home/user1/.config/app_name"), xdg.app_config()?);
        assert_eq!(
            Path::new("/home/user1/.local/share/app_name"),
            xdg.app_data()?
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/app_name"),
            xdg.app_state()?
        );

        env::set_var("XDG_CACHE_HOME", "./app_name/cache");
        env::set_var("XDG_CONFIG_HOME", "./app_name/config");
        env::set_var("XDG_DATA_HOME", "./app_name/data");
        env::set_var("XDG_STATE_HOME", "./app_name/state");
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_CACHE_HOME",
                path: PathBuf::from("./app_name/cache")
            },
            xdg.app_cache().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_CONFIG_HOME",
                path: PathBuf::from("./app_name/config")
            },
            xdg.app_config().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_DATA_HOME",
                path: PathBuf::from("./app_name/data")
            },
            xdg.app_data().unwrap_err()
        );
        assert_eq!(
            XdgError::EnvVarRelativePath {
                env_var_key: "XDG_STATE_HOME",
                path: PathBuf::from("./app_name/state")
            },
            xdg.app_state().unwrap_err()
        );

        let invalid_unicode = OsStr::from_bytes(&INVALID_UNICODE_BYTES);
        env::set_var("XDG_CACHE_HOME", invalid_unicode);
        env::set_var("XDG_CONFIG_HOME", invalid_unicode);
        env::set_var("XDG_DATA_HOME", invalid_unicode);
        env::set_var("XDG_STATE_HOME", invalid_unicode);
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_CACHE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.app_cache().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_CONFIG_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.app_config().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_DATA_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.app_data().unwrap_err(),
        );
        assert_eq!(
            XdgError::InvalidUnicode {
                env_var_key: "XDG_STATE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            },
            xdg.app_state().unwrap_err(),
        );

        Ok(())
    }

    #[test]
    fn sys_base_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CONFIG_DIRS");
        env::remove_var("XDG_DATA_DIRS");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = Xdg::new()?;
        assert_eq!(vec![PathBuf::from("/etc/xdg")], xdg.sys_config()?);
        assert_eq!(
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share")
            ],
            xdg.sys_data()?,
        );

        env::set_var(
            "XDG_CONFIG_DIRS",
            "/config/dir1:/config/dir2:/config/dir3:/config/dir4",
        );
        env::set_var(
            "XDG_DATA_DIRS",
            "/data/dir1:/data/dir2:/data/dir3:/data/dir4",
        );
        assert_eq!(
            vec![
                PathBuf::from("/config/dir1"),
                PathBuf::from("/config/dir2"),
                PathBuf::from("/config/dir3"),
                PathBuf::from("/config/dir4"),
            ],
            xdg.sys_config()?,
        );
        assert_eq!(
            vec![
                PathBuf::from("/data/dir1"),
                PathBuf::from("/data/dir2"),
                PathBuf::from("/data/dir3"),
                PathBuf::from("/data/dir4"),
            ],
            xdg.sys_data()?,
        );

        Ok(())
    }

    #[test]
    fn sys_app_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CONFIG_DIRS");
        env::remove_var("XDG_DATA_DIRS");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = XdgApp::new("app_name")?;

        assert_eq!(
            vec![PathBuf::from("/etc/xdg/app_name")],
            xdg.app_sys_config()?
        );
        assert_eq!(
            vec![
                PathBuf::from("/usr/local/share/app_name"),
                PathBuf::from("/usr/share/app_name")
            ],
            xdg.app_sys_data()?,
        );

        env::set_var(
            "XDG_CONFIG_DIRS",
            "/config/dir1:/config/dir2:/config/dir3:/config/dir4",
        );
        env::set_var(
            "XDG_DATA_DIRS",
            "/data/dir1:/data/dir2:/data/dir3:/data/dir4",
        );
        assert_eq!(
            vec![
                PathBuf::from("/config/dir1/app_name"),
                PathBuf::from("/config/dir2/app_name"),
                PathBuf::from("/config/dir3/app_name"),
                PathBuf::from("/config/dir4/app_name"),
            ],
            xdg.app_sys_config()?,
        );
        assert_eq!(
            vec![
                PathBuf::from("/data/dir1/app_name"),
                PathBuf::from("/data/dir2/app_name"),
                PathBuf::from("/data/dir3/app_name"),
                PathBuf::from("/data/dir4/app_name"),
            ],
            xdg.app_sys_data()?,
        );

        Ok(())
    }
}
