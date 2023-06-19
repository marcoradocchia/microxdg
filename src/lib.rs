//! A minimal XDG base directory Specification library for the Rust programming
//! language.
//! See <https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html>
//! for specification details.

mod error;

use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

pub use error::XdgError;

/// XDG Base Directory Specification directories.
enum XdgBaseDir {
    Cache,
    Config,
    Data,
    State,
}

impl XdgBaseDir {
    /// Returns the XDG environment variable associated to the XDG base directory.
    fn env_var(&self) -> &'static str {
        match self {
            XdgBaseDir::Cache => "XDG_CACHE_HOME",
            XdgBaseDir::Config => "XDG_CONFIG_HOME",
            XdgBaseDir::Data => "XDG_DATA_HOME",
            XdgBaseDir::State => "XDG_STATE_HOME",
        }
    }

    /// Returns the fallback directory in the case the XDG environment variable
    /// is not set.
    fn fallback(&self) -> &str {
        match self {
            XdgBaseDir::Cache => ".cache",
            XdgBaseDir::Config => ".config",
            XdgBaseDir::Data => ".local/share",
            XdgBaseDir::State => ".local/state",
        }
    }
}

/// XDG Base Directory Specification.
#[derive(Debug)]
pub struct Xdg {
    /// User's home directory.
    home: PathBuf,
}

impl Xdg {
    /// Constructs a new [`Xdg`] instance.
    ///
    /// # Errors
    ///
    /// This function returns an error if neither `HOME` or `USER` environment
    /// variable is set.
    ///
    /// # Examples
    ///
    /// The example below retrieves the XDG config directory with the
    /// `XDG_CONFIG_DIR` set:
    /// ```rust
    /// # use std::{error::Error, path::PathBuf};
    /// # use microxdg::Xdg;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// std::env::set_var("XDG_CONFIG_HOME", "/home/user/.config");
    ///
    /// let xdg = Xdg::new()?;
    /// assert_eq!(PathBuf::from("/home/user/.config"), xdg.config()?);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// In the case the `XDG_CONFIG_DIR` is not set, but `HOME` is, allowing a
    /// fallback:
    /// ```rust
    /// # use std::{error::Error, path::PathBuf};
    /// # use microxdg::Xdg;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// std::env::remove_var("XDG_CONFIG_DIR");
    /// std::env::set_var("USER", "/home/user");
    ///
    /// let xdg = Xdg::new()?;
    /// assert_eq!(PathBuf::from("/home/user/.config"), xdg.config()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Xdg, XdgError> {
        if let Ok(home) = env::var("HOME") {
            return Ok(Self { home: home.into() });
        }

        if let Ok(user) = env::var("USER") {
            return Ok(Self {
                home: format!("/home/{user}").into(),
            });
        }

        Err(XdgError::HomeNotFound)
    }

    /// TODO: DOCUMENT THIS
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
    fn get_path(&self, dir: XdgBaseDir) -> Result<PathBuf, XdgError> {
        let env_var_key = dir.env_var();
        match env::var(env_var_key) {
            // XDG environment variable is set and non-empty.
            Ok(env_var_val) if !env_var_val.is_empty() => {
                let env_var_val = PathBuf::from(env_var_val);

                if env_var_val.is_relative() {
                    // XDG environment is set to a relative path.
                    return Err(XdgError::EnvVarRelativePath {
                        env_var_key,
                        env_var_val,
                    });
                }

                Ok(env_var_val)
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

    /// Returns the user-specific XDG **cache** directory specified by
    /// the `XDG_CACHE_HOME` environment variable. Falls back to
    /// `$HOME/.cache` if `XDG_CACHE_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following cases:
    /// - the XDG environment variable is set to a relative path;
    /// - the XDG environment variable contains invalid unicode.
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgBaseDir::Cache)
    }

    /// Returns the user-specific XDG **configuration** directory specified by
    /// the `XDG_CONFIG_HOME` environment variable. Falls back to
    /// `$HOME/.config` if `XDG_CONFIG_HOME` is not set or is set to an empty
    /// value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's "Errors" section.
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgBaseDir::Config)
    }

    /// Returns the user-specific XDG **data** directory specified by
    /// the `XDG_DATA_HOME` environment variable. Falls back to
    /// `$HOME/.local/share` if `XDG_DATA_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's "Errors" section.
    #[inline]
    pub fn data(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgBaseDir::Data)
    }

    /// Returns the user-specific XDG **state** directory specified by
    /// the `XDG_STATE_HOME` environment variable. Falls back to
    /// `$HOME/.local/state` if `XDG_STATE_HOME` is not set or is set to an
    /// empty value.
    ///
    /// # Errors
    ///
    /// See [`Xdg::cache`] method's "Errors" section.
    #[inline]
    pub fn state(&self) -> Result<PathBuf, XdgError> {
        self.get_path(XdgBaseDir::State)
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
    /// TODO: DOCUMENT THIS
    #[inline]
    pub fn cache(&self) -> Result<PathBuf, XdgError> {
        self.xdg.cache()
    }

    /// TODO: DOCUMENT THIS
    #[inline]
    pub fn config(&self) -> Result<PathBuf, XdgError> {
        self.xdg.config()
    }

    /// TODO: DOCUMENT THIS
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
    /// See [`Xdg::cache`] method's "Errors" section.
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
    /// See [`Xdg::cache`] method's "Errors" section.
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
    /// See [`Xdg::cache`] method's "Errors" section.
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
    /// See [`Xdg::cache`] method's "Errors" section.
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
        env::remove_var("HOME");
        env::remove_var("USER");
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
    fn base_dirs() {
        env::remove_var("HOME");
        env::remove_var("USER");
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
    fn app_dirs() {
        env::remove_var("HOME");
        env::remove_var("USER");
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

        env::remove_var("HOME");
        env::remove_var("USER");
    }
}
