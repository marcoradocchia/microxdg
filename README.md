<div align="center">
  <h1 align="center"><code>microxdg</code></h1>

  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/microxdg?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/microxdg?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/microxdg?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  ![Crates.io downloads](https://img.shields.io/crates/d/microxdg?label=crates.io%20downloads&logo=rust)
  ![Crates.io version](https://img.shields.io/crates/v/microxdg?logo=rust&color=%23d8a657)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/microxdg?color=%23e78a4e)
</div>

An **XDG Base Directory Specification** Rust library that aims to be
conservative on memory allocation and overall memory footprint.

# Usage

## Dependency

Add `microxdg` as a dependency to your Rust project by running the following
`cargo` command in your project directory:
```sh
cargo add microxdg
```

Alternatively, add the following line in the `[dependencies]` section of your
`Cargo.toml`:
```toml
microxdg = "0.1.0"
```

## API

The `microxdg` API consists in two main `structs`:
- `Xdg`, an implementation of the _XDG Base Directory Specification_;
- `XdgApp`, an implementation of the _XDG Base Directory Specification_ with
  extent to application-specific (or project-specific) subdirectories.

> **Note**: the latter's associated functions and methods are a superset of
> those implemented for `Xdg`. For this reason, it should be preferred only in
> case you need access to application-specific subdirectories.

### Retrieve user-specific XDG directories

The following example illustrates how to retrieve the _user-specific_ XDG
**configuration** directory:
```rust
use microxdg::{Xdg, XdgError};

fn main() -> Result<(), XdgError> {
    let xdg = Xdg::new()?;
    let config_dir = xdg.config()?;

    /* Do something with `config_dir`... */

    Ok(())
}
```
The `Xdg::config` method prefers the path specified by the `XDG_CONFIG_HOME`
environment variable. However, it falls back to `$HOME/.config` or
`/home/$USER/.config` if such environment variable is not set, or is set to an
empty value.

Also, it returns an error (`XdgError`) in the following cases:
- the `XDG_CONFIG_HOME` environment variable is set, but its value represents
  a relative path;
- the `XDG_CONFIG_HOME` environment variable is set, but its value represents
  invalid unicode.

Analogous methods are available for each of the other XDG directories listed
in the specification: `Xdg::cache`, `Xdg::data`, `Xdg::state`, `Xdg::runtime`,
`Xdg::exec`. Below a table illustrating the environment variable and
corresponding fallbacks for each of the XDG directories:

| XDG Base Directory | Environment variable | Fallback - `$HOME` set | Fallback - `$HOME` not set |
| ------------------ | -------------------- | ---------------------- | -------------------------- |
| _Cache_            | `$XDG_CACHE_HOME`    | `$HOME/.cache`         | `/home/$USER/.cache`       |
| _Configuration_    | `$XDG_CONFIG_HOME`   | `$HOME/.config`        | `/home/$USER/.config`      |
| _Data_             | `$XDG_DATA_HOME`     | `$HOME/.local/share`   | `/home/$USER/.local/share` |
| _State_            | `$XDG_STATE_HOME`    | `$HOME/.local/state`   | `/home/$USER/.local/state` |
| _Runtime_          | `$XDG_RUNTIME_DIR`   | -                      | -                          |
| _Executable_       | -                    | `$HOME/.local/bin`     | `/home/$USER/.local/bin`   |

### Retrieve user-specific XDG application subdirectories

The following example illustrates how to retrieve the _user-specific_ XDG
**data** application subdirectory:
```rust
use microxdg::{XdgApp, XdgError};

fn main() -> Result<(), XdgError> {
    let xdg = XdgApp::new("app_name")?;
    let app_data_dir = xdg.app_data()?;

    /* Do something with `app_data_dir`... */

    Ok(())
}
```
The `Xdg::app_data` method prefers the path specified by the `XDG_DATA_HOME`
environment variable and appends `app_name` to it. However, it falls back to
`$HOME/.local/share/app_name` or `/home/$USER/.local/share/app_name` if such
environment variable is not set, or is set to an empty value.

Also, it returns an error (`XdgError`) in the following cases:
- the `XDG_DATA_HOME` environment variable is set, but its value represents
  a relative path;
- the `XDG_DATA_HOME` environment variable is set, but its value represents
  invalid unicode.

Analogous methods are available for each of the other XDG application
subdirectories: `Xdg::app_cache`, `Xdg::app_config`, `Xdg::app_state`,
`Xdg::app_runtime`, `Xdg::app_exec`. Below a table illustrating the environment
variable and corresponding fallbacks for each of the XDG directories:

| XDG Application Subdirectory | Environment variable | Fallback - `$HOME` set        | Fallback - `$HOME` not set          |
| ---------------------------- | -------------------- | ----------------------------- | ----------------------------------- |
| _App Cache_                  | `$XDG_CACHE_HOME`    | `$HOME/.cache/app_name`       | `/home/$USER/.cache/app_name`       |
| _App Configuration_          | `$XDG_CONFIG_HOME`   | `$HOME/.config/app_name`      | `/home/$USER/.config/app_name`      |
| _App Data_                   | `$XDG_DATA_HOME`     | `$HOME/.local/share/app_name` | `/home/$USER/.local/share/app_name` |
| _App State_                  | `$XDG_STATE_HOME`    | `$HOME/.local/state/app_name` | `/home/$USER/.local/state/app_name` |

### Retrieve user-specific XDG files

The following example illustrates how to retrieve the path to a file contained
in the _user-specific_ XDG **cache** directory:
```rust
use microxdg::{XdgApp, XdgError};

fn main() -> Result<(), XdgError> {
    let xdg = Xdg::new()?;
    let config_file = xdg.cache_file("file_name");

    /* Do something with `config_file`... */

    Ok(())
}
```
The `Xdg::cache_file` method prefers the XDG directory path specified by the
`XDG_CACHE_HOME` environment variable and appends `file_name` to it. However,
it falls back to `$HOME/.cache/file_name` or `/home/$USER/.cache/file_name` if
such environment variable is not set, or is set to an empty value.

Also, it returns an error (`XdgError`) in the following cases:
- the `XDG_CACHE_HOME` environment variable is set, but its value represents
  a relative path;
- the `XDG_CACHE_HOME` environment variable is set, but its value represents
  invalid unicode.

Analogous methods are available for each of the other XDG directories listed
in the specification: `Xdg::cache_file`, `Xdg::data_file`, `Xdg::state_file`,
`Xdg::runtime_file`, `Xdg::exec_file`.

### Retrieve user-specific XDG files

TODO

### Retrieve system-wide, preference-ordered, XDG directories

The following example illustrates how to retireve the _system-wide_,
preference-ordered, XDG **data** directories:
```rust
use microxdg::{Xdg, XdgError};

fn main() -> Result<(), XdgError> {
    let xdg = Xdg::new()?;
    let sys_data_dirs = Xdg::sys_data()?;
  
    /* Do something with `sys_data_dirs`... */
  
    Ok(())
}
```
The `Xdg::sys_data` associated function prefers the preference-ordered, colon
separated paths specified by the `XDG_DATA_DIRS` environment variable.
However, it falls back to `/usr/local/share:/usr/share` if such environment
variable is not set, or is set to an empty value.

Also, it returns an error (`XdgError`) in the following cases:
- the `XDG_DATA_DIRS` environment variable is set, but one (or more) path(s)
  in the colon separated value represents a relative path;
- the `XDG_DATA_DIRS` environment variable is set, but its value represents
  invalid unicode.

An analogous method is available for the `XDG_CONFIG_DIRS` system-wide XDG
directories: `Xdg::sys_config`. Below a table illustrating the environment
variable and corresponding fallbacks for each of the system-wide,
preference-ordered, XDG directories:

| XDG Base Directory  | Environment variable | Fallback                      |
| ------------------- | -------------------- | ----------------------------- |
| _Configuration_     | `$XDG_CONFIG_DIRS`   | `/etc/xdg`                    |
| _Data_              | `$XDG_DATA_DIRS`     | `/usr/local/share:/usr/share` |


### Retrieve system-wide, preference-ordered, XDG application subdirectories

The following example illustrates how to retrieve the _system-wide_,
preference-ordered, XDG **config** application subdirectories:
```rust
use microxdg::{XdgApp, XdgError};

fn main() -> Result<(), XdgError> {
    let xdg = XdgApp::new("app_name")?;
    let app_sys_config_dirs = xdg.app_sys_config()?;

    /* Do something with `app_sys_config_dirs`... */

    Ok(())
}
```
The `XdgApp::app_sys_config` method prefers the preference-ordered, colon
separated paths specified by the `XDG_CONFIG_DIRS` environment variable and
appends `app_name` to each of its paths. However, it falls back to
`/etc/xdg/app_name` if such environment variable is not set, or is set to an
empty value.

Also, it returns an error (`XdgError`) in the following cases:
- the `XDG_CONFIG_DIRS` environment variable is set, but one (or more) path(s)
  in the colon separated value represents a relative path;
- the `XDG_CONFIG_DIRS` environment variable is set, but its value represents
  invalid unicode.

An analogous method is available for the `XDG_DATA_DIRS` system-wide XDG
directories: `XdgApp::app_sys_data`. Below a table illustrating the environment
variable and corresponding fallbacks for each of the system-wide,
preference-ordered, XDG app subdirectories:

| XDG Base Directory | Environment variable | Fallback                                        |
| ------------------ | -------------------- | ----------------------------------------------- |
| _Configuration_    | `$XDG_CONFIG_DIRS`   | `/etc/xdg/app_name`                             |
| _Data_             | `$XDG_DATA_DIRS`     | `/usr/local/share/app_name:/usr/share/app_name` |
