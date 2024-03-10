//! An [**XDG Base Directory Specification**](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)
//! Rust library that aims to be conservative on memory allocation and overall memory footprint.
//!
//! # Examples
//!
//! The following example illustrates how to retrieve the _user-specific_ XDG **configuration**
//! base directory:
//! ```rust
//! # use std::{env, path::Path};
//! use microxdg::{Xdg, XdgError};
//!
//! fn main() -> Result<(), XdgError> {
//!     # env::set_var("XDG_CONFIG_HOME", "/home/user/.config");
//!     let xdg = Xdg::new()?;
//!     let config_dir = xdg.config()?;
//!     debug_assert_eq!(Path::new("/home/user/.config"), &config_dir);
//!
//!     /* Do something with `config_dir`... */
//!
//!     Ok(())
//! }
//! ```
//!
//! One may also want to retrieve the _user-specific_ XDG **state** subdirectories relative to a
//! certain application:
//! ```rust
//! # use std::{env, path::Path};
//! use microxdg::{XdgApp, XdgError};
//!
//! fn main() -> Result<(), XdgError> {
//!     # env::set_var("XDG_STATE_HOME", "/home/user/.local/state");
//!     let xdg = XdgApp::new("app_name")?;
//!     let app_state_dir = xdg.app_state()?;
//!     debug_assert_eq!(Path::new("/home/user/.local/state/app_name"), &app_state_dir);
//!
//!     /* Do something with `app_state_dir`... */
//!
//!     Ok(())
//! }

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]

mod app;
mod error;

use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

pub use app::XdgApp;
pub use error::XdgError;

trait Append {
    fn append<P>(self, path: P) -> Self
    where
        P: AsRef<Path>;
}

impl Append for PathBuf {
    #[inline]
    fn append<P>(mut self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        self.push(path);
        self
    }
}

/// XDG Base Directory Specification's directories.
#[derive(Debug, Clone, Copy)]
enum XdgDir {
    Cache,
    Config,
    Data,
    State,
}

impl XdgDir {
    /// Returns the XDG environment variable associated to the XDG base
    /// directory.
    #[inline]
    fn env_var(self) -> &'static str {
        match self {
            XdgDir::Cache => "XDG_CACHE_HOME",
            XdgDir::Config => "XDG_CONFIG_HOME",
            XdgDir::Data => "XDG_DATA_HOME",
            XdgDir::State => "XDG_STATE_HOME",
        }
    }

    /// Returns the _user-specific_ fallback directory in the case the XDG
    /// environment variable is not set.
    #[inline]
    fn fallback(self) -> &'static str {
        match self {
            XdgDir::Cache => ".cache",
            XdgDir::Config => ".config",
            XdgDir::Data => ".local/share",
            XdgDir::State => ".local/state",
        }
    }

    /// Returns the associated variant of [`XdgSysDirs`].
    #[inline]
    fn to_sys(self) -> Option<XdgSysDirs> {
        match self {
            XdgDir::Cache | XdgDir::State => None,
            XdgDir::Config => Some(XdgSysDirs::Config),
            XdgDir::Data => Some(XdgSysDirs::Data),
        }
    }
}

/// XDG Base Directory Specification's _system-wide_ directories.
#[derive(Debug, Clone, Copy)]
enum XdgSysDirs {
    Config,
    Data,
}

impl XdgSysDirs {
    /// Returns the XDG environment variable associated to the XDG base directories.
    fn env_var(self) -> &'static str {
        match self {
            XdgSysDirs::Config => "XDG_CONFIG_DIRS",
            XdgSysDirs::Data => "XDG_DATA_DIRS",
        }
    }

    /// Returns the fallback directories in the case the XDG environment variable is not set.
    #[inline]
    fn fallback(self) -> impl Iterator<Item = PathBuf> {
        match self {
            XdgSysDirs::Config => ["/etc/xdg"].as_slice(),
            XdgSysDirs::Data => ["/usr/local/share", "/usr/share"].as_slice(),
        }
        .iter()
        .map(PathBuf::from)
    }
}

/// _An implementation of the [XDG Base Directory Specification](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)_.
///
/// Each of the base directory methods privileges the relative environment variable's value and
/// falls back to the corresponding default whenever the environment variable is not set or set to
/// an empty value.
///
/// User-specific Base Directories:
///
/// | XDG Base Directory                    | Environment variable | Fallback - `HOME` set  | Fallback - `HOME` not set  |
/// | ------------------------------------- | -------------------- | ---------------------- | -------------------------- |
/// | [_Cache_](method@Xdg::cache)          | `XDG_CACHE_HOME`     | `$HOME/.cache`         | `/home/$USER/.cache`       |
/// | [_Configuration_](method@Xdg::config) | `XDG_CONFIG_HOME`    | `$HOME/.config`        | `/home/$USER/.config`      |
/// | [_Data_](method@Xdg::data)            | `XDG_DATA_HOME`      | `$HOME/.local/share`   | `/home/$USER/.local/share` |
/// | [_State_](method@Xdg::state)          | `XDG_STATE_HOME`     | `$HOME/.local/state`   | `/home/$USER/.local/state` |
/// | [_Runtime_](method@Xdg::runtime)      | `XDG_RUNTIME_DIR`    | -                      | -                          |
/// | [_Executable_](method@Xdg::exec)      | -                    | `$HOME/.local/bin`     | `/home/$USER/.local/bin`   |
///
/// System-wide, preference-ordered, Base Directories:
///
/// | XDG Base Directory                        | Environment variable | Fallback                      |
/// | ----------------------------------------- | -------------------- | ----------------------------- |
/// | [_Configuration_](method@Xdg::sys_config) | `XDG_CONFIG_DIRS`    | `/etc/xdg`                    |
/// | [_Data_](method@Xdg::sys_data)            | `XDG_DATA_DIRS`      | `/usr/local/share:/usr/share` |
///
/// # Examples
///
/// The example below retrieves the _user-specific XDG configuration directory_ by reading the
/// value of the `XDG_CONFIG_HOME` environment variable (similarly the other XDG directories):
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
/// In the case the `XDG_CONFIG_DIR` environment variable is not set, `$HOME/.config` is used as a
/// fallback (similarly the other XDG base directories):
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
/// Ultimately, if also the `HOME` environment variable is not set (very unlikely),
/// `/home/$USER/.config` is used as a fallback (similarly the other XDG base directories):
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
#[derive(Debug, Clone)]
pub struct Xdg {
    /// Home directory of the user owning the process.
    home: PathBuf,
}

impl Xdg {
    /// Constructs a new [`Xdg`] instance from the `HOME` environment varaible's value.
    #[inline]
    #[must_use]
    fn from_string(home: String) -> Xdg {
        Xdg {
            home: PathBuf::from(home),
        }
    }

    /// Constructs a new [`Xdg`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment variable is set.
    pub fn new() -> Result<Xdg, XdgError> {
        if let Ok(home) = env::var("HOME") {
            return Ok(Xdg::from_string(home));
        }

        if let Ok(user) = env::var("USER") {
            return Ok(Xdg::from_string(format!("/home/{user}")));
        }

        Err(XdgError::HomeNotFound)
    }

    /// Constructs a new [`XdgApp`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment variable is set.
    pub fn new_app(app_name: &'static str) -> Result<XdgApp, XdgError> {
        XdgApp::new(app_name)
    }

    /// Returns the **home** directory of the user owning the process.
    #[inline]
    #[must_use]
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// Returns a validated path from an XDG environment variable.
    ///
    /// # Errors
    ///
    /// This function returns an error if the XDG environment variable is set, but its value
    /// represents a relative path: XDG environment variables must be set to absolute paths.
    #[inline]
    fn validate_path<P>(env_var_key: &'static str, env_var_val: P) -> Result<PathBuf, XdgError>
    where
        P: Into<PathBuf>,
    {
        let path: PathBuf = env_var_val.into();
        if path.is_relative() {
            // XDG environment variable set, but its value represents a relative path.
            return Err(XdgError::RelativePath { env_var_key, path });
        }

        Ok(path)
    }

    /// Returns the value of an XDG environment variable.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the XDG environment variable is set;
    /// - `None` if the XDG environment variable is missing or set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_env_var(env_var_key: &'static str) -> Result<Option<String>, XdgError> {
        match env::var(env_var_key) {
            // XDG environment variable is set to a non-empty value.
            Ok(env_var_val) if !env_var_val.is_empty() => Ok(Some(env_var_val)),
            // XDG environment variable is set, but its value represents invalid unicode.
            Err(VarError::NotUnicode(env_var_val)) => Err(XdgError::InvalidUnicode {
                env_var_key,
                env_var_val,
            }),
            // XDG environment variable is not set or set to an empty value.
            _ => Ok(None),
        }
    }

    /// Returns the path set to an XDG environment variable or a fallback in the case the
    /// environment variable is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_dir_path(&self, dir: XdgDir) -> Result<PathBuf, XdgError> {
        let env_var_key = dir.env_var();
        match Xdg::get_env_var(env_var_key)? {
            Some(env_var_val) => Xdg::validate_path(env_var_key, env_var_val),
            None => Ok(self.home.join(dir.fallback())),
        }
    }

    /// Returns the _user-specific_ XDG **cache** directory specified by the `XDG_CACHE_HOME`
    /// environment variable. Falls back to `$HOME/.cache` if `XDG_CACHE_HOME` is not set or is
    /// set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment is set, but its value represents invalid unicode.
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
        self.get_dir_path(XdgDir::Cache)
    }

    /// Returns the _user-specific_ XDG **configuration** directory specified by the
    /// `XDG_CONFIG_HOME` environment variable. Falls back to `$HOME/.config` if `XDG_CONFIG_HOME`
    /// is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_HOME` environment is set, but its value represents invalid unicode.
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
        self.get_dir_path(XdgDir::Config)
    }

    /// Returns the _user-specific_ XDG **data** directory specified by the `XDG_DATA_HOME`
    /// environment variable. Falls back to `$HOME/.local/share` if `XDG_DATA_HOME` is not set or
    /// is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
        self.get_dir_path(XdgDir::Data)
    }

    /// Returns the _user-specific_ XDG **state** directory specified by the `XDG_STATE_HOME`
    /// environment variable. Falls back to `$HOME/.local/state` if `XDG_STATE_HOME` is not set or
    /// is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_STATE_HOME` environment is set, but its value represents invalid unicode.
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
        self.get_dir_path(XdgDir::State)
    }

    /// Returns the XDG **runtime** directory specified by the `XDG_RUNTIME_DIR` environment
    /// variable.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the `XDG_RUNTIME_DIR` environment variable is set;
    /// - `None` if the `XDG_RUNTIME_DIR` environment variable is not set or is set to an empty
    ///   value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_RUNTIME_DIR` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_RUNTIME_DIR` environment is set, but its value represents invalid unicode.
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
        Xdg::get_env_var(XDG_RUNTIME_DIR)?
            .map(|env_var_val| Xdg::validate_path(XDG_RUNTIME_DIR, env_var_val))
            .transpose()
    }

    /// Returns the _user-specific_ XDG **executable** directory specified by `$HOME/.local/bin`.
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
    #[must_use]
    pub fn exec(&self) -> PathBuf {
        self.home.join(".local/bin")
    }

    /// Returns an iterator over the _sistem-wide_ directories set to a system XDG environment
    /// variable.
    #[inline]
    fn iter_sys_dir_paths<'val>(
        env_var_key: &'static str,
        env_var_val: &'val str,
    ) -> impl Iterator<Item = Result<PathBuf, XdgError>> + 'val {
        env_var_val
            .split(':')
            .map(move |path| Xdg::validate_path(env_var_key, path))
    }

    /// Returns the _system-wide_, preference-ordered, XDG directories or a fallback if the
    /// environment variable is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_sys_dir_paths(dirs: XdgSysDirs) -> Result<Vec<PathBuf>, XdgError> {
        let env_var_key = dirs.env_var();
        match Xdg::get_env_var(env_var_key)? {
            Some(env_var_val) => Xdg::iter_sys_dir_paths(env_var_key, &env_var_val).collect(),
            None => Ok(dirs.fallback().collect()),
        }
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration** directories specified
    /// by the `XDG_CONFIG_DIRS` environment variable. Falls back to `/etc/xdg` if
    /// `XDG_CONFIG_DIRS` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// Used to search for config files in addition to the `XDG_CONFIG_HOME` user-specific base
    /// directory.
    ///
    /// The order denotes the importance: the first directory the most important, the last
    /// directory the least important.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_DIRS` environment variable is set, but one (or more) path(s) in the
    ///   colon separated value represents a relative path;
    /// - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let sys_config_dirs = Xdg::sys_config()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn sys_config() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::get_sys_dir_paths(XdgSysDirs::Config)
    }

    /// Returns the system-wide, preference-ordered, XDG **data** directories specified by the
    /// `XDG_DATA_DIRS` environment variable. Falls back to `/usr/local/share:/usr/share` if
    /// `XDG_DATA_DIRS` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// Used to search for data files in addition to the `XDG_DATA_HOME` user-specific base
    /// directory.
    ///
    /// The order denotes the importance: the first directory the most important, the last
    /// directory the least important.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_DIRS` environment variable is set, but one (or more) path(s) in the colon
    ///   separated value represents a relative path;
    /// - the `XDG_DATA_DIRS` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let sys_data_dirs = Xdg::sys_data()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn sys_data() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::get_sys_dir_paths(XdgSysDirs::Data)
    }

    /// Returns the _user-specific_ XDG file path as `<xdg_dir>/<file>`.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_file_path<P>(&self, dir: XdgDir, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_dir_path(dir).map(|path| path.append(file))
    }

    /// Returns the _user-specific_ XDG **cache** file as `$XDG_CACHE_HOME/<file>`. Falls back to
    /// `$HOME/.cache/<file>` if `XDG_CACHE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a regular file.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment is set, but its value represents invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let cache_file = xdg.cache_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn cache_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_file_path(XdgDir::Cache, file)
    }

    /// Returns the _user-specific_ XDG **config** file as `$XDG_CONFIG_HOME/<file>`. Falls back
    /// to `$HOME/.config/<file>` if `XDG_CONFIG_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a regular file.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_HOME` environment is set, but its value represents invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let config_file = xdg.config_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn config_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_file_path(XdgDir::Config, file)
    }

    /// Returns the _user-specific_ XDG **data** file as `$XDG_DATA_HOME/<file>`. Falls back to
    /// `$HOME/.local/share/<file>` if `XDG_DATA_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a regular file.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_DATA_HOME` environment is set, but its value represents invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let data_file = xdg.data_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn data_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_file_path(XdgDir::Data, file)
    }

    /// Returns the _user-specific_ XDG **state** file as `$XDG_STATE_HOME/<file>`. Falls back to
    /// `$HOME/.local/state/<file>` if `XDG_STATE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a regular file.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_STATE_HOME` environment is set, but its value represents invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let state_file = xdg.state_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn state_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_file_path(XdgDir::State, file)
    }

    /// Searches for `file` inside a _user-specific_ XDG base directory.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside the specified XDG directory;
    /// - `None` if the file is **not** found inside the specified XDG directory.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn search_usr_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_dir_path(dir).map(|mut path| {
            path.push(file);
            path.is_file().then_some(path)
        })
    }

    /// Searches for `file` inside a _system-wide_, preference-ordered, set of XDG directories.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside one of the preference-ordered set of XDG system
    ///   directories;
    /// - `None` if the file is **not** found inside any of the preference-ordered set of XDG
    ///   system directories.
    ///
    /// # Errors
    ///
    /// This funciton returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn search_sys_file<P>(dirs: XdgSysDirs, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        let env_var_key = dirs.env_var();
        match Xdg::get_env_var(env_var_key)? {
            Some(env_var_val) => Xdg::iter_sys_dir_paths(env_var_key, &env_var_val)
                .map(|result| result.map(|path| path.append(&file)))
                .find(|path| path.as_ref().is_ok_and(|path| path.is_file()))
                .transpose(),
            None => Ok(dirs
                .fallback()
                .map(|path| path.append(&file))
                .find(|path| path.is_file())),
        }
    }

    /// Searches for `file` inside XDG directories in the following order:
    /// - _user-specific_ XDG base directory;
    /// - _system-wide_, preference-ordered, set of XDG directories.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside one of the XDG directories;
    /// - `None` if the file is **not** found inside one of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set, but its value
    ///   represents a relative path;
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set, but its value
    ///   contains invalid unicode.
    #[inline]
    fn search_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        if let Some(path) = self.search_usr_file(dir, &file)? {
            return Ok(Some(path));
        }

        if let Some(sys_dirs) = dir.to_sys() {
            if let Some(path) = Xdg::search_sys_file(sys_dirs, &file)? {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    /// Searches for `file` inside the _user-specific_ XDG **cache** directory specified by the
    /// `XDG_CACHE_HOME` environment variable. The search falls back to `$HOME/.cache` if
    /// `XDG_CACHE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside any of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// match xdg.search_cache_file("file")? {
    ///     Some(cache_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn search_cache_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_file(XdgDir::Cache, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **configuration** directory specified
    /// by the `XDG_CONFIG_HOME` environment variable. If `XDG_CONFIG_HOME` is not set or is set
    /// to an empty value, the search falls back to `$HOME/.config`.
    ///
    /// If `file` is not found inside the _user-specific_ XDG directory, a lookup is performed on
    /// the _system-wide_, preference ordered directories specified by the `XDG_CONFIG_DIRS`.
    /// If `XDG_CONFIG_DIRS` is not set or is set to an empty value, the search falls back to
    /// `/etc/xdg`.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside any of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents invalid
    ///   unicode;
    /// - `file` was **not** found inside the _user-specific_ XDG config directory and:
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but one (or more) path(s) in the
    ///       colon separated value represents a relative path;
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents invalid
    ///       unicode.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// match xdg.search_config_file("file")? {
    ///     Some(config_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn search_config_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_file(XdgDir::Config, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **data** directory specified by the
    /// `XDG_DATA_HOME` environment variable. If `XDG_DATA_HOME` is not set or is set to an empty
    /// value, the search falls back to `$HOME/.local/share`.
    ///
    /// If `file` is not found inside the _user-specific_ XDG directory, a lookup is performed on
    /// the _system-wide_, preference ordered directories specified by the `XDG_DATA_DIRS`.
    /// If `XDG_DATA_DIRS` is not set or is set to an empty value, the search falls back to
    /// `/usr/local/share:/usr/share`.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside any of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents invalid
    ///   unicode;
    /// - `file` was **not** found inside the _user-specific_ XDG data directory and:
    ///     - the `XDG_DATA_DIRS` environment variable is set, but one (or more) path(s) in the
    ///       colon separated value represents a relative path;
    ///     - the `XDG_DATA_DIRS` environment variable is set, but its value represents invalid
    ///       unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// match xdg.search_data_file("file")? {
    ///     Some(data_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn search_data_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_file(XdgDir::Data, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **state** directory specified by the
    /// `XDG_STATE_HOME` environment variable. The search falls back to `$HOME/.local/state` if
    /// `XDG_STATE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside any of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// match xdg.search_state_file("file")? {
    ///     Some(state_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn search_state_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_file(XdgDir::State, file)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, ffi::OsStr, os::unix::prelude::OsStrExt};

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

        env::remove_var("HOME");
        assert_eq!(Path::new("/home/user2"), Xdg::new()?.home());
        assert_eq!(Path::new("/home/user2"), Xdg::new_app("app_name")?.home());

        env::remove_var("USER");
        assert_eq!(XdgError::HomeNotFound, Xdg::new().unwrap_err());
        assert_eq!(
            XdgError::HomeNotFound,
            Xdg::new_app("app_name").unwrap_err()
        );

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
            Err(XdgError::RelativePath {
                env_var_key: "XDG_CACHE_HOME",
                path: PathBuf::from("./cache"),
            }),
            xdg.cache(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_CONFIG_HOME",
                path: PathBuf::from("./config"),
            }),
            xdg.config(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_DATA_HOME",
                path: PathBuf::from("./data"),
            }),
            xdg.data(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_STATE_HOME",
                path: PathBuf::from("./state"),
            }),
            xdg.state(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_RUNTIME_DIR",
                path: PathBuf::from("./runtime"),
            }),
            xdg.runtime(),
        );

        let invalid_unicode = OsStr::from_bytes(&INVALID_UNICODE_BYTES);
        env::set_var("XDG_CACHE_HOME", invalid_unicode);
        env::set_var("XDG_CONFIG_HOME", invalid_unicode);
        env::set_var("XDG_DATA_HOME", invalid_unicode);
        env::set_var("XDG_STATE_HOME", invalid_unicode);
        env::set_var("XDG_RUNTIME_DIR", invalid_unicode);
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_CACHE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.cache(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_CONFIG_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.config(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_DATA_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.data(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_STATE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.state(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_RUNTIME_DIR",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.runtime(),
        );

        Ok(())
    }

    #[test]
    fn sys_base_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CONFIG_DIRS");
        env::remove_var("XDG_DATA_DIRS");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        assert_eq!(vec![PathBuf::from("/etc/xdg")], Xdg::sys_config()?);
        assert_eq!(
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share"),
            ],
            Xdg::sys_data()?,
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
            Xdg::sys_config()?,
        );
        assert_eq!(
            vec![
                PathBuf::from("/data/dir1"),
                PathBuf::from("/data/dir2"),
                PathBuf::from("/data/dir3"),
                PathBuf::from("/data/dir4"),
            ],
            Xdg::sys_data()?,
        );

        Ok(())
    }

    #[test]
    fn usr_file() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = Xdg::new()?;
        assert_eq!(Path::new("/home/user/.cache/file"), xdg.cache_file("file")?);
        assert_eq!(
            Path::new("/home/user/.config/file"),
            xdg.config_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user/.local/share/file"),
            xdg.data_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user/.local/state/file"),
            xdg.state_file("file")?,
        );

        env::set_var("XDG_CACHE_HOME", "/home/user1/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user1/.config");
        env::set_var("XDG_DATA_HOME", "/home/user1/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user1/.local/state");

        assert_eq!(
            Path::new("/home/user1/.cache/file"),
            xdg.cache_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.config/file"),
            xdg.config_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/share/file"),
            xdg.data_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/file"),
            xdg.state_file("file")?,
        );

        Ok(())
    }

    #[test]
    fn search_file() -> Result<(), Box<dyn Error>> {
        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = Xdg::new()?;

        assert_eq!(None, xdg.search_cache_file("microxdg")?);
        assert_eq!(None, xdg.search_config_file("microxdg")?);
        assert_eq!(None, xdg.search_data_file("microxdg")?);
        assert_eq!(None, xdg.search_state_file("microxdg")?);

        let mut tmp_dir_builder = tempfile::Builder::new();
        tmp_dir_builder.prefix("microxdg");
        tmp_dir_builder.rand_bytes(4);

        let cache_home = tmp_dir_builder.tempdir()?;
        let config_home = tmp_dir_builder.tempdir()?;
        let data_home = tmp_dir_builder.tempdir()?;
        let state_home = tmp_dir_builder.tempdir()?;

        env::set_var("XDG_CACHE_HOME", cache_home.path());
        env::set_var("XDG_CONFIG_HOME", config_home.path());
        env::set_var("XDG_DATA_HOME", data_home.path());
        env::set_var("XDG_STATE_HOME", state_home.path());

        let mut tmp_file_builder = tempfile::Builder::new();
        tmp_file_builder.prefix("microxdg");
        tmp_file_builder.rand_bytes(0);

        let cache_file = tmp_file_builder.tempfile_in(cache_home.path())?;
        let config_file = tmp_file_builder.tempfile_in(config_home.path())?;
        let data_file = tmp_file_builder.tempfile_in(data_home.path())?;
        let state_file = tmp_file_builder.tempfile_in(state_home.path())?;

        assert_eq!(
            Some(cache_file.path().into()),
            xdg.search_cache_file("microxdg")?,
        );
        assert_eq!(
            Some(config_file.path().into()),
            xdg.search_config_file("microxdg")?,
        );
        assert_eq!(
            Some(data_file.path().into()),
            xdg.search_data_file("microxdg")?,
        );
        assert_eq!(
            Some(state_file.path().into()),
            xdg.search_state_file("microxdg")?,
        );

        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        let data_dirs = tmp_dir_builder.tempdir()?;
        let config_dirs = tmp_dir_builder.tempdir()?;

        env::set_var("XDG_DATA_DIRS", data_dirs.path());
        env::set_var("XDG_CONFIG_DIRS", config_dirs.path());

        let data_file = tmp_file_builder.tempfile_in(data_dirs.path())?;
        let config_file = tmp_file_builder.tempfile_in(config_dirs.path())?;

        assert_eq!(
            Some(data_file.path().into()),
            xdg.search_data_file("microxdg")?
        );
        assert_eq!(
            Some(config_file.path().into()),
            xdg.search_config_file("microxdg")?
        );

        Ok(())
    }
}
