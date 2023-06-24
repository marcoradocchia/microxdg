use super::{Xdg, XdgDir, XdgError, XdgSysDirs};
use std::path::{Path, PathBuf};

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
#[derive(Debug, Clone)]
pub struct XdgApp {
    /// [`Xdg`] instance.
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
    #[must_use]
    pub fn from_xdg(xdg: Xdg, app_name: &'static str) -> XdgApp {
        XdgApp {
            xdg,
            name: app_name,
        }
    }

    /// Returns the **home** directory of the user owning the process.
    #[inline]
    #[must_use]
    pub fn home(&self) -> &Path {
        self.xdg.home()
    }

    /// Returns the _user-specific_ XDG **cache** directory specified by the
    /// `XDG_CACHE_HOME` environment variable.
    /// Falls back to `$HOME/.cache` if `XDG_CACHE_HOME` is not set or is set
    // to an empty value.
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

    /// Returns the _user-specific_ XDG **configuration** directory specified
    /// by the `XDG_CONFIG_HOME` environment variable.
    /// Falls back to `$HOME/.config` if `XDG_CONFIG_HOME` is not set or is set
    /// to an empty value.
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

    /// Returns the _user-specific_ XDG **data** directory specified by the
    /// `XDG_DATA_HOME` environment variable.
    /// Falls back to `$HOME/.local/share` if `XDG_DATA_HOME` is not set or is
    /// set to an empty value.
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

    /// Returns the _user-specific_ XDG **state** directory specified by the
    /// `XDG_STATE_HOME` environment variable.
    /// Falls back to `$HOME/.local/state` if `XDG_STATE_HOME` is not set or is
    /// set to an empty value.
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
    #[must_use]
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
    /// The order denotes the importance: the first directory is the most
    /// important, the last directory is the least important.
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
    pub fn sys_config() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::sys_config()
    }

    /// Returns the system-wide, preference-ordered, XDG **data** directories
    /// specified by the `XDG_DATA_DIRS` environment variable.
    /// Falls back to `/usr/local/share:/usr/share` if `XDG_DATA_DIRS` is not
    /// set or is set to an empty value.
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
    pub fn sys_data() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::sys_data()
    }

    /// Returns the path to the application subdirectory of an XDG base `dir`.
    #[inline]
    fn get_app_dir_path(&self, dir: XdgDir) -> Result<PathBuf, XdgError> {
        let mut dir = self.xdg.get_dir_path(dir)?;
        dir.push(self.name);
        Ok(dir)
    }

    /// Returns the _user-specific_ XDG **cache** directory for the current
    /// application.
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
        self.get_app_dir_path(XdgDir::Cache)
    }

    /// Returns the _user-specific_ XDG **configuration** directory for the
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
        self.get_app_dir_path(XdgDir::Config)
    }

    /// Returns the _user-specific_ XDG **data** directory for the current
    /// application.
    ///
    /// # Note
    ///
    /// This method uses the XDG data directory specified by the
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
        self.get_app_dir_path(XdgDir::Data)
    }

    /// Returns the _user-specific_ XDG **state** directory for the current
    /// application.
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
        self.get_app_dir_path(XdgDir::State)
    }

    /// Returns the _system-wide_, preference-ordered, paths set to a system XDG
    /// environment variable or a fallback in the case the environment variable
    /// is not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn get_app_sys_dir_paths(&self, dirs: XdgSysDirs) -> Result<Vec<PathBuf>, XdgError> {
        // TODO: may be improved appending `self.name` to each path while
        // collecting the vector in `Xdg::get_sys_dir_paths`.
        let mut sys_dir_paths = Xdg::get_sys_dir_paths(dirs)?;
        for path in &mut sys_dir_paths {
            path.push(self.name);
        }

        Ok(sys_dir_paths)
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration**
    /// directories for the current application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered configuration directories
    /// specified by the `XDG_CONFIG_DIRS` environment variable.
    /// Falls back to `/etc/xdg/<app_name>` if `XDG_CONFIG_DIRS` is not set or
    /// is set to an empty value.
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
    /// let mut app_config_dirs = xdg.app_sys_config()?;
    /// app_config_dirs.push(xdg.app_config()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn app_sys_config(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_app_sys_dir_paths(XdgSysDirs::Config)
    }

    /// Returns the _system-wide_, preference-ordered, XDG **data** directories
    /// for the current application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered data directories specified by
    /// the `XDG_DATA_DIRS` environment variable.
    /// Falls back to `/usr/local/share:/usr/share` if `XDG_DATA_DIRS` is not
    /// set or is set to an empty value.
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
    /// let mut data_dirs = xdg.app_sys_data()?;
    /// data_dirs.push(xdg.app_data()?);
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn app_sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_app_sys_dir_paths(XdgSysDirs::Data)
    }

    // TODO
    #[inline]
    fn get_app_file_path<P>(&self, dir: XdgDir, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        todo!()
        // let mut dir = self.get_app
    }
}
