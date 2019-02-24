use image::GenericImageView;
use image::{ImageBuffer, Rgb};
use wallpaper;
use chrono::{Datelike, NaiveDateTime, Timelike, DateTime, Utc};
use crate::himawari8;
use std::env;
use std::io;
use std::path::{PathBuf, Path};

fn get_wallpaper_info() -> Result<(u32, u32), Box<std::error::Error>>{
    let path = wallpaper::get()?;
    let image = image::open(path)?;
    Ok(image.dimensions())
}

fn download_4x4() -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<std::error::Error>>{
    let mut timestamp = Utc::now().timestamp_millis();
    //减去16分钟
    timestamp -= 16*60*1000;
    let utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp/1000, 0), Utc);
    println!("时间:{:?}", utc);
    //20分钟之前的
    let file_name = format!("{}_{}_{}_{}_{}.png", utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute()/10);
    if Path::new(&file_name).exists(){
        Ok(image::open(file_name)?.to_rgb())
    }else{
        let image = himawari8::combine_4x4(utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute())?;
        image.save(file_name)?;
        Ok(image)
    }
}

//整幅画
pub fn set_full() -> Result<(), Box<std::error::Error>>{
    let image = download_4x4()?;
    println!("拼接完成:{:?}", image.dimensions());
    
    let mut width = 1920;
    let mut height = 1200;
    //按照桌面最窄缩放
    if let Ok((w, h)) = get_wallpaper_info(){
        width = w;
        height = h;
    }
    let min = (std::cmp::min(width, height) as f64*0.87) as u32;
    let image = image::imageops::resize(&image, min, min, image::FilterType::Nearest);
    println!("缩放完成:{:?} 背景图大小:{}x{}", image.dimensions(), width, height);
    // image.save("image.png").unwrap();

    let mut wallpaper:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    //拼接
    let offset_x = ((wallpaper.width()-image.width())/2) as usize;
    let offset_y = ((wallpaper.height()-image.height())/4) as usize;
    let ew = image.width() as usize;
    let image = image.into_raw();
    for (y, buf) in image.chunks(ew*3).enumerate(){
        let offset = width as usize*3*(y+offset_y)+offset_x*3;
        wallpaper.get_mut(offset..offset+buf.len()).unwrap().copy_from_slice(buf);
    }

    wallpaper.save("wallpaper.png")?;
    wallpaper::set_from_path(absolute_path("wallpaper.png")?.to_str().unwrap())?;

    Ok(())
}

pub fn absolute_path<P>(path: P) -> io::Result<PathBuf> where P: AsRef<Path>{
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

//上半边
pub fn set_half() -> Result<(), Box<std::error::Error>>{
    let image = download_4x4()?;
    println!("拼接完成:{:?}", image.dimensions());
    
    let mut width = 1920;
    let mut height = 1200;
    if let Ok((w, h)) = get_wallpaper_info(){
        width = w;
        height = h;
    }
    //将图片缩放到桌面宽度大小
    let size = (width as f64*1.0) as u32;
    let image = image::imageops::resize(&image, size, size, image::FilterType::Nearest);
    println!("缩放完成:{:?} 背景图大小:{}x{}", image.dimensions(), width, height);
    // image.save("image_half.png")?;

    let mut wallpaper:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    //拼接
    let offset_x = ((wallpaper.width()-image.width())/2) as usize;;
    let offset_y = (height as f64 * 0.03) as usize;
    let ew = image.width() as usize;
    let image = image.into_raw();
    for (y, buf) in image.chunks(ew*3).enumerate(){
        if (y+offset_y)<wallpaper.height() as usize{
            let offset = width as usize*3*(y+offset_y)+offset_x*3;
            wallpaper.get_mut(offset..offset+buf.len()).unwrap().copy_from_slice(buf);
        }else{
            break;
        }
    }

    wallpaper.save("wallpaper.png")?;
    wallpaper::set_from_path(absolute_path("wallpaper.png")?.to_str().unwrap())?;

    Ok(())
}