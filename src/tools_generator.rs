//
// Artefact generator
//

// Windows API
use windows::core::{Result, PCSTR, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GENERIC_WRITE, HANDLE};
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::Storage::FileSystem::{
    CreateFileA, WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_WRITE,
    PIPE_ACCESS_DUPLEX,
};
use windows::Win32::System::Pipes::{CreateNamedPipeA, PIPE_TYPE_MESSAGE};
use windows::Win32::System::Services::{
    ControlService, CreateServiceW, DeleteService, OpenSCManagerW, StartServiceW,
    ENUM_SERVICE_TYPE, SC_MANAGER_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_ERROR,
    SERVICE_START_TYPE,
};
use windows::Win32::UI::Shell::IsUserAnAdmin;

// Some others
use std::{thread, time};

// For regex to string
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};

pub fn create_name_pipe(name: &String, wait: u64) {
    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0", name);
    let pipe_name: PCSTR = PCSTR::from_raw(full_malware_pipe.as_ptr());
    let server_pipe: Result<HANDLE> = unsafe {
        CreateNamedPipeA(
            pipe_name,
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE,
            1,
            2048,
            2048,
            0,
            None,
        )
    };
    let sleep_duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe = unsafe { CloseHandle(server_pipe.unwrap()) };
}

pub fn create_driver_service(name: String, details: String, path: String) -> bool {
    println!("Open the service manager");
    let scmanager =
        unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ALL_ACCESS) }
            .expect("Sc Manager open failure");

    let mut service_name: Vec<u16> = name.encode_utf16().collect();
    service_name.push(0);
    let mut service_display: Vec<u16> = details.encode_utf16().collect();
    service_display.push(0);
    let mut service_path: Vec<u16> = path.encode_utf16().collect();
    service_path.push(0);

    println!("Create the service manager");

    let service_handle: SC_HANDLE = match unsafe {
        CreateServiceW(
            scmanager,
            PCWSTR::from_raw(service_name.as_ptr()),
            PCWSTR::from_raw(service_display.as_ptr()),
            0xF003F,
            ENUM_SERVICE_TYPE(1),
            SERVICE_START_TYPE(2),
            SERVICE_ERROR(0),
            PCWSTR::from_raw(service_path.as_ptr()),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )
    } {
        Ok(value) => value,
        Err(_) => {
            println!("Service creation failure");
            return false;
        }
    };

    println!("Start Service ");

    match unsafe { StartServiceW(service_handle, None) } {
        Ok(_) => {
            println!("Wait a little");
            let sleep_duration = time::Duration::from_millis(2000);
            thread::sleep(sleep_duration);
            let mut service_status = unsafe { std::mem::zeroed() };
            println!("Stop Service");
            let _result_stop = unsafe {
                ControlService(service_handle, SERVICE_CONTROL_STOP, &mut service_status)
            };
        }
        Err(value) => {
            // let error_code = unsafe { GetLastError() };
            println!("Service Start failure with code : {:#06x}", value.code().0);
        }
    };

    match unsafe { DeleteService(service_handle) } {
        Ok(_) => {
            println!("Service remove succeed");
            return true;
        }
        Err(value) => {
            println!("Service remove failure with code : {:#06x}", value.code().0);
            return false;
        }
    }
}

// File Creation
pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

pub fn create_file(fullpath: String, hex_data: Vec<u8>) -> bool {
    println!("Try to create : {}", fullpath);
    let file_path = std::path::Path::new(&fullpath);
    if !file_path.exists() {
        let folder = file_path.parent().unwrap();

        let ret_folder = std::fs::create_dir_all(folder);
        match ret_folder {
            Ok(_) => println!("The folder is valid"),
            Err(_) => return false,
        }

        let ret_file = std::fs::write(file_path, hex_data);
        match ret_file {
            Ok(_) => println!("The file is created"),
            Err(_) => return false,
        }

        let sleep_duration = time::Duration::from_millis(2000);
        thread::sleep(sleep_duration);

        let ret_remove = std::fs::remove_file(file_path);
        match ret_remove {
            Ok(_) => println!("The file is removed"),
            Err(_) => return false,
        }

        return true;
    }
    return false;
}

pub fn create_ads(fullpath: String, adsname: String, hex_data: Vec<u8>) -> bool {
    let file_path = format!("{}:{}\0", fullpath, adsname);
    println!("ads: {}", file_path);

    let handle = unsafe {
        CreateFileA(
            PCSTR::from_raw(file_path.as_ptr()),
            GENERIC_WRITE.0,
            FILE_SHARE_WRITE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    }
    .unwrap();

    let result = unsafe {
        WriteFile(
            handle,
            Some(hex_data.as_slice()),
            Some(hex_data.len() as *mut u32),
            None,
        )
    };

    let _ = unsafe { CloseHandle(handle) };

    match result {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

/*
Some usefull fn
*/
pub fn regex_to_string(name: &String) -> String {
    let mut gen = Generator::new(name, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer = vec![];
    gen.generate(&mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    return output;
}

pub fn process_is_admin() -> bool {
    return unsafe { IsUserAnAdmin().into() };
}
