#![no_main]
mod himawari8;
mod wallpaper;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use winapi::shared::{ntdef::LPSTR, minwindef::{HINSTANCE}};

use std::time::Duration;
use std::thread;
use std::sync::mpsc::channel;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain(hInstance: HINSTANCE, hPrevInstance:HINSTANCE, szCmdLine:LPSTR, iCmdShow: i32) -> i32 {
    windows::win_main(hInstance, hPrevInstance, szCmdLine, iCmdShow)
}

#[cfg(not(windows))]
fn main() {
    wallpaper::set_full().unwrp();
    println!("程序结束.");
}