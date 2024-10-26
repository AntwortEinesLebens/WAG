// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{actions::Runnable, windows::users::is_administrator};
use clap::Parser;
use std::{error::Error, path::PathBuf};
use windows::{
    core::{Owned, HSTRING, PCWSTR},
    Win32::{
        Foundation::GENERIC_READ,
        System::Services::{
            CreateServiceW, OpenSCManagerW, OpenServiceW, StartServiceW, SC_HANDLE,
            SC_MANAGER_ALL_ACCESS, SC_MANAGER_CREATE_SERVICE, SERVICE_AUTO_START,
            SERVICE_ERROR_IGNORE, SERVICE_KERNEL_DRIVER,
        },
    },
};

#[derive(Debug, Parser)]
pub struct Byovd {
    #[clap(required = true, help = "Name of the service")]
    service_name: String,
    #[clap(required = true, help = "Displayed name of the service")]
    displayed_name: String,
    #[clap(required = true, help = "Path to the driver")]
    path: PathBuf,
}

impl Runnable for Byovd {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if !is_administrator()? {
            return Ok(());
        }

        if !self.path.try_exists()? || !self.path.is_file() {
            return Ok(());
        }

        unsafe {
            let service_manager: Owned<SC_HANDLE> = Owned::new(OpenSCManagerW(
                PCWSTR::null(),
                PCWSTR::null(),
                SC_MANAGER_CREATE_SERVICE,
            )?);

            if OpenServiceW(
                *service_manager,
                &HSTRING::from(self.service_name.as_str()),
                GENERIC_READ.0,
            )
            .is_ok()
            {
                return Ok(());
            }

            let service: Owned<SC_HANDLE> = Owned::new(CreateServiceW(
                *service_manager,
                &HSTRING::from(self.service_name.as_str()),
                &HSTRING::from(self.displayed_name.as_str()),
                SC_MANAGER_ALL_ACCESS,
                SERVICE_KERNEL_DRIVER,
                SERVICE_AUTO_START,
                SERVICE_ERROR_IGNORE,
                &HSTRING::from(self.path.to_str().unwrap()),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
            )?);

            Ok(StartServiceW(*service, None)?)
        }
    }
}
