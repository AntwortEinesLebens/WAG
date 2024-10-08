// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use windows::{
    core::Error as WindowsError,
    Win32::{
        Foundation::BOOL,
        Security::{
            AllocateAndInitializeSid, CheckTokenMembership, FreeSid, PSID, SECURITY_NT_AUTHORITY,
        },
        System::SystemServices::{DOMAIN_ALIAS_RID_ADMINS, SECURITY_BUILTIN_DOMAIN_RID},
    },
};

pub fn is_administrator() -> Result<bool, WindowsError> {
    let is_admin: *mut BOOL = &mut BOOL::from(false);
    let mut administrators_group: PSID = PSID::default();

    unsafe {
        AllocateAndInitializeSid(
            &SECURITY_NT_AUTHORITY,
            2,
            SECURITY_BUILTIN_DOMAIN_RID as u32,
            DOMAIN_ALIAS_RID_ADMINS as u32,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut administrators_group,
        )?;

        let result: Result<(), WindowsError> =
            CheckTokenMembership(None, administrators_group, is_admin);

        FreeSid(administrators_group);

        result?;
    }

    Ok(unsafe { (*is_admin).into() })
}
