//#![no_main]
mod himawari8;
mod wallpaper;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use winapi::shared::{minwindef::HINSTANCE, ntdef::LPSTR};
#[macro_use]
extern crate lazy_static;
use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
extern crate wallpaper as wp;

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
#[cfg(windows)]
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

pub fn absolute_path<P>(path: P) -> std::io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

//设置壁纸
pub fn set_wallpaper(
    wallpaper: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Result<(), Box<std::error::Error>> {
    wallpaper.save("wallpaper.png")?;
    if let Some(path) = absolute_path("wallpaper.png")?.to_str() {
        wp::set_from_path(path)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "壁纸设置失败",
        )))
    }
}

#[cfg(not(windows))]
fn main() -> Result<(), Box<std::error::Error>> {
    let mut conf = init_dir();
    use std::env;
    use std::{thread, time};

    for argument in env::args() {
        if argument == "freq10" {
            conf.freq = 10;
        } else if argument == "freq20" {
            conf.freq = 20;
        } else if argument == "freq30" {
            conf.freq = 30;
        } else if argument == "freq60" {
            conf.freq = 60;
        } else if argument == "mode0" {
            conf.show_type = 0;
        } else if argument == "mode1" {
            conf.show_type = 1;
        }
    }

    let (mut screen_width, mut screen_height) = (1920, 1200);

    //if cfg!(target_os = "linux")
    let _ = {
        use std::process::Command;
        let dim = String::from_utf8(
            Command::new("sh")
                .arg("-c")
                .arg("xdpyinfo | grep dimensions")
                .output()?
                .stdout,
        )?;

        for item in dim.split(" ") {
            if item.contains("x") && item != "pixels" && !item.contains("(") && !item.contains(")")
            {
                let mut dim = item.split("x");
                screen_width = dim.next().unwrap().parse::<i32>().unwrap();
                screen_height = dim.next().unwrap().parse::<i32>().unwrap();
            }
        }
    };

    println!("屏幕分辨率:{}x{}", screen_width, screen_height);

    loop {
        if conf.show_type == TYPE_HALF {
            if let Err(err) =
                wallpaper::set_half(screen_width, screen_height, |current: i32, total: i32| {
                    println!("下载壁纸{}/{}", current, total);
                })
            {
                println!("壁纸下载失败:{:?}", err);
            }
        } else if conf.show_type == TYPE_FULL {
            if let Err(err) =
                wallpaper::set_full(screen_width, screen_height, |current: i32, total: i32| {
                    println!("下载壁纸{}/{}", current, total);
                })
            {
                println!("壁纸下载失败:{:?}", err);
            }
        }
        //延时切换壁纸
        thread::sleep(time::Duration::from_millis(conf.freq as u64 * 60 * 1000));
    }
}
