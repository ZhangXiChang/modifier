use std::{ffi::c_void, iter};

use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{
            Diagnostics::{Debug::*, ToolHelp::*},
            Memory::*,
            Threading::*,
        },
    },
};

pub struct MemoryInfo {
    value: MEMORY_BASIC_INFORMATION,
}
impl MemoryInfo {
    fn get_base_address(&self) -> *const c_void {
        self.value.BaseAddress
    }
    fn get_region_size(&self) -> usize {
        self.value.RegionSize
    }
}

pub struct Process {
    value: HANDLE,
}
impl Process {
    pub fn new(pid: u32) -> Result<Self> {
        Ok(Self {
            value: unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? },
        })
    }
    pub fn get_memory_region_info_list(&self) -> Vec<MemoryInfo> {
        let mut memory_region_info_list = Vec::new();
        let mut offset_address = 0x0;
        let mut memory_info = MEMORY_BASIC_INFORMATION::default();
        while unsafe {
            VirtualQueryEx(
                self.value,
                Some(offset_address as *const _),
                &mut memory_info,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        } != 0
        {
            if !(memory_info.BaseAddress as usize > 0x70000000
                && (memory_info.BaseAddress as usize) < 0x80000000)
                && (memory_info.Protect & PAGE_EXECUTE_READWRITE
                    | PAGE_EXECUTE_WRITECOPY
                    | PAGE_READWRITE
                    | PAGE_WRITECOPY)
                    != PAGE_PROTECTION_FLAGS(0)
            {
                memory_region_info_list.push(MemoryInfo { value: memory_info });
            }
            offset_address = memory_info.BaseAddress as usize + memory_info.RegionSize;
        }
        memory_region_info_list
    }
    pub fn find_memory(&self, memory_info: MemoryInfo) -> Result<()> {
        let base_address = memory_info.get_base_address() as usize;
        let mut read_memory_buffer = Vec::<u8>::with_capacity(memory_info.get_region_size());
        for (offset, buf) in read_memory_buffer.iter_mut().enumerate() {
            unsafe {
                ReadProcessMemory(
                    self.value,
                    (base_address + offset) as *const _,
                    buf as *mut u8 as *mut _,
                    1,
                    None,
                )?
            };
        }
        println!("{:?}", read_memory_buffer);
        Ok(())
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        unsafe { self.value.free() };
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
