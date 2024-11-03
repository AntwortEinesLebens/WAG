// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod traces;
mod windows;

use clap::Parser;
use cli::{Arguments, Commands};
use std::error::Error;
use traces::Runnable;

fn main() -> Result<(), Box<dyn Error>> {
    match Arguments::parse().command {
        Commands::Traces(action) => action.run()?,
    };

    Ok(())
}
