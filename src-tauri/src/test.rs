extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::winuser::{FindWindowA, SetWindowPos};

pub fn find_window() {
  unsafe {
    let window_name = CString::new("RestLoop-Second-Blocker").unwrap();
    let hwnd = FindWindowA(std::ptr::null_mut(), window_name.as_ptr());
    println!("find window: {:#?}", hwnd);
    SetWindowPos(hwnd, null_mut(), -100, 0, 500, 500, 1);
  }
}
