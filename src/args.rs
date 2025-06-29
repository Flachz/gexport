use crate::utils::*;
use std::ffi::OsString;
use std::path::PathBuf;
use clap::{Args, Parser, ValueEnum};
use clap::builder::styling::{Styles, AnsiColor, Effects, Style};
use clap_stdin::FileOrStdin;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(styles = STYLE)]
#[command(override_usage = "\
\tgexport [-p [NAME]...]
\tgexport [-udn] <NAME[=VALUE]>...
\tgexport --import [FILE]
\tgexport --init <SHELL>
")]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) args: Option<GexportArgs>,

    /// Prints all the globally exported environment variables. 
    /// 
    /// This is the default action if no other arguments are provided. Optionally a list 
    /// of identifiers can be given to select which environment variables are shown.
    #[arg(short, long)]
    #[arg(value_name = "NAME")]
    #[arg(num_args = 0..)]
    #[arg(conflicts_with_all = ["init", "import"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) print: Option<Vec<OsString>>,

    /// Import environment variables from file or stdin. For expected format see -p / --print.
    /// 
    /// Examples: gexport -p VAR1 VAR2 | ssh user@example 'gexport --import'
    ///           gexport --import vars.txt
    #[arg(long)]
    #[arg(value_name = "FILE")]
    #[arg(default_missing_value = "-")]
    #[arg(num_args = 0..=1)]
    #[arg(conflicts_with_all = ["print", "init"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) import: Option<FileOrStdin>,
    
    /// Bash: echo 'eval "$(gexport --init bash)"' >> ~/.bashrc
    ///  Zsh: echo 'eval "$(gexport --init zsh)"' >> ~/.zshrc
    #[arg(long)]
    #[arg(value_name = "SHELL")]
    #[arg(value_enum)]
    #[arg(conflicts_with_all = ["print", "import"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) init: Option<Shell>,
    
    #[arg(long)]
    #[arg(env = "XDG_CONFIG_HOME")]
    #[arg(default_value_os_t = get_default_config_dir())]
    pub(crate) config_home: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub(crate) enum Shell {
    Bash,
    Zsh,
}

#[derive(Args)]
#[group(conflicts_with_all = ["init", "print", "import"])]
pub(crate) struct GexportArgs {
    /// Environment variables to be globally exported.
    /// 
    /// Note: If no value is given tries to globally reexport, this requires the environment
    /// variable to already be shell exported, otherwise will set to empty string.
    #[arg(value_name = "NAME[=VALUE]")]
    #[arg(verbatim_doc_comment)]
    pub(crate) vars: Vec<OsString>,

    /// Unsets the given environment variables.
    ///
    /// Note: This will continuously clear the environment variables as the shells sources
    /// gexport's env-var files.
    #[arg(short, long)]
    #[arg(conflicts_with_all = ["delete", "unexport"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) unset: bool,

    /// Marks the environment variables as not exported to child processes.
    /// 
    /// Note: The environment variables are still shared across interactive shell sessions.
    #[arg(short = 'n', long)]
    #[arg(conflicts_with_all = ["delete", "unset"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) unexport: bool,
    
    /// Delete the specified environment variables.
    /// 
    /// Note: This deletes the variable from gexport's env-var files, causing all open shell sessions
    /// to indefinitely maintain the value until manually unset or restarted.
    #[arg(short, long)]
    #[arg(conflicts_with_all = ["unset", "unexport"])]
    #[arg(verbatim_doc_comment)]
    pub(crate) delete: bool,
}

const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

const STYLE: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);