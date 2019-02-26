// #![no_main]
mod himawari8;
mod wallpaper;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use winapi::shared::{minwindef::HINSTANCE, ntdef::LPSTR};
#[cfg(windows)]
#[macro_use]
extern crate lazy_static;
use std::env;
use std::io::{Error, ErrorKind};
use std::fs::create_dir;

static ICON:&[u8] = include_bytes!("../icon.ico");

fn init_dir() -> Result<String, Box<std::error::Error>>{
    //获取临时文件夹或者当前文件夹
    let tmp_dir = match {
        //首先读取临时文件夹
        let tmp_dir = { let mut d = env::temp_dir(); d.push("himawari-8-wallpaper"); d};
        if !tmp_dir.exists(){
            create_dir(tmp_dir.clone())?;
        }
        
        match tmp_dir.to_str(){
            Some(path) => Ok(path.to_string()),
            None => Err(Box::new(Error::new(ErrorKind::Other, "临时文件夹创建失败!")))
        }
    }{
        Ok(dir) => dir,
        Err(err) => {
            println!("临时文件夹创建失败:{:?}", err);
            //临时文件夹读取失败，读取当前文件夹
            match env::current_dir()?.to_str(){
                Some(path) => path.to_string(),
                None => return Err(Box::new(Error::new(ErrorKind::Other, "临时文件夹创建失败!")))
            }
        }
    };
    println!("tmp_dir={}", tmp_dir);
    //解压ico文件
    if cfg!(windows){
        use std::io::Write;
        use std::fs::File;
        match File::create(format!("{}\\icon.ico", tmp_dir)){
            Ok(mut file) => {
                let _ = file.write_all(ICON);
            },
            Err(err) => {
                println!("icon.ico创建失败:{:?}", err);
            }
        }
    }
    
    Ok(tmp_dir)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn WinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    szCmdLine: LPSTR,
    iCmdShow: i32,
) -> i32 {
    if let Ok(path) = init_dir(){
        windows::win_main(hInstance, hPrevInstance, szCmdLine, iCmdShow, path)
    }else{
        windows::alert(windows::APP_NAME, "临时文件夹创建失败，请重试！");
        0
    }
}

#[cfg(windows)]
fn main() {
    if let Ok(path) = init_dir(){
        windows::win_main(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), 0, path);
    }else{
        windows::alert(windows::APP_NAME, "临时文件夹创建失败，请重试！");
    }
}

#[cfg(not(windows))]
fn main() {
    wallpaper::set_full().unwrp();
    println!("程序结束.");
}
