//
// Artefact generator
//

// Windows API
use widestring::U16CString;
use winapi::shared::minwindef::{DWORD,LPVOID};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess,OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE};
use winapi::um::winbase::CreateNamedPipeA;
use winapi::um::winnt::*;
use winapi::um::winsvc::*;
//use winapi::um::winsvc::{ControlService, DeleteService, OpenSCManagerW, StartServiceW, CreateServiceW,SC_MANAGER_ALL_ACCESS,SERVICE_CONTROL_STOP};



// Some others
use std::{mem,thread, time};
use std::ptr::null_mut;

// For regex to string
use regex_generate::{DEFAULT_MAX_REPEAT, Generator};

pub fn create_name_pipe(name:&String,wait:u64) {
    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0",name);
    let pipe_name : LPCSTR = full_malware_pipe.as_ptr() as *const i8;
    let server_pipe : HANDLE = unsafe {CreateNamedPipeA(pipe_name,PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,1,2048,2048,0,null_mut())};
    let sleep_duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe = unsafe { CloseHandle(server_pipe) };
}


fn open_sc_manager(desired_access: DWORD) -> Result<*mut SC_HANDLE__, DWORD> {
    let sc_manager_handle = unsafe { OpenSCManagerW(null_mut(), null_mut(), desired_access) };
    if sc_manager_handle.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(sc_manager_handle)
    }
}

pub fn create_driver_service(name:String,details:String,path:String) -> bool {
    println!("Open the service manager");
    let scmanager = open_sc_manager(SC_MANAGER_ALL_ACCESS).expect("Sc Manager open failure");


    let service_name = U16CString::from_str(name).unwrap();
    let service_display = U16CString::from_str(details).unwrap();
    let service_path = U16CString::from_str(path).unwrap();

    println!("Create the service manager");
    let service_handle= unsafe { CreateServiceW(scmanager,
        service_name.as_ptr(),
        service_display.as_ptr(),
        0xF003F,
        1,
        2,
        0,
        service_path.as_ptr(),
        std::ptr::null(),
        std::ptr::null_mut(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null()
    ) };

    if service_handle.is_null() {
        println!("Service creation failure");
        return false
    }

    println!("Start Service ");
    let result = unsafe { StartServiceW(service_handle, 0, null_mut()) };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        println!("Service Start failure with code : {:#06x}",error_code);
    } else {
        println!("Wait a little");
        let sleep_duration = time::Duration::from_millis(2000);//2s
        thread::sleep(sleep_duration); 
        let mut service_status = unsafe { std::mem::zeroed() };
        println!("Stop Service");
        let _result_stop = unsafe { ControlService(service_handle,SERVICE_CONTROL_STOP,&mut service_status,)};
    }

    let result_delete = unsafe { DeleteService(service_handle) };

    if result_delete == 0 {
        let error_code = unsafe { GetLastError() };
        println!("Service remove failure with code : {:#06x}",error_code);
        return false
    } else {
        println!("Service remove succeed");
        return true
    }

}

// File Creation
pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                      .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

pub fn create_file(fullpath:String,hex_data:Vec<u8>) -> bool{
    println!("Try to create : {}",fullpath);
    let file_path= std::path::Path::new(&fullpath);
    if !file_path.exists() {
        let folder = file_path.clone().parent().unwrap();

        let ret_folder = std::fs::create_dir_all(folder);
        match ret_folder{
            Ok(_) => println!("The folder is valid"),
            Err(_) => return false,
        }

        let ret_file = std::fs::write(file_path, hex_data);
        match ret_file{
            Ok(_) => println!("The file is created"),
            Err(_) => return false,
        }

        let sleep_duration = time::Duration::from_millis(2000);//2s
        thread::sleep(sleep_duration);

        let ret_remove = std::fs::remove_file(file_path);
        match ret_remove{
            Ok(_) => println!("The file is removed"),
            Err(_) => return false,
        }       

        return true;
    }
    return false;
}

pub fn create_ads(fullpath:String,adsname:String,hex_data:Vec<u8>) -> bool{
    let file_path= std::path::Path::new(&fullpath);
    if file_path.exists() {
        let ads_file_name = format!("{}:{}", fullpath, adsname);
        let ret_file = std::fs::write(ads_file_name, hex_data);
        match ret_file{
            Ok(_) => println!("The ADS is created"),
            Err(_) => return false,
        }

        let sleep_duration = time::Duration::from_millis(2000);//2s
        thread::sleep(sleep_duration);

      /*   let ret_remove = std::fs::remove_file(file_path);
        match ret_remove{
            Ok(_) => println!("The base file is removed"),
            Err(_) => return false,
        }*/
        
        return true;
    }

    return false;
}

/*
Some usefull fn
*/
pub fn regex_to_string(name:&String) -> String {
    let mut gen = Generator::new(name, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer = vec![];
    gen.generate(&mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    return output;
}

pub fn process_is_admin()-> bool {
        // from  https://github.com/yandexx/is_elevated
        unsafe {
            let mut current_token_ptr: HANDLE = mem::zeroed();
            let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
            let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
            let mut size: DWORD = 0;
    
            let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr);
    
            if result != 0 {
                let result = GetTokenInformation(
                    current_token_ptr,
                    TokenElevation,
                    token_elevation_type_ptr as LPVOID,
                    mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION_TYPE>() as u32,
                    &mut size,
                );
                if result != 0 {
                    return token_elevation.TokenIsElevated != 0;
                }
            }
        }
        false
}