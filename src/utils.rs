use crate::set_failure;
use std::path::PathBuf;
use std::env;
use std::fmt::Display;
use colored::Colorize;

#[macro_export]
macro_rules! root_include_str {
    ($arg:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $arg))
    };
}

#[macro_export]
macro_rules! str {
    ($arg:expr) => {
        $arg.to_string()
    }
}

pub(crate) fn get_default_config_dir() -> PathBuf {
    if let Some(home) = env::home_dir()
        && !home.as_os_str().is_empty()
    {
        home.join(".config")
    } else if let Ok(cwd) = env::current_dir() {
        cwd.join(".config")
    } else {
        PathBuf::from(".config")
    }
}

pub(crate) fn print_error<E: Display + ?Sized>(error: &E) {
    eprintln!("{} {}", "error:".red().bold(), error);
    set_failure();
}
