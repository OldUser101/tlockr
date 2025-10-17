// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    cli.rs:
        CLI entry point, contains basic argument parsing, and initializes
        various state objects.
*/

use crate::cli::{LogLevel, build_cli, run_lock};
use crate::config::{RunConfig, load_root_config};

use tracing::{debug, error};

/// Run the command line parser for tlockr
pub fn start() {
    let matches = build_cli().get_matches();

    let log_level = matches
        .get_one::<LogLevel>("log_level")
        .unwrap_or(&LogLevel::INFO);

    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_max_level(log_level.to_level())
        .init();

    // Load the root configuration, exiting if it is invalid
    let root_config = load_root_config();

    let now = chrono::Local::now();
    debug!("tlockr started at {}", now.to_rfc3339());

    let result: Result<_, Box<dyn std::error::Error>> = match matches.subcommand() {
        Some(("run", args)) => {
            let theme = args.get_one::<String>("theme");
            let develop = args.get_flag("develop");

            let stored_run_config = root_config.run.unwrap_or_default();

            let run_config = RunConfig {
                theme: theme.or(stored_run_config.theme.as_ref()),
                develop,
            };

            run_lock(&run_config)
        }
        None => {
            build_cli().print_help().unwrap();
            Ok(())
        }
        _ => unreachable!(),
    };

    if let Err(e) = result {
        error!("{:?}", e);
    }

    let now = chrono::Local::now();
    debug!("tlockr exited at {}", now.to_rfc3339());
}
