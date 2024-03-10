use crate::{Append, Xdg, XdgDir, XdgError, XdgSysDirs};
use std::path::{Path, PathBuf};

/// _An implementation of the [XDG Base Directory Specification](<https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>)_
/// with extent to application-specific subdirectories.
///
/// Each of the base directories methods privileges the relative environment variable's value and
/// falls back to the corresponding default whenever the environment variable is not set or set to
/// an empty value.
///
/// User-specific Base Directories:
///
/// | XDG Base Directory                       | Environment variable | Fallback - `HOME` set  | Fallback - `HOME` not set  |
/// | ---------------------------------------- | -------------------- | ---------------------- | -------------------------- |
/// | [_Cache_](method@XdgApp::cache)          | `XDG_CACHE_HOME`     | `$HOME/.cache`         | `/home/$USER/.cache`       |
/// | [_Configuration_](method@XdgApp::config) | `XDG_CONFIG_HOME`    | `$HOME/.config`        | `/home/$USER/.config`      |
/// | [_Data_](method@XdgApp::data)            | `XDG_DATA_HOME`      | `$HOME/.local/share`   | `/home/$USER/.local/share` |
/// | [_State_](method@XdgApp::state)          | `XDG_STATE_HOME`     | `$HOME/.local/state`   | `/home/$USER/.local/state` |
/// | [_Runtime_](method@XdgApp::runtime)      | `XDG_RUNTIME_DIR`    | -                      | -                          |
/// | [_Executable_](method@XdgApp::exec)      | -                    | `$HOME/.local/bin`     | `/home/$USER/.local/bin`   |
///
/// User-specific XDG Application Subdirectories:
///
/// | XDG Application Subdirectory                     | Environment variable | Fallback - `HOME` set           | Fallback - `HOME` not set             |
/// | ------------------------------------------------ | -------------------- | ------------------------------- | ------------------------------------- |
/// | [_App Cache_](method@XdgApp::app_cache)          | `XDG_CACHE_HOME`     | `$HOME/.cache/<app_name>`       | `/home/$USER/.cache/<app_name>`       |
/// | [_App Configuration_](method@XdgApp::app_config) | `XDG_CONFIG_HOME`    | `$HOME/.config/<app_name>`      | `/home/$USER/.config/<app_name>`      |
/// | [_App Data_](method@XdgApp::app_data)            | `XDG_DATA_HOME`      | `$HOME/.local/share/<app_name>` | `/home/$USER/.local/share/<app_name>` |
/// | [_App State_](method@XdgApp::app_state)          | `XDG_STATE_HOME`     | `$HOME/.local/state/<app_name>` | `/home/$USER/.local/state/<app_name>` |
///
/// System-wide, preference-ordered, XDG Base Directories:
///
/// | XDG Base Directory                           | Environment variable | Fallback                      |
/// | -------------------------------------------- | -------------------- | ----------------------------- |
/// | [_Configuration_](method@XdgApp::sys_config) | `XDG_CONFIG_DIRS`    | `/etc/xdg`                    |
/// | [_Data_](method@XdgApp::sys_data)            | `XDG_DATA_DIRS`      | `/usr/local/share:/usr/share` |
///
/// System-wide, preference-ordered, XDG Application Subdirectories:
///
/// | XDG Base Directory                               | Environment variable | Fallback                                            |
/// | ------------------------------------------------ | -------------------- | --------------------------------------------------- |
/// | [_Configuration_](method@XdgApp::app_sys_config) | `XDG_CONFIG_DIRS`    | `/etc/xdg/<app_name>`                               |
/// | [_Data_](method@XdgApp::app_sys_data)            | `XDG_DATA_DIRS`      | `/usr/local/share/<app_name>:/usr/share/<app_name>` |
///
/// # Examples
///
/// The example below retrieves the _user-specific XDG app configuration subdirectory_ by reading
/// the value of the `XDG_CONFIG_HOME` environment variable as `$XDG_CONFIG_HOME/<app_name>`
/// (similarly the other XDG application subdirectories):
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
/// In the case the `XDG_CONFIG_DIR` environment variable is not set, `$HOME/.config/<app_name>`
/// is used as a fallback (similarly the other XDG application subdirectories):
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
/// Ultimately, if also the `HOME` environment variable is not set (very unlikely),
/// `/home/$USER/.config/<app_name>` is used as a fallback (similarly the other XDG directories):
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
    /// This function returns an error if neither `HOME` or `USER` environment variable is set.
    pub fn new(app_name: &'static str) -> Result<XdgApp, XdgError> {
        Ok(XdgApp {
            xdg: Xdg::new()?,
            name: app_name,
        })
    }

    /// Converts an [`Xdg`] instance to [`XdgApp`].
    #[inline]
    #[must_use]
    pub fn from_xdg(xdg: Xdg, app_name: &'static str) -> XdgApp {
        XdgApp {
            xdg,
            name: app_name,
        }
    }

    /// Returns the **home** directory of the user owning the process.
    #[must_use]
    pub fn home(&self) -> &Path {
        self.xdg.home()
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
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.xdg.cache()
    }

    /// Returns the _user-specific_ XDG **configuration** directory specified by the
    /// `XDG_CONFIG_HOME` environment variable. Falls back to `$HOME/.config` if `XDG_CONFIG_HOME`
    /// is not set or is set
    /// to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.xdg.config()
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let data_dir = xdg.data()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.xdg.data()
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
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.xdg.state()
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
    /// - the `XDG_RUNTIME_DIR` environment variable is set, but its value represents invalid
    ///   unicode.
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

    /// Returns the _user-specific_ XDG **executable** directory specified by `$HOME/.local/bin`.
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
    #[must_use]
    pub fn exec(&self) -> PathBuf {
        self.xdg.exec()
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration** directories specified
    /// by the `XDG_CONFIG_DIRS` environment variable, Falls back to `/etc/xdg` if
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let config_dirs = XdgApp::sys_config()?;
    /// # Ok(())
    /// # }
    /// ````
    pub fn sys_config() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::sys_config()
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let data_dirs = XdgApp::sys_data()?;
    /// # Ok(())
    /// # }
    /// ````
    pub fn sys_data() -> Result<Vec<PathBuf>, XdgError> {
        Xdg::sys_data()
    }

    /// Returns the path to the application subdirectory of an XDG base directory by the
    /// associated XDG environment variable or a fallback in the case the environment variable is
    /// not set or is set to an empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_app_dir_path(&self, dir: XdgDir) -> Result<PathBuf, XdgError> {
        self.xdg
            .get_dir_path(dir)
            .map(|path| path.append(self.name))
    }

    /// Returns the _user-specific_ XDG **cache** subdirectory for the current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG cache directory specified by the `XDG_CACHE_HOME`, if available.
    /// Falls back to `$HOME/.cache/<app_name>` if `XDG_CACHE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// See [`XdgApp::cache`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    pub fn app_cache(&self) -> Result<PathBuf, XdgError> {
        self.get_app_dir_path(XdgDir::Cache)
    }

    /// Returns the _user-specific_ XDG **configuration** subdirectory for the current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG configuration directory specified by the `XDG_CONFIG_HOME` if
    /// available. Falls back to `$HOME/.config/<app_name>` if `XDG_CONFIG_HOME` is not set or is
    /// set to an empty value.
    ///
    /// See [`XdgApp::config`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let app_config_dir = xdg.app_config()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_config(&self) -> Result<PathBuf, XdgError> {
        self.get_app_dir_path(XdgDir::Config)
    }

    /// Returns the _user-specific_ XDG **data** subdirectory for the current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG data directory specified by the `XDG_DATA_HOME`, if available.
    /// Falls back to `$HOME/.local/share/<app_name>` if `XDG_DATA_HOME` is not set or is set to
    /// an empty value.
    ///
    /// See [`XdgApp::data`] for further deatils.
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let app_data_dir = xdg.app_data()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_data(&self) -> Result<PathBuf, XdgError> {
        self.get_app_dir_path(XdgDir::Data)
    }

    /// Returns the _user-specific_ XDG **state** subdirectory for the current application.
    ///
    /// # Note
    ///
    /// This method uses the XDG state directory specified by the `XDG_STATE_HOME`, if available.
    /// Falls back to `$HOME/.local/state/<name>` if `XDG_STATE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// See [`XdgApp::state`] for further deatils.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let app_state_dir = xdg.app_state()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_state(&self) -> Result<PathBuf, XdgError> {
        self.get_app_dir_path(XdgDir::State)
    }

    /// Returns the _system-wide_, preference-ordered, paths set to a system XDG environment
    /// variable or a fallback in the case the environment variable is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_app_sys_dir_paths(&self, dirs: XdgSysDirs) -> Result<Vec<PathBuf>, XdgError> {
        let env_var_key = dirs.env_var();
        match Xdg::get_env_var(env_var_key)? {
            Some(env_var_val) => Xdg::iter_sys_dir_paths(env_var_key, &env_var_val)
                .map(|result| result.map(|path| path.append(self.name)))
                .collect(),
            None => Ok(dirs.fallback().map(|path| path.append(self.name)).collect()),
        }
    }

    /// Returns the _system-wide_, preference-ordered, XDG **configuration** subdirectories for
    /// the current application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered configuration directories specified by the
    /// `XDG_CONFIG_DIRS` environment variable. Falls back to `/etc/xdg/<app_name>` if
    /// `XDG_CONFIG_DIRS` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::sys_config`] for further details.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents invalid
    ///   unicode.
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
    pub fn app_sys_config(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_app_sys_dir_paths(XdgSysDirs::Config)
    }

    /// Returns the _system-wide_, preference-ordered, XDG **data** subdirectories for the current
    /// application.
    ///
    /// # Note
    ///
    /// This method uses the preference-ordered data directories specified by the `XDG_DATA_DIRS`
    /// environment variable. Falls back to `/usr/local/share/<app_name>:/usr/share/<app_name>` if
    /// `XDG_DATA_DIRS` is not set or is set to an empty value.
    ///
    /// See [`XdgApp::sys_data`] for further details.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_DIRS` environment variable is set, but its value represents a relative
    ///   path;
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
    pub fn app_sys_data(&self) -> Result<Vec<PathBuf>, XdgError> {
        self.get_app_sys_dir_paths(XdgSysDirs::Data)
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
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let cache_file = xdg.cache_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cache_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.cache_file(file)
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
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let config_file = xdg.config_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn config_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.config_file(file)
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
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let data_file = xdg.data_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn data_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.data_file(file)
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
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Exapmles
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// let state_file = xdg.state_file("file")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn state_file<P>(&self, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.state_file(file)
    }

    /// Returns the _user-specific_ XDG file path as `<xdg_dir>/<app_name>/<file>`.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn get_app_file_path<P>(&self, dir: XdgDir, file: P) -> Result<PathBuf, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg
            .get_dir_path(dir)
            .map(|path| path.append(self.name).append(file))
    }

    /// Returns the _user-specific_ XDG **cache** application file as
    /// `$XDG_CACHE_HOME/<app_name>/<file>`. Falls back to `$HOME/.cache/<app_name>/<file>` if
    /// `XDG_CACHE_HOME` is not set or is set to an empty value.
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
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    /// `$XDG_CONFIG_HOME/<app_name>/<file>`. Falls back to `$HOME/.config/<app_name>/<file>` if
    /// `XDG_CONFIG_HOME` is not set or is set to an empty value.
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
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    /// `$XDG_DATA_HOME/<app_name>/<file>`. Falls back to `$HOME/.local/share/<app_name>/<file>`
    /// if `XDG_DATA_HOME` is not set or is set to an empty value.
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
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    /// `$XDG_STATE_HOME/<app_name>/<file>`. Falls back to `$HOME/.local/state/<app_name>/<file>`
    /// if `XDG_STATE_HOME` is not set or is set to an empty value.
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
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_cache_file("file")? {
    ///     Some(cache_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_cache_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_cache_file(file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **configuration** directory specified
    /// by the`XDG_CONFIG_HOME` environment variable. If `XDG_CONFIG_HOME` is not set or is set
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
    /// - the `XDG_CONFIG_HOME` environment variable is set to invalid unicode;
    /// - `file` was **not** found inside the _user-specific_ XDG config directory and:
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but one (or more) path(s) in the
    ///       colon separated value represents a relative path;
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents invalid
    ///       unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_config_file("file")? {
    ///     Some(config_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_config_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_config_file(file)
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
    /// - the `XDG_DATA_HOME` environment variable is set to invalid unicode;
    /// - `file` was **not** found inside the _user-specific_ XDG data directory and:
    ///     - the `XDG_DATA_DIRS` environment variable is set, but one (or more) path(s) in the
    ///       colon separated value represents a relative path;
    ///     - the `XDG_DATA_DIRS` environment variable is set, but its value represents invalid
    ///       unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_data_file("file")? {
    ///     Some(data_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_data_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_data_file(file)
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
    /// - the `XDG_STATE_HOME` environment variable is set to invalid unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_state_file("file")? {
    ///     Some(state_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_state_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.search_state_file(file)
    }

    /// Searches for `file` inside a _user-specific_ XDG app subdirectory.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside the specified XDG app subdirectory;
    /// - `None` if the file is **not** found inside the specified XDG app directory.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn search_app_usr_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.xdg.get_dir_path(dir).map(|mut path| {
            path.push(self.name);
            path.push(file);
            path.is_file().then_some(path)
        })
    }

    /// Searches for `file` inside a _system-wide_, preference-ordered, set of XDG app
    /// subdirectories.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside one of the preference-ordered set of XDG system
    ///   subdirectories for the current application;
    /// - `None` if the file is **not** found inside any of the preference-ordered set of XDG
    ///   system subdirectory for the current application.
    ///
    /// # Errors
    ///
    /// This funciton returns an error in the following cases:
    /// - the XDG environment variable is set, but its value represents a relative path;
    /// - the XDG environment variable is set, but its value represents invalid unicode.
    #[inline]
    fn search_app_sys_file<P>(&self, dirs: XdgSysDirs, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        let env_var_key = dirs.env_var();
        match Xdg::get_env_var(env_var_key)? {
            Some(env_var_val) => Xdg::iter_sys_dir_paths(env_var_key, &env_var_val)
                .map(|result| result.map(|path| path.append(self.name).append(&file)))
                .find(|path| path.as_ref().is_ok_and(|path| path.is_file()))
                .transpose(),
            None => Ok(dirs
                .fallback()
                .map(|path| path.append(self.name).append(&file))
                .find(|path| path.is_file())),
        }
    }

    /// Searches for `file` inside XDG app subdirectories in the following order:
    /// - _user-specific_ XDG subdirectory for the current application;
    /// - _system-wide_, preference-ordered, set of XDG subdirectories for the current application.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if the file is found inside one of the XDG subdirectories for the current
    ///   application;
    /// - `None` if the file is **not** found inside one of the XDG subdirectories for the
    ///   current.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set, but its value
    ///   represents a relative path;
    /// - the XDG environment variable ([`XdgDir`] or [`XdgSysDir`]) is set, but its value
    ///   represents invalid unicode.
    #[inline]
    fn search_app_file<P>(&self, dir: XdgDir, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        if let Some(path) = self.search_app_usr_file(dir, &file)? {
            return Ok(Some(path));
        }

        if let Some(sys_dirs) = dir.to_sys() {
            if let Some(path) = self.search_app_sys_file(sys_dirs, &file)? {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    /// Searches for `file` inside the _user-specific_ XDG **cache** app subdirectory specified by
    /// `$XDG_CACHE_HOME/<app_name>`. The search falls back to `$HOME/.cache/<app_name>` if
    /// `XDG_CACHE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG subdirectories for the current
    ///   application;   
    /// - `None` if `file` is **not** found inside any of the XDG subdirectories for the current
    ///   application.
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_app_cache_file("file")? {
    ///     Some(app_cache_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_app_cache_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Cache, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **config** app subdirectory specified
    /// by `$XDG_CONFIG_HOME/<app_name>`. The search falls back to `$HOME/.config/<app_name>` if
    /// `XDG_CONFIG_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG subdirectories for the current
    ///   application;   
    /// - `None` if `file` is **not** found inside any of the XDG subdirectories for the current
    ///   application.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_CONFIG_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode;
    /// - `file` was **not** found inside the _user-specific_ XDG config directory and:
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but one (or more) path(s) in the
    ///       colon separated value represents a relative path;
    ///     - the `XDG_CONFIG_DIRS` environment variable is set, but its value represents invalid
    ///       unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_app_config_file("file")? {
    ///     Some(app_config_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_app_config_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Config, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **data** app subdirectory specified by
    /// `$XDG_DATA_HOME/<app_name>`. The search falls back to `$HOME/.data/<app_name>` if
    /// `XDG_DATA_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG subdirectories for the current
    ///   application;   
    /// - `None` if `file` is **not** found inside any of the XDG subdirectories for the current
    ///   application.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_DATA_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
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
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_app_data_file("file")? {
    ///     Some(app_data_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_app_data_file<P>(&self, file: P) -> Result<Option<PathBuf>, XdgError>
    where
        P: AsRef<Path>,
    {
        self.search_app_file(XdgDir::Data, file)
    }

    /// Searches for `file` inside the _user-specific_ XDG **state** app subdirectory specified by
    /// `$XDG_STATE_HOME/<app_name>`. The search falls back to `$HOME/.state/<app_name>` if
    /// `XDG_STATE_HOME` is not set or is set to an empty value.
    ///
    /// # Note
    ///
    /// This method returns:
    /// - `Some` if `file` is found inside one of the XDG subdirectories for the current
    ///   application;   
    /// - `None` if `file` is **not** found inside any of the XDG subdirectories for the current
    ///   application.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the `XDG_STATE_HOME` environment variable is set, but its value represents a relative
    ///   path;
    /// - the `XDG_CACHE_HOME` environment variable is set, but its value represents invalid
    ///   unicode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use microxdg::{XdgApp, XdgError};
    /// # fn main() -> Result<(), XdgError> {
    /// let xdg = XdgApp::new("app_name")?;
    /// match xdg.search_app_state_file("file")? {
    ///     Some(app_state_file) => { /* ... */ }
    ///     None => { /* ... */ }
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    use std::{env, error::Error, ffi::OsStr, fs, os::unix::prelude::OsStrExt};

    const INVALID_UNICODE_BYTES: [u8; 4] = [0xF0, 0x90, 0x80, 0x67];

    #[test]
    fn new_xdg_app() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user1");
        env::set_var("USER", "user2");

        assert_eq!(Path::new("/home/user1"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user1"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home(),
        );

        env::remove_var("HOME");

        assert_eq!(Path::new("/home/user2"), XdgApp::new("app_name")?.home());
        assert_eq!(
            Path::new("/home/user2"),
            XdgApp::from_xdg(Xdg::new()?, "app_name").home(),
        );

        env::remove_var("USER");

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

        let xdg = XdgApp::new("app_name")?;
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

        assert_eq!(vec![PathBuf::from("/etc/xdg")], XdgApp::sys_config()?);
        assert_eq!(
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share")
            ],
            XdgApp::sys_data()?,
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
            XdgApp::sys_config()?,
        );
        assert_eq!(
            vec![
                PathBuf::from("/data/dir1"),
                PathBuf::from("/data/dir2"),
                PathBuf::from("/data/dir3"),
                PathBuf::from("/data/dir4"),
            ],
            XdgApp::sys_data()?,
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

        let xdg = XdgApp::new("app_name")?;
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

        let xdg = XdgApp::new("app_name")?;
        assert_eq!(
            Some(cache_file.path().into()),
            xdg.search_cache_file("microxdg")?
        );
        assert_eq!(
            Some(config_file.path().into()),
            xdg.search_config_file("microxdg")?
        );
        assert_eq!(
            Some(data_file.path().into()),
            xdg.search_data_file("microxdg")?
        );
        assert_eq!(
            Some(state_file.path().into()),
            xdg.search_state_file("microxdg")?
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

    #[test]
    fn app_usr_dirs() -> Result<(), XdgError> {
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
            xdg.app_data()?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/app_name"),
            xdg.app_state()?,
        );

        env::set_var("XDG_CACHE_HOME", "/home/user2/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user2/.config");
        env::set_var("XDG_DATA_HOME", "/home/user2/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user2/.local/state");
        assert_eq!(Path::new("/home/user2/.cache/app_name"), xdg.app_cache()?);
        assert_eq!(Path::new("/home/user2/.config/app_name"), xdg.app_config()?);
        assert_eq!(
            Path::new("/home/user2/.local/share/app_name"),
            xdg.app_data()?,
        );
        assert_eq!(
            Path::new("/home/user2/.local/state/app_name"),
            xdg.app_state()?,
        );

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");
        assert_eq!(Path::new("/home/user1/.cache/app_name"), xdg.app_cache()?);
        assert_eq!(Path::new("/home/user1/.config/app_name"), xdg.app_config()?);
        assert_eq!(
            Path::new("/home/user1/.local/share/app_name"),
            xdg.app_data()?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/app_name"),
            xdg.app_state()?,
        );

        env::set_var("XDG_CACHE_HOME", "./app_name/cache");
        env::set_var("XDG_CONFIG_HOME", "./app_name/config");
        env::set_var("XDG_DATA_HOME", "./app_name/data");
        env::set_var("XDG_STATE_HOME", "./app_name/state");
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_CACHE_HOME",
                path: PathBuf::from("./app_name/cache"),
            }),
            xdg.app_cache(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_CONFIG_HOME",
                path: PathBuf::from("./app_name/config")
            }),
            xdg.app_config(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_DATA_HOME",
                path: PathBuf::from("./app_name/data")
            }),
            xdg.app_data(),
        );
        assert_eq!(
            Err(XdgError::RelativePath {
                env_var_key: "XDG_STATE_HOME",
                path: PathBuf::from("./app_name/state")
            }),
            xdg.app_state(),
        );

        let invalid_unicode = OsStr::from_bytes(&INVALID_UNICODE_BYTES);
        env::set_var("XDG_CACHE_HOME", invalid_unicode);
        env::set_var("XDG_CONFIG_HOME", invalid_unicode);
        env::set_var("XDG_DATA_HOME", invalid_unicode);
        env::set_var("XDG_STATE_HOME", invalid_unicode);
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_CACHE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.app_cache(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_CONFIG_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.app_config(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_DATA_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.app_data(),
        );
        assert_eq!(
            Err(XdgError::InvalidUnicode {
                env_var_key: "XDG_STATE_HOME",
                env_var_val: invalid_unicode.to_os_string(),
            }),
            xdg.app_state(),
        );

        Ok(())
    }

    #[test]
    fn app_sys_dirs() -> Result<(), XdgError> {
        env::remove_var("XDG_CONFIG_DIRS");
        env::remove_var("XDG_DATA_DIRS");

        let xdg = XdgApp::new("app_name")?;

        assert_eq!(
            vec![PathBuf::from("/etc/xdg/app_name")],
            xdg.app_sys_config()?,
        );
        assert_eq!(
            vec![
                PathBuf::from("/usr/local/share/app_name"),
                PathBuf::from("/usr/share/app_name"),
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

    #[test]
    fn app_usr_file() -> Result<(), XdgError> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = XdgApp::new("app_name")?;
        assert_eq!(
            Path::new("/home/user/.cache/app_name/file"),
            xdg.app_cache_file("file")?
        );
        assert_eq!(
            Path::new("/home/user/.config/app_name/file"),
            xdg.app_config_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user/.local/share/app_name/file"),
            xdg.app_data_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user/.local/state/app_name/file"),
            xdg.app_state_file("file")?,
        );

        env::set_var("XDG_CACHE_HOME", "/home/user1/.cache");
        env::set_var("XDG_CONFIG_HOME", "/home/user1/.config");
        env::set_var("XDG_DATA_HOME", "/home/user1/.local/share");
        env::set_var("XDG_STATE_HOME", "/home/user1/.local/state");
        assert_eq!(
            Path::new("/home/user1/.cache/app_name/file"),
            xdg.app_cache_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.config/app_name/file"),
            xdg.app_config_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/share/app_name/file"),
            xdg.app_data_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user1/.local/state/app_name/file"),
            xdg.app_state_file("file")?,
        );

        env::remove_var("HOME");
        env::set_var("USER", "user2");

        env::set_var("XDG_CACHE_HOME", "");
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_STATE_HOME", "");

        let xdg = XdgApp::new("app_name")?;
        assert_eq!(
            Path::new("/home/user2/.cache/app_name/file"),
            xdg.app_cache_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user2/.config/app_name/file"),
            xdg.app_config_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user2/.local/share/app_name/file"),
            xdg.app_data_file("file")?,
        );
        assert_eq!(
            Path::new("/home/user2/.local/state/app_name/file"),
            xdg.app_state_file("file")?,
        );

        Ok(())
    }

    #[test]
    fn search_app_file() -> Result<(), Box<dyn Error>> {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        env::set_var("HOME", "/home/user");
        env::set_var("USER", "user");

        let xdg = XdgApp::new("app_name")?;

        assert_eq!(None, xdg.search_app_cache_file("microxdg")?);
        assert_eq!(None, xdg.search_app_config_file("microxdg")?);
        assert_eq!(None, xdg.search_app_data_file("microxdg")?);
        assert_eq!(None, xdg.search_app_state_file("microxdg")?);

        let mut tmp_dir_builder = tempfile::Builder::new();
        tmp_dir_builder.prefix("microxdg");
        tmp_dir_builder.rand_bytes(4);

        let cache_home = tmp_dir_builder.tempdir()?;
        let app_cache_dir = cache_home.path().join("app_name");
        fs::create_dir(&app_cache_dir)?;
        let config_home = tmp_dir_builder.tempdir()?;
        let app_config_dir = config_home.path().join("app_name");
        fs::create_dir(&app_config_dir)?;
        let data_home = tmp_dir_builder.tempdir()?;
        let app_data_dir = data_home.path().join("app_name");
        fs::create_dir(&app_data_dir)?;
        let state_home = tmp_dir_builder.tempdir()?;
        let app_state_dir = state_home.path().join("app_name");
        fs::create_dir(&app_state_dir)?;

        env::set_var("XDG_CACHE_HOME", cache_home.path());
        env::set_var("XDG_CONFIG_HOME", config_home.path());
        env::set_var("XDG_DATA_HOME", data_home.path());
        env::set_var("XDG_STATE_HOME", state_home.path());

        let mut tmp_file_builder = tempfile::Builder::new();
        tmp_file_builder.prefix("microxdg");
        tmp_file_builder.rand_bytes(0);

        let cache_file = tmp_file_builder.tempfile_in(app_cache_dir)?;
        let config_file = tmp_file_builder.tempfile_in(app_config_dir)?;
        let data_file = tmp_file_builder.tempfile_in(app_data_dir)?;
        let state_file = tmp_file_builder.tempfile_in(app_state_dir)?;

        assert_eq!(
            Some(cache_file.path().into()),
            xdg.search_app_cache_file("microxdg")?,
        );
        assert_eq!(
            Some(config_file.path().into()),
            xdg.search_app_config_file("microxdg")?,
        );
        assert_eq!(
            Some(data_file.path().into()),
            xdg.search_app_data_file("microxdg")?,
        );
        assert_eq!(
            Some(state_file.path().into()),
            xdg.search_app_state_file("microxdg")?,
        );

        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_STATE_HOME");

        let data_dirs = tmp_dir_builder.tempdir()?;
        let app_data_dirs = data_dirs.path().join("app_name");
        fs::create_dir(&app_data_dirs)?;
        let config_dirs = tmp_dir_builder.tempdir()?;
        let app_config_dirs = config_dirs.path().join("app_name");
        fs::create_dir(&app_config_dirs)?;

        env::set_var("XDG_DATA_DIRS", &app_data_dirs);
        env::set_var("XDG_CONFIG_DIRS", &app_config_dirs);

        let data_file = tmp_file_builder.tempfile_in(app_data_dirs)?;
        let config_file = tmp_file_builder.tempfile_in(app_config_dirs)?;

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

    #[test]
    fn clone_debug() -> Result<(), XdgError> {
        env::set_var("HOME", "/home/user");

        let xdg = XdgApp::new("app_name")?;
        assert_eq!(
            "XdgApp { xdg: Xdg { home: \"/home/user\" }, name: \"app_name\" }",
            format!("{xdg:?}")
        );

        #[allow(clippy::redundant_clone)]
        let cloned_xdg = xdg.clone();
        assert_eq!(
            "XdgApp { xdg: Xdg { home: \"/home/user\" }, name: \"app_name\" }",
            format!("{cloned_xdg:?}")
        );

        Ok(())
    }
}
