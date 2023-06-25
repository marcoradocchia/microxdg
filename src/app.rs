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
    /// let config_dirs = XdgApp::sys_config()?;
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
    /// let data_dirs = XdgApp::sys_data()?;
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn sys_data() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::sys_data()
    }

    /// Returns the path to the application subdirectory of an XDG base
    /// directory by the associated XDG environment variable or a fallback
    /// in the case the environment variable is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn get_app_dir_path(&self, dir: XdgDir) -> Result<PathBuf, XdgError> {
        let mut path = self.xdg.get_dir_path(dir)?;
        path.push(self.name);

        Ok(path)
    }

    /// Returns the _user-specific_ XDG **cache** directory for the current
    /// application.
    ///
    /// # Note
    ///
    /// This method uses the XDG cache directory specified by the
    /// `XDG_CACHE_HOME` if available.
    /// Falls back to `$HOME/.cache/<app_name>` if `XDG_CACHE_HOME` is not set
    /// or is set to an empty value.
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
    /// `XDG_CONFIG_HOME` if available.
    /// Falls back to `$HOME/.config/<app_name>` if `XDG_CONFIG_HOME` is not
    /// set or is set to an empty value.
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
    /// `XDG_DATA_HOME` if available.
    /// Falls back to `$HOME/.local/share/<app_name>` if `XDG_DATA_HOME` is not
    /// set or is set to an empty value.
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
    /// `XDG_STATE_HOME` if available.
    /// Falls back to `$HOME/.local/state/<name>` if `XDG_STATE_HOME` is not
    /// set or is set to an empty value.
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
    /// - the XDG environment variable is set to a relative path;
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
    /// let app_sys_config_dirs = xdg.app_sys_config()?;
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
    /// let app_data_sys_dirs = xdg.app_sys_data()?;
    /// # Ok(())
    /// # }
    /// ````
    #[inline]
    pub fn app_sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_app_sys_dir_paths(XdgSysDirs::Data)
    }

    /// Returns the _user-specific_ XDG file path as `<xdg_dir_path>/<app_name>/<file>`.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn get_app_file_path<P>(&self, dir: XdgDir, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        let mut path = self.xdg.get_dir_path(dir)?;
        path.push(self.name);
        path.push(file);

        Ok(path)
    }

    /// Returns the _user-specific_ XDG **cache** application file as
    /// `$XDG_CACHE_HOME/<app_name>/<file>`.
    /// Falls back to `$HOME/.cache/<app_name>/<file>` if `XDG_CACHE_HOME` is
    /// not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a
    /// regular file.
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
    /// let app_cache_file = xdg.app_cache_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_cache_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_app_file_path(XdgDir::Cache, file)
    }

    /// Returns the _user-specific_ XDG **config** application file as
    /// `$XDG_CONFIG_HOME/<app_name>/<file>`.
    /// Falls back to `$HOME/.config/<app_name>/<file>` if `XDG_CONFIG_HOME` is
    /// not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a
    /// regular file.
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
    /// let app_config_file = xdg.app_config_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_config_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_app_file_path(XdgDir::Config, file)
    }

    /// Returns the _user-specific_ XDG **data** application file as
    /// `$XDG_DATA_HOME/<app_name>/<file>`.
    /// Falls back to `$HOME/.data/<app_name>/<file>` if `XDG_DATA_HOME` is
    /// not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a
    /// regular file.
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
    /// let app_data_file = xdg.app_data_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_data_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_app_file_path(XdgDir::Data, file)
    }

    /// Returns the _user-specific_ XDG **state** application file as
    /// `$XDG_STATE_HOME/<app_name>/<file>`.
    /// Falls back to `$HOME/.state/<app_name>/<file>` if `XDG_STATE_HOME` is
    /// not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method does not guarantee either the path exists or points to a
    /// regular file.
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
    /// let app_state_file = xdg.app_state_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_state_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.get_app_file_path(XdgDir::State, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **cache** directory
    /// specified by the`XDG_CACHE_HOME` environment variable.
    /// The search falls back to `$HOME/.cache` if `XDG_CACHE_HOME` is not set
    /// or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside one of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set to a relative path;
    /// - the `XDG_CACHE_HOME` environment variable is set to invalid unicode.
    pub fn search_cache_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_file(XdgDir::Cache, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **configuration**
    /// directory specified by the`XDG_CONFIG_HOME` environment variable.
    /// If `XDG_CONFIG_HOME` is not set or is set to an empty value, the
    /// search falls back to `$HOME/.config`.
    ///
    /// If `file` is not found inside the _user-specific_ XDG directory, a
    /// lookup is performed on the _system-wide_, preference ordered
    /// directories specified by the `XDG_CONFIG_DIRS`.
    /// If `XDG_CONFIG_DIRS` is not set or is set to an empty value, the
    /// search falls back to `/etc/xdg`.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside one of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_HOME` environment variable is set to invalid unicode.
    pub fn search_config_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_file(XdgDir::Config, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **data**
    /// directory specified by the`XDG_DATA_HOME` environment variable.
    /// If `XDG_DATA_HOME` is not set or is set to an empty value, the
    /// search falls back to `$HOME/.local/share`.
    ///
    /// If `file` is not found inside the _user-specific_ XDG directory, a
    /// lookup is performed on the _system-wide_, preference ordered
    /// directories specified by the `XDG_DATA_DIRS`.
    /// If `XDG_DATA_DIRS` is not set or is set to an empty value, the
    /// search falls back to `/usr/local/share/:/usr/share/`.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside one of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set to a relative path;
    /// - the `XDG_DATA_HOME` environment variable is set to invalid unicode.
    pub fn search_data_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_file(XdgDir::Data, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **state** directory
    /// specified by the`XDG_STATE_HOME` environment variable.
    /// The search falls back to `$HOME/.local/state` if `XDG_STATE_HOME` is
    /// not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG directories;   
    /// - `None` if `file` is **not** found inside one of the XDG directories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set to a relative path;
    /// - the `XDG_STATE_HOME` environment variable is set to invalid unicode.
    pub fn search_state_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_file(XdgDir::State, file)
    }

    /* -----------------------------------------------------------------
    ---------------------  TODO: SEARCH APP FILES-----------------------
    ----------------------------------------------------------------- */

    /// Searches for `file` inside a _user-specific_ XDG app subdirectory.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside the specified XDG app subdirectory;
    /// - `None` if the file is **not** found inside the specified XDG app
    ///   directory.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn search_app_usr_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        let mut usr_path = self.xdg.get_dir_path(dir)?;
        usr_path.push(self.name);
        usr_path.push(file);

        if usr_path.is_file() {
            return Ok(Some(usr_path));
        }

        Ok(None)
    }

    /// Searches for `file` inside a _system-wide_, preference-ordered, set of
    /// XDG app subdirectories.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside one of the preference-ordered set
    ///   of XDG app system subdirectories;
    /// - `None` if the file is **not** found inside any of the
    ///   preference-ordered set of XDG app system subdirectory.
    ///
    /// # Errors
    ///
    /// This funciton returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable is set to invalid unicode.
    #[inline]
    fn search_app_sys_file<P>(&self, dirs: XdgSysDirs, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        for mut sys_path in Xdg::get_sys_dir_paths(dirs)? {
            sys_path.push(self.name);
            sys_path.push(&file);

            if sys_path.is_file() {
                return Ok(Some(sys_path));
            }
        }

        Ok(None)
    }

    /// Searches for `file` inside XDG app subdirectories in the following order:
    /// - _user-specific_ XDG app subdirectory;
    /// - _system-wide_, preference-ordered, set of XDG app subdirectories.
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
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set to
    ///   a relative path;
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set to
    ///   invalid unicode.
    #[inline]
    fn search_app_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        if let Some(file_path) = self.search_app_usr_file(dir, &file)? {
            return Ok(Some(file_path));
        }

        if let Some(sys_dirs) = dir.to_sys() {
            if let Some(file_path) = self.search_app_sys_file(sys_dirs, &file)? {
                return Ok(Some(file_path));
            }
        }

        Ok(None)
    }

    /// Searches for `file` inside the _user-specific_ XDG **cache** app
    /// subdirectory specified by `$XDG_CACHE_HOME/<app_name>`.
    /// The search falls back to `$HOME/.cache/<app_name>` if `XDG_CACHE_HOME`
    /// is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG app subdirectories;   
    /// - `None` if `file` is **not** found inside one of the XDG app
    ///   subdirectories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set to a relative path;
    /// - the `XDG_CACHE_HOME` environment variable is set to invalid unicode.
    pub fn search_app_cache_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Cache, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **config** app
    /// subdirectory specified by `$XDG_CONFIG_HOME/<app_name>`.
    /// The search falls back to `$HOME/.config/<app_name>` if `XDG_CONFIG_HOME`
    /// is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG app subdirectories;   
    /// - `None` if `file` is **not** found inside one of the XDG app
    ///   subdirectories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set to a relative path;
    /// - the `XDG_CONFIG_HOME` environment variable is set to invalid unicode.
    pub fn search_app_config_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Config, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **data** app
    /// subdirectory specified by `$XDG_DATA_HOME/<app_name>`.
    /// The search falls back to `$HOME/.data/<app_name>` if `XDG_DATA_HOME`
    /// is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG app subdirectories;   
    /// - `None` if `file` is **not** found inside one of the XDG app
    ///   subdirectories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set to a relative path;
    /// - the `XDG_DATA_HOME` environment variable is set to invalid unicode.
    pub fn search_app_data_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Data, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **state** app
    /// subdirectory specified by `$XDG_STATE_HOME/<app_name>`.
    /// The search falls back to `$HOME/.state/<app_name>` if `XDG_STATE_HOME`
    /// is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG app subdirectories;   
    /// - `None` if `file` is **not** found inside one of the XDG app
    ///   subdirectories.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set to a relative path;
    /// - the `XDG_STATE_HOME` environment variable is set to invalid unicode.
    pub fn search_app_state_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::State, file)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, ffi::OsStr, os::unix::prelude::OsStrExt, path::Path};

    const INVALID_UNICODE_BYTES: [u8; 4] = [0xF0, 0x90, 0x80, 0x67];

    #[test]
    fn new() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user2");
        assert_eq!(Path::new("/home/user1"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user1"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home()
        );

        env::remove_var("HOME");
        assert_eq!(Path::new("/home/user2"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user2"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home()
        );

        env::remove_var("USER");
        assert_eq!(XdgError::HomeNotFound, XdgApp::new("app_name").unwrap_err());

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
