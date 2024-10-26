// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{drivers::byovd::Byovd, Runnable};
use clap::{Args, Subcommand};
use std::error::Error;

pub mod byovd;

#[derive(Debug, Args)]
pub struct Drivers {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Byovd(Byovd),
}

impl Runnable for Drivers {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Commands::Byovd(byovd) => byovd as &dyn Runnable,
        }
        .run()
    }
}
