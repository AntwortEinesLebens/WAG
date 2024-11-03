// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::traces::{drivers::Drivers, processes::Processes};
use clap::{Args, Subcommand};
use std::error::Error;

pub mod drivers;
pub mod processes;

#[derive(Debug, Args)]
pub struct Traces {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Drivers(Drivers),
    Processes(Processes),
}

pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl Runnable for Traces {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Commands::Drivers(drivers) => drivers as &dyn Runnable,
            Commands::Processes(processes) => processes,
        }
        .run()
    }
}
