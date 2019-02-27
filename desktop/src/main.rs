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
use std::env;
use std::fs::create_dir;

const TYPE_FULL: i32 = 0; //整幅图
const TYPE_HALF: i32 = 1; //半副图

static ICON: &[u8] = include_bytes!("../icon.ico");
pub struct Config {
    freq: i32,
    show_type: i32,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            freq: 30,
            show_type: TYPE_FULL,
        }
    }
}

fn init_dir() -> Config {
    //设置临时文件夹为当前文件夹
    let tmp_dir = {
        let mut d = env::temp_dir();
        d.push("himawari-8-wallpaper");
        d
    };
    if !tmp_dir.exists() {
        let _ = create_dir(tmp_dir.clone());
    }
    println!("current_dir={:?}", tmp_dir);
    if let Err(err) = env::set_current_dir(tmp_dir) {
        println!("当前工作文件夹设置失败:{:?}", err);
    }
    //解压ico文件
    if cfg!(windows) {
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;
        if !Path::new("icon.ico").exists() {
            match File::create("icon.ico") {
                Ok(mut file) => {
                    let _ = file.write_all(ICON);
                }
                Err(err) => {
                    println!("icon.ico创建失败:{:?}", err);
                }
            }
        }
    }
    //读取配置文件
    if let Ok(conf) = ini::Ini::load_from_file("conf.ini") {
        Config {
            freq: conf
                .get_from(Some("def"), "freq")
                .unwrap_or("10")
                .parse::<i32>()
                .unwrap_or(10),
            show_type: conf
                .get_from(Some("def"), "show_type")
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0),
        }
    } else {
        Config::default()
    }
}

pub fn write_config(conf: &Config) {
    let mut ini = if let Ok(ini) = ini::Ini::load_from_file("conf.ini") {
        ini
    } else {
        ini::Ini::new()
    };
    ini.with_section(Some("def"))
        .set("freq", format!("{}", conf.freq))
        .set("show_type", format!("{}", conf.show_type));
    let _ = ini.write_to_file("conf.ini");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn WinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    szCmdLine: LPSTR,
    iCmdShow: i32,
) -> i32 {
    windows::win_main(hInstance, hPrevInstance, szCmdLine, iCmdShow, init_dir())
}

// #[cfg(windows)]
// fn main() {
//     windows::win_main(
//         std::ptr::null_mut(),
//         std::ptr::null_mut(),
//         std::ptr::null_mut(),
//         0,
//         init_dir(),
//     );
// }

#[cfg(not(windows))]
fn main() {
    wallpaper::set_full().unwrp();
    println!("程序结束.");
}
