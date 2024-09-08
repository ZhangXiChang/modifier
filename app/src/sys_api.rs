use anyhow::Result;
use windows::Win32::{
    Foundation::*,
    System::{Diagnostics::ToolHelp::*, ProcessStatus::*, Threading::*},
};

#[derive(Debug)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub exe_file_name: String,
}

pub fn get_all_process_info() -> Result<Vec<ProcessInfo>> {
    let mut all_process_info_list = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        loop {
            let mut process_entry = PROCESSENTRY32::default();
            process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
            match Process32Next(snapshot, &mut process_entry) {
                Ok(_) => all_process_info_list.push(ProcessInfo {
                    process_id: process_entry.th32ProcessID,
                    exe_file_name: std::ffi::CStr::from_ptr(process_entry.szExeFile.as_ptr())
                        .to_str()?
                        .to_string(),
                }),
                Err(_) => break,
            }
        }
        CloseHandle(snapshot)?;
    }
    Ok(all_process_info_list)
}

pub fn test(process_id: u32) -> Result<()> {
    unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, false, process_id)?;
        let mut modules_u81024buffer = [HMODULE::default(); 1024];
        let mut modules_u8buffer_size = 0;
        EnumProcessModules(
            process,
            modules_u81024buffer.as_mut_ptr(),
            size_of_val(&modules_u81024buffer) as u32,
            &mut modules_u8buffer_size,
        )?;
        let modules_buffer =
            &modules_u81024buffer[..modules_u8buffer_size as usize / size_of::<HMODULE>()];
        for module in modules_buffer {
            let mut module_name_u8maxpathbuffer = [0; MAX_PATH as usize];
            let module_name_u8buffer_size =
                GetModuleBaseNameA(process, *module, &mut module_name_u8maxpathbuffer);
            if module_name_u8buffer_size > 0 {
                let module_name = String::from_utf8(
                    module_name_u8maxpathbuffer[..module_name_u8buffer_size as usize].to_vec(),
                )?;
                println!("{}", module_name);
            }
        }
        Ok(())
    }
}
