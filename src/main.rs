mod utils;
mod args;
mod env_vars;

use crate::args::*;
use crate::utils::*;
use crate::env_vars::EnvironmentVariable;
use crate::env_vars::writer::{WriteManager, WriterError};
use std::fmt::Display;
use std::fmt;
use std::process::ExitCode;
use std::sync::{LazyLock, OnceLock};
use std::io::Read;
use clap::Parser;
use clap_stdin::FileOrStdin;

static CLI: LazyLock<Cli> = LazyLock::new(|| {
    Cli::parse()
});

static STATUS: OnceLock<ExitCode> = OnceLock::new();

fn set_failure() {
    STATUS.get_or_init(|| ExitCode::FAILURE);
}

fn main() -> ExitCode {
    if let Some(shell) = CLI.init {
        print!("{shell}");
    } else if let Some(args) = &CLI.args {
        export(args);
    } else if let Some(data) = &CLI.import {
        import(data);
    } else if CLI.clear {
        clear();
    } else {
        print();
    }

    *STATUS.get_or_init(|| ExitCode::SUCCESS)
}

fn export(args: &GexportArgs) {
    EnvironmentVariable::from_args(&args.vars)
        .iter()
        .try_for_each(|var| {
            var.delete()?;
            if !args.delete {
                var.append()?;
            }
            Ok(())
        })
        .and_then(|_| WriteManager::finalize())
        .unwrap_or_else(|error| {
            print_error(&error);
        });
}

fn import(data: &FileOrStdin) {
    let mut buf = Vec::new();
    let success = data.clone()
        .into_reader()
        .ok()
        .and_then(|mut import| {
            import.read_to_end(&mut buf).ok()
        });

    if success.is_some() {
        WriteManager::import(buf)
            .and_then(|_| WriteManager::finalize())
            .inspect_err(|error| print_error(error)).ok();
    } else {
        print_error("could not read import");
    }
}

fn print() {
    if let Some(names) = &CLI.print
        && !names.is_empty()
    {
        if let Err(error) = EnvironmentVariable::from_names(names)
            .iter()
            .map(EnvironmentVariable::get_line)
            .try_for_each(|var| {
                if let Some(var) = var? {
                    println!("{var}");
                }
                Ok::<_, WriterError>(())
            })
        {
            print_error(&error);
        }
    } else {
        match WriteManager::get() {
            Ok(write_mananager) => {
                write_mananager
                    .iter()
                    .for_each(|line| {
                        println!("{}", line);
                    });
            },
            Err(error) => print_error(&error),
        }
    }
}

fn clear() {
    WriteManager::clear()
        .and_then(|_| WriteManager::finalize())
        .inspect_err(|error| print_error(error)).ok();
}

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", root_include_str!("init/init.sh"))?;
        match self {
            Self::Bash => writeln!(f, "{}", root_include_str!("init/hook.bash")),
            Self::Zsh => writeln!(f, "{}", root_include_str!("init/hook.zsh")),
        }
    }
}
