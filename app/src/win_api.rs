use std::{iter, rc::Rc, sync::Mutex};

use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{
            Diagnostics::{Debug::*, ToolHelp::*},
            ProcessStatus::*,
            Threading::*,
        },
    },
};

use crate::lock::{MutexLock, Pointer};

pub struct Process {
    value: Pointer<Mutex<HANDLE>>,
}
impl Process {}
impl Drop for Process {
    fn drop(&mut self) {
        let _ = unsafe { self.value.lock().free() };
    }
}

pub struct ProcessEntry {
    value: Rc<PROCESSENTRY32>,
}
impl ProcessEntry {
    pub fn get_process_id(&self) -> u32 {
        self.value.th32ProcessID
    }
    pub fn get_exe_file_name(&self) -> Result<String> {
        Ok(
            unsafe { std::ffi::CStr::from_ptr(self.value.szExeFile.as_ptr()) }
                .to_str()?
                .to_string(),
        )
    }
    pub fn open_process(&self) -> Result<Process> {
        Ok(Process {
            value: Pointer::new_mutex(unsafe {
                OpenProcess(PROCESS_ALL_ACCESS, false, self.get_process_id())?
            }),
        })
    }
}

pub struct SystemSnapshot {
    value: Pointer<Mutex<HANDLE>>,
}
impl SystemSnapshot {
    pub fn new() -> Result<Self> {
        Ok(Self {
            value: Pointer::new_mutex(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? }),
        })
    }
    pub fn process_entry_iter(&self) -> impl Iterator<Item = ProcessEntry> {
        iter::from_fn({
            let value = self.value.clone();
            move || {
                let mut process_entry = PROCESSENTRY32::default();
                process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
                match unsafe { Process32Next(*value.lock(), &mut process_entry) } {
                    Ok(_) => Some(ProcessEntry {
                        value: Rc::new(process_entry),
                    }),
                    Err(_) => None,
                }
            }
        })
    }
}
impl Drop for SystemSnapshot {
    fn drop(&mut self) {
        let _ = unsafe { self.value.lock().free() };
    }
}

// pub fn get_process_module_info_list(process: HANDLE) -> Result<Vec<ModuleInfo>> {
//     unsafe {
//         let mut process_module_info_list = Vec::new();
//         let mut process_module_list_u81024buffer = [HMODULE::default(); 1024];
//         let mut process_module_list_u8buffer_size = 0;
//         EnumProcessModules(
//             process,
//             process_module_list_u81024buffer.as_mut_ptr(),
//             size_of_val(&process_module_list_u81024buffer) as u32,
//             &mut process_module_list_u8buffer_size,
//         )?;
//         let process_module_list = &process_module_list_u81024buffer
//             [..process_module_list_u8buffer_size as usize / size_of::<HMODULE>()];
//         for process_module in process_module_list {
//             let mut module_name_u8maxpathbuffer = [0; MAX_PATH as usize];
//             let module_name_u8buffer_size =
//                 GetModuleBaseNameA(process, *process_module, &mut module_name_u8maxpathbuffer);
//             let mut module_info = MODULEINFO::default();
//             GetModuleInformation(
//                 process,
//                 *process_module,
//                 &mut module_info,
//                 size_of_val(&module_info) as u32,
//             )?;
//             process_module_info_list.push(ModuleInfo {
//                 name: String::from_utf8(
//                     module_name_u8maxpathbuffer[..module_name_u8buffer_size as usize].to_vec(),
//                 )?,
//                 address: module_info.lpBaseOfDll as usize,
//             });
//         }
//         Ok(process_module_info_list)
//     }
// }
