// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    args.rs:
        Argument parsing with the `clap` crate
*/

use clap::{
    Arg, ArgAction, Command, ValueEnum,
    builder::styling::{AnsiColor, Effects, Styles},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ConfigScope {
    USER,
    SYSTEM,
}

impl LogLevel {
    pub fn to_level(self) -> tracing::Level {
        match self {
            Self::TRACE => tracing::Level::TRACE,
            Self::DEBUG => tracing::Level::DEBUG,
            Self::INFO => tracing::Level::INFO,
            Self::WARN => tracing::Level::WARN,
            Self::ERROR => tracing::Level::ERROR,
        }
    }
}

/// Generate styles for use with clap's command parser
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::BrightYellow.on_default())
        .valid(AnsiColor::BrightGreen.on_default())
        .invalid(AnsiColor::BrightRed.on_default())
}

/// Build the full CLI command tree
pub fn build_cli() -> Command {
    Command::new("tlockr")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A highly customisable screen locker for wlroots-based compositors")
        .styles(styles())
        .arg(
            Arg::new("log_level")
                .help("Level of logging to display")
                .short('l')
                .long("log-level")
                .global(true)
                .value_name("LOG LEVEL")
                .value_parser(clap::builder::EnumValueParser::<LogLevel>::new())
                .default_value("info"),
        )
        .subcommand(
            Command::new("run")
                .about("Run the screen locker")
                .arg(
                    Arg::new("theme")
                        .help("Name of theme to run")
                        .value_name("THEME")
                        .required(false),
                )
                .arg(
                    Arg::new("config")
                        .help("Configuration file to use for this session")
                        .short('c')
                        .long("config")
                        .value_name("FILE"),
                )
                .arg(
                    Arg::new("develop")
                        .help("Run the screen locker in development mode")
                        .short('d')
                        .long("develop")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new theme")
                .arg(
                    Arg::new("name")
                        .help("Name of the theme to create")
                        .required(true),
                )
                .arg(
                    Arg::new("directory")
                        .help("Directory to create the theme in")
                        .short('d')
                        .long("directory")
                        .value_name("DIR"),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Manage configuration")
                .subcommand(
                    Command::new("edit")
                        .about("Edit configuration files")
                        .arg(
                            Arg::new("editor")
                                .help("Editor to open configuration files in")
                                .short('e')
                                .long("editor")
                                .value_name("EDITOR"),
                        )
                        .arg(
                            Arg::new("user")
                                .long("user")
                                .help("Target the user configuration")
                                .action(ArgAction::SetTrue)
                                .conflicts_with("system")
                                .required_unless_present("system"),
                        )
                        .arg(
                            Arg::new("system")
                                .long("system")
                                .help("Target the system configuration")
                                .action(ArgAction::SetTrue)
                                .conflicts_with("user")
                                .required_unless_present("user"),
                        ),
                )
                .subcommand(Command::new("view").about("Display configuration file contents"))
                .subcommand(Command::new("path").about("Display configuration file paths")),
        )
}
