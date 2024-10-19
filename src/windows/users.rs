// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use windows::{
    core::{Owned, Result as WindowsResult},
    Win32::{
        Security::{AllocateAndInitializeSid, CheckTokenMembership, PSID, SECURITY_NT_AUTHORITY},
        System::SystemServices::{DOMAIN_ALIAS_RID_ADMINS, SECURITY_BUILTIN_DOMAIN_RID},
    },
};

pub fn is_administrator() -> WindowsResult<bool> {
    let mut is_admin: bool = false;

    unsafe {
        let mut administrators_group: Owned<PSID> = Owned::new(PSID::default());

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
            &mut *administrators_group,
        )?;

        CheckTokenMembership(
            None,
            *administrators_group,
            &mut is_admin as *mut _ as *mut _,
        )?;
    }

    Ok(is_admin)
}
