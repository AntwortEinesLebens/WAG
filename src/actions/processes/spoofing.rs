// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{actions::Runnable, windows::processes::get_pid};
use clap::Parser;
use std::{
    error::Error, ffi::OsString, iter::once, mem::size_of, os::windows::ffi::OsStrExt,
    path::PathBuf,
};
use windows::{
    core::{Owned, PWSTR},
    Win32::{
        Foundation::HANDLE,
        System::Threading::{
            CreateProcessW, InitializeProcThreadAttributeList, OpenProcess,
            UpdateProcThreadAttribute, EXTENDED_STARTUPINFO_PRESENT, LPPROC_THREAD_ATTRIBUTE_LIST,
            PROCESS_CREATE_PROCESS, PROCESS_INFORMATION, PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
            STARTUPINFOEXW, STARTUPINFOW,
        },
    },
};

#[derive(Debug, Parser)]
pub struct Spoofing {
    #[clap(required = true, help = "Path to the executable")]
    executable: PathBuf,
    #[clap(required = true, help = "Name of the parent executable")]
    parent_executable: String,
}

impl Runnable for Spoofing {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if !self.executable.try_exists()? || !self.executable.is_file() {
            return Ok(());
        }

        let mut required_size: usize = 0;

        unsafe {
            let _ = InitializeProcThreadAttributeList(
                LPPROC_THREAD_ATTRIBUTE_LIST::default(),
                1,
                0,
                &mut required_size,
            );
        };

        let mut attributes: Box<[u8]> = vec![0; required_size].into_boxed_slice();
        let attributes_list: Owned<LPPROC_THREAD_ATTRIBUTE_LIST> = unsafe {
            Owned::new(LPPROC_THREAD_ATTRIBUTE_LIST(
                attributes.as_mut_ptr() as *mut _
            ))
        };
        let startup_informations: STARTUPINFOEXW = STARTUPINFOEXW {
            StartupInfo: STARTUPINFOW {
                cb: size_of::<STARTUPINFOEXW>() as u32,
                ..Default::default()
            },
            lpAttributeList: *attributes_list,
        };

        unsafe {
            InitializeProcThreadAttributeList(
                startup_informations.lpAttributeList,
                1,
                0,
                &mut required_size,
            )?;

            let mut parent_process: Owned<HANDLE> = Owned::new(OpenProcess(
                PROCESS_CREATE_PROCESS,
                false,
                get_pid(self.parent_executable.as_str())?,
            )?);
            UpdateProcThreadAttribute(
                startup_informations.lpAttributeList,
                0,
                PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
                Some(&mut *parent_process as *mut _ as *mut _),
                size_of::<HANDLE>(),
                None,
                None,
            )?;

            CreateProcessW(
                None,
                PWSTR(
                    OsString::from(self.executable.as_os_str())
                        .encode_wide()
                        .chain(once(0))
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                ),
                None,
                None,
                false,
                EXTENDED_STARTUPINFO_PRESENT,
                None,
                None,
                &startup_informations.StartupInfo,
                &mut PROCESS_INFORMATION::default(),
            )?;
        };

        Ok(())
    }
}
