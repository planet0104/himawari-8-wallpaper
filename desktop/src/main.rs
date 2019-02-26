#![no_main]
mod himawari8;
mod wallpaper;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use winapi::shared::{minwindef::HINSTANCE, ntdef::LPSTR};
#[cfg(windows)]
#[macro_use]
extern crate lazy_static;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn WinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    szCmdLine: LPSTR,
    iCmdShow: i32,
) -> i32 {
    windows::win_main(hInstance, hPrevInstance, szCmdLine, iCmdShow)
}

// #[cfg(windows)]
// fn main() {
//     windows::win_main(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), 0);
// }

#[cfg(not(windows))]
fn main() {
    wallpaper::set_full().unwrp();
    println!("程序结束.");
}
