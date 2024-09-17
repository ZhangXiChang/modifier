use std::{ffi::c_void, iter, rc::Rc};

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

pub struct Module {
    value: HMODULE,
    process: Rc<HANDLE>,
}
impl Module {
    pub fn get_name(&self) -> Result<String> {
        let mut module_name_u8maxpathbuffer = [0; MAX_PATH as usize];
        let module_name_u8buffer_size = unsafe {
            GetModuleBaseNameA(*self.process, self.value, &mut module_name_u8maxpathbuffer)
        };
        Ok(String::from_utf8(
            module_name_u8maxpathbuffer[..module_name_u8buffer_size as usize].to_vec(),
        )?)
    }
    pub fn get_base_address(&self) -> Result<usize> {
        let mut module_info = MODULEINFO::default();
        unsafe {
            GetModuleInformation(
                *self.process,
                self.value,
                &mut module_info,
                size_of_val(&module_info) as u32,
            )?;
            Ok(module_info.lpBaseOfDll as usize)
        }
    }
}
impl Drop for Module {
    fn drop(&mut self) {
        unsafe { self.value.free() };
    }
}

pub struct Process {
    value: Rc<HANDLE>,
}
impl Process {
    pub fn new(pid: u32) -> Result<Self> {
        Ok(Self {
            value: Rc::new(unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? }),
        })
    }
    pub fn get_module_list(&self) -> Result<Vec<Module>> {
        let mut module_list_u81024buffer = [HMODULE::default(); 1024];
        let mut module_list_u8buffer_size = 0;
        unsafe {
            EnumProcessModules(
                *self.value,
                module_list_u81024buffer.as_mut_ptr(),
                size_of_val(&module_list_u81024buffer) as u32,
                &mut module_list_u8buffer_size,
            )?
        };
        Ok(
            module_list_u81024buffer[..module_list_u8buffer_size as usize / size_of::<HMODULE>()]
                .iter()
                .map(|module| Module {
                    value: module.clone(),
                    process: self.value.clone(),
                })
                .collect(),
        )
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(*self.value) };
    }
}

pub struct ProcessEntry {
    value: PROCESSENTRY32,
}
impl ProcessEntry {
    pub fn get_pid(&self) -> u32 {
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
        Ok(Process::new(self.get_pid())?)
    }
}

pub struct SystemSnapshot {
    value: HANDLE,
}
impl SystemSnapshot {
    pub fn new() -> Result<Self> {
        Ok(Self {
            value: unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? },
        })
    }
    pub fn process_entry_iter(&self) -> impl Iterator<Item = ProcessEntry> {
        iter::from_fn({
            let value = self.value.clone();
            move || {
                let mut process_entry = PROCESSENTRY32::default();
                process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
                match unsafe { Process32Next(value, &mut process_entry) } {
                    Ok(_) => Some(ProcessEntry {
                        value: process_entry,
                    }),
                    Err(_) => None,
                }
            }
        })
    }
}
impl Drop for SystemSnapshot {
    fn drop(&mut self) {
        unsafe { self.value.free() };
    }
}
