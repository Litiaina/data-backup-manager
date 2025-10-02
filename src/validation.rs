use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};

use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
use winapi::um::{errhandlingapi::GetLastError, synchapi::CreateMutexW};

pub fn check_single_instance(mutex_name: &str) -> bool {
    let wide_name: Vec<u16> = OsStr::new(mutex_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let handle = CreateMutexW(null_mut(), 0, wide_name.as_ptr());
        if handle.is_null() {
            return false;
        }
        GetLastError() != ERROR_ALREADY_EXISTS
    }
}
