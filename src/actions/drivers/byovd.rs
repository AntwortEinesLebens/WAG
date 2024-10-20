// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{actions::Runnable, windows::users::is_administrator};
use clap::Parser;
use std::{error::Error, path::PathBuf};
use windows::{
    core::{Owned, HSTRING, PCWSTR},
    Win32::System::Services::{
        CreateServiceW, OpenSCManagerW, StartServiceW, SC_HANDLE, SC_MANAGER_ALL_ACCESS,
        SC_MANAGER_CREATE_SERVICE, SERVICE_AUTO_START, SERVICE_ERROR_IGNORE, SERVICE_KERNEL_DRIVER,
    },
};

#[derive(Debug, Parser)]
pub struct Byovd {
    #[clap(required = true, help = "Internal Name of the service")]
    internal: String,
    #[clap(required = true, help = "Displayed Name of the service")]
    display: String,
    #[clap(required = true, help = "Full path to the driver eg: c:\\temp...")]
    path: PathBuf,
}

impl Runnable for Byovd {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if !is_administrator()? {
            return Ok(());
        }

        unsafe {
            let service_manager: Owned<SC_HANDLE> = Owned::new(OpenSCManagerW(
                PCWSTR::null(),
                PCWSTR::null(),
                SC_MANAGER_CREATE_SERVICE,
            )?);

            let service: Owned<SC_HANDLE> = Owned::new(CreateServiceW(
                *service_manager,
                &HSTRING::from(self.internal.as_str()),
                &HSTRING::from(self.display.as_str()),
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
