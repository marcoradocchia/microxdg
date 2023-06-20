#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
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

mod error;

use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

pub use error::XdgError;

/// XDG Base Directory Specification's _user-specific_ directories.
enum XdgUsrDir {
    Cache,
    Config,
    Data,
    State,
    // Runtime,
    // Executable,
}

impl XdgUsrDir {
    /// Returns the XDG environment variable associated to the XDG base
    /// directory.
    fn env_var(&self) -> &'static str {
        match self {
            XdgUsrDir::Cache => "XDG_CACHE_HOME",
            XdgUsrDir::Config => "XDG_CONFIG_HOME",
            XdgUsrDir::Data => "XDG_DATA_HOME",
            XdgUsrDir::State => "XDG_STATE_HOME",
            // XdgBaseDir::Runtime => "XDG_RUNTIME_DIR",
            // XdgBaseDir::Executable => ???,
        }
    }

    /// Returns the fallback directory in the case the XDG environment variable
    /// is not set.
    fn fallback(&self) -> &'static str {
        match self {
            XdgUsrDir::Cache => ".cache",
            XdgUsrDir::Config => ".config",
            XdgUsrDir::Data => ".local/share",
            XdgUsrDir::State => ".local/state",
            // XdgBaseDir::Runtime => ???,
            // XdgBaseDir::Executable => ".local/bin",
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

/// _An implementation of the XDG Base Directory Specification_.
///
/// # Examples
///
/// The example below retrieves the _user-specific XDG configuration directory_
/// by reading the value of the `XDG_CONFIG_HOME` environment variable:
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
        Xdg { home: home.into() }
    }

    /// Constructs a new [`Xdg`] instance.
    ///
    /// # <span id="xdg-errors">Errors</span>
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
    /// See [`Xdg::new`] function's [Error](#xdg-errors) section.
    pub fn new_app(app_name: &'static str) -> Result<XdgApp, XdgError> {
        Ok(XdgApp {
            xdg: Xdg::new()?,
            name: app_name,
        })
    }

    /// Returns the **home** directory of the user owning the process.
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// Returns a validated path from an XDG environment variable.
    ///
    /// # Errors
    ///
    /// This function returns an error if the value represents a relative path.
    /// XDG environment variables must be set to absolute paths.
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
    /// - the environment variable is set to a path that is not absolute;
    /// - the environment variable is set to invalid unicode.
    #[inline]
    fn get_path(&self, dir: XdgUsrDir) -> Result<PathBuf, XdgError> {
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
    /// # <span id="cache-errors">Errors</span>
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable contains invalid unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let cache = xdg.cache()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgUsrDir::Cache)
    }

    /// Returns the _user-specific_ XDG **configuration** directory specified by
    /// the `XDG_CONFIG_HOME` environment variable. Falls back to
    /// `$HOME/.config` if `XDG_CONFIG_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let config = xdg.config()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgUsrDir::Config)
    }

    /// Returns the _user-specific_ XDG **data** directory specified by
    /// the `XDG_DATA_HOME` environment variable. Falls back to
    /// `$HOME/.local/share` if `XDG_DATA_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let data = xdg.data()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgUsrDir::Data)
    }

    /// Returns the _user-specific_ XDG **state** directory specified by
    /// the `XDG_STATE_HOME` environment variable. Falls back to
    /// `$HOME/.local/state` if `XDG_STATE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{Xdg, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = Xdg::new()?;
    /// let state = xdg.state()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgUsrDir::State)
    }

    /// Returns the preference-ordered _system-widw_ paths set to a system XDG
    /// environment variable or a fallback in the case the environment variable
    /// is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the environment variable is set to a path that is not absolute;
    /// - the environment variable is set to invalid unicode.
    #[inline]
    fn get_sys_paths(&self, dirs: XdgSysDirs) -> Result<Vec<PathBuf>, XdgError> {
        let env_var_key = dirs.env_var();
        match env::var(env_var_key) {
            // XDG environment variable is set and non-empty.
            Ok(env_var_val) if !env_var_val.is_empty() => env_var_val
                .split(':')
                .map(|path_str| Xdg::validate_path(env_var_key, path_str))
                .collect(),
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
    #[inline]
    pub fn sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_sys_paths(XdgSysDirs::Data)
    }
}

/// TODO: XDG Base Directory Specification for applications.
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
    /// See [`Xdg::new`] function's [Error](#xdg-errors) section.
    pub fn new(app_name: &'static str) -> Result<XdgApp, XdgError> {
        Ok(XdgApp {
            xdg: Xdg::new()?,
            name: app_name,
        })
    }

    /// TODO: DOCUMENT THIS
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.xdg.cache()
    }

    /// TODO: DOCUMENT THIS
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.xdg.config()
    }

    /// TODO: DOCUMENT THIS
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.xdg.data()
    }

    /// TODO: DOCUMENT THIS
    #[inline]
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.xdg.state()
    }

    /// Returns the user-specific XDG **cache** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG cache directory specified by the
    /// `XDG_CACHE_HOME` if available. Falls back to `$HOME/.cache/<name>`
    /// if `XDG_CACHE_HOME` is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn app_cache(&self) -> Result<PathBuf, XdgError> {
        let mut cache = self.xdg.cache()?;
        cache.push(self.name);

        Ok(cache)
    }

    /// Returns the user-specific XDG **configuration** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG configuration directory specified by the
    /// `XDG_CONFIG_HOME` if available. Falls back to `$HOME/.config/<name>`
    /// if `XDG_CONFIG_HOME` is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn app_config(&self) -> Result<PathBuf, XdgError> {
        let mut config = self.xdg.config()?;
        config.push(self.name);

        Ok(config)
    }

    /// Returns the user-specific XDG **data** directory for the
    /// current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG state directory specified by the
    /// `XDG_DATA_HOME` if available. Falls back to `$HOME/.local/share/<name>`
    /// if `XDG_DATA_HOME` is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn app_data(&self) -> Result<PathBuf, XdgError> {
        let mut data = self.xdg.data()?;
        data.push(self.name);

        Ok(data)
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
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's [Errors](#cache-errors) section.
    #[inline]
    pub fn app_state(&self) -> Result<PathBuf, XdgError> {
        let mut state = self.xdg.state()?;
        state.push(self.name);

        Ok(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, path::Path};

    #[test]
    fn new_xdg() {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user2");
        assert_eq!(Path::new("/home/user1"), Xdg::new().unwrap().home);

        env::remove_var("HOME");
        assert_eq!(Path::new("/home/user2"), Xdg::new().unwrap().home);

        env::remove_var("USER");
        assert_eq!(XdgError::HomeNotFound, Xdg::new().unwrap_err());
    }

    #[test]
    fn base_dirs_usr() {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user1");

        let xdg = Xdg::new().unwrap();
        assert_eq!(Path::new("/home/user1"), xdg.home());
        assert_eq!(Path::new("/home/user1/.cache"), xdg.cache().unwrap());
        assert_eq!(Path::new("/home/user1/.config"), xdg.config().unwrap());
        assert_eq!(Path::new("/home/user1/.local/share"), xdg.data().unwrap());
        assert_eq!(Path::new("/home/user1/.local/state"), xdg.state().unwrap());

        env::set_var("XDG_CACHE_HOME", "/home/user2/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user2/.config");
        env::set_var("XDG_DATA_HOME", "/home/user2/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user2/.local/state");
        assert_eq!(Path::new("/home/user2/.cache"), xdg.cache().unwrap());
        assert_eq!(Path::new("/home/user2/.config"), xdg.config().unwrap());
        assert_eq!(Path::new("/home/user2/.local/share"), xdg.data().unwrap());
        assert_eq!(Path::new("/home/user2/.local/state"), xdg.state().unwrap());

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");
        assert_eq!(Path::new("/home/user1/.cache"), xdg.cache().unwrap());
        assert_eq!(Path::new("/home/user1/.config"), xdg.config().unwrap());
        assert_eq!(Path::new("/home/user1/.local/share"), xdg.data().unwrap());
        assert_eq!(Path::new("/home/user1/.local/state"), xdg.state().unwrap());
    }

    #[test]
    fn app_dirs_usr() {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user1");

        let xdg = Xdg::new_app("my_app").unwrap();
        assert_eq!(
            Path::new("/home/user1/.cache/my_app"),
            xdg.app_cache().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.config/my_app"),
            xdg.app_config().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.local/share/my_app"),
            xdg.app_data().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/my_app"),
            xdg.app_state().unwrap()
        );

        env::set_var("XDG_CACHE_HOME", "/home/user2/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user2/.config");
        env::set_var("XDG_DATA_HOME", "/home/user2/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user2/.local/state");
        let xdg = Xdg::new_app("my_app").unwrap();
        assert_eq!(
            Path::new("/home/user2/.cache/my_app"),
            xdg.app_cache().unwrap()
        );
        assert_eq!(
            Path::new("/home/user2/.config/my_app"),
            xdg.app_config().unwrap()
        );
        assert_eq!(
            Path::new("/home/user2/.local/share/my_app"),
            xdg.app_data().unwrap()
        );
        assert_eq!(
            Path::new("/home/user2/.local/state/my_app"),
            xdg.app_state().unwrap()
        );

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");
        let xdg = Xdg::new_app("my_app").unwrap();
        assert_eq!(
            Path::new("/home/user1/.cache/my_app"),
            xdg.app_cache().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.config/my_app"),
            xdg.app_config().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.local/share/my_app"),
            xdg.app_data().unwrap()
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/my_app"),
            xdg.app_state().unwrap()
        );
    }

    #[test]
    fn base_dirs_sys() {
        todo!()
    }

    #[test]
    fn app_dirs_sys() {
        todo!()
    }
}
