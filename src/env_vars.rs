mod encode;
mod parse;
pub(crate) mod writer;

use crate::utils::print_error;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub(crate) struct EnvironmentVariable {
    name: String,
    value: Vec<u8>,
}

impl EnvironmentVariable {
    pub(crate) fn from_args(args: &Vec<OsString>) -> Vec<Self> {
        args.iter()
            .unique()
            .map(OsString::as_os_str)
            .map(EnvironmentVariable::from_parse)
            .filter_map(|var| {
                var.inspect_err(|error| {
                    print_error(error);
                }).ok()
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn from_names(names: &Vec<OsString>) -> Vec<Self> {
        names.iter()
            .unique()
            .map(|str| str.as_bytes().to_vec())
            .map(EnvironmentVariable::new)
            .filter_map(|var| {
                var.inspect_err(|error| {
                    print_error(error);
                }).ok()
            })
            .collect::<Vec<_>>()
    }

    const LINE_PATTERN: &'static str = r"^(?:declare -g(?: \+)?x|unset) ";

    fn from_lines(lines: &Vec<String>) -> Vec<Self> {
        let capture_name = Regex::new(
            &format!("{}(?<name>[^=]+)(?:[ =]|$)", Self::LINE_PATTERN)
        ).unwrap();

        lines.iter()
            .filter_map(|line| {
                capture_name.captures(line)
            })
            .map(|capture| {
                capture["name"].as_bytes().to_vec()
            })
            .unique()
            .map(EnvironmentVariable::new)
            .filter_map(|var| {
                var.inspect_err(|error| {
                    print_error(error);
                }).ok()
            })
            .collect::<Vec<_>>()
    }
}