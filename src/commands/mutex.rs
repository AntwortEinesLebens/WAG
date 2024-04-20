//
// Mutex
//
// Last update 20240224

use windows::core::{Result as WindowsResult, PCSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::CreateMutexA;

use crate::commands::tools::{regex_to_string, EXIST_ALL_GOOD};
use clap::Parser;
use std::{thread, time};

#[derive(Parser)]
pub struct Mutex {
    #[clap(
        short = 'n',
        long,
        required = true,
        default_value = "wag",
        help = "Regex of the Mutex to Create"
    )]
    name: String,
}

fn create_mutex(name: &String, wait: u64) {
    let full_malware_mutex: String = format!("{}\0", name);
    let mutex_name: PCSTR = PCSTR::from_raw(full_malware_mutex.as_ptr());
    let mutex_handle: WindowsResult<HANDLE> = unsafe { CreateMutexA(None, true, mutex_name) };
    let sleep_duration: time::Duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe: WindowsResult<()> = unsafe { CloseHandle(mutex_handle.unwrap()) };
}

impl Mutex {
    pub fn run(&self) -> i32 {
        println!("Create Mutex");
        let full_payload: String;
        full_payload = regex_to_string(&self.name);
        println!("Create the Mutex : {}", full_payload);
        create_mutex(&full_payload, 2000);
        return EXIST_ALL_GOOD;
    }
}
